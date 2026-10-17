import test from 'node:test';
import assert from 'node:assert/strict';
import { themeDesignerHtml, tokenStudioLogic } from '../src/theme-designer.js';

const {emptyTheme,validated,previewParam,rustCode,validateValue,previewViewportWidth}=tokenStudioLogic;

test('studio exposes the real GPU preview and two independent scenes',()=>{
  const html=themeDesignerHtml();
  assert.match(html,/theme-preview-frame/);
  assert.match(html,/id="theme-share"/);
  assert.match(html,/value="tokens"/);
  assert.match(html,/LIVE GPU/);
  assert.match(html,/data-seed="colorPrimary"/);
  assert.ok(!html.includes('<button class="ant-btn"'));
});

test('responsive preview viewport changes the real GPU iframe width without theme changes',()=>{
  const html=themeDesignerHtml();
  assert.match(html,/id="theme-viewport"/);
  assert.match(html,/id="theme-preview-stage"/);
  assert.match(html,/value="mobile"/);
  assert.match(html,/value="tablet"/);
  assert.equal(previewViewportWidth('desktop'),'100%');
  assert.equal(previewViewportWidth('tablet'),'768px');
  assert.equal(previewViewportWidth('mobile'),'390px');
  assert.equal(previewViewportWidth('bogus'),'100%');
  const config=emptyTheme();
  assert.equal(previewParam(config,'tokens'),previewParam(config,'tokens'));
});

test('independent colorInfo/colorLink seeds roundtrip into WASM and Rust',()=>{
  const state=emptyTheme();
  state.seed.colorPrimary='#1677ff';
  state.seed.colorInfo='#114488';
  state.seed.colorLink='#beef12';
  const route=previewParam(state,'tokens');
  assert.match(route,/colorInfo=114488/);
  assert.match(route,/colorLink=beef12/);
  assert.match(route,/scene=tokens/);
  const code=rustCode(state);
  assert.match(code,/seed\.color_info = 0x114488;/);
  assert.match(code,/seed\.color_link = 0xbeef12;/);
  assert.ok(!code.includes('seed.color_info = seed.color_primary'));
  assert.equal(validated(state).seed.colorInfo,'#114488');
});

test('component scoped overrides remain independent in generated Rust',()=>{
  const theme=emptyTheme();
  theme.algorithm='dark-compact';
  theme.motion=false;
  theme.components.Button={colorPrimary:'#12aa77',borderRadius:'12'};
  theme.components.Card={padding:'32'};
  const params=previewParam(theme,'components');
  assert.match(params,/Button\.colorPrimary=12aa77/);
  assert.match(params,/Card\.padding=32/);
  assert.doesNotMatch(params,/scene=tokens/);
  const code=rustCode(theme);
  assert.match(code,/AntThemeAlgorithm::DarkCompact/);
  assert.match(code,/seed\.motion = false/);
  assert.match(code,/AntComponent::Button/);
  assert.match(code,/AntComponent::Card/);
  assert.match(code,/color_primary\(color\(0x12aa77\)\)/);
});

test('Map and Alias layers roundtrip through JSON, preview URL and Rust API',()=>{
  const state=emptyTheme();
  state.seed.colorPrimary='#635bdf';
  state.map={colorPrimaryHover:'#776ce9',colorPrimaryBg:'#f2eeff',
    controlHeightSM:'22.5'};
  state.alias={colorLink:'#cc5500',colorTextDisabled:'#919191'};
  state.components.Button={colorPrimary:'#228866'};
  const imported=validated(JSON.parse(JSON.stringify(state)));
  assert.deepEqual(imported.map,state.map);
  assert.deepEqual(imported.alias,state.alias);
  const route=previewParam(imported,'tokens');
  assert.match(route,/Map\.colorPrimaryHover=776ce9/);
  assert.match(route,/Map\.controlHeightSM=22\.5/);
  assert.match(route,/Alias\.colorLink=cc5500/);
  assert.match(route,/Button\.colorPrimary=228866/);
  const code=rustCode(imported);
  assert.match(code,/with_map_overrides\(AntMapTokenOverrides::default\(\)/);
  assert.match(code,/color_primary_bg\(color\(0xf2eeff\)\)/);
  assert.match(code,/control_height_sm\(22\.5\)/);
  assert.match(code,/with_alias_overrides\(AntAliasTokenOverrides::default\(\)/);
  assert.match(code,/color_link\(color\(0xcc5500\)\)/);
  assert.doesNotMatch(code,/22\.5\./);
  assert.match(code,/with_component\(AntComponent::Button/);
});

test('studio layers are editable and partial dark imports get dark defaults',()=>{
  const html=themeDesignerHtml();
  assert.match(html,/theme-map-fields/);
  assert.match(html,/theme-alias-fields/);
  assert.match(html,/Seed Token/);
  assert.match(html,/Component Token/);
  const partial=validated({algorithm:'dark',map:{controlHeightLG:'1000'},
    alias:{colorLink:'not a color',colorError:'#bb4411'}});
  assert.equal(partial.seed.colorBgBase,'#141414');
  assert.equal(partial.seed.colorTextBase,'#f3f3f3');
  assert.equal(partial.map.controlHeightLG,'72');
  assert.equal(partial.alias.colorLink,undefined);
  assert.equal(partial.alias.colorError,'#bb4411');
  assert.equal(validateValue('Infinity','number','controlHeightSM'),null);
  assert.equal(validateValue('1','number','controlHeightSM'),'18');
  const code=rustCode(partial);
  assert.match(code,/seed\.control_height = 32\.;/);
});

test('theme import rejects untrusted values and clamps scalar limits',()=>{
  assert.throws(()=>validated(null),/Expected theme object/);
  assert.throws(()=>validated([]),/Expected theme object/);
  assert.equal(validateValue('#gggggg','color','colorPrimary'),null);
  assert.equal(validateValue('NaN','number','height'),null);
  assert.equal(validateValue('','number','height'),null);
  assert.equal(validateValue('-8','number','height'),'20');
  assert.equal(validateValue('500','number','radius'),'32');
  const theme=validated({algorithm:'unknown',seed:{
    colorPrimary:'javascript:alert(1)',height:'10000',radius:'-300'
  },components:{
    Button:{colorBg:'#00cc66',controlHeight:'1000'},
    ReactOnly:{colorBg:'#bbbbbb'},
  }});
  assert.equal(theme.algorithm,'default');
  assert.equal(theme.seed.colorPrimary,'#1677ff');
  assert.equal(theme.seed.height,'64');
  assert.equal(theme.seed.radius,'0');
  assert.equal(theme.components.Button.colorBg,'#00cc66');
  assert.equal(theme.components.Button.controlHeight,'64');
  assert.equal(theme.components.ReactOnly,undefined);
});
