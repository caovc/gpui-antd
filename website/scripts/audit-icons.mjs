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

// The geometry corresponds to Gallery::icon_audit_demo: root padding 24px,
// 6 columns x 4 rows of 136x112px cards, with 60px icons at x+38,y+12.
// Only the icon rectangle is sampled, not its text caption or card border.
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
    const ratio=window.devicePixelRatio||1;
    const counts=[];
    for(let i=0;i<entries.length;i++){
      const icon=entries[i];
      const col=i%6,row=Math.floor(i/6);
      const x0=Math.round((24+136*col+39)*ratio);
      const x1=Math.round((24+136*col+97)*ratio);
      const y0=Math.round((24+112*row+13)*ratio);
      const y1=Math.round((24+112*row+71)*ratio);
      let painted=0;
      for(let y=y0;y<y1;y++) for(let x=x0;x<x1;x++){
        if(x>=cv.width||y>=cv.height)continue;
        const k=(y*cv.width+x)*4;
        const r=data[k],g=data[k+1],b=data[k+2];
        if(icon.style==='twotone'){
          if(r<115 && g>52 && g<200 && b>185 && b-r>95)painted++;
        }else{
          if(r<145 && g<145 && b<145)painted++;
        }
      }
      counts.push({name:icon.name,style:icon.style,pixels:painted});
    }
    return counts;
  },{base64:png.toString('base64'),entries});
}

let browser;
const report={count:catalog.count,pages:Math.ceil(catalog.count/24),
  checked:0,failed:[],screenshots:[],check:'GPU glyph paint presence'};
const sample=process.argv.includes('--sample');
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
    const page=await browser.newPage({viewport:{width:930,height:545}});
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
  console.log('GPUI icon audit: '+report.checked+' icon previews checked, '+
    report.failed.length+' failures; '+report.screenshots.length+' mosaics archived');
  if(report.failed.length)throw new Error('Official icon audit failed: '+JSON.stringify(report.failed.slice(0,12)));
  if(!sample && report.checked!==848)throw new Error('Expected 848 checked icon previews');
}finally{
  if(browser)await browser.close();
  server.kill('SIGTERM');
  server.stdout.destroy();server.stderr.destroy();
}
