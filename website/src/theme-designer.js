/**
 * Token Studio: DOM editing UI around the REAL Rust GPUI/WASM ThemeLab.
 * Never substitute DOM components for the GPU preview.
 *
 * The first native token milestone supports all 31 namespaces, but only
 * Button/Card/Input/Menu/Layout currently consume their scoped overrides.
 */
const COMPONENTS='Button Input Select DatePicker Form Checkbox Radio Switch Tag Badge Alert Card Progress Divider Tabs Tooltip Icons Typography Space Flex Grid Layout Menu Dropdown Popover Modal Popconfirm Avatar Statistic Empty Breadcrumb'.split(' ');
const WIRED=new Set(['Button','Card','Input','Menu','Layout']);
const SEEDS=[
  ['colorPrimary','Primary color','#1677ff','#1668dc','color'],
  ['colorSuccess','Success','#52c41a','#49aa19','color'],
  ['colorWarning','Warning','#faad14','#d89614','color'],
  ['colorError','Error','#ff4d4f','#dc4446','color'],
  ['colorBgBase','Base surface','#ffffff','#141414','color'],
  ['colorTextBase','Base text','#262626','#f3f3f3','color'],
  ['radius','Border radius','6','6','number'],
  ['height','Control height','32','32','number'],
  ['fontSize','Font size','14','14','number'],
  ['spacing','Spacing unit','4','4','number'],
];
const FIELDS=[
  ['colorPrimary','Primary','color'],['colorBg','Background','color'],
  ['colorText','Text','color'],['colorBorder','Border','color'],
  ['colorBgHover','Hover bg','color'],['colorBgSelected','Selected bg','color'],
  ['colorHeaderBg','Header bg','color'],['borderRadius','Radius','number'],
  ['controlHeight','Height','number'],['fontSize','Font size','number'],
  ['padding','Padding','number'],
];
const PRESETS={'Ant Blue':'#1677ff','Emerald':'#12a676','Indigo':'#635bdf','Sunset':'#fa7048'};
const escapeHtml=x=>String(x).replaceAll('&','&amp;').replaceAll('<','&lt;')
  .replaceAll('>','&gt;').replaceAll('"','&quot;');
