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
import { stripVTControlCharacters } from 'node:util';
import { checkIconSources } from './icon-source-smoke.mjs';
import { checkIconPreviewRecovery } from './icon-preview-smoke.mjs';
import { checkTabsInteractions } from './tabs-smoke.mjs';
import { checkKitTheme } from './kit-theme-smoke.mjs';

const root = new URL('../', import.meta.url);
const port = Number(process.env.GPUI_SMOKE_PORT ?? 4173);
if (!Number.isInteger(port) || port < 1 || port > 65535) {
  throw new Error('GPUI_SMOKE_PORT must be an integer between 1 and 65535');
}
const base = 'http://127.0.0.1:' + port + '/gpui-antd/';
const screenshots = join(process.env.RUNNER_TEMP ?? '/tmp', 'gpui-antd-smoke');
await mkdir(screenshots, { recursive: true });

// Spawn Vite directly, not through `npm run`. The npm wrapper can exit
// while its Vite grandchild keeps inherited stdio pipes open forever, causing
// an otherwise successful smoke test to hang until the workflow timeout.
const viteCli = fileURLToPath(new URL('../node_modules/vite/bin/vite.js', import.meta.url));
const server = spawn(process.execPath, [viteCli, 'preview', '--host', '127.0.0.1', '--port', String(port), '--strictPort'], {
  cwd: fileURLToPath(root),
  stdio: ['ignore', 'pipe', 'pipe'],
  env: { ...process.env, CI: '1' },
});
let serverLog = '';
server.stdout.on('data', chunk => { serverLog += chunk.toString(); });
server.stderr.on('data', chunk => { serverLog += chunk.toString(); });

const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

