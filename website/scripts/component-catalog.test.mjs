import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { PUBLIC_COMPONENTS, WITHDRAWN_COMPONENTS } from '../src/component-names.js';
import { tokenStudioLogic, themeDesignerHtml } from '../src/theme-designer.js';

const read = path => readFile(new URL(path, import.meta.url), 'utf8');

test('native Gallery, docs, Studio preview and smoke share the public catalog', async () => {
  const gallery = await read('../../crates/gallery/src/lib.rs');
  const block = gallery.match(/pub const COMPONENTS: &\[&str\] = &\[([\s\S]*?)\];/)[1];
  const native = [...block.matchAll(/"([A-Za-z]+)"/g)].map(match => match[1]);
  assert.deepEqual(PUBLIC_COMPONENTS, native);
  assert.equal(new Set(native).size, native.length);
  const docs = await read('../src/main.js');
  const documented = [...docs.matchAll(/\{name:'([^']+)'/g)].map(match => match[1]);
  assert.deepEqual([...documented].sort(), [...native].sort());
  const html = themeDesignerHtml();
  const options = html.match(/id="theme-component">([\s\S]*?)<\/select>/)[1];
  assert.deepEqual([...options.matchAll(/value="([^"]+)"/g)].map(match => match[1]), native);
  assert.match(await read('../index.html'), new RegExp('sidebar-count">' + native.length + '<'));
  assert.match(await read('../../README.md'), new RegExp('\\*\\*' + native.length + ' alpha component pages'));
  assert.match(await read('./smoke.mjs'), /const components = \[\.\.\.PUBLIC_COMPONENTS,/);
});

test('withdrawn names cannot resolve to public Studio previews or Rust exports', async () => {
  const exports = await read('../../crates/antd/src/lib.rs');
  for (const name of ['AntConfig', 'AntDirection'])
    assert.doesNotMatch(exports, new RegExp('\\b' + name + '\\b'));
  const state = tokenStudioLogic.emptyTheme();
  for (const name of WITHDRAWN_COMPONENTS) {
    assert.ok(!PUBLIC_COMPONENTS.includes(name));
    assert.doesNotMatch(exports, new RegExp('\\bAnt' + name + '\\b'));
    assert.match(tokenStudioLogic.previewParam(state, 'single', name), /preview=Button(?:\||$)/);
  }
});
