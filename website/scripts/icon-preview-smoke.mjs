import assert from 'node:assert/strict';
import { join } from 'node:path';
import { waitForIconPreview } from './icon-source-smoke.mjs';

export async function checkIconPreviewRecovery(browser, base, screenshots) {
  for (const dark of [false, true]) for (const mobile of [false, true]) {
    const label = (mobile ? 'mobile' : 'desktop') + (dark ? '-dark' : '-light');
    const page = await browser.newPage({ viewport: mobile
      ? { width: 390, height: 844 } : { width: 1100, height: 750 } });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => {
      if (message.type() !== 'error') return;
      const text = message.text().trim();
      if (text !== 'Wasm assets loading, will be available soon...') errors.push(text);
    });
    let releaseInitial;
    const initialGate = new Promise(resolve => { releaseInitial = resolve; });
    let initialRequested;
    const initialSeen = new Promise(resolve => { initialRequested = resolve; });
    let releaseIntermediate;
    const intermediateGate = new Promise(resolve => { releaseIntermediate = resolve; });
    let intermediateRequested;
    const intermediateSeen = new Promise(resolve => { intermediateRequested = resolve; });
    try {
      await page.addInitScript(value => localStorage.setItem('gpui-antd-dark', value), dark ? '1' : '0');
      await page.route('**/demo.html?component=Icon%3ASearchOutlined*', async route => {
        initialRequested();
        await initialGate;
        await route.continue().catch(() => {}); // a newer selection can cancel it
      }, { times: 1 });
      await page.route('**/demo.html?component=Icon%3ACheckCircleFilled*', async route => {
        intermediateRequested();
        await intermediateGate;
        await route.continue().catch(() => {});
      }, { times: 1 });
      await page.goto(base + '#/components/Icons', { waitUntil: 'domcontentloaded' });
      await page.locator('#icon-source[href]').waitFor();
      await initialSeen;
      const frame = page.locator('#icon-live-preview');
      assert.equal(await frame.getAttribute('data-preview-state'), 'loading');
      assert.equal(await frame.evaluate(f => f.inert), true);
      assert.equal(await frame.evaluate(f => getComputedStyle(f).opacity), '0');
      // Change the desired selection while the initial real document is held.
      for (const name of ['CheckCircleFilled', 'CiTwoTone']) {
        await page.locator('#icon-query').fill(name);
        const hit = page.locator('button[data-icon-name="' + name + '"]');
        await hit.focus();
        if (name === 'CiTwoTone') {
          // Complete the prior GPUI startup between focus and Enter. Its IME
          // initialization must not redirect this key into the old canvas.
          releaseIntermediate();
          await waitForIconPreview(page);
          assert.equal(await hit.evaluate(el => el === document.activeElement), true);
        }
        await page.keyboard.press('Enter');
        if (name === 'CheckCircleFilled') await intermediateSeen;
      }
      releaseInitial();
      await waitForIconPreview(page);
      assert.ok((await frame.getAttribute('src')).includes('Icon%3ACiTwoTone'));
      const select = async name => {
        await page.locator('#icon-query').fill(name);
        await page.locator('button[data-icon-name="' + name + '"]').click();
      };
      const cancelNext = async name => {
        let aborted;
        const seen = new Promise(resolve => { aborted = resolve; });
        await page.route('**/demo.html?component=Icon%3A' + name + '*', async route => {
          await route.abort('aborted');
          aborted();
        }, { times: 1 });
        await select(name);
        await seen;
        await page.waitForFunction(() => {
          const f = document.querySelector('#icon-live-preview');
          return f.src !== f.contentDocument?.URL;
        });
        assert.equal(await frame.getAttribute('data-preview-state'), 'loading');
        assert.equal(await frame.evaluate(f => f.inert), true);
        assert.equal(await frame.evaluate(f => getComputedStyle(f).opacity), '0');
      };
      await cancelNext('CheckCircleTwoTone');
      // Same requested src, different actual document: reselect must navigate.
      const repeatedRequest = page.waitForRequest(request => request.isNavigationRequest() &&
        request.url().includes('Icon%3ACheckCircleTwoTone'));
      await select('CheckCircleTwoTone');
      await repeatedRequest;
      await waitForIconPreview(page);
      assert.ok((await page.locator('#icon-source').getAttribute('href')).endsWith('/twotone/check-circle.svg'));

      // Advance the real parent deadline without adding a minute to CI.
      await page.clock.install();
      await cancelNext('CarryOutTwoTone');
      await page.clock.fastForward(60_001);
      await page.waitForFunction(() => document.querySelector('#icon-live-preview')?.dataset.previewState === 'error');
      assert.match(await page.locator('#icon-preview-status').textContent(), /taking too long/);
      assert.equal(await page.locator('#icon-preview-retry').isVisible(), true);
      assert.equal(await frame.evaluate(f => f.inert), true);
      await page.screenshot({ path: join(screenshots, 'icon-preview-timeout-' + label + '.png') });
      await page.locator('#icon-preview-retry').focus();
      await page.keyboard.press('Enter');
      assert.equal(await page.locator('#icon-preview-status').evaluate(el => el === document.activeElement), true);
      await waitForIconPreview(page);
      assert.equal(await page.locator('#icon-preview-status').evaluate(el => el === document.activeElement), true);
      assert.equal(await frame.evaluate(f => f.inert), false);
      assert.equal(await frame.evaluate(f => getComputedStyle(f).opacity), '1');
      assert.equal(await page.locator('#icon-preview-retry').isVisible(), false);
      assert.equal(await page.locator('#icon-preview-status').textContent(), 'Preview ready');
      assert.ok((await frame.getAttribute('src')).includes('Icon%3ACarryOutTwoTone'));
      assert.deepEqual(errors, []);
      await page.screenshot({ path: join(screenshots, 'icon-preview-recovered-' + label + '.png') });
      console.log('PASS: GPUI icon delayed startup, cancelled navigation, reselection and timeout/retry (' + label + ')');
    } catch (error) {
      console.error('Icon recovery state:', await page.evaluate(() => {
        const f = document.querySelector('#icon-live-preview');
        return { target: f?.src, documentUrl: f?.contentDocument?.URL, state: f?.dataset.previewState,
          status: f?.contentDocument?.querySelector('#status')?.textContent,
          focused: document.activeElement?.outerHTML };
      }).catch(() => null));
      await page.screenshot({ path: join(screenshots, 'icon-preview-failed-' + label + '.png') }).catch(() => {});
      throw error;
    } finally {
      releaseInitial();
      releaseIntermediate();
      await page.close();
    }
  }
}