async function serverReady() {
  for (let i = 0; i < 90; i++) {
    if (server.exitCode !== null) throw new Error('Preview server exited early: ' + serverLog);
    try {
      // A different project's existing server may return 200 before this
      // strict-port child reports its bind failure. Wait for *our* Vite URL.
      if (stripVTControlCharacters(serverLog).includes(base)) {
        const res = await fetch(base);
        if (res.ok && server.exitCode === null) return;
      }
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
  const components = ['Button', 'Radio', 'Input', 'Select', 'DatePicker', 'Form', 'Icons', 'Typography', 'Space', 'Flex', 'Grid', 'Layout', 'Menu', 'MenuTree', 'Dropdown', 'Popover', 'Modal', 'Popconfirm', 'Avatar', 'Statistic', 'Empty', 'Breadcrumb', 'Tabs', 'Tooltip', 'Pagination', 'Steps', 'Rate', 'Collapse', 'Descriptions', 'List', 'Result', 'Skeleton', 'Spin', 'InputNumber', 'Slider', 'Segmented', 'TimePicker', 'AutoComplete', 'Drawer', 'Message', 'Notification', 'Tree', 'Table', 'Timeline', 'Calendar', 'FloatButton', 'Affix', 'Anchor', 'App', 'BorderBeam', 'Carousel', 'Cascader', 'ColorPicker', 'ConfigProvider', 'Image', 'Listy', 'Masonry', 'Mentions', 'QRCode', 'Splitter', 'Tour', 'Transfer', 'TreeSelect', 'Upload', 'Watermark', 'Icon:SearchOutlined', ...twoToneCases.map(name => 'Icon:' + name)];
  const previews = [
    ...components.map(name => ({name, dark:false})),
    {name:'Icon:CheckCircleTwoTone',dark:true},
    {name:'Icon:CiTwoTone',dark:true},
    {name:'IconPalette',dark:false},
    {name:'IconPalette',dark:true},
    {name:'LayoutDrawer',dark:false},
    {name:'LayoutDrawer',dark:true},
    {name:'MenuCollapsed',dark:false},
    {name:'MenuCollapsed',dark:true},
    {name:'MenuModes',dark:false},
    {name:'MenuModes',dark:true},
    {name:'MenuHover',dark:false},
    {name:'MenuHover',dark:true},
    {name:'ThemeLab|colorPrimary=9a44b2|colorInfo=114488|Card.padding=32|scene=tokens',dark:false},
    {name:'ThemeLab|colorPrimary=1677ff|Map.colorPrimaryBg=d9f7be|Alias.colorLink=de7f22|scene=tokens',dark:false},
    {name:'ThemeLab|colorPrimary=00aa88|Checkbox.colorPrimary=ff9900|Tag.colorBg=fff1cc|motion=0|compact=1',dark:false},
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
  // Exercise the documentation editor using the Rust WASM canvas, not a DOM copy.
  const studio = await browser.newPage({viewport: {width: 1380, height: 900}});
  try {
    const response=await studio.goto(base+'#/theme',{waitUntil:'domcontentloaded',timeout:30000});
    if(response?.status()!==200)throw Error('Token Studio route returned '+response?.status());
    await studio.locator('[data-seed="colorPrimary"]').waitFor();
    await studio.locator('#theme-scene').selectOption('tokens');
    await studio.locator('[data-seed="colorPrimary"]').fill('#129977');
    await studio.locator('details').filter({hasText:'Map Token'}).locator('summary').click();
    await studio.locator('#theme-map-fields [data-token-toggle="colorPrimaryBg"]').check();
    await studio.locator('#theme-map-fields [data-token-value="colorPrimaryBg"]').fill('#d9f7be');
    await studio.locator('details').filter({hasText:'Alias Token'}).locator('summary').click();
    await studio.locator('#theme-alias-fields [data-token-toggle="colorLink"]').check();
    await studio.locator('#theme-alias-fields [data-token-value="colorLink"]').fill('#de7f22');
    await studio.waitForFunction(()=>{
      const frame=document.querySelector('#theme-preview-frame');
      if(!frame?.src)return false;
      const route=new URL(frame.src).searchParams.get('component')||'';
      return route.includes('scene=tokens') &&
        route.includes('colorPrimary=129977') &&
        route.includes('Map.colorPrimaryBg=d9f7be') &&
        route.includes('Alias.colorLink=de7f22');
    },null,{timeout:20000});
    const canvas=studio.frameLocator('#theme-preview-frame').locator('canvas').first();
    await canvas.waitFor({state:'visible',timeout:60000});
    await studio.waitForFunction(()=>
      document.querySelector('#theme-preview-status')?.textContent.includes('ready'),
      null,{timeout:60000});
    // Resizing the preview MUST retain the running Rust/WASM instance and
    // change the real canvas viewport, not replace it with HTML media mockups.
    const before=await studio.locator('#theme-preview-frame').getAttribute('src');
    await studio.locator('#theme-viewport').selectOption('mobile');
    const after=await studio.locator('#theme-preview-frame').getAttribute('src');
    if(before!==after)throw Error('Viewport switch unexpectedly reloaded WASM');
    const mobileWidth=await studio.locator('#theme-preview-frame')
      .evaluate(frame=>Math.round(frame.getBoundingClientRect().width));
    if(Math.abs(mobileWidth-390)>2)
      throw Error('Mobile GPUI viewport wrong width: '+mobileWidth);
    await studio.screenshot({path:join(screenshots,'token-studio-mobile.png')});
    await studio.locator('#theme-viewport').selectOption('desktop');
    await studio.screenshot({path:join(screenshots,'token-studio-resolved.png')});
    // A selected-component scene routes to an existing Rust Gallery demo.
    // Verify two independent components with the *same* edited theme.
    await studio.locator('#theme-scene').selectOption('single');
    for (const target of ['Tabs','Menu']) {
      await studio.locator('#theme-component').selectOption(target);
      await studio.waitForFunction(name=>{
        const src=document.querySelector('#theme-preview-frame')?.src;
        const route=src?new URL(src).searchParams.get('component')||'':'';
        return route.includes('scene=single') &&
          route.includes('preview='+name) &&
          route.includes('Map.colorPrimaryBg=d9f7be') &&
          route.includes('Alias.colorLink=de7f22');
      },target,{timeout:20000});
      await studio.frameLocator('#theme-preview-frame').locator('canvas').first()
        .waitFor({state:'visible',timeout:60000});
      await studio.waitForFunction(()=>
        document.querySelector('#theme-preview-status')?.textContent.includes('ready'),
        null,{timeout:60000});
      const shown=await studio.locator('#theme-preview-target').textContent();
      if(!shown?.includes(target))
        throw Error('Selected native component label not updated: '+target);
      await studio.screenshot({path:join(screenshots,
        'token-studio-component-'+target.toLowerCase()+'.png')});
    }
    console.log('PASS: Theme Studio Seed/Map/Alias and single-component Rust/GPUI scenes');
  } catch (error) {
    failures++;
    console.error('FAIL: Token Studio browser interaction:',String(error));
  } finally {
    await studio.close();
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
    { name: 'MenuModes', x: 90, y: 88, minChanged: 2200, minOutside: 1200 },
    { name: 'Layout', x: 98, y: 500, minChanged: 12000, minOutside: 8000 },
  ];

  const changedPixels = async (page, beforePng, afterPng, viewportWidth=900, triggerArea={right:220,bottom:112}) => page.evaluate(async ({before,after,viewportWidth,triggerArea}) => {
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
        // Exclude the actual trigger area to reject hover/focus-only changes.
        if (x>triggerArea.right || y>triggerArea.bottom) outside++;
      }
    }
    return {total,outside};
  }, {before:beforePng.toString('base64'),after:afterPng.toString('base64'),viewportWidth,triggerArea});

  // Exercise a real GPUI two-color spinning SVG at 2x device scale.
  // Neither colored DOM controls nor a nonempty canvas count as proof.
  const spinning = await browser.newPage({
    viewport: {width:900, height:550}, deviceScaleFactor:2,
  });
  try {
    const component='Icon:CheckCircleTwoTone|size=96|spin=1|primary=52c41a|secondary=f6ffed';
    await spinning.goto(base+'demo.html?component='+encodeURIComponent(component),
      {waitUntil:'domcontentloaded'});
    await spinning.waitForFunction(() => {
      const canvas=document.querySelector('canvas');
      return canvas && canvas.width>=900 && canvas.height>=550 &&
        !document.querySelector('#status');
    },null,{timeout:60000});
    await spinning.waitForTimeout(450);
    const pixels=await twoTonePixelCounts(spinning);
    if(pixels.green<35 || pixels.greenFill<35) {
      throw new Error('2x GPUI TwoTone animation lost color layers: '+JSON.stringify(pixels));
    }
    const before=await spinning.screenshot({
      path:join(screenshots,'icon-twotone-animated-before-2x.png'),
    });
    await spinning.waitForTimeout(300);
    const after=await spinning.screenshot({
      path:join(screenshots,'icon-twotone-animated-after-2x.png'),
    });
    const diff=await changedPixels(spinning,before,after,1800);
    if(diff.total<85) {
      throw new Error('GPUI TwoTone spin did not animate: '+JSON.stringify(diff));
    }
    console.log('PASS: TwoTone primary/secondary SVG layers rotate at 2x DPI ('+
      diff.total+' changed pixels)');
  } catch (error) {
    failures++;
    console.error('FAIL: TwoTone 2x animation',String(error));
  } finally {
    await spinning.close();
  }

  // Real pointer hover opens controlled Rust menus. Exact delay boundaries
  // use the native executor-clock tests; browser waits include GPU scheduling.
  for (const dark of [false,true]) {
    const page=await browser.newPage({viewport:{width:900,height:550}});
    const errors=[];
    page.on('pageerror',error=>errors.push(error.message));
    page.on('console',message=>{
      if(message.type()==='error' && message.text().trim()!=='Wasm assets loading, will be available soon...')
        errors.push(message.text());
    });
    try {
      await page.goto(base+'demo.html?component=MenuHover&dark='+(dark?'1':'0'),{waitUntil:'domcontentloaded'});
      await page.waitForFunction(()=>document.querySelector('canvas')&&!document.querySelector('#status'),null,{timeout:60000});
      await page.mouse.move(880,530);
      await page.waitForTimeout(650);
      const baseline=await page.screenshot();
      await page.mouse.move(85,88);
      await page.waitForTimeout(450);
      const opened=await page.screenshot({path:join(screenshots,'menu-hover-open'+(dark?'-dark':'')+'.png')});
      const delta=await changedPixels(page,baseline,opened);
      if(delta.outside<1500)throw Error('Hover did not open GPUI panel: '+JSON.stringify(delta));
      await page.mouse.move(90,185);
      await page.waitForTimeout(450);
      const panel=await page.screenshot({path:join(screenshots,'menu-hover-panel'+(dark?'-dark':'')+'.png')});
      if((await changedPixels(page,baseline,panel)).outside<1500)throw Error('Panel closed while pointer was inside it');
      await page.mouse.move(880,530);
      await page.waitForTimeout(450);
      const closed=await page.screenshot({path:join(screenshots,'menu-hover-left'+(dark?'-dark':'')+'.png')});
      const residual=await changedPixels(page,baseline,closed);
      if(residual.outside>delta.outside*0.25+100)throw Error('Leaving did not dismiss panel: '+JSON.stringify(residual));
      await page.mouse.move(85,88);
      await page.waitForTimeout(450);
      await page.keyboard.press('Escape');
      await page.waitForTimeout(450);
      const escaped=await changedPixels(page,baseline,await page.screenshot());
      if(escaped.outside>delta.outside*0.25+100)throw Error('Escape reopened the hovered panel: '+JSON.stringify(escaped));
      await page.mouse.move(880,530);
      await page.mouse.move(480,88);
      await page.waitForTimeout(450);
      const disabled=await changedPixels(page,baseline,await page.screenshot());
      if(disabled.outside>300)throw Error('Disabled submenu reacted to hover: '+JSON.stringify(disabled));
      if(errors.length)throw Error(errors.join('; '));
      console.log('PASS: MenuHover'+(dark?' dark':'')+' real GPUI hover, panel traversal, leave, Escape and disabled behavior');
    } catch(error) {
      failures++;
      console.error('FAIL: MenuHover'+(dark?' dark':''),String(error));
    } finally { await page.close(); }
  }

  // Catalog panels must paint outside their real Rust triggers and dismiss
  // through Kit. Exercise both palettes; these are GPUI canvas pixels.
  for (const dark of [false, true]) {
    for (const spec of [
      {name:'Cascader',initialOpen:true,bottom:56},
      {name:'ColorPicker',bottom:56},
      {name:'TreeSelect',bottom:56},
      {name:'Image',bottom:164},
      {name:'Tour',initialOpen:true,bottom:58},
    ]) {
      const page=await browser.newPage({viewport:{width:900,height:550}});
      const errors=[];
      page.on('pageerror',error=>errors.push(error.message));
      page.on('console',message=>{
        if(message.type()==='error' && message.text().trim()!=='Wasm assets loading, will be available soon...')
          errors.push(message.text());
      });
      try {
        const url=base+'demo.html?component='+spec.name+'&dark='+(dark?'1':'0');
        const ready=async()=>{
          await page.goto(url,{waitUntil:'domcontentloaded'});
          await page.waitForFunction(()=>document.querySelector('canvas')&&!document.querySelector('#status'),null,{timeout:60000});
          await page.waitForTimeout(600);
        };
        await ready();
        let opened;
        if(spec.initialOpen) opened=await page.screenshot();
        await page.keyboard.press('Escape');
        await page.mouse.move(880,530);
        await page.waitForTimeout(300);
        const closed=await page.screenshot();
        if(!spec.initialOpen) {
          await page.mouse.click(40,40);
          await page.mouse.move(880,530);
          await page.waitForTimeout(400);
          opened=await page.screenshot();
        }
        const area={right:876,bottom:spec.bottom};
        const delta=await changedPixels(page,closed,opened,900,area);
        if(delta.outside<200)throw Error('No panel pixels below trigger: '+JSON.stringify(delta));
        // For initially open demos, reload instead of assuming the Tour
        // close-only callback can request a new controlled session.
        if(spec.initialOpen) await ready();
        await page.keyboard.press('Escape');
        await page.waitForTimeout(350);
        const escaped=await page.screenshot({path:join(screenshots,spec.name.toLowerCase()+'-catalog-escape'+(dark?'-dark':'')+'.png')});
        const residual=await changedPixels(page,closed,escaped,900,area);
        if(residual.outside>delta.outside*0.3+80)throw Error('Escape left panel pixels: '+JSON.stringify(residual));
        if(spec.initialOpen) await ready();
        else {
          await page.mouse.click(40,40);
          await page.waitForTimeout(300);
        }
        await page.mouse.click(880,530);
        await page.waitForTimeout(350);
        const outside=await changedPixels(page,closed,await page.screenshot(),900,area);
        if(outside.outside>delta.outside*0.3+80)throw Error('Outside click left panel pixels: '+JSON.stringify(outside));
        if(errors.length)throw Error(errors.join('; '));
        console.log('PASS: '+spec.name+(dark?' dark':'')+' GPUI panel below trigger, Escape and outside dismissal');
      } catch(error) {
        failures++;
        console.error('FAIL: catalog '+spec.name+(dark?' dark':''),String(error));
      } finally { await page.close(); }
    }
  }

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

      if (['Dropdown','Popover','Modal','Popconfirm','MenuModes'].includes(spec.name)) {
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

  // Wave-two interaction smoke: all coordinates target Rust-painted GPUI
  // controls, not HTML stand-ins. Make sure a visual state transition follows
  // native pointer activation, beyond simply booting a WASM canvas.
  const inputInteractions = [
    {name:'Segmented', x:65, y:78, minChanged:250},
    {name:'Slider', x:450, y:76, minChanged:300},
    {name:'AutoComplete', x:78, y:118, minChanged:650},
  ];
  for (const spec of inputInteractions) {
    const page=await browser.newPage({viewport:{width:900,height:550}});
    const errors=[];
    page.on('pageerror',error=>errors.push(String(error.message)));
    try {
      await page.goto(base+'demo.html?component='+spec.name,{waitUntil:'domcontentloaded'});
      await page.waitForFunction(()=>{
        const canvas=document.querySelector('canvas');
        return canvas&&canvas.width>0&&canvas.height>0&&!document.querySelector('#status');
      },null,{timeout:60000});
      await page.waitForTimeout(750);
      const baseline=await page.screenshot({
        path:join(screenshots,spec.name.toLowerCase()+'-control-before.png')
      });
      await page.mouse.click(spec.x,spec.y);
      await page.mouse.move(880,530);
      await page.waitForTimeout(650);
      const changed=await page.screenshot({
        path:join(screenshots,spec.name.toLowerCase()+'-control-after.png')
      });
      const diff=await changedPixels(page,baseline,changed);
      if(diff.total<spec.minChanged){
        throw new Error('Click did not cause expected GPUI state transition: '+JSON.stringify(diff));
      }
      if(errors.length)throw new Error(errors.join('; '));
      console.log('PASS: native '+spec.name+' pointer action changed '+diff.total+' GPUI pixels');
      if(spec.name==='Segmented'){
        // A button click should focus its native segment; Right changes
        // selection to the next enabled option without a second mouse click.
        await page.keyboard.press('ArrowRight');
        await page.waitForTimeout(450);
        const keyboard=await page.screenshot({
          path:join(screenshots,'segmented-keyboard-next.png')
        });
        const nav=await changedPixels(page,changed,keyboard);
        if(nav.total<180) {
          throw new Error('Segmented ArrowRight failed after pointer focus: '+JSON.stringify(nav));
        }
        console.log('PASS: Segmented ArrowRight updates controlled Rust selection');
      }
    } catch(error) {
      failures++;
      console.error('FAIL: native '+spec.name+' GPUI interaction',String(error));
    } finally {
      await page.close();
    }
  }

  // Wave-three pointer acceptance. Click the REAL GPUI canvas, require
  // meaningful ink changes from the native Sheet/toast/tree/table state.
  // The docs shell has no HTML stand-in for these components.
  const wave3Interactions = [
    {name:'Drawer', x:114, y:78, minChanged:2500, minOutside:1800},
    {name:'Message', x:72, y:78, minChanged:650, minOutside:500},
    {name:'Notification', x:87, y:78, minChanged:700, minOutside:550},
    {name:'Tree', x:46, y:83, minChanged:1000, minOutside:650},
    {name:'Table', x:406, y:78, minChanged:1000, minOutside:700},
  ];
  for (const spec of wave3Interactions) {
    const page=await browser.newPage({viewport:{width:900,height:550}});
    const errors=[];
    page.on('pageerror',error=>errors.push(String(error.message)));
    page.on('console',message=>{
      if(message.type()!=='error')return;
      const detail=message.text().trim();
      if(detail!=='Wasm assets loading, will be available soon...')errors.push(detail);
    });
    try {
      await page.goto(base+'demo.html?component='+spec.name,{waitUntil:'domcontentloaded'});
      await page.waitForFunction(()=>{
        const canvas=document.querySelector('canvas');
        return canvas&&canvas.width>0&&canvas.height>0&&!document.querySelector('#status');
      },null,{timeout:60000});
      await page.waitForTimeout(700);
      const before=await page.screenshot({
        path:join(screenshots,spec.name.toLowerCase()+'-wave3-before.png')
      });
      await page.mouse.click(spec.x,spec.y);
      await page.mouse.move(880,530);
      await page.waitForTimeout(850);
      const after=await page.screenshot({
        path:join(screenshots,spec.name.toLowerCase()+'-wave3-after.png')
      });
      const diff=await changedPixels(page,before,after);
      if(diff.total<spec.minChanged || diff.outside<spec.minOutside){
        throw new Error(spec.name+' native control did not repaint enough after click: '+
          JSON.stringify(diff)+' expected total '+spec.minChanged+' and outside '+spec.minOutside);
      }
      if(errors.length)throw new Error('Browser errors: '+errors.join('; '));
      console.log('PASS: wave3 '+spec.name+' real GPUI click '+diff.total+' pixels');
      if(spec.name==='Drawer'){
        await page.keyboard.press('Escape');
        await page.waitForTimeout(600);
        const closed=await page.screenshot({
          path:join(screenshots,'drawer-wave3-escape.png')
        });
        const residual=await changedPixels(page,before,closed);
        if(residual.total>diff.total*0.48+1000){
          throw new Error('Native Drawer Escape failed to dismiss Sheet: '+
            JSON.stringify({open:diff,afterEscape:residual}));
        }
        console.log('PASS: wave3 Drawer Escape restores the undimmed GPUI view');
      }
    }catch(error){
      failures++;
      console.error('FAIL: wave3 '+spec.name+' native GPUI interaction',String(error));
      console.error('Browser errors:',errors.join('; '));
    }finally{
      await page.close();
    }
  }

  try {
    await checkKitTheme(browser,base,screenshots);
    await checkTabsInteractions(browser,base,screenshots);
  } catch (error) {
    failures++;
    console.error('FAIL: native GPUI Tabs keyboard controls',String(error));
  }

  // Interactive nested GPUI Menu regression on the real Rust canvas.
  const nested=await browser.newPage({viewport:{width:900,height:550}});
  const nestedErrors=[];
  nested.on('pageerror',error=>nestedErrors.push(error.message));
  nested.on('console',message=>{
    if (message.type()!=='error') return;
    const msg=message.text().trim();
    if (msg!=='Wasm assets loading, will be available soon...') nestedErrors.push(msg);
  });
  try {
    await nested.goto(base+'demo.html?component=MenuTree',{waitUntil:'domcontentloaded'});
    await nested.waitForFunction(()=>{
      const canvas=document.querySelector('canvas');
      return canvas&&canvas.width>0&&canvas.height>0&&!document.querySelector('#status');
    },null,{timeout:60000});
    await nested.waitForTimeout(700);
    const initial=await nested.screenshot({
      path:join(screenshots,'menu-tree-closed.png')
    });
    // The first visible row is the Reports submenu title.
    await nested.mouse.click(110,82);
    await nested.mouse.move(870,520);
    await nested.waitForTimeout(650);
    const open=await nested.screenshot({
      path:join(screenshots,'menu-tree-open.png')
    });
    const opening=await changedPixels(nested,initial,open);
    if(opening.total<1500||opening.outside<900) {
      throw new Error('GPUI nested submenu did not visibly expand: '+JSON.stringify(opening));
    }
    console.log('PASS: nested GPUI SubMenu expands from real canvas pointer activation');
    await nested.keyboard.press('ArrowDown');
    await nested.keyboard.press('Enter');
    await nested.waitForTimeout(550);
    const selected=await nested.screenshot({
      path:join(screenshots,'menu-tree-keyboard-selected.png')
    });
    const changed=await changedPixels(nested,open,selected);
    if(changed.total<850) {
      throw new Error('ArrowDown + Enter did not select native child: '+JSON.stringify(changed));
    }
    console.log('PASS: GPUI Menu roving ArrowDown + Enter activates a child');
    await nested.keyboard.press('Escape');
    await nested.waitForTimeout(600);
    const closed=await nested.screenshot({
      path:join(screenshots,'menu-tree-escape-closed.png')
    });
    const closing=await changedPixels(nested,selected,closed);
    if(closing.total<1200) {
      throw new Error('Escape did not close nested submenu: '+JSON.stringify(closing));
    }
    console.log('PASS: GPUI Menu Escape closes SubMenu and restores parent focus');
    if(nestedErrors.length) throw new Error('Browser errors: '+nestedErrors.join('; '));
  } catch(error) {
    failures++;
    console.error('FAIL: native Menu nested pointer/keyboard cycle',String(error));
    console.error('Browser errors:',nestedErrors.join('; '));
  } finally {
    await nested.close();
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

  // Mobile Drawer interaction: the actual Rust/GPUI Sheet must paint above
  // app content, dismiss on Escape/mask, and restore the closed layout.
  const sheetPage=await browser.newPage({viewport:{width:520,height:680}});
  const sheetErrors=[];
  sheetPage.on('pageerror',error=>sheetErrors.push(error.message));
  try {
    await sheetPage.goto(base+'demo.html?component=LayoutDrawer',{waitUntil:'domcontentloaded'});
    await sheetPage.waitForFunction(()=>{
      const canvas=document.querySelector('canvas');
      return canvas&&canvas.width>0&&canvas.height>0&&!document.querySelector('#status');
    },null,{timeout:60000});
    await sheetPage.waitForTimeout(700);
    const before=await sheetPage.screenshot({
      path:join(screenshots,'layout-drawer-closed.png')
    });
    // The mobile Menu icon in AntHeader is painted by Rust GPUI at ~58,84.
    await sheetPage.mouse.click(58,84);
    await sheetPage.mouse.move(500,600);
    await sheetPage.waitForTimeout(700);
    const after=await sheetPage.screenshot({
      path:join(screenshots,'layout-drawer-open.png')
    });
    const delta=await changedPixels(sheetPage,before,after,520);
    if(delta.total<9000||delta.outside<6500)
      throw new Error('Native GPUI Sheet did not open: '+JSON.stringify(delta));
    await sheetPage.keyboard.press('Escape');
    await sheetPage.waitForTimeout(450);
    const closed=await sheetPage.screenshot({
      path:join(screenshots,'layout-drawer-escape-closed.png')
    });
    const restored=await changedPixels(sheetPage,before,closed,520);
    if(restored.total>delta.total*0.45+1000)
      throw new Error('Native Drawer failed Escape dismissal: '+JSON.stringify(restored));
    await sheetPage.mouse.click(58,84);
    await sheetPage.waitForTimeout(500);
    await sheetPage.mouse.click(488,335);
    await sheetPage.waitForTimeout(450);
    const dismissed=await sheetPage.screenshot({
      path:join(screenshots,'layout-drawer-mask-closed.png')
    });
    const maskDelta=await changedPixels(sheetPage,before,dismissed,520);
    if(maskDelta.total>delta.total*0.45+1000)
      throw new Error('Native Drawer failed mask dismissal: '+JSON.stringify(maskDelta));
    if(sheetErrors.length)throw new Error(sheetErrors.join('; '));
    console.log('PASS: GPUI Layout Drawer opens Sheet, Escape/mask close and restore');
  } catch(error) {
    failures++;
    console.error('FAIL: mobile Layout native Drawer interactions',String(error));
  } finally {
    await sheetPage.close();
  }

  // Exercise the real Rust GPUI popover rather than a CSS stand-in.
  // Keep a fixed pointer position outside the collapsed Sider to confirm
  // the popover occupies actual screen space and can dismiss on Escape.
  const flyout=await browser.newPage({viewport:{width:660,height:570}});
  const flyoutErrors=[];
  flyout.on('pageerror',error=>flyoutErrors.push(error.message));
  try {
    await flyout.goto(base+'demo.html?component=MenuCollapsed',{waitUntil:'domcontentloaded'});
    await flyout.waitForFunction(()=>{
      const canvas=document.querySelector('canvas');
      return canvas&&canvas.width>0&&canvas.height>0&&!document.querySelector('#status');
    },null,{timeout:60000});
    await flyout.waitForTimeout(550);
    const before=await flyout.screenshot({path:join(screenshots,'menu-collapsed-before.png')});
    await flyout.mouse.click(58,94);
    await flyout.mouse.move(625,480);
    await flyout.waitForTimeout(650);
    const opened=await flyout.screenshot({path:join(screenshots,'menu-collapsed-open.png')});
    // The collapsed rail ends at x=100. The usual top-left button exclusion
    // (x<=220, y<=112) masks most of this right-hand popup, including its title.
    const railArea={right:100,bottom:238};
    const delta=await changedPixels(flyout,before,opened,660,railArea);
    if(delta.total<3500||delta.outside<2500)
      throw new Error('Collapsed native flyout not visible: '+JSON.stringify(delta));
    await flyout.keyboard.press('Escape');
    await flyout.waitForTimeout(470);
    const closed=await flyout.screenshot({path:join(screenshots,'menu-collapsed-escape-closed.png')});
    const residual=await changedPixels(flyout,before,closed,660,railArea);
    if(residual.total>delta.total*0.5+1200)
      throw new Error('Collapsed flyout failed native Escape dismissal: '+JSON.stringify(residual));
    if(flyoutErrors.length)throw new Error(flyoutErrors.join('; '));
    console.log('PASS: collapsed Menu native GPUI flyout opens and Escape dismisses');
  } catch(error) {
    failures++;
    console.error('FAIL: collapsed Menu native Popover interaction',String(error));
  } finally {
    await flyout.close();
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
    // Clearing a native search input through Playwright can skip Chrome's
    // incremental search event in some headless runs. Dispatch the input
    // event explicitly and wait for the full 848-item catalog to reappear
    // before pressing a pagination button that was hidden by the old filter.
    await iconDocs.locator('#icon-query').fill('');
    await iconDocs.locator('#icon-query').dispatchEvent('input');
    await iconDocs.waitForFunction(()=>
      document.querySelector('#icon-query')?.value==='' &&
      document.querySelector('#icon-load-more')?.hidden===false &&
      document.querySelectorAll('button[data-icon-name]').length===80,
      null,{timeout:15000});
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
    await docs.locator('#icon-primary').fill('#52c41a');
    await docs.locator('#icon-secondary').fill('#f6ffed');
    await docs.locator('#icon-size').selectOption('96');
    await docs.locator('#icon-spin').check();
    const chosen = await docs.locator('#icon-live-preview').getAttribute('src');
    const decoded = decodeURIComponent(chosen);
    if (!decoded.includes('primary=52c41a') ||
        !decoded.includes('secondary=f6ffed') ||
        !decoded.includes('spin=1') || !decoded.includes('size=96')) {
      throw new Error('Inspector controls did not update GPUI URL: '+decoded);
    }
    const sample=await docs.locator('#icon-selected-code').textContent();
    if (!sample.includes('.two_tone_colors(') || !sample.includes('.spin(true)')) {
      throw new Error('Copied Rust code disagrees with inspector settings: '+sample);
    }
    await docs.locator('#icon-reset').click();
    const resetUrl=decodeURIComponent(await docs.locator('#icon-live-preview').getAttribute('src'));
    if (resetUrl.includes('primary=') || resetUrl.includes('spin=1') ||
        !resetUrl.includes('size=64')) {
      throw new Error('Inspector reset did not restore theme defaults: '+resetUrl);
    }
    console.log('PASS: full official icon catalog search and live GPUI color/size/spin inspector');
  } catch(error) {
    failures++;
    console.error('FAIL: official icon search and GPUI preview', String(error));
  } finally {
    await docs.close();
  }
  try {
    await checkIconSources(browser, base, screenshots);
  } catch (error) {
    failures++;
    console.error('FAIL: pinned official icon source links', String(error));
  }
  try {
    await checkIconPreviewRecovery(browser, base, screenshots);
  } catch (error) {
    failures++;
    console.error('FAIL: icon preview navigation recovery', String(error));
  }
  if (failures > 0) throw new Error(failures + ' GPUI/WASM browser smoke tests failed');
} finally {
  if (browser) await browser.close();
  server.kill('SIGTERM');
  server.stdout.destroy();
  server.stderr.destroy();
}
