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
    '<div class="icon-explorer-preview">' +
    '<div><span class="section-eyebrow">RUST / GPUI / WASM</span>' +
    '<h3 id="icon-selected-title">SearchOutlined</h3>' +
    '<p>Original SVG, rendered on the GPUI canvas. Click a name above to change the preview.</p>' +
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
  const code = root.querySelector('#icon-selected-code code');
  const preview = root.querySelector('#icon-live-preview');
  const copy = root.querySelector('#icon-copy');

  const frameSrc = iconName =>
    base + 'demo.html?component=' + encodeURIComponent('Icon:' + iconName) +
    '&dark=' + (dark ? '1' : '0');

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
    selected.textContent = name;
    code.textContent = 'AntIcon::new(AntIconName::' + name + ')';
    preview.src = frameSrc(name);
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
    count.textContent = filtered.length + ' matching icons · ' +
      catalog.count + ' official SVGs included · showing up to 80 results';
    grid.innerHTML = filtered.length
      ? filtered.slice(0,80).map(icon =>
          '<button type="button" class="icon-name-hit" data-icon-name="' +
          escapeHtml(icon.name) + '" aria-pressed="false">' +
          '<span>' + escapeHtml(icon.name) + '</span>' +
          '<small>' + escapeHtml(icon.style) + '</small></button>'
        ).join('')
      : '<p class="icon-empty-results">No matching icons. Try a shorter name.</p>';
    const active = selected.textContent;
    grid.querySelectorAll('button[data-icon-name]').forEach(button => {
      if (button.dataset.iconName === active) {
        button.classList.add('selected');
        button.setAttribute('aria-pressed','true');
      }
    });
  }

  query.addEventListener('input', updateResults);
  style.addEventListener('change', updateResults);
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
