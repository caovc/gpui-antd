/**
 * Searchable official Ant Design icon catalog.
 *
 * This DOM UI only searches names and switches previews. Every icon image is
 * rendered by the real Rust GPUI/WASM gallery, never by an HTML SVG replica.
 * The JSON catalog is generated deterministically from the pinned upstream
 * source by scripts/sync_antd_icons.py and committed alongside the SVG assets.
 */

const escapeHtml = value => String(value)
  .replaceAll('&','&amp;').replaceAll('<','&lt;')
  .replaceAll('>','&gt;').replaceAll('"','&quot;');

// Resolve provenance from the generated manifest, never from the Rust name:
// irregular names such as CiTwoTone should retain their exact SVG slugs.
export function iconSourceUrl(catalog, icon) {
  if (catalog?.source !== 'ant-design/ant-design-icons' ||
      catalog?.revision?.length !== 40 ||
      !/^[0-9a-f]{40}$/.test(catalog?.revision ?? '')) return null;
  const path = /^antd-icons\/(outlined|filled|twotone)\/([a-z0-9]+(?:-[a-z0-9]+)*\.svg)$/.exec(icon?.path ?? '');
  if (!path || path[0] !== icon.path || path[1] !== icon.style) return null;
  return 'https://github.com/' + catalog.source + '/blob/' + catalog.revision +
    '/packages/icons-svg/svg/' + path[1] + '/' + path[2];
}

export function iconExplorerHtml() {
  return '<section id="icon-explorer" class="icon-explorer">' +
    '<div class="doc-section-heading"><h2>Official icon catalog</h2>' +
    '<p>Explore every bundled Ant Design SVG by name and style. Select a result to render that icon in real GPUI/WebAssembly.</p></div>' +
    '<div class="icon-explorer-toolbar">' +
    '<label for="icon-query" class="sr-only">Search icon names</label>' +
    '<input id="icon-query" type="search" placeholder="Search icons, e.g. CheckCircle or Home" autocomplete="off" />' +
    '<label for="icon-style" class="sr-only">Icon style</label>' +
    '<select id="icon-style"><option value="all">All styles</option>' +
    '<option value="outlined">Outlined</option>' +
    '<option value="filled">Filled</option>' +
    '<option value="twotone">TwoTone</option></select>' +
    '</div>' +
    '<p id="icon-count" class="icon-count" role="status" aria-live="polite">Loading official icon catalog…</p>' +
    '<div id="icon-result-grid" class="icon-result-grid" aria-label="Icon name search results"></div>' +
    '<button id="icon-load-more" type="button" class="icon-explorer-load" hidden>Show more icons</button>' +
    '<div class="icon-explorer-preview">' +
    '<div><span class="section-eyebrow">RUST / GPUI / WASM</span>' +
    '<h3 id="icon-selected-title">SearchOutlined</h3>' +
    '<p>Official icon rendered by Rust/GPUI. Adjust its colors, size or rotation and copy the Rust API.</p>' +
    '<p><a id="icon-source" class="icon-source" target="_blank" rel="noopener noreferrer" hidden>View upstream SVG</a></p>' +
    '<div class="icon-preview-controls">' +
    '<label>Size <select id="icon-size" aria-label="Icon preview size">' +
      '<option value="32">32px</option><option value="64" selected>64px</option>' +
      '<option value="96">96px</option><option value="128">128px</option></select></label>' +
    '<label>Primary <input id="icon-primary" type="color" aria-label="Primary color" value="#1677ff" /></label>' +
    '<label id="icon-secondary-wrap">Secondary <input id="icon-secondary" type="color" aria-label="Secondary color" value="#e6f4ff" /></label>' +
    '<label class="icon-spin-toggle"><input id="icon-spin" type="checkbox" /> Spin</label>' +
    '<button type="button" id="icon-reset" class="icon-reset">Reset</button>' +
    '</div>' +
    '<pre id="icon-selected-code"><code>AntIcon::new(AntIconName::SearchOutlined)</code></pre>' +
    '<button id="icon-copy" class="icon-copy" type="button">Copy Rust code</button></div>' +
    '<iframe id="icon-live-preview" title="Selected Ant Design icon in real GPUI WebAssembly" loading="lazy" allow="clipboard-read; clipboard-write"></iframe>' +
    '</div></section>';
}

let catalogPromise;
function fetchCatalog(base) {
  if (!catalogPromise) {
    catalogPromise = fetch(base + 'antd-icons.json')
      .then(response => {
        if (!response.ok) throw new Error('Icon catalog HTTP ' + response.status);
        return response.json();
      }).catch(error => { catalogPromise = null; throw error; });
  }
  return catalogPromise;
}

