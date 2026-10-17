/**
 * Browser-level GPUI WASM smoke test.
 *
 * CI previously only checked that a .wasm file existed, which does not prove
 * that WebGPU/WebGL initialized or that a real canvas was painted. This test
 * opens the bundled gallery in Chromium and requires its GPUI startup overlay
 * to disappear after the WASM renderer creates a non-empty canvas.
 */
import { chromium } from 'playwright';
import { spawn } from 'node:child_process';
import { mkdir } from 'node:fs/promises';
import { join } from 'node:path';

const root = new URL('../', import.meta.url);
const screenshots = join(process.env.RUNNER_TEMP ?? '/tmp', 'gpui-antd-smoke');
await mkdir(screenshots, { recursive: true });

const command = process.platform === 'win32' ? 'npm.cmd' : 'npm';
const server = spawn(command, ['run', 'preview', '--', '--host', '127.0.0.1', '--port', '4173', '--strictPort'], {
  cwd: root,
  stdio: ['ignore', 'pipe', 'pipe'],
  env: { ...process.env, CI: '1' },
});
let serverLog = '';
server.stdout.on('data', chunk => { serverLog += chunk.toString(); });
server.stderr.on('data', chunk => { serverLog += chunk.toString(); });

const base = 'http://127.0.0.1:4173/gpui-antd/';
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

async function serverReady() {
  for (let i = 0; i < 90; i++) {
    if (server.exitCode !== null) throw new Error('Preview server exited early: ' + serverLog);
    try {
      const res = await fetch(base);
      if (res.ok) return;
    } catch {}
    await sleep(500);
  }
  throw new Error('Vite preview never became ready: ' + serverLog);
}

let browser;
let failures = 0;
try {
  await serverReady();
  browser = await chromium.launch({
    headless: true,
    args: [
      '--no-sandbox',
      '--enable-webgl',
      '--ignore-gpu-blocklist',
      '--enable-unsafe-swiftshader',
      '--use-gl=angle',
      '--use-angle=swiftshader',
    ],
  });

  const components = ['Button', 'Radio', 'Input', 'Select'];
  for (const name of components) {
    const page = await browser.newPage({ viewport: { width: 900, height: 550 } });
    const errors = [];
    page.on('pageerror', error => errors.push(String(error.message)));
    page.on('console', message => {
      if (message.type() === 'error') errors.push(message.text());
    });
    try {
      const url = base + 'demo.html?component=' + encodeURIComponent(name);
      const result = await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 30000 });
      if (result?.status() !== 200) throw new Error('Unexpected response: ' + result?.status());

      await page.waitForFunction(() => {
        const canvas = document.querySelector('canvas');
        return canvas && canvas.width > 0 && canvas.height > 0
          && !document.querySelector('#status');
      }, null, { timeout: 60000 });

      // A runtime panic can surface after GPUI's initial canvas creation.
      await page.waitForTimeout(500);
      const canvas = await page.locator('canvas').first();
      if (!await canvas.isVisible()) throw new Error('Canvas was not visible');
      if (errors.length) throw new Error('JavaScript console errors: ' + errors.join('; '));
      console.log('PASS: ' + name + ' rendered through GPUI/WASM');
    } catch (error) {
      failures++;
      const text = await page.locator('#status').textContent().catch(() => '');
      console.error('FAIL: ' + name + ':', String(error));
      console.error('Startup overlay:', text);
      console.error('Browser errors:', errors.join('\n'));
    } finally {
      await page.screenshot({ path: join(screenshots, name.toLowerCase() + '.png'), fullPage: true })
        .catch(() => {});
      await page.close();
    }
  }
  if (failures > 0) throw new Error(failures + ' real GPUI/WASM gallery pages failed browser smoke tests');
} finally {
  if (browser) await browser.close();
  server.kill('SIGTERM');
}
