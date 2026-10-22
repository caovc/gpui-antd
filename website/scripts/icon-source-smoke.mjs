import assert from 'node:assert/strict';
import { join } from 'node:path';

// Used by full smoke and the focused local QA run. Every preview remains
// the production Rust/WASM iframe; only provenance navigation is DOM UI.
export async function checkIconSources(browser, base, screenshots) {
  for (const dark of [false, true]) {
    const page = await browser.newPage({ viewport: dark
      ? { width: 390, height: 844 } : { width: 1100, height: 750 } });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => {
      if (message.type() !== 'error') return;
      const text = message.text().trim();
      if (text !== 'Wasm assets loading, will be available soon...') errors.push(text);
    });
    try {
      await page.addInitScript(value => localStorage.setItem('gpui-antd-dark', value), dark ? '1' : '0');
      const response = await page.goto(base + '#/components/Icons', { waitUntil: 'domcontentloaded' });
      assert.equal(response.status(), 200);
      assert.equal(new URL(page.url()).hash, '#/components/Icons');
      assert.match(await page.title(), /GPUI Ant Design/i);
      await page.locator('#icon-source[href]').waitFor({ state: 'visible' });
      const source = page.locator('#icon-source');
      const pin = 'https://github.com/ant-design/ant-design-icons/blob/7f2516ac91226d2b41f93b35cb5197c8d94f7189/packages/icons-svg/svg/';
      assert.equal(await source.getAttribute('href'), pin + 'outlined/search.svg');
      assert.equal(await source.getAttribute('target'), '_blank');
      assert.equal(await source.getAttribute('rel'), 'noopener noreferrer');
      for (const [style, name, path] of [
        ['outlined', 'SearchOutlined', 'outlined/search.svg'],
        ['filled', 'CheckCircleFilled', 'filled/check-circle.svg'],
        ['twotone', 'CiTwoTone', 'twotone/ci.svg'],
      ]) {
        await page.locator('#icon-style').selectOption(style);
        await page.locator('#icon-query').fill(name);
        const result = page.locator('button[data-icon-name="' + name + '"]');
        await result.focus();
        await page.keyboard.press('Enter');
        assert.equal(await source.getAttribute('href'), pin + path);
        assert.equal(await source.getAttribute('aria-label'),
          'View upstream SVG for ' + name + ' (opens in new tab)');
        assert.equal(await page.locator('#icon-selected-title').textContent(), name);
        const src = await page.locator('#icon-live-preview').getAttribute('src');
        assert.equal(new URL(src, base).pathname, '/gpui-antd/demo.html');
        assert.ok(decodeURIComponent(src).includes('Icon:' + name));
        assert.equal(new URL(src, base).searchParams.get('dark'), dark ? '1' : '0');
      }
      // WASM initialization focuses its canvas. Wait for the selected frame
      // before testing the surrounding documentation's keyboard order.
      await page.frameLocator('#icon-live-preview').locator('canvas').first()
        .waitFor({ state: 'visible', timeout: 60000 });
      await page.waitForFunction(() => {
        const doc = document.querySelector('#icon-live-preview')?.contentDocument;
        return doc?.querySelector('canvas')?.width > 0 && !doc.querySelector('#status');
      }, null, { timeout: 60000 });
      await page.locator('#icon-size').focus();
      await page.keyboard.press('Shift+Tab');
      assert.equal(await source.evaluate(el => el === document.activeElement), true,
        'Expected source link focus; got ' + await page.evaluate(() => document.activeElement?.outerHTML));
      assert.equal(await source.evaluate(el => getComputedStyle(el).outlineStyle), 'solid');
      const selectedSource = await source.getAttribute('href');
      await page.locator('#icon-size').selectOption('96');
      await page.locator('#icon-spin').check();
      await page.locator('#icon-primary').fill('#52c41a');
      await page.locator('#icon-reset').click();
      await page.locator('#icon-query').fill('no-icon-with-this-name');
      await page.locator('.icon-empty-results').waitFor();
      assert.equal(await source.getAttribute('href'), selectedSource);
      await page.frameLocator('#icon-live-preview').locator('canvas').first()
        .waitFor({ state: 'visible', timeout: 60000 });
      await page.waitForFunction(() => {
        const doc = document.querySelector('#icon-live-preview')?.contentDocument;
        return doc?.querySelector('canvas')?.width > 0 && !doc.querySelector('#status');
      }, null, { timeout: 60000 });
      await source.scrollIntoViewIfNeeded();
      const bounds = await source.boundingBox();
      assert.ok(bounds.x >= 0 && bounds.x + bounds.width <= page.viewportSize().width);
      assert.equal(await page.locator('vite-error-overlay').count(), 0);
      assert.deepEqual(errors, []);
      await page.screenshot({ path: join(screenshots, 'icon-source-' + (dark ? 'mobile-dark' : 'desktop-light') + '.png') });
      console.log('PASS: pinned icon source, keyboard selection/focus, empty search and real GPUI preview (' + (dark ? 'mobile dark' : 'desktop light') + ')');
    } finally {
      await page.close();
    }
  }
}
