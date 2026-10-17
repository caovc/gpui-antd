/**
 * Real GPUI/WASM icon catalog visual audit (not a DOM SVG reproduction).
 *
 * Visits fixed Rust gallery grids, checks that each of the 848 official icons
 * painted a nonempty glyph on the GPU canvas, and saves one screenshot per
 * 24-icon page for human / future reference-image comparisons.
 *
 * This is a *paint presence* regression, not Ant Design pixel-perfect parity.
 */
import { chromium } from 'playwright';
import { spawn } from 'node:child_process';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const website = fileURLToPath(new URL('../', import.meta.url));
const output = join(process.env.RUNNER_TEMP ?? '/tmp', 'gpui-antd-icon-audit');
const catalog = JSON.parse(await readFile(new URL('../public/antd-icons.json', import.meta.url), 'utf8'));
if (catalog.count !== 848 || catalog.icons?.length !== 848) {
  throw new Error('Official icon catalog changed; audit inventory must be reviewed');
}
await mkdir(output, { recursive: true });

const vite = fileURLToPath(new URL('../node_modules/vite/bin/vite.js', import.meta.url));
const server = spawn(process.execPath,
  [vite, 'preview', '--host', '127.0.0.1', '--port', '4174', '--strictPort'],
  {cwd:website,stdio:['ignore','pipe','pipe'],env:{...process.env,CI:'1'}});
let logs='';
server.stdout.on('data', part => { logs += part.toString(); });
server.stderr.on('data', part => { logs += part.toString(); });
const base='http://127.0.0.1:4174/gpui-antd/';
const wait=ms=>new Promise(resolve=>setTimeout(resolve,ms));
async function ready(){
  for(let i=0;i<80;i++){
    if(server.exitCode!==null) throw new Error('Vite exited: '+logs);
    try{if((await fetch(base)).ok)return;}catch{}
    await wait(500);
  }
  throw new Error('Vite preview did not start: '+logs);
}

// Geometry corresponds to Gallery::icon_audit_demo: 24px root padding,
// six columns x four rows of 136x112px cards and 60px icons at x+38,y+12.
// The pinned GPUI/Web backend applies the device scale to its logical pixel
// layout, while the Chromium screenshot applies DPR again. At DPR=2, adjacent
// 136px cards are 136 * 2 * 2 = 544 PNG pixels apart, *not* 272.
// Scale the screenshot viewport as well so every one of the 24 glyphs remains
// in view. The geometry assertion fails closed if that ever changes.
// Only the glyph rectangle is sampled (never captions or card borders).
async function paintedIcons(page,png,entries){
  return page.evaluate(async ({base64,entries})=>{
    const image = new Image();
    image.src='data:image/png;base64,'+base64;
    await image.decode();
    const cv=document.createElement('canvas');
    cv.width=image.naturalWidth;cv.height=image.naturalHeight;
    const ctx=cv.getContext('2d',{willReadFrequently:true});
    ctx.drawImage(image,0,0);
    const data=ctx.getImageData(0,0,cv.width,cv.height).data;
    // One factor from GPUI layout and one from Chromium PNG rasterization.
    const deviceScale=window.devicePixelRatio||1;
    const ratio=deviceScale*deviceScale;
    const counts=[];
    for(let i=0;i<entries.length;i++){
      const icon=entries[i];
      const col=i%6,row=Math.floor(i/6);
      const x0=Math.round((24+136*col+39)*ratio);
      const x1=Math.round((24+136*col+97)*ratio);
      const y0=Math.round((24+112*row+13)*ratio);
      const y1=Math.round((24+112*row+71)*ratio);
      if(x0<0 || y0<0 || x1>cv.width || y1>cv.height){
        counts.push({name:icon.name,style:icon.style,pixels:0,
          secondaryPixels:0,coverage:0,offscreen:true,
          bounds:{x0,y0,x1,y1},screenshot:{width:cv.width,height:cv.height}});
        continue;
      }
      let painted=0,secondary=0;
      for(let y=y0;y<y1;y++) for(let x=x0;x<x1;x++){
        if(x>=cv.width||y>=cv.height)continue;
        const k=(y*cv.width+x)*4;
        const r=data[k],g=data[k+1],b=data[k+2];
        if(icon.style==='twotone'){
          if(r<115 && g>52 && g<200 && b>185 && b-r>95)painted++;
          // The secondary fill is intentionally light; count its own pixels
          // separately so missing interior details are visible in report.json.
          if(r>=224 && r<=236 && g>=238 && g<=250 && b>=250)secondary++;
        }else{
          if(r<145 && g<145 && b<145)painted++;
        }
      }
      const area=(x1-x0)*(y1-y0);
      counts.push({
        name:icon.name,style:icon.style,pixels:painted,
        secondaryPixels:secondary,coverage:Math.round(1000*painted/area)/10,
      });
    }
    return counts;
  },{base64:png.toString('base64'),entries});
}

