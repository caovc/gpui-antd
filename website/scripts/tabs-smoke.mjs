import {join} from 'node:path';

// Compare screenshot regions of the actual Rust canvas. The temporary decode
// canvas only reads PNGs; it is never displayed as a component preview.
async function changed(page, before, after) {
  return page.evaluate(async ([a, b]) => {
    const decode = async base64 => {
      const image = new Image();
      image.src = 'data:image/png;base64,' + base64;
      await image.decode();
      const canvas = document.createElement('canvas');
      canvas.width = image.width;
      canvas.height = image.height;
      const context = canvas.getContext('2d', {willReadFrequently:true});
      context.drawImage(image, 0, 0);
      return context.getImageData(0, 0, image.width, image.height).data;
    };
    const left = await decode(a), right = await decode(b);
    if (left.length !== right.length) throw Error('Screenshot dimensions changed');
    let pixels = 0;
    for (let i=0; i<left.length; i+=4) {
      if (Math.abs(left[i]-right[i])>35 || Math.abs(left[i+1]-right[i+1])>35 ||
          Math.abs(left[i+2]-right[i+2])>35) pixels++;
    }
    return pixels;
  }, [before.toString('base64'), after.toString('base64')]);
}

export async function checkTabsInteractions(browser, base, screenshots) {
  for (const dark of [false, true]) {
    const theme = dark ? 'dark' : 'light';
    const page = await browser.newPage({viewport:{width:900,height:550}});
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => {
      if (message.type()==='error' && message.text().trim()!==
          'Wasm assets loading, will be available soon...') errors.push(message.text());
    });
    const settle = () => page.waitForTimeout(650); // real GPUI indicator animation
    const panel = () => page.screenshot({clip:{x:24,y:134,width:852,height:102}});
    const toggle = () => page.screenshot({clip:{x:42,y:186,width:40,height:18}});
    try {
      const url = base+'demo.html?component=Tabs&dark='+Number(dark);
      await page.goto(url, {waitUntil:'domcontentloaded'});
      if (page.url()!==url || !(await page.title())) throw Error('Unexpected Tabs document');
      await page.waitForFunction(() => {
        const canvas = document.querySelector('canvas');
        return canvas?.width>0 && canvas?.height>0 && !document.querySelector('#status');
      }, null, {timeout:60000});
      await settle();
      await page.mouse.click(42,101);
      await page.mouse.move(880,530);
      await settle();
      const overview = await panel();
      await page.keyboard.press('ArrowRight');
      await settle();
      const settings = await panel();
      if (await changed(page, overview, settings)<1200) throw Error('Right did not replace the panel');
      await page.keyboard.press('ArrowRight');
      await settle();
      if (await changed(page, overview, await panel())>250) throw Error('Right did not wrap over disabled tab');
      await page.keyboard.press('ArrowLeft');
      await settle();
      if (await changed(page, settings, await panel())>250) throw Error('Left did not wrap to Settings');
      await page.keyboard.press('Home');
      await settle();
      if (await changed(page, overview, await panel())>250) throw Error('Home did not select Overview');
      await page.keyboard.press('End');
      await settle();
      if (await changed(page, settings, await panel())>250) throw Error('End did not select Settings');

      // Tab must leave the tab list for its *real Rust* panel control.
      // Space changes the controlled switch; focus-only paint cannot account
      // for the changed pixels inside the track, away from its focus ring.
      await page.keyboard.press('Tab');
      await settle();
      const checked = await toggle();
      await page.keyboard.press('Space');
      await settle();
      const unchecked = await toggle();
      if (await changed(page, checked, unchecked)<200) throw Error('Tab/Space did not toggle the native panel switch');
      const panelWithFocus = await panel();
      for (const key of ['ArrowLeft','Home','End']) await page.keyboard.press(key);
      await settle();
      if (await changed(page, panelWithFocus, await panel())>250) throw Error('Panel key escaped into tab navigation');
      await page.keyboard.press('Shift+Tab');
      await page.keyboard.press('ArrowRight');
      await settle();
      if (await changed(page, overview, await panel())>250) throw Error('Shift+Tab did not restore selected-tab focus');
      await page.keyboard.press('ArrowLeft');
      await settle();
      if (await changed(page, unchecked, await toggle())>40) throw Error('Owner panel state was lost after remount');
      await page.keyboard.press('Tab');
      await page.keyboard.press('Space');
      await settle();
      if (await changed(page, checked, await toggle())>40) throw Error('Panel switch did not return to checked');
      await page.screenshot({path:join(screenshots,`tabs-panel-keyboard-${theme}.png`)});
      if (errors.length) throw Error(errors.join('; '));
      console.log(`PASS: GPUI Tabs arrows/Home/End, disabled wrap, Tab/Shift-Tab, panel Space and retained state (${theme})`);
    } catch (error) {
      await page.screenshot({path:join(screenshots,`tabs-panel-keyboard-${theme}-failure.png`)}).catch(()=>{});
      throw Error(`Tabs ${theme}: ${error.message}`, {cause:error});
    } finally {
      await page.close();
    }
  }
}