const isColor=v=>/^#[0-9a-f]{6}$/i.test(String(v));
const validateValue=(v,type)=>{
  if(type==='color')return isColor(v)?String(v).toLowerCase():null;
  const n=Number(v);
  return Number.isFinite(n)?String(Math.min(64,Math.max(0,n))):null;
};
function emptyTheme(){
  return {format:'gpui-antd-theme-v1',algorithm:'default',motion:true,
    seed:Object.fromEntries(SEEDS.map(([k,,v])=>[k,v])),components:{}};
}
function validated(raw){
  if(!raw||typeof raw!=='object'||Array.isArray(raw))throw Error('Expected theme object');
  const state=emptyTheme();
  if(['default','dark','compact','dark-compact'].includes(raw.algorithm))
    state.algorithm=raw.algorithm;
  state.motion=raw.motion!==false;
  for(const [key,,, ,kind] of SEEDS) {
    const value=raw.seed?.[key];
    const valid=value==null?null:validateValue(value,kind);
    if(valid!==null)state.seed[key]=valid;
  }
  for(const component of COMPONENTS) {
    if(!raw.components?.[component]||typeof raw.components[component]!=='object')continue;
    const fieldValues={};
    for(const [key,,kind] of FIELDS) {
      const v=raw.components[component][key];
      const valid=v==null?null:validateValue(v,kind);
      if(valid!==null)fieldValues[key]=valid;
    }
    if(Object.keys(fieldValues).length)state.components[component]=fieldValues;
  }
  return state;
}
const modeDark=s=>s.algorithm.includes('dark');
const rgbParam=value=>String(value).replace('#','');
function previewParam(state){
  const parts=['ThemeLab'];
  for(const [key] of SEEDS)parts.push(key+'='+rgbParam(state.seed[key]));
  if(state.algorithm.includes('compact'))parts.push('compact=1');
  if(!state.motion)parts.push('motion=0');
  for(const [component,values] of Object.entries(state.components))
    for(const [key,value] of Object.entries(values))
      parts.push(component+'.'+key+'='+rgbParam(value));
  return parts.join('|');
}
function rustCode(state){
  const algorithm={
    default:'Default',dark:'Dark',compact:'Compact','dark-compact':'DarkCompact'
  }[state.algorithm];
  const rustNames={
    colorPrimary:'color_primary',colorSuccess:'color_success',colorWarning:'color_warning',
    colorError:'color_error',colorBgBase:'color_bg_base',colorTextBase:'color_text_base',
    radius:'border_radius',height:'control_height',fontSize:'font_size',spacing:'spacing_unit',
  };
  const methods={
    colorPrimary:'color_primary',colorBg:'color_bg',colorText:'color_text',
    colorBorder:'color_border',colorBgHover:'color_bg_hover',
    colorBgSelected:'color_bg_selected',colorHeaderBg:'color_header_bg',
    borderRadius:'border_radius',controlHeight:'control_height',
    fontSize:'font_size',padding:'padding',
  };
  const lines=['use gpui_antd::*;','',
    'let mut seed = '+(modeDark(state)?'AntSeedTokens::dark()':'AntSeedTokens::default()')+';'];
  for(const [key,,, ,type] of SEEDS) {
    const value=state.seed[key];
    lines.push('seed.'+rustNames[key]+' = '+(type==='color'?'0x'+rgbParam(value):value+'.')+';');
  }
  lines.push('seed.color_info = seed.color_primary;',
    'seed.color_link = seed.color_primary;',
    'seed.motion = '+state.motion+';',
    'let theme = AntTheme::from_seed(seed, AntThemeAlgorithm::'+algorithm+')');
  for(const [component,values] of Object.entries(state.components)){
    const chain=Object.entries(values).map(([key,value])=>'.'+methods[key]+'('+
      (isColor(value)?'color(0x'+rgbParam(value)+')':value+'.')+')').join('');
    if(chain)lines.push('    .with_component(AntComponent::'+component+
      ', AntComponentToken::default()'+chain+')');
  }
  lines.push('    ;');
  return lines.join('\n');
}
function renderSeed([key,label,light,,kind]){
  return '<label class="token-field"><span>'+escapeHtml(label)+'</span>'+
    '<input data-seed="'+key+'" type="'+kind+'" value="'+escapeHtml(light)+'" '+
    (kind==='number'?'min="0" max="64" step="1" ':'')+
    'aria-label="'+escapeHtml(label)+'" /><small>'+key+'</small></label>';
}
export function themeDesignerHtml(){
  return '<div class="page-container theme-designer" id="theme-studio">'+
    '<div class="breadcrumbs"><a href="#/">GPUI Ant Design</a><span>›</span><b>Token Studio</b></div>'+
    '<span class="section-eyebrow">DESIGN TOKENS / NATIVE RUST / WEBASSEMBLY</span>'+
    '<div class="component-heading"><div><h1>Design Token Studio</h1>'+
    '<p>Design a theme with Seed and component tokens and preview real Rust/GPUI components.</p></div></div>'+
    '<div class="theme-toolbar">'+
    '<label>Algorithm<select id="theme-mode"><option value="default">Light</option>'+
    '<option value="dark">Dark</option><option value="compact">Compact</option>'+
    '<option value="dark-compact">Dark + Compact</option></select></label>'+
    '<label>Brand preset<select id="theme-preset"><option value="">Custom</option>'+
    Object.keys(PRESETS).map(k=>'<option value="'+k+'">'+k+'</option>').join('')+
    '</select></label>'+
    '<button id="theme-reset" type="button">Reset</button>'+
    '<button id="theme-export" type="button">Export JSON</button>'+
    '<label class="theme-import">Import JSON<input id="theme-import" type="file" accept="application/json,.json"></label>'+
    '</div>'+
    '<div class="theme-workspace">'+
    '<section class="theme-edit-panel">'+
    '<details open><summary>Seed Token · global</summary>'+
    '<div class="token-form-grid">'+SEEDS.map(renderSeed).join('')+'</div>'+
    '<label class="token-motion"><input id="theme-motion" type="checkbox" checked /> Enable motion</label>'+
    '</details>'+
    '<details open><summary>Component Token · independent overrides</summary>'+
    '<p class="theme-caption">All 31 component namespaces are registered. Overrides are applied only by components whose native token adoption is complete.</p>'+
    '<label>Component<select id="theme-component">'+COMPONENTS.map(x=>
      '<option value="'+x+'">'+x+'</option>').join('')+'</select></label>'+
    '<p id="theme-coverage" class="theme-coverage"></p>'+
    '<div id="theme-fields" class="token-form-grid"></div>'+
    '</details>'+
    '<details><summary>Seed → Map → Alias results</summary>'+
    '<p class="theme-caption">Resolved values are calculated by the Rust theme engine; colors in this summary show supplied seeds, not a JavaScript imitation of Map token derivation.</p>'+
    '<div id="theme-seed-summary" class="theme-summary"></div></details>'+
    '<details><summary>Generated Rust configuration</summary>'+
    '<button type="button" id="theme-copy">Copy Rust</button>'+
    '<pre class="theme-code"><code id="theme-code"></code></pre></details>'+
    '<p id="theme-message" role="status" aria-live="polite" class="theme-caption"></p>'+
    '</section>'+
    '<section class="theme-live-panel">'+
    '<div class="theme-preview-title"><div><span class="section-eyebrow">LIVE GPU / RUST</span>'+
    '<h2>Theme preview</h2></div><span id="theme-preview-status">Loading GPUI...</span></div>'+
    '<iframe id="theme-preview-frame" title="Live Rust GPUI WASM token preview" loading="eager"></iframe>'+
    '<p class="theme-caption">The editor controls are DOM; all UI inside this iframe is the native Rust component library running as GPUI/WASM. Edits recreate the GPU preview.</p>'+
    '</section></div>'+
    '<p class="theme-studio-note">This is a native approximation of Ant Design’s token derivation and compact/dark modes, not its React CSS-in-JS engine. '+
    '<a href="#/compatibility">Review component coverage and technology differences →</a></p>'+
    '</div>';
}
export function initThemeDesigner(base){
  const root=document.querySelector('#theme-studio');
  if(!root)return;
  const $=selector=>root.querySelector(selector);
  const frame=$('#theme-preview-frame'),fields=$('#theme-fields');
  let state;
  try{state=validated(JSON.parse(localStorage.getItem('gpui-antd-theme-v1')||'null'));}
  catch{state=emptyTheme();}
  let timer;
  function fillAll(){
    $('#theme-mode').value=state.algorithm;
    $('#theme-motion').checked=state.motion;
    for(const [key] of SEEDS)$('[data-seed="'+key+'"]').value=state.seed[key];
    drawFields();
  }
  function drawFields(){
    const component=$('#theme-component').value;
    const config=state.components[component]||{};
    $('#theme-coverage').textContent=WIRED.has(component)?
      'Runtime wired: changing these overrides affects native '+component+' components.':
      'Registered namespace; native '+component+' styling has not yet adopted these overrides.';
    fields.innerHTML=FIELDS.map(([key,label,type])=>{
      const enabled=Object.hasOwn(config,key);
      const value=enabled?config[key]:(type==='color'?'#1677ff':'6');
      return '<label class="token-field"><span><input type="checkbox" data-enable="'+key+
        '" '+(enabled?'checked':'')+' /> '+escapeHtml(label)+'</span>'+
        '<input data-field="'+key+'" type="'+type+'" value="'+escapeHtml(value)+'" '+
        (!enabled?'disabled':'')+(type==='number'?' min="0" max="64" step="1"':'')+
        ' aria-label="'+component+' '+escapeHtml(label)+'" /><small>'+key+'</small></label>';
    }).join('');
  }
  function drawPreview(){
    clearTimeout(timer);
    const params=new URLSearchParams({component:previewParam(state),dark:modeDark(state)?'1':'0'});
    $('#theme-preview-status').textContent='Rendering…';
    frame.src=base+'demo.html?'+params.toString();
    $('#theme-code').textContent=rustCode(state);
    $('#theme-seed-summary').innerHTML=SEEDS.map(([key,label])=>{
      const value=state.seed[key];
      return '<div><b>'+escapeHtml(label)+'</b><span>'+
        (isColor(value)?'<i style="background:'+value+'"></i>':'')+escapeHtml(value)+'</span></div>';
    }).join('');
    try{localStorage.setItem('gpui-antd-theme-v1',JSON.stringify(state));}catch{}
  }
  function schedule(){clearTimeout(timer);timer=setTimeout(drawPreview,250);}
  root.querySelectorAll('[data-seed]').forEach(control=>control.addEventListener('input',()=>{
    const config=SEEDS.find(([key])=>key===control.dataset.seed);
    const valid=validateValue(control.value,config[4]);
    if(valid!==null){state.seed[config[0]]=valid;schedule();}
  }));
  $('#theme-mode').addEventListener('change',()=>{
    const before=modeDark(state);
    state.algorithm=$('#theme-mode').value;
    if(before!==modeDark(state))for(const [key,,light,darkValue] of SEEDS){
      if(['colorBgBase','colorTextBase'].includes(key))
        state.seed[key]=modeDark(state)?darkValue:light;
    }
    fillAll();schedule();
  });
  $('#theme-preset').addEventListener('change',()=>{
    const chosen=PRESETS[$('#theme-preset').value];
    if(!chosen)return;
    state.seed.colorPrimary=chosen;fillAll();schedule();
  });
  $('#theme-motion').addEventListener('change',()=>{
    state.motion=$('#theme-motion').checked;schedule();
  });
  $('#theme-component').addEventListener('change',drawFields);
  fields.addEventListener('change',e=>{
    const input=e.target.closest('[data-enable]');
    if(!input)return;
    const component=$('#theme-component').value;
    const key=input.dataset.enable;
    state.components[component]||={};
    const field=fields.querySelector('[data-field="'+key+'"]');
    field.disabled=!input.checked;
    if(input.checked)state.components[component][key]=field.value;
    else delete state.components[component][key];
    if(!Object.keys(state.components[component]).length)delete state.components[component];
    schedule();
  });
  fields.addEventListener('input',e=>{
    const input=e.target.closest('[data-field]');
    if(!input||input.disabled)return;
    const component=$('#theme-component').value,key=input.dataset.field;
    const kind=FIELDS.find(([field])=>field===key)[2];
    const value=validateValue(input.value,kind);
    if(value===null)return;
    state.components[component]||={};
    state.components[component][key]=value;
    schedule();
  });
  $('#theme-reset').addEventListener('click',()=>{
    state=emptyTheme();fillAll();drawPreview();
    $('#theme-message').textContent='Token overrides reset.';
  });
  $('#theme-export').addEventListener('click',()=>{
    const href=URL.createObjectURL(new Blob([JSON.stringify(state,null,2)+'\n'],
      {type:'application/json'}));
    const a=document.createElement('a');a.href=href;
    a.download='gpui-antd-theme.json';document.body.append(a);a.click();a.remove();
    setTimeout(()=>URL.revokeObjectURL(href),2500);
    $('#theme-message').textContent='Theme JSON exported.';
  });
  $('#theme-import').addEventListener('change',async e=>{
    const file=e.target.files?.[0];if(!file)return;
    try{
      if(file.size>128000)throw Error('Theme file too large');
      state=validated(JSON.parse(await file.text()));
      fillAll();drawPreview();
      $('#theme-message').textContent='Validated theme imported.';
    }catch(error){$('#theme-message').textContent='Import failed: '+error.message;}
    e.target.value='';
  });
  $('#theme-copy').addEventListener('click',async()=>{
    try{await navigator.clipboard.writeText(rustCode(state));$('#theme-message').textContent='Rust code copied.';}
    catch{$('#theme-message').textContent='Clipboard unavailable.';}
  });
  frame.addEventListener('load',()=>{$('#theme-preview-status').textContent='GPUI / WASM';});
  fillAll();drawPreview();
}