let browser;
const sample=process.argv.includes('--sample');
const dpr=Number(process.argv.find(arg=>arg.startsWith('--dpr='))?.split('=')[1]??1);
if(![1,2].includes(dpr))throw new Error('Only --dpr=1 and --dpr=2 are supported');
const report={
  count:catalog.count,revision:catalog.revision,gitSha:process.env.GITHUB_SHA??null,
  pages:Math.ceil(catalog.count/24),sample,deviceScaleFactor:dpr,
  checked:0,failed:[],suspicious:[],results:[],screenshots:[],
  check:'GPUI GPU glyph paint presence and palette/coverage diagnostics',
};
const samplePages=[0,18,29,35];
try{
  await ready();
  browser=await chromium.launch({headless:true,args:[
    '--no-sandbox','--enable-webgl','--ignore-gpu-blocklist',
    '--enable-unsafe-swiftshader','--use-gl=angle','--use-angle=swiftshader',
  ]});
  const pages=sample?samplePages:Array.from({length:report.pages},(_,i)=>i);
  for(const index of pages){
    const entries=catalog.icons.slice(index*24,(index+1)*24);
    // GPUI logical layout is already DPR-scaled. A larger CSS viewport at
    // Retina resolution keeps all 6x4 cells visible in the PNG screenshot.
    // At DPR=2 the screenshot will be 3720 x 2180 physical pixels.
    const page=await browser.newPage({
      viewport:{width:930*dpr,height:545*dpr},deviceScaleFactor:dpr,
    });
    const errors=[];
    page.on('pageerror',error=>errors.push(error.message));
    page.on('console',message=>{
      if(message.type()!=='error')return;
      const text=message.text().trim();
      if(text!=='Wasm assets loading, will be available soon...')errors.push(text);
    });
    try{
      const route='IconAudit:'+index;
      await page.goto(base+'demo.html?component='+encodeURIComponent(route),
        {waitUntil:'domcontentloaded',timeout:30000});
      await page.waitForFunction(()=>{
        const canvas=document.querySelector('canvas');
        return canvas && canvas.width>0 && canvas.height>0 &&
          !document.querySelector('#status');
      },null,{timeout:60000});
      let counts=[];
      const filename='page-'+String(index).padStart(2,'0')+'.png';
      for(let attempt=0;attempt<9;attempt++){
        await page.waitForTimeout(attempt===0?500:400);
        const screenshot=await page.screenshot({path:join(output,filename)});
        counts=await paintedIcons(page,screenshot,entries);
        if(counts.every(icon=>icon.pixels>=7))break;
      }
      const missing=counts.filter(icon=>icon.pixels<7);
      if(errors.length)report.failed.push({page:index,errors});
      for(const entry of missing)report.failed.push({page:index,...entry});
      report.results.push(...counts.map(icon=>({page:index,...icon})));
      // A near-solid mask often indicates incorrectly flattened paths.
      // Record suspected regressions for review without misrepresenting this
      // heuristic as an official reference-pixel comparison.
      for(const entry of counts.filter(icon=>icon.coverage>=90)) {
        report.suspicious.push({page:index,...entry});
      }
      report.checked+=counts.length;
      report.screenshots.push(filename);
      console.log('AUDIT '+(index+1)+'/'+report.pages+': '+counts.length+' GPUI icons; '+
        missing.length+' missing pixels');
    }catch(error){
      report.failed.push({page:index,error:String(error)});
      console.error('FAIL: IconAudit:'+index,String(error));
    }finally{
      await page.close();
    }
  }
  await writeFile(join(output,'report.json'),JSON.stringify(report,null,2)+'\n');
  console.log('GPUI icon audit: '+report.checked+' icon previews checked at '+dpr+'x, '+
    report.failed.length+' failures, '+report.suspicious.length+
    ' high-coverage candidates; '+report.screenshots.length+' mosaics archived');
  if(report.failed.length)throw new Error('Official icon audit failed: '+JSON.stringify(report.failed.slice(0,12)));
  if(!sample && report.checked!==848)throw new Error('Expected 848 checked icon previews');
}finally{
  if(browser)await browser.close();
  server.kill('SIGTERM');
  server.stdout.destroy();server.stderr.destroy();
}
