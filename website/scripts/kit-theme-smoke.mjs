import assert from 'node:assert/strict';
import {join} from 'node:path';

// Read PNG pixels from the actual Rust canvas; the decoder is never displayed.
async function palettePixels(page, png, primary, mobile) {
  return page.evaluate(async ({png,primary,mobile}) => {
    const image = new Image();
    image.src = 'data:image/png;base64,'+png;
    await image.decode();
    const decoder = document.createElement('canvas');
    decoder.width = image.width; decoder.height = image.height;
    const context = decoder.getContext('2d', {willReadFrequently:true});
    context.drawImage(image,0,0);
    const {data} = context.getImageData(0,0,image.width,image.height);
    const at = (x,y) => (y*image.width+x)*4;
    const color = i => Math.abs(data[i]-primary[0])<8 &&
      Math.abs(data[i+1]-primary[1])<8 && Math.abs(data[i+2]-primary[2])<8;
    let primaryPixels=0, maxLightRun=0;
    for(let y=0;y<image.height;y++) {
      let run=0;
      for(let x=0;x<image.width;x++) {
        const i=at(x,y);
        if(color(i)) primaryPixels++;
        run=data[i]>200&&data[i+1]>200&&data[i+2]>200 ? run+1 : 0;
        maxLightRun=Math.max(maxLightRun,run);
      }
    }
    if(mobile) return {primaryPixels,maxLightRun};
    // Stable logical geometry of the existing 900x550 Rust Tabs Gallery.
    const region = (x,y,w,h,predicate) => {
      let n=0;
      for(let row=y;row<y+h;row++) for(let col=x;col<x+w;col++) if(predicate(at(col,row))) n++;
      return n;
    };
    const dark = i => data[i]<80&&data[i+1]<80&&data[i+2]<80;
    const light = i => data[i]>200&&data[i+1]>200&&data[i+2]>200;
    return {primaryPixels,maxLightRun,
      cardDark:region(200,313,600,28,dark)/(600*28),
      cardLight:region(200,313,600,28,light)/(600*28),
      segmentedDark:region(250,505,550,24,dark)/(550*24),
      iconLight:region(77,102,22,24,light),
      indicator:region(24,127,40,7,color),
    };
  }, {png:png.toString('base64'),primary,mobile});
}

export async function checkKitTheme(browser,base,screenshots) {
  const cases=[
    {name:'dark',dark:true,primary:[0x16,0x68,0xdc]},
    {name:'light',dark:false,primary:[0x16,0x77,0xff]},
    {name:'custom-dark',dark:true,custom:true,primary:[0x9a,0x44,0xb2]},
    {name:'custom-light',dark:false,custom:true,primary:[0x9a,0x44,0xb2]},
    {name:'mobile-dark',dark:true,mobile:true,primary:[0x16,0x68,0xdc]},
    {name:'mobile-custom-dark',dark:true,custom:true,mobile:true,primary:[0x9a,0x44,0xb2]},
  ];
  for(const test of cases) {
    const page=await browser.newPage({viewport:test.mobile?{width:390,height:844}:{width:900,height:550}});
    const errors=[];
    page.on('pageerror',e=>errors.push(e.message));
    page.on('console',m=>{
      if(m.type()==='error'&&m.text().trim()!=='Wasm assets loading, will be available soon...') errors.push(m.text());
    });
    try {
      const component=test.custom?'ThemeLab|scene=single|preview=Tabs|colorPrimary=9a44b2':'Tabs';
      const url=base+'demo.html?component='+encodeURIComponent(component)+'&dark='+Number(test.dark);
      const response=await page.goto(url,{waitUntil:'domcontentloaded'});
      assert.equal(response.status(),200);
      assert.equal(page.url(),url);
      assert.ok(await page.title());
      await page.waitForFunction(()=>document.querySelector('canvas')?.width>0&&
        document.querySelector('body > textarea')&&!document.querySelector('#status'),null,{timeout:60000});
      await page.waitForTimeout(650); // Kit's real selection indicator animation
      const png=await page.screenshot({path:join(screenshots,'kit-theme-'+test.name+'.png')});
      const pixels=await palettePixels(page,png,test.primary,test.mobile);
      assert.ok(pixels.primaryPixels>500,'Kit selection must use the Ant primary: '+JSON.stringify(pixels));
      if(test.dark) {
        assert.ok(pixels.maxLightRun<100,'Dark tabs must not contain a bright background band: '+JSON.stringify(pixels));
        if(!test.mobile) {
          assert.ok(pixels.cardDark>0.9&&pixels.segmentedDark>0.9,'Dark Card/Segmented surfaces: '+JSON.stringify(pixels));
          assert.ok(pixels.iconLight>20,'Unselected dark tab icon needs a visible foreground: '+JSON.stringify(pixels));
        }
      } else assert.ok(pixels.cardLight>0.9,'Light Card background: '+JSON.stringify(pixels));
      if(!test.mobile) assert.ok(pixels.indicator>30,'Line indicator must use the Ant primary: '+JSON.stringify(pixels));
      assert.deepEqual(errors,[]);
      console.log('PASS: actual GPUI Kit/Ant palette and tab contrast ('+test.name+') '+JSON.stringify(pixels));
    } finally { await page.close(); }
  }
}
