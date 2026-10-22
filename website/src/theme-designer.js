/**
 * Token Studio: DOM editing UI around the REAL Rust GPUI/WASM ThemeLab.
 * Never substitute DOM components for the GPU preview.
 *
 * All 53 component namespaces can be serialized. Exact native consumption
 * is listed per field: unsupported fields are not disguised as live changes.
 */
const COMPONENTS='Button Input Select DatePicker Form Checkbox Radio Switch Tag Badge Alert Card Progress Divider Tabs Tooltip Icons Typography Space Flex Grid Layout Menu Dropdown Popover Modal Popconfirm Avatar Statistic Empty Breadcrumb Pagination Steps Rate Collapse Descriptions Result Skeleton Spin InputNumber Slider Segmented TimePicker AutoComplete Drawer Message Notification Tree Table Timeline List Calendar FloatButton'.split(' ');
const COVERAGE={
  Button:['colorPrimary','colorBg','colorText','colorBorder','colorBgHover','borderRadius','controlHeight','fontSize','padding'],
  Card:['colorBg','colorText','colorBorder','colorHeaderBg','borderRadius','fontSize','padding'],
  Input:['colorBg','colorBorder','borderRadius'],
  Menu:['colorBg','colorText','colorBgHover','colorBgSelected','borderRadius','fontSize','controlHeight'],
  Layout:['colorBg','colorText','colorBgHover'],
  Checkbox:['colorPrimary','colorBg','colorText','colorBorder','borderRadius','controlHeight','fontSize'],
  Switch:['colorPrimary','colorBg','controlHeight'],
  Tag:['colorBg','colorText','colorBorder','borderRadius','controlHeight','padding','fontSize'],
  Badge:['colorBg','colorText','controlHeight','padding','fontSize'],
  Alert:['colorPrimary','colorBg','colorText','colorBorder','borderRadius','fontSize','padding'],
  Progress:['colorPrimary','colorBg','colorText','controlHeight','fontSize'],
  Divider:['colorBorder','controlHeight'],
  Typography:['colorPrimary','colorBg','colorText','colorBorder','borderRadius','fontSize'],
  Tooltip:['colorBg','colorText','colorBorder','borderRadius'],
  Tree:['colorPrimary','colorBg','colorText','colorBgHover','colorBgSelected','borderRadius','controlHeight','fontSize'],
  Table:['colorBg','colorText','colorBorder','colorHeaderBg','colorBgSelected','borderRadius','fontSize'],
  Timeline:['colorPrimary','colorText','colorBorder','fontSize'],
  Calendar:['colorBg','colorText','colorBorder','borderRadius'],
  FloatButton:['colorPrimary','colorBg','colorText','colorBorder','borderRadius','controlHeight'],
};
const SEEDS=[
  ['colorPrimary','Primary color','#1677ff','#1668dc','color'],
  ['colorInfo','Info color','#1677ff','#1668dc','color'],
  ['colorLink','Link color','#1677ff','#3c89e8','color'],
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
// These are intentionally bounded, independently editable subsets of Ant Design's
// Map and Alias layers. Only include values consumed by native Rust today.
const MAP_FIELDS=[
  ['colorPrimaryHover','Primary hover','color'],
  ['colorPrimaryActive','Primary active','color'],
  ['colorPrimaryBg','Primary background','color'],
  ['colorPrimaryBorder','Primary border','color'],
  ['colorBgElevated','Elevated surface','color'],
  ['colorBgLayout','Layout surface','color'],
  ['controlHeightSM','Small control height','number'],
  ['controlHeightLG','Large control height','number'],
];
const ALIAS_FIELDS=[
  ['colorLink','Link color','color'],
  ['colorTextDisabled','Disabled text','color'],
  ['colorSuccess','Success color','color'],
  ['colorWarning','Warning color','color'],
  ['colorError','Error color','color'],
];
const LAYER_FIELDS={map:MAP_FIELDS,alias:ALIAS_FIELDS};
const PREVIEW_WIDTHS=Object.freeze({desktop:'100%',tablet:'768px',mobile:'390px'});
const previewViewportWidth=mode=>PREVIEW_WIDTHS[mode]||PREVIEW_WIDTHS.desktop;
const PRESETS={'Ant Blue':'#1677ff','Emerald':'#12a676','Indigo':'#635bdf','Sunset':'#fa7048'};
const escapeHtml=x=>String(x).replaceAll('&','&amp;').replaceAll('<','&lt;')
  .replaceAll('>','&gt;').replaceAll('"','&quot;');
const isColor=v=>/^#[0-9a-f]{6}$/i.test(String(v));
const LIMITS={radius:[0,32],height:[20,64],fontSize:[10,28],
  spacing:[1,16],borderRadius:[0,32],controlHeight:[20,64],padding:[0,64],controlHeightSM:[18,64],controlHeightLG:[20,72]};
const validateValue=(v,type,key)=>{
  if(type==='color')return isColor(v)?String(v).toLowerCase():null;
  if(v==null||String(v).trim()==='')return null;
  const n=Number(v);
  const [min,max]=LIMITS[key]??[0,64];
  return Number.isFinite(n)?String(Math.min(max,Math.max(min,n))):null;
};
function emptyTheme(){
  return {format:'gpui-antd-theme-v1',algorithm:'default',motion:true,
    seed:Object.fromEntries(SEEDS.map(([k,,v])=>[k,v])),map:{},alias:{},components:{}};
}
function validated(raw){
  if(!raw||typeof raw!=='object'||Array.isArray(raw))throw Error('Expected theme object');
  const state=emptyTheme();
  if(['default','dark','compact','dark-compact'].includes(raw.algorithm))
    state.algorithm=raw.algorithm;
  state.motion=raw.motion!==false;
  // Partial dark-theme imports must resolve dark seed defaults, rather than
  // accidentally rendering light surfaces inside a dark shell.
  if(state.algorithm.includes('dark'))
    for(const [key,,,darkValue] of SEEDS)state.seed[key]=darkValue;
  for(const [key,,, ,kind] of SEEDS) {
    const value=raw.seed?.[key];
    const valid=value==null?null:validateValue(value,kind,key);
    if(valid!==null)state.seed[key]=valid;
  }
  for(const [layer,fields] of Object.entries(LAYER_FIELDS)) {
    for(const [key,,kind] of fields) {
      const v=raw[layer]?.[key];
      const valid=v==null?null:validateValue(v,kind,key);
      if(valid!==null)state[layer][key]=valid;
    }
  }
  for(const component of COMPONENTS) {
    if(!raw.components?.[component]||typeof raw.components[component]!=='object')continue;
    const fieldValues={};
    for(const [key,,kind] of FIELDS) {
      const v=raw.components[component][key];
      const valid=v==null?null:validateValue(v,kind,key);
      if(valid!==null)fieldValues[key]=valid;
    }
    if(Object.keys(fieldValues).length)state.components[component]=fieldValues;
  }
  return state;
}
const modeDark=s=>s.algorithm.includes('dark');
const rgbParam=value=>String(value).replace('#','');
function previewParam(state,scene='components',selectedComponent='Button'){
  const parts=['ThemeLab'];
  for(const [key] of SEEDS)parts.push(key+'='+rgbParam(state.seed[key]));
  if(state.algorithm.includes('compact'))parts.push('compact=1');
  if(!state.motion)parts.push('motion=0');
  if(scene==='tokens')parts.push('scene=tokens');
  if(scene==='single'){
    // Strict allowlist: never forward arbitrary editor text as a gallery route.
    const target=COMPONENTS.includes(selectedComponent)?selectedComponent:'Button';
    parts.push('scene=single','preview='+target);
  }
  for(const [layer,values] of [['map',state.map],['alias',state.alias]])
    for(const [key,value] of Object.entries(values))
      parts.push((layer==='map'?'Map':'Alias')+'.'+key+'='+rgbParam(value));
  for(const [component,values] of Object.entries(state.components))
    for(const [key,value] of Object.entries(values))
      parts.push(component+'.'+key+'='+rgbParam(value));
  return parts.join('|');
}
const rustNumber=value=>{
  const n=Number(value);
  return String(n)+(Number.isInteger(n)?'.':'');
};
function rustCode(state){
  const algorithm={
    default:'Default',dark:'Dark',compact:'Compact','dark-compact':'DarkCompact'
  }[state.algorithm];
  const rustNames={
    colorPrimary:'color_primary',colorInfo:'color_info',colorLink:'color_link',
    colorSuccess:'color_success',colorWarning:'color_warning',
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
  const mapMethods={
    colorPrimaryHover:'color_primary_hover',colorPrimaryActive:'color_primary_active',
    colorPrimaryBg:'color_primary_bg',colorPrimaryBorder:'color_primary_border',
    colorBgElevated:'color_bg_elevated',colorBgLayout:'color_bg_layout',
    controlHeightSM:'control_height_sm',controlHeightLG:'control_height_lg',
  };
  const aliasMethods={
    colorLink:'color_link',colorTextDisabled:'color_text_disabled',
    colorSuccess:'color_success',colorWarning:'color_warning',colorError:'color_error',
  };
  const lines=['use gpui_antd::*;','',
    'let mut seed = '+(modeDark(state)?'AntSeedTokens::dark()':'AntSeedTokens::default()')+';'];
  for(const [key,,, ,type] of SEEDS) {
    const value=state.seed[key];
    lines.push('seed.'+rustNames[key]+' = '+(type==='color'?'0x'+rgbParam(value):rustNumber(value))+';');
  }
  lines.push('seed.motion = '+state.motion+';',
    'let theme = AntTheme::from_seed(seed, AntThemeAlgorithm::'+algorithm+')');
  for(const [layer,methodNames,kind] of [
    ['map',mapMethods,'AntMapTokenOverrides'],
    ['alias',aliasMethods,'AntAliasTokenOverrides'],
  ]) {
    const chain=Object.entries(state[layer]).map(([key,value])=>
      '.'+methodNames[key]+'('+
      (isColor(value)?'color(0x'+rgbParam(value)+')':rustNumber(value))+')').join('');
    if(chain)lines.push('    .with_'+layer+'_overrides('+kind+'::default()'+chain+')');
  }
  for(const [component,values] of Object.entries(state.components)){
    const chain=Object.entries(values).map(([key,value])=>'.'+methods[key]+'('+
      (isColor(value)?'color(0x'+rgbParam(value)+')':rustNumber(value))+')').join('');
    if(chain)lines.push('    .with_component(AntComponent::'+component+
      ', AntComponentToken::default()'+chain+')');
  }
  lines.push('    ;','',
    '// In application init or theme-change updates, after gpui_kit::init(cx):',
    '// Applies shared Kit colors to every window; pass .theme(theme) to Ant controls.',
    'theme.apply_to_kit(cx);');
  return lines.join('\n');
}
function renderLayerFields(layer,fields,values){
  return fields.map(([key,label,kind])=>{
    const enabled=Object.hasOwn(values,key);
    const value=enabled?values[key]:
      (kind==='color'?'#1677ff':key==='controlHeightLG'?'40':'24');
    return '<label class="token-field"><span><input data-token-toggle="'+key+
      '" type="checkbox" '+(enabled?'checked ':'')+'/> '+
      escapeHtml(label)+' <em>'+layer.toUpperCase()+'</em></span>'+
      '<input data-token-value="'+key+'" type="'+kind+'" value="'+escapeHtml(value)+
      '" '+(enabled?'':'disabled ')+(kind==='number'?'min="0" max="72" step="1" ':'')+
      'aria-label="'+layer+' '+escapeHtml(label)+'" />'+
      '<small>'+layer+'.'+key+'</small></label>';
  }).join('');
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
    '<p>Design Seed, Map, Alias and Component tokens with real Rust/GPUI previews.</p></div></div>'+
    '<div class="theme-toolbar">'+
    '<label>Algorithm<select id="theme-mode"><option value="default">Light</option>'+
    '<option value="dark">Dark</option><option value="compact">Compact</option>'+
    '<option value="dark-compact">Dark + Compact</option></select></label>'+
    '<label>Preview<select id="theme-scene"><option value="components">Overview</option>'+
    '<option value="single">Selected component</option>'+
    '<option value="tokens">Resolved Tokens</option></select></label>'+
    '<label>Viewport<select id="theme-viewport" aria-label="GPUI preview viewport">'+
    '<option value="desktop">Desktop · fluid</option>'+
    '<option value="tablet">Tablet · 768px</option>'+
    '<option value="mobile">Mobile · 390px</option></select></label>'+
    '<label>Brand preset<select id="theme-preset"><option value="">Custom</option>'+
    Object.keys(PRESETS).map(k=>'<option value="'+k+'">'+k+'</option>').join('')+
    '</select></label>'+
    '<button id="theme-reset" type="button">Reset</button>'+
    '<button id="theme-export" type="button">Export JSON</button>'+
    '<button id="theme-share" type="button" title="Share the actual GPUI/WASM preview with current tokens">Copy preview URL</button>'+
    '<label class="theme-import">Import JSON<input id="theme-import" type="file" accept="application/json,.json"></label>'+
    '</div>'+
    '<div class="theme-workspace">'+
    '<section class="theme-edit-panel">'+
    '<details open><summary>Seed Token · global</summary>'+
    '<div class="token-form-grid">'+SEEDS.map(renderSeed).join('')+'</div>'+
    '<label class="token-motion"><input id="theme-motion" type="checkbox" checked /> Enable motion</label>'+
    '</details>'+
    '<details><summary>Map Token · derived state colors and geometry</summary>'+
    '<p class="theme-caption">Optional overrides of derived Map values. Unchecked fields continue to follow the Seed algorithm; the native GPUI token inspector shows the resolved values.</p>'+
    '<div id="theme-map-fields" class="token-form-grid"></div></details>'+
    '<details><summary>Alias Token · global semantic overrides</summary>'+
    '<p class="theme-caption">Alias overrides apply after Map derivation and before component-specific values.</p>'+
    '<div id="theme-alias-fields" class="token-form-grid"></div></details>'+
    '<details open><summary>Component Token · independent overrides</summary>'+
    '<p class="theme-caption">All 53 namespaces can be exported. Only fields marked Live are connected to the Rust renderer; unimplemented fields cannot be edited here.</p>'+
    '<label>Component<select id="theme-component">'+COMPONENTS.map(x=>
      '<option value="'+x+'">'+x+'</option>').join('')+'</select></label>'+
    '<p id="theme-coverage" class="theme-coverage"></p>'+
    '<div id="theme-fields" class="token-form-grid"></div>'+
    '</details>'+
    '<details><summary>Seed Token values and derivation</summary>'+
    '<p class="theme-caption">These are the editable inputs. For actual derived Map and Alias values, choose <strong>Resolved Tokens</strong> in Preview: those tiles are rendered by the Rust engine, not calculated by the DOM.</p>'+
    '<div id="theme-seed-summary" class="theme-summary"></div></details>'+
    '<details><summary>Generated Rust configuration</summary>'+
    '<button type="button" id="theme-copy">Copy Rust</button>'+
    '<pre class="theme-code"><code id="theme-code"></code></pre></details>'+
    '<p id="theme-message" role="status" aria-live="polite" class="theme-caption"></p>'+
    '</section>'+
    '<section class="theme-live-panel">'+
    '<div class="theme-preview-title"><div><span class="section-eyebrow">LIVE GPU / RUST</span>'+
    '<h2>Theme preview</h2><small id="theme-preview-target">Component overview</small></div>'+
    '<span id="theme-preview-status">Loading GPUI...</span></div>'+
    '<div id="theme-preview-stage" class="theme-preview-stage" data-viewport="desktop">'+
    '<iframe id="theme-preview-frame" title="Live Rust GPUI WASM token preview" loading="eager"></iframe></div>'+
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
  let scene='components';
  // Viewport preference belongs to the editor, not the exported theme.
  // Resizing the iframe changes the actual GPUI/WASM window's viewport.
  let viewport='desktop';
  try {
    const stored=localStorage.getItem('gpui-antd-preview-viewport');
    if(Object.hasOwn(PREVIEW_WIDTHS,stored))viewport=stored;
  }catch{}
  function applyViewport(){
    $('#theme-viewport').value=viewport;
    $('#theme-preview-stage').dataset.viewport=viewport;
    frame.style.width=previewViewportWidth(viewport);
  }
  applyViewport();
  function fillAll(){
    $('#theme-mode').value=state.algorithm;
    $('#theme-motion').checked=state.motion;
    for(const [key] of SEEDS)$('[data-seed="'+key+'"]').value=state.seed[key];
    drawFields();
    drawLayerFields();
  }
  function drawLayerFields(){
    for(const [layer,defs] of Object.entries(LAYER_FIELDS))
      $('#theme-'+layer+'-fields').innerHTML=renderLayerFields(layer,defs,state[layer]);
  }
  function drawFields(){
    const component=$('#theme-component').value;
    const config=state.components[component]||{};
    const live=COVERAGE[component]||[];
    $('#theme-coverage').textContent=live.length?
      component+': '+live.length+' live native token fields. Other fields are not implemented.':
      component+': namespace registered, but native component token adoption is not implemented.';
    fields.innerHTML=FIELDS.map(([key,label,type])=>{
      const enabled=Object.hasOwn(config,key);
      const supported=live.includes(key);
      const value=enabled?config[key]:(type==='color'?'#1677ff':'6');
      return '<label class="token-field'+(supported?'':' token-field-unwired')+'">'+
        '<span><input type="checkbox" data-enable="'+key+'" '+
        (enabled?'checked ':'')+(supported?'':'disabled ')+' /> '+
        escapeHtml(label)+' <em>'+(supported?'Live':'Planned')+'</em></span>'+
        '<input data-field="'+key+'" type="'+type+'" value="'+escapeHtml(value)+'" '+
        (!enabled||!supported?'disabled':'')+(type==='number'?' min="0" max="64" step="1"':'')+
        ' aria-label="'+component+' '+escapeHtml(label)+'" /><small>'+key+'</small></label>';
    }).join('');
  }
  function drawPreview(){
    clearTimeout(timer);
    const selectedComponent=$('#theme-component').value;
    const label=scene==='tokens'?'Derived Map / Alias tokens':
      scene==='single'?selectedComponent+' · native component':'Component overview';
    $('#theme-preview-target').textContent=label;
    const params=new URLSearchParams({
      component:previewParam(state,scene,selectedComponent),
      dark:modeDark(state)?'1':'0'
    });
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
    const valid=validateValue(control.value,config[4],config[0]);
    if(valid!==null){
      const key=config[0],previous=state.seed[key];
      state.seed[key]=valid;
      if(key==='colorPrimary')for(const follow of ['colorInfo','colorLink']){
        if(state.seed[follow]===previous){state.seed[follow]=valid;$('[data-seed="'+follow+'"]').value=valid;}
      }
      schedule();
    }
  }));
  $('#theme-mode').addEventListener('change',()=>{
    const before=modeDark(state);
    state.algorithm=$('#theme-mode').value;
    if(before!==modeDark(state))for(const [key,,light,darkValue] of SEEDS){
      if(state.seed[key]===(before?darkValue:light))
        state.seed[key]=modeDark(state)?darkValue:light;
    }
    fillAll();schedule();
  });
  $('#theme-preset').addEventListener('change',()=>{
    const chosen=PRESETS[$('#theme-preset').value];
    if(!chosen)return;
    const old=state.seed.colorPrimary;
    for(const key of ['colorInfo','colorLink']){
      if(state.seed[key]===old)state.seed[key]=chosen;
    }
    state.seed.colorPrimary=chosen;fillAll();schedule();
  });
  $('#theme-scene').addEventListener('change',()=>{
    scene=$('#theme-scene').value;
    drawPreview();
  });
  $('#theme-viewport').addEventListener('change',()=>{
    viewport=$('#theme-viewport').value;
    applyViewport(); // Preserve running Rust GPUI instance; do not reload WASM.
    try{localStorage.setItem('gpui-antd-preview-viewport',viewport);}catch{}
  });
  $('#theme-motion').addEventListener('change',()=>{
    state.motion=$('#theme-motion').checked;schedule();
  });
  $('#theme-component').addEventListener('change',()=>{
    drawFields();
    // Choosing a component is an explicit intent to inspect its real native
    // preview. Keep the token inspector scene if the user chose it.
    if(scene!=='tokens'){
      scene='single';$('#theme-scene').value='single';schedule();
    }
  });
  for(const [layer,defs] of Object.entries(LAYER_FIELDS)){
    const container=$('#theme-'+layer+'-fields');
    container.addEventListener('change',e=>{
      const toggle=e.target.closest('[data-token-toggle]');
      if(!toggle)return;
      const key=toggle.dataset.tokenToggle;
      const field=container.querySelector('[data-token-value="'+key+'"]');
      if(toggle.checked){
        const kind=defs.find(([k])=>k===key)[2];
        const value=validateValue(field.value,kind,key);
        if(value!==null)state[layer][key]=value;
      }else delete state[layer][key];
      drawLayerFields();schedule();
    });
    container.addEventListener('input',e=>{
      const field=e.target.closest('[data-token-value]');
      if(!field||field.disabled)return;
      const key=field.dataset.tokenValue;
      const kind=defs.find(([k])=>k===key)[2];
      const value=validateValue(field.value,kind,key);
      if(value===null)return;
      state[layer][key]=value;schedule();
    });
  }
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
    const value=validateValue(input.value,kind,key);
    if(value===null)return;
    state.components[component]||={};
    state.components[component][key]=value;
    schedule();
  });
  $('#theme-reset').addEventListener('click',()=>{
    state=emptyTheme();fillAll();drawPreview();
    $('#theme-message').textContent='Token overrides reset.';
  });
  $('#theme-share').addEventListener('click',async()=>{
    // Share the real standalone GPUI/WASM scene, not a DOM screenshot.
    // URL parameters include Seed, Map, Alias and Component values.
    try{
      await navigator.clipboard.writeText(frame.src);
      $('#theme-message').textContent='WASM preview link copied.';
    }catch{
      $('#theme-message').textContent='Clipboard unavailable; open the preview frame to share its URL.';
    }
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
  frame.addEventListener('load',()=>{
    $('#theme-preview-status').textContent='Initializing GPUI…';
  });
  window.addEventListener('message',event=>{
    if(event.origin!==location.origin||event.source!==frame.contentWindow)return;
    if(event.data?.type==='gpui-antd:ready')
      $('#theme-preview-status').textContent='GPUI / WASM ready';
    if(event.data?.type==='gpui-antd:error'){
      $('#theme-preview-status').textContent='GPUI failed';
      $('#theme-message').textContent=String(event.data.error||'Renderer error');
    }
  });
  fillAll();drawPreview();
}

// Pure functions are exported for node:test without mounting a DOM or a fake GPUI preview.
export const tokenStudioLogic={
  emptyTheme,validated,previewParam,rustCode,validateValue,renderLayerFields,
  previewViewportWidth,
};
