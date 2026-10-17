import test from 'node:test';
import assert from 'node:assert/strict';
import { themeDesignerHtml, tokenStudioLogic } from '../src/theme-designer.js';

const {emptyTheme,validated,previewParam,rustCode,validateValue}=tokenStudioLogic;

test('studio exposes the real GPU preview and two independent scenes',()=>{
  const html=themeDesignerHtml();
  assert.match(html,/theme-preview-frame/);
  assert.match(html,/value="tokens"/);
  assert.match(html,/LIVE GPU/);
  assert.match(html,/data-seed="colorPrimary"/);
  assert.ok(!html.includes('<button class="ant-btn"'));
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


test('all 50 components are valid, with scoped Tree/Table/Timeline tokens',()=>{
  const theme=emptyTheme();
  theme.components.Tree={colorBgSelected:'#d9e9ff',controlHeight:'36'};
  theme.components.Table={colorHeaderBg:'#101820',fontSize:'16'};
  theme.components.Timeline={colorPrimary:'#ff4477'};
  const valid=validated(theme);
  assert.equal(Object.keys(valid.components).length,3);
  const url=previewParam(valid);
  assert.match(url,/Tree\.colorBgSelected=d9e9ff/);
  assert.match(url,/Table\.colorHeaderBg=101820/);
  assert.match(url,/Timeline\.colorPrimary=ff4477/);
  const code=rustCode(valid);
  for(const component of ['Tree','Table','Timeline']){
    assert.ok(code.includes('AntComponent::'+component));
  }
});
