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
import { fileURLToPath } from 'node:url';

const root = new URL('../', import.meta.url);
const screenshots = join(process.env.RUNNER_TEMP ?? '/tmp', 'gpui-antd-smoke');
await mkdir(screenshots, { recursive: true });

// Spawn Vite directly, not through `npm run`. The npm wrapper can exit
// while its Vite grandchild keeps inherited stdio pipes open forever, causing
// an otherwise successful smoke test to hang until the workflow timeout.
const viteCli = fileURLToPath(new URL('../node_modules/vite/bin/vite.js', import.meta.url));
const server = spawn(process.execPath, [viteCli, 'preview', '--host', '127.0.0.1', '--port', '4173', '--strictPort'], {
  cwd: fileURLToPath(root),
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

// The existence of a GPUI canvas cannot catch opaque, single-color TwoTone
// icons: GPUI's svg() renders alpha masks. Screenshot pixels must prove that
// the official saturated blue and pale-blue interior both survived painting.
async function twoTonePixelCounts(page) {
  const screenshot = await page.screenshot();
  return page.evaluate(async base64 => {
    const image = new Image();
    image.src = 'data:image/png;base64,' + base64;
    await image.decode();
    const surface = document.createElement('canvas');
    surface.width = image.naturalWidth;
    surface.height = image.naturalHeight;
    const context = surface.getContext('2d', { willReadFrequently: true });
    context.drawImage(image, 0, 0);
    const data = context.getImageData(0, 0, surface.width, surface.height).data;
    let blue = 0;
    let pale = 0;
    let darkBlue = 0;
    let darkFill = 0;
    let green = 0;
    let greenFill = 0;
    const within = (r,g,b,R,G,B,delta=7) =>
      Math.abs(r-R)<=delta && Math.abs(g-G)<=delta && Math.abs(b-B)<=delta;
    for (let i = 0; i < data.length; i += 4) {
      const r = data[i], g = data[i + 1], b = data[i + 2];
      if (r < 90 && g > 75 && g < 180 && b > 195 && b - g > 50) blue++;
      if (r >= 207 && r < 247 && g >= 225 && g <= 254 && b >= 245 && b - r >= 10) pale++;
      if (within(r,g,b,0x16,0x68,0xdc)) darkBlue++;
      if (within(r,g,b,0x11,0x1d,0x2c,5)) darkFill++;
      if (within(r,g,b,0x52,0xc4,0x1a)) green++;
      if (within(r,g,b,0xf6,0xff,0xed,5)) greenFill++;
    }
    return { blue, pale, darkBlue, darkFill, green, greenFill };
  }, screenshot.toString('base64'));
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

  const twoToneCases = ['CarryOutTwoTone', 'CheckCircleTwoTone', 'CheckSquareTwoTone', 'CiCircleTwoTone', 'CiTwoTone'];
  const components = ['Button', 'Radio', 'Input', 'Select', 'DatePicker', 'Form', 'Icons', 'Typography', 'Space', 'Flex', 'Grid', 'Layout', 'Menu', 'Dropdown', 'Popover', 'Modal', 'Popconfirm', 'Avatar', 'Statistic', 'Empty', 'Breadcrumb', 'Tabs', 'Tooltip', 'Icon:SearchOutlined', ...twoToneCases.map(name => 'Icon:' + name)];
  const previews = [
    ...components.map(name => ({name, dark:false})),
    {name:'Icon:CheckCircleTwoTone',dark:true},
    {name:'Icon:CiTwoTone',dark:true},
    {name:'IconPalette',dark:false},
    {name:'IconPalette',dark:true},
  ];
  for (const {name, dark} of previews) {
    const page = await browser.newPage({ viewport: { width: 900, height: 550 } });
    const errors = [];
    page.on('pageerror', error => errors.push(String(error.message)));
    page.on('console', message => {
      if (message.type() !== 'error') return;
      const detail = message.text().trim();
      // GPUI Kit WASM Assets.load deliberately returns this recoverable
      // error while its SVG request is pending. The renderer retries after
      // loading; this is not a JavaScript exception or a failed resource.
      // Do not filter ANY other console error.
      if (detail === 'Wasm assets loading, will be available soon...') return;
      errors.push(detail);
    });
    try {
      const url = base + 'demo.html?component=' + encodeURIComponent(name) +
        '&dark=' + (dark ? '1' : '0');
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
      if (twoToneCases.some(icon => name === 'Icon:' + icon) || name === 'IconPalette') {
        let colors = { blue: 0, pale: 0, darkBlue: 0, darkFill: 0, green: 0, greenFill: 0 };
        for (let attempt = 0; attempt < 8; attempt++) {
          colors = await twoTonePixelCounts(page);
          const good = name === 'IconPalette'
            ? colors.green >= 15 && colors.greenFill >= 15
            : dark
              ? colors.darkBlue >= 15 && colors.darkFill >= 15
              : colors.blue >= 15 && colors.pale >= 15;
          if (good) break;
          await page.waitForTimeout(300); // Full-color img() loading is asynchronous.
        }
        const good = name === 'IconPalette'
          ? colors.green >= 15 && colors.greenFill >= 15
          : dark
            ? colors.darkBlue >= 15 && colors.darkFill >= 15
            : colors.blue >= 15 && colors.pale >= 15;
        if (!good) {
          throw new Error('TwoTone SVG lost expected color layers in GPUI screenshot: ' +
            JSON.stringify({name,dark,colors}));
        }
      }
      if (errors.length) throw new Error('JavaScript console errors: ' + errors.join('; '));
      console.log('PASS: ' + name + (dark ? ' dark' : '') + ' rendered through GPUI/WASM');
    } catch (error) {
      failures++;
      const text = await page.locator('#status').textContent({ timeout: 500 }).catch(() => '');
      console.error('FAIL: ' + name + ':', String(error));
      console.error('Startup overlay:', text);
      console.error('Browser errors:', errors.join('\n'));
    } finally {
      await page.screenshot({ path: join(screenshots,
        name.toLowerCase().replace(/[^a-z0-9_-]/g, '-') + (dark ? '-dark' : '') + '.png'
      ), fullPage: true })
        .catch(() => {});
      await page.close();
    }
  }
  // These tests click actual pixels on the Rust/GPUI canvas, rather than
  // DOM controls. A large painted-pixel difference is required so a mere
  // hover tint does not count as an opened popup or collapsed dashboard.
  // Keep both screenshots as CI artifacts for manual visual review.
  const interactions = [
    // GPUI draws translucent surfaces on the same GPU canvas. Count changed
    // ink pixels rather than assuming the entire popup rectangle is opaque.
    { name: 'Dropdown', x: 76, y: 78, minChanged: 2100, minOutside: 700 },
    { name: 'Popover', x: 78, y: 78, minChanged: 3300, minOutside: 1200 },
    { name: 'Modal', x: 82, y: 78, minChanged: 8000, minOutside: 4000 },
    { name: 'Popconfirm', x: 82, y: 78, minChanged: 4300, minOutside: 1400 },
    // The initial selected menu item is Dashboard. This clicks Analytics,
    // changing the selected row in all three native Menu variants.
    { name: 'Menu', x: 110, y: 135, minChanged: 3500, minOutside: 2500 },
    { name: 'Layout', x: 98, y: 500, minChanged: 12000, minOutside: 8000 },
  ];

  const changedPixels = async (page, beforePng, afterPng, viewportWidth=900) => page.evaluate(async ({before,after,viewportWidth}) => {
    async function decode(base64) {
      const binary = atob(base64);
      const bytes = Uint8Array.from(binary, c => c.charCodeAt(0));
      const bitmap = await createImageBitmap(new Blob([bytes], {type:'image/png'}));
      const canvas = document.createElement('canvas');
      canvas.width = bitmap.width;
      canvas.height = bitmap.height;
      const ctx = canvas.getContext('2d', {willReadFrequently:true});
      ctx.drawImage(bitmap,0,0);
      bitmap.close();
      return ctx.getImageData(0,0,canvas.width,canvas.height).data;
    }
    const a=await decode(before), b=await decode(after);
    if (a.length !== b.length) throw new Error('Screenshot dimensions changed');
    let total=0,outside=0;
    for (let i=0;i<a.length;i+=4) {
      if (Math.abs(a[i]-b[i])>35 || Math.abs(a[i+1]-b[i+1])>35 ||
          Math.abs(a[i+2]-b[i+2])>35) {
        total++;
        const pixel=i/4, x=pixel%viewportWidth, y=Math.floor(pixel/viewportWidth);
        // All four popup triggers live in the top-left area of the canvas.
        // Require change OUTSIDE that button area to reject hover/focus-only.
        if (x>220 || y>112) outside++;
      }
    }
    return {total,outside};
  }, {before:beforePng.toString('base64'),after:afterPng.toString('base64'),viewportWidth});

  for (const spec of interactions) {
    const page=await browser.newPage({viewport:{width:900,height:550}});
    const errors=[];
    page.on('pageerror', err => errors.push(err.message));
    page.on('console', message => {
      if (message.type() !== 'error') return;
      const detail=message.text().trim();
      if (detail !== 'Wasm assets loading, will be available soon...') errors.push(detail);
    });
    try {
      await page.goto(base+'demo.html?component='+spec.name,{waitUntil:'domcontentloaded'});
      await page.waitForFunction(() => {
        const canvas=document.querySelector('canvas');
        return canvas && canvas.width>0 && canvas.height>0 &&
          !document.querySelector('#status');
      },null,{timeout:60000});
      await page.waitForTimeout(800);
      const baseline=await page.screenshot({
        path:join(screenshots,spec.name.toLowerCase()+'-before.png')
      });
      await page.mouse.click(spec.x,spec.y);
      await page.mouse.move(880,530);
      await page.waitForTimeout(800);
      const opened=await page.screenshot({
        path:join(screenshots,spec.name.toLowerCase()+'-after.png')
      });
      const diff=await changedPixels(page,baseline,opened);
      if(diff.total<spec.minChanged || diff.outside<spec.minOutside) {
        throw new Error(spec.name+' canvas changed '+JSON.stringify(diff)+
          ' pixels; expected total >= '+spec.minChanged+
          ' and outside trigger >= '+spec.minOutside);
      }
      if(errors.length) throw new Error('Browser errors: '+errors.join('; '));
      console.log('PASS: '+spec.name+' real GPUI click changed '+diff.total+
        ' pixels ('+diff.outside+' outside trigger)');

      if (spec.name === 'Menu') {
        // The fourth light-menu item is disabled. Clicking it must not
        // change the selected Rust route key or repaint the selected row.
        await page.mouse.click(110,211);
        await page.mouse.move(880,530);
        await page.waitForTimeout(550);
        const disabled=await page.screenshot({
          path:join(screenshots,'menu-disabled-click.png')
        });
        const afterDisabled=await changedPixels(page,opened,disabled);
        if (afterDisabled.total > 1200) {
          throw new Error('Disabled Menu item changed selected state or appearance: '+
            JSON.stringify(afterDisabled));
        }
        console.log('PASS: disabled native GPUI Menu item does not change selection');
      }

      if (['Dropdown','Popover','Modal','Popconfirm'].includes(spec.name)) {
        // Native Escape dismissal is a required interaction, not just a
        // startup/canvas check. Compare against the pre-popup frame;
        // the trigger can retain its focus ring, but the popup must vanish.
        await page.keyboard.press('Escape');
        await page.waitForTimeout(550);
        const closed=await page.screenshot({
          path:join(screenshots,spec.name.toLowerCase()+'-escape.png')
        });
        const residual=await changedPixels(page,baseline,closed);
        const tolerance=diff.total*0.62+500;
        if(residual.total>tolerance) {
          throw new Error(spec.name+' did not visually dismiss on Escape: '+
            JSON.stringify(residual)+' pixels remain vs '+diff.total+' when open');
        }
        console.log('PASS: '+spec.name+' Escape closed the native GPUI overlay ('+
          residual.total+' changed pixels after close)');
      }
    } catch(error) {
      failures++;
      console.error('FAIL: '+spec.name+' native GPUI pointer interaction',String(error));
      console.error('Browser errors:',errors.join('; '));
    } finally {
      await page.close();
    }
  }

  // Responsive Layout regression: in a narrow GPUI Web viewport the
  // breakpoint initially collapses Sider, and a click must expand it.
  // A second click returns the panel to its initial narrow state.
  const narrow=await browser.newPage({viewport:{width:600,height:650}});
  const narrowErrors=[];
  narrow.on('pageerror', error=>narrowErrors.push(error.message));
  try {
    await narrow.goto(base+'demo.html?component=Layout',{waitUntil:'domcontentloaded'});
    await narrow.waitForFunction(() => {
      const canvas=document.querySelector('canvas');
      return canvas && canvas.width>0 && canvas.height>0 && !document.querySelector('#status');
    },null,{timeout:60000});
    await narrow.waitForTimeout(750);
    const compact=await narrow.screenshot({
      path:join(screenshots,'layout-mobile-collapsed.png')
    });
    await narrow.mouse.click(50,500);
    await narrow.mouse.move(570,620);
    await narrow.waitForTimeout(650);
    const expanded=await narrow.screenshot({
      path:join(screenshots,'layout-mobile-expanded.png')
    });
    const opened=await changedPixels(narrow,compact,expanded,600);
    if(opened.total<9000 || opened.outside<6500) {
      throw new Error('Narrow Sider did not expand: '+JSON.stringify(opened));
    }
    await narrow.mouse.click(50,500);
    await narrow.mouse.move(570,620);
    await narrow.waitForTimeout(650);
    const collapsedAgain=await narrow.screenshot({
      path:join(screenshots,'layout-mobile-collapsed-again.png')
    });
    const residual=await changedPixels(narrow,compact,collapsedAgain,600);
    if(residual.total>opened.total*0.42+1000) {
      throw new Error('Narrow Sider could not collapse after expansion: '+
        JSON.stringify(residual)+' vs '+JSON.stringify(opened));
    }
    if(narrowErrors.length) throw new Error(narrowErrors.join('; '));
    console.log('PASS: responsive Sider can expand and collapse while below md breakpoint');
  } catch(error) {
    failures++;
    console.error('FAIL: narrow native GPUI Sider controlled state',String(error));
  } finally {
    await narrow.close();
  }

  // Regression: long official icon names are visible, and all 848 names
  // remain browsable instead of the first 80 results being silently clipped.
  const iconDocs = await browser.newPage({ viewport: {width: 980, height: 700} });
  try {
    await iconDocs.goto(base + '#/components/Icons', {waitUntil:'domcontentloaded'});
    await iconDocs.locator('#icon-query').fill('CheckCircle');
    const name = iconDocs.locator('button[data-icon-name="CheckCircleTwoTone"] span');
    await name.waitFor({timeout:15000});
    const style = await name.evaluate(el => ({
      whiteSpace:getComputedStyle(el).whiteSpace,
      overflowWrap:getComputedStyle(el).overflowWrap,
      textOverflow:getComputedStyle(el).textOverflow,
    }));
    if (style.whiteSpace==='nowrap' || style.overflowWrap!=='anywhere' || style.textOverflow==='ellipsis') {
      throw new Error('Icon name CSS clipping persists: '+JSON.stringify(style));
    }
    await iconDocs.locator('#icon-query').fill('');
    await iconDocs.locator('#icon-load-more').click();
    if (await iconDocs.locator('button[data-icon-name]').count() < 160) {
      throw new Error('Catalog pagination hides icons after the first 80');
    }
    console.log('PASS: full icon names wrap and icon catalog paginates correctly');
  } catch(error) {
    failures++;
    console.error('FAIL: icon name visibility/pagination',String(error));
  } finally {
    await iconDocs.close();
  }
  // The search catalog must select a real GPUI/WASM standalone preview.
  const docs = await browser.newPage({ viewport: {width: 1100, height: 750} });
  try {
    await docs.goto(base + '#/components/Icons', {waitUntil: 'domcontentloaded'});
    await docs.locator('#icon-query').fill('CheckCircleTwoTone');
    await docs.locator('button[data-icon-name="CheckCircleTwoTone"]').waitFor({timeout: 15000});
    await docs.locator('button[data-icon-name="CheckCircleTwoTone"]').click();
    const src = await docs.locator('#icon-live-preview').getAttribute('src');
    if (!src.includes('Icon%3ACheckCircleTwoTone')) {
      throw new Error('Icon explorer did not switch the Rust GPUI iframe: ' + src);
    }
    console.log('PASS: full official icon catalog search controls live GPUI icon preview');
  } catch(error) {
    failures++;
    console.error('FAIL: official icon search and GPUI preview', String(error));
  } finally {
    await docs.close();
  }
  if (failures > 0) throw new Error(failures + ' GPUI/WASM browser smoke tests failed');
} finally {
  if (browser) await browser.close();
  server.kill('SIGTERM');
  server.stdout.destroy();
  server.stderr.destroy();
}