export async function initIconExplorer(base, dark) {
  const root = document.getElementById('icon-explorer');
  if (!root) return;
  const count = root.querySelector('#icon-count');
  const grid = root.querySelector('#icon-result-grid');
  const query = root.querySelector('#icon-query');
  const style = root.querySelector('#icon-style');
  const selected = root.querySelector('#icon-selected-title');
  const source = root.querySelector('#icon-source');
  const code = root.querySelector('#icon-selected-code code');
  const preview = root.querySelector('#icon-live-preview');
  const copy = root.querySelector('#icon-copy');
  const loadMore = root.querySelector('#icon-load-more');
  const size = root.querySelector('#icon-size');
  const primary = root.querySelector('#icon-primary');
  const secondary = root.querySelector('#icon-secondary');
  const secondaryWrap = root.querySelector('#icon-secondary-wrap');
  const spin = root.querySelector('#icon-spin');
  const reset = root.querySelector('#icon-reset');
  const themePalette = dark
    ? { primary:'#1668dc', secondary:'#111d2c', mono:'#f3f3f3' }
    : { primary:'#1677ff', secondary:'#e6f4ff', mono:'#262626' };
  let visibleLimit = 80;
  let paletteCustomized = false;
  let selectedStyle = 'outlined';

  function resetDisplay() {
    size.value = '64';
    spin.checked = false;
    paletteCustomized = false;
    primary.value = selectedStyle === 'twotone' ? themePalette.primary : themePalette.mono;
    secondary.value = themePalette.secondary;
    secondaryWrap.hidden = selectedStyle !== 'twotone';
  }

  const selectedName = () => selected.textContent;

  function frameSrc(name) {
    const options = ['size=' + size.value];
    if (spin.checked) options.push('spin=1');
    if (paletteCustomized) {
      options.push('primary=' + primary.value.slice(1));
      if (selectedStyle === 'twotone') options.push('secondary=' + secondary.value.slice(1));
    }
    return base + 'demo.html?component=' +
      encodeURIComponent('Icon:' + name + '|' + options.join('|')) +
      '&dark=' + (dark ? '1' : '0');
  }

  function refreshPreview() {
    const name = selectedName();
    if (!name) return;
    const nameCode = 'AntIcon::new(AntIconName::' + name + ')';
    const defaultColor = selectedStyle === 'twotone' ? themePalette.primary : themePalette.mono;
    let snippet = nameCode + '\\n    .size(' + size.value + '.)\\n    .theme(theme)';
    if (paletteCustomized) {
      const p = primary.value.replace('#','0x');
      if (selectedStyle === 'twotone') {
        const s = secondary.value.replace('#','0x');
        snippet += '\\n    .two_tone_colors(gpui_antd::color(' + p + '), gpui_antd::color(' + s + '))';
      } else {
        snippet += '\\n    .color(gpui_antd::color(' + p + '))';
      }
    }
    if (spin.checked) snippet += '\\n    .spin(true)';
    code.textContent = snippet.replaceAll('\\n','\n');
    const target = frameSrc(name);
    if (!preview.src.endsWith(target)) preview.src = target;
  }

  let catalog;
  try { catalog = await fetchCatalog(base); }
  catch (error) {
    if (root.isConnected) count.textContent = 'Unable to load icon catalog: ' + error.message;
    return;
  }
  if (!root.isConnected) return;
  if (!Array.isArray(catalog.icons) || !catalog.icons.length) {
    count.textContent = 'No icons found in the bundled catalog.';
    return;
  }

  function selectIcon(name) {
    const match = catalog.icons.find(icon => icon.name === name);
    if (!match) return;
    const switchedStyle = selectedStyle !== match.style;
    selected.textContent = name;
    const sourceUrl = iconSourceUrl(catalog, match);
    source.hidden = !sourceUrl;
    if (sourceUrl) source.href = sourceUrl;
    else source.removeAttribute('href');
    source.setAttribute('aria-label', 'View upstream SVG for ' + name + ' (opens in new tab)');
    selectedStyle = match.style;
    if (switchedStyle) resetDisplay();
    refreshPreview();
    grid.querySelectorAll('button[data-icon-name]').forEach(button => {
      const active = button.dataset.iconName === name;
      button.classList.toggle('selected', active);
      button.setAttribute('aria-pressed', String(active));
    });
  }

  function updateResults() {
    const q = query.value.trim().toLowerCase();
    const wantedStyle = style.value;
    const filtered = catalog.icons.filter(icon =>
      (wantedStyle === 'all' || icon.style === wantedStyle) &&
      (!q || icon.name.toLowerCase().includes(q))
    );
    const shown = Math.min(filtered.length, visibleLimit);
    count.textContent = filtered.length + ' matching icons · ' +
      catalog.count + ' official SVGs included · showing ' + shown;
    grid.innerHTML = filtered.length
      ? filtered.slice(0,visibleLimit).map(icon =>
          '<button type="button" class="icon-name-hit" data-icon-name="' +
          escapeHtml(icon.name) + '" aria-pressed="false">' +
          '<span>' + escapeHtml(icon.name) + '</span>' +
          '<small>' + escapeHtml(icon.style) + '</small></button>'
        ).join('')
      : '<p class="icon-empty-results">No matching icons. Try a shorter name.</p>';
    loadMore.hidden = filtered.length <= visibleLimit;
    loadMore.textContent = 'Show more icons (' + shown + ' of ' + filtered.length + ')';
    const active = selected.textContent;
    grid.querySelectorAll('button[data-icon-name]').forEach(button => {
      if (button.dataset.iconName === active) {
        button.classList.add('selected');
        button.setAttribute('aria-pressed','true');
      }
    });
  }

  for (const input of [size, spin]) {
    input.addEventListener('change', refreshPreview);
  }
  for (const input of [primary, secondary]) {
    input.addEventListener('change', () => {
      paletteCustomized = true;
      refreshPreview();
    });
  }
  reset.addEventListener('click', () => {
    resetDisplay();
    refreshPreview();
  });
  resetDisplay();
  query.addEventListener('input', () => { visibleLimit = 80; updateResults(); });
  style.addEventListener('change', () => { visibleLimit = 80; updateResults(); });
  loadMore.addEventListener('click', () => { visibleLimit += 80; updateResults(); });
  grid.addEventListener('click', event => {
    const button = event.target.closest('button[data-icon-name]');
    if (button) selectIcon(button.dataset.iconName);
  });
  copy.addEventListener('click', async () => {
    try {
      await navigator.clipboard.writeText(code.textContent);
      copy.textContent = 'Copied ✓';
    } catch {
      copy.textContent = 'Copy unavailable — select the code above';
    }
    setTimeout(() => { if (copy.isConnected) copy.textContent = 'Copy Rust code'; }, 1900);
  });

  updateResults();
  selectIcon('SearchOutlined');
}
