import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, statSync } from 'node:fs';
import { iconSourceUrl } from '../src/icons-explorer.js';

const catalog = JSON.parse(readFileSync(new URL('../public/antd-icons.json', import.meta.url)));
const pin = '7f2516ac91226d2b41f93b35cb5197c8d94f7189';

test('every official icon links to its bundled SVG at the pinned upstream commit', () => {
  assert.equal(catalog.revision, pin);
  assert.equal(catalog.count, 848);
  assert.equal(catalog.icons.length, catalog.count);
  const urls = new Set();
  const counts = { outlined: 0, filled: 0, twotone: 0 };
  for (const icon of catalog.icons) {
    assert.ok(statSync(new URL('../../crates/antd/assets/' + icon.path, import.meta.url)).isFile());
    const url = new URL(iconSourceUrl(catalog, icon));
    assert.equal(url.origin, 'https://github.com');
    assert.equal(url.pathname, '/ant-design/ant-design-icons/blob/' + pin +
      '/packages/icons-svg/svg/' + icon.path.slice('antd-icons/'.length));
    assert.equal(url.search, '');
    assert.equal(url.hash, '');
    urls.add(url.href);
    counts[icon.style]++;
  }
  assert.equal(urls.size, 848);
  assert.deepEqual(counts, { outlined: 447, filled: 251, twotone: 150 });
});

test('source paths come from metadata rather than inferring Rust name spelling', () => {
  for (const [name, path] of [
    ['SearchOutlined', 'outlined/search.svg'],
    ['CheckCircleFilled', 'filled/check-circle.svg'],
    ['CiTwoTone', 'twotone/ci.svg'],
    ['CiCircleTwoTone', 'twotone/ci-circle.svg'],
  ]) {
    const icon = catalog.icons.find(icon => icon.name === name);
    assert.ok(icon, name);
    assert.equal(iconSourceUrl(catalog, icon),
      'https://github.com/ant-design/ant-design-icons/blob/' + pin + '/packages/icons-svg/svg/' + path);
    assert.equal(iconSourceUrl(catalog, { ...icon, name: 'UnrelatedName' }), iconSourceUrl(catalog, icon));
  }
});

test('missing or invalid provenance never produces a source link', () => {
  const icon = catalog.icons[0];
  for (const value of [null, {}, { ...catalog, source: 'other/repo' },
    { ...catalog, revision: 'main' }, { ...catalog, revision: pin + '\n' }]) {
    assert.equal(iconSourceUrl(value, icon), null);
  }
  for (const value of [null, {}, { ...icon, style: 'filled' },
    ...['../search.svg', 'antd-icons/outlined/../../search.svg',
      'antd-icons/outlined/search.svg?raw=1', 'antd-icons/outlined/search.svg\n',
      'https://example.com/search.svg'].map(path => ({ ...icon, path }))]) {
    assert.equal(iconSourceUrl(catalog, value), null);
  }
});
