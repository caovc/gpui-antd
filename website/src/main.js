import './style.css';

const BASE = import.meta.env.BASE_URL;
const REPO = 'https://github.com/caovc/gpui-antd';
const ITEMS = [
  {name:'Button', group:'General', icon:'▣', summary:'Trigger an action or event.', description:'Primary, default, dashed, text and link button styles with size, danger, disabled, ghost and loading states.', code:'AntButton::new("submit", "Submit")\\n    .primary()\\n    .size(ControlSize::Middle)\\n    .on_click(|_, _, _| { /* action */ })', props:[['new(id, label)','ElementId, SharedString','Create a button'],['variant(value)','ButtonVariant','Default / Primary / Dashed / Text / Link'],['size(value)','ControlSize','Small / Middle / Large'],['danger(bool)','bool','Danger presentation'],['disabled(bool)','bool','Disable pointer and keyboard activation'],['loading(bool)','bool','Show pending state'],['on_click(handler)','closure','Pointer / Enter / Space']]},
  {name:'Input', group:'Data Entry', icon:'▤', summary:'Enter text in a field.', description:'Ant Design-style frame around GPUI Kit’s native editor, preserving cursor, selection, clipboard and platform input handling.', code:'let state = cx.new(|cx| InputState::new(window, cx)\\n    .placeholder("Enter text"));\\n\\nAntInput::new(&state).clearable(true)', props:[['new(state)','Entity<InputState>','Bind GPUI text editor'],['size(value)','ControlSize','Small / Middle / Large'],['disabled(bool)','bool','Prevent editing'],['clearable(bool)','bool','Display clear action']]},
  {name:'Checkbox', group:'Data Entry', icon:'☑', summary:'Select one or more options.', description:'Controlled checkbox with GPUI Base focus, keyboard navigation, checked presentation and accessibility semantics.', code:'AntCheckbox::new("terms", "I agree")\\n    .checked(accepted)\\n    .on_change(|checked, _, _| { /* update */ })', props:[['new(id, label)','ElementId, SharedString','Create a checkbox'],['checked(bool)','bool','Controlled value'],['disabled(bool)','bool','Disable control'],['on_change(handler)','closure','Receive next checked value']]},
  {name:'Switch', group:'Data Entry', icon:'◉', summary:'Toggle an immediate state.', description:'Compact switch with controlled value and GPUI Base keyboard and semantic support.', code:'AntSwitch::new("notifications")\\n    .checked(enabled)\\n    .on_change(|next, _, _| { /* update */ })', props:[['new(id)','ElementId','Create a switch'],['checked(bool)','bool','Controlled value'],['disabled(bool)','bool','Disable control'],['on_change(handler)','closure','Receive next checked value']]},
  {name:'Tag', group:'Data Display', icon:'◇', summary:'Label and categorize objects.', description:'Semantic status tags styled with Ant Design blue, green, gold, red, purple, and default color families.', code:'AntTag::new("Processing")\\n    .tone(Tone::Blue)', props:[['new(text)','SharedString','Label'],['tone(value)','Tone','Default / Blue / Green / Gold / Red / Purple'],['bordered(bool)','bool','Show border']]},
  {name:'Badge', group:'Data Display', icon:'●', summary:'Highlight a count or status.', description:'Small counters and status dots with the traditional Ant Design status red.', code:'AntBadge::count(42)\\n    .color(theme.error)', props:[['count(usize)','usize','Display count, capped at 99+'],['dot()','—','Show dot instead of count'],['color(Hsla)','Hsla','Custom badge color']]},
  {name:'Card', group:'Data Display', icon:'▧', summary:'Display content in a container.', description:'A grouped information surface with Ant Design padding, border and optional title.', code:'AntCard::new()\\n    .title("Project status")\\n    .child(div().child("Everything is up to date"))', props:[['new()','—','Create empty card'],['title(text)','SharedString','Header text'],['bordered(bool)','bool','Show border'],['child(element)','IntoElement','Append GPUI content']]},
  {name:'Progress', group:'Data Display', icon:'◔', summary:'Track an operation.', description:'Linear determinate progress with a percentage label and semantic color overrides.', code:'AntProgress::new(68.0)\\n    .tone(Tone::Green)', props:[['new(percent)','f32','Progress clamped to 0–100'],['tone(value)','Tone','Color family'],['show_info(bool)','bool','Show percentage']]},
  {name:'Alert', group:'Feedback', icon:'ⓘ', summary:'Display a short notice.', description:'Contextual banners for information, success, warning and error feedback.', code:'AntAlert::new("Saved successfully")\\n    .tone(Tone::Green)', props:[['new(message)','SharedString','Message'],['tone(value)','Tone','Blue / Green / Gold / Red']]},
  {name:'Divider', group:'Layout', icon:'━', summary:'Separate content.', description:'A lightweight horizontal divider matching Ant Design color tokens.', code:'AntDivider::new()', props:[['new()','—','Create a horizontal divider'],['theme(value)','AntTheme','Theme override']]}
];
const GROUPS = ['General','Data Entry','Data Display','Feedback','Layout'];
const byName = Object.fromEntries(ITEMS.map(x => [x.name, x]));
const content = document.querySelector('#content');
const sidebar = document.querySelector('#sidebar');
const toc = document.querySelector('#toc');
let dark = localStorage.getItem('gpui-antd-dark') === '1';
document.documentElement.classList.toggle('dark', dark);
const esc = value => String(value).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;').replaceAll('"','&quot;');
const link = name => '#/components/' + encodeURIComponent(name);
const iframe = name => '<iframe class="wasm-iframe" title="Live GPUI WebAssembly preview of ' + esc(name) + '" loading="lazy" allow="clipboard-read; clipboard-write" src="' + BASE + 'demo.html?component=' + encodeURIComponent(name) + '&dark=' + (dark ? '1' : '0') + '"></iframe>';
const codeBox = code => '<div class="code-toolbar"><span class="code-label"><span class="code-dot"></span> RUST</span><button class="copy-button" type="button" data-copy>Copy code</button></div><pre class="code-pre"><code>' + esc(code.replaceAll('\\n','\n')) + '</code></pre>';

function sidebarHtml() {
  return GROUPS.map(group => {
    const filtered = ITEMS.filter(item => item.group === group);
    return '<div class="nav-group"><p class="nav-group-title">' + group.toUpperCase() + '</p>' +
      filtered.map(item => '<a class="nav-item component-nav" data-nav="' + item.name + '" href="' + link(item.name) + '"><span class="nav-icon">' + item.icon + '</span><span>' + item.name + '</span><span class="nav-chevron">›</span></a>').join('') + '</div>';
  }).join('');
}
document.getElementById('sidebar-components').innerHTML = sidebarHtml();

function home() {
  content.innerHTML = '<div class="page-container home-page">' +
    '<div class="hero"><div class="eyebrow"><span class="pulse-dot"></span> OPEN SOURCE · RUST NATIVE</div>' +
    '<h1>Design for <span>native.</span><br>Built with Rust.</h1>' +
    '<p class="hero-desc">The familiar Ant Design language, thoughtfully rebuilt as GPU-accelerated, cross-platform GPUI components.</p>' +
    '<div class="hero-actions"><a class="primary-action" href="' + link('Button') + '">Explore components <span>→</span></a>' +
    '<a class="secondary-action" href="#/getting-started">Get started <span>↗</span></a></div>' +
    '<div class="hero-meta"><span><b>10</b> initial components</span><span><b>100%</b> Rust preview</span><span><b>WASM</b> live gallery</span></div></div>' +
    '<div class="feature-title"><span>LIVE COMPONENT PREVIEW</span><span class="preview-meta"><span class="green-dot"></span> RUNNING GPUI IN WEBASSEMBLY</span></div>' +
    '<div class="home-preview">' + iframe('Button') + '</div>' +
    '<div class="section-title"><div><p class="section-eyebrow">EXPLORE THE LIBRARY</p><h2>Components, crafted for GPUI.</h2><p>Browse the first building blocks. Each demo renders from the Rust component library, not HTML replicas.</p></div></div>' +
    '<div class="component-grid">' + ITEMS.map(item =>
      '<a class="component-card" href="' + link(item.name) + '"><span class="card-icon">' + item.icon + '</span><div><h3>' + item.name + ' <span>→</span></h3><p>' + item.summary + '</p></div></a>'
    ).join('') + '</div>' +
    '<div class="footer-callout"><div><strong>Built on GPUI Kit + GPUI Base</strong><p>Behavior primitives for keyboard, focus and semantic accessibility. Ant Design tokens for the finish.</p></div><a href="' + REPO + '" target="_blank" rel="noreferrer">View repository ↗</a></div>' +
    '</div>';
  toc.innerHTML = '<span class="toc-title">ON THIS PAGE</span><a href="#/">Overview</a><a href="#/components/Button">Live preview</a><a href="#/getting-started">Installation</a>';
}
function docs(item) {
  const rows = item.props.map(p => '<tr><td><code>' + esc(p[0]) + '</code></td><td><code>' + esc(p[1]) + '</code></td><td>' + esc(p[2]) + '</td></tr>').join('');
  content.innerHTML = '<div class="page-container component-page">' +
    '<div class="breadcrumbs"><a href="#/">GPUI Ant Design</a><span>›</span><span>Components</span><span>›</span><b>' + item.name + '</b></div>' +
    '<div class="component-heading"><div><span class="section-eyebrow">' + esc(item.group.toUpperCase()) + ' / COMPONENT</span><h1>' + item.name + '</h1><p>' + esc(item.summary) + '</p></div><a class="edit-link" href="' + REPO + '/tree/main/crates/antd/src" target="_blank" rel="noreferrer">Source ↗</a></div>' +
    '<div class="intro-text">' + esc(item.description) + '</div>' +
    '<div class="doc-section-heading" id="examples"><h2>Examples</h2><p>Interact with the component below. Rendered by the <strong>gpui-antd</strong> Rust crate via WebAssembly.</p></div>' +
    '<div class="example-card"><div class="example-toolbar"><span class="example-title"><span class="play-triangle">▶</span> Basic usage &amp; states</span><span class="wasm-chip"><span class="green-dot"></span> GPUI / WASM</span></div>' +
    iframe(item.name) +
    '<div class="example-bottom"><span>Native GPUI elements · WebGPU / WebGL2</span><a href="' + BASE + 'demo.html?component=' + item.name + '&dark=' + (dark ? '1' : '0') + '" target="_blank" rel="noopener">Open standalone ↗</a></div></div>' +
    '<div class="doc-section-heading" id="code"><h2>Usage</h2><p>Import from <code>gpui_antd</code> in your Rust application.</p></div>' +
    '<div class="code-panel">' + codeBox('use gpui_antd::*;\\n\\n' + item.code) + '</div>' +
    '<div class="doc-section-heading" id="api"><h2>API</h2><p>Current public builder surface. These APIs are experimental and may evolve before 1.0.</p></div>' +
    '<div class="api-table-wrap"><table><thead><tr><th>Property</th><th>Type</th><th>Description</th></tr></thead><tbody>' + rows + '</tbody></table></div>' +
    '<div class="tip-box"><b>Implementation note</b><p>This is an early milestone toward Ant Design parity. Styling and behavior are being implemented incrementally; not every feature of the React library is available.</p></div>' +
    '<div class="next-link"><span>Ready to contribute?</span><a href="' + REPO + '/issues" target="_blank" rel="noreferrer">Help build the next component →</a></div>' +
    '</div>';
  toc.innerHTML = '<span class="toc-title">ON THIS PAGE</span><a href="#examples" data-anchor>Examples</a><a href="#code" data-anchor>Usage</a><a href="#api" data-anchor>API</a><div class="toc-divider"></div><span class="toc-caption">CURRENT STATUS</span><span class="alpha-status"><span class="pulse-dot"></span> Experimental</span>';
}
function gettingStarted() {
  content.innerHTML = '<div class="page-container guide-page">' +
    '<div class="breadcrumbs"><a href="#/">GPUI Ant Design</a><span>›</span>Guide<span>›</span><b>Getting Started</b></div>' +
    '<span class="section-eyebrow">START HERE</span><h1>Getting Started</h1>' +
    '<p class="page-lead">Build native Rust interfaces with familiar Ant Design components, while reusing GPUI Kit’s interaction infrastructure.</p>' +
    '<div class="doc-section-heading" id="install"><h2>1. Add the dependency</h2><p>Until the first crates.io release, depend on the repository directly.</p></div>' +
    '<div class="code-panel">' + codeBox('[dependencies]\\ngpui-antd = { git = "https://github.com/caovc/gpui-antd" }') + '</div>' +
    '<div class="doc-section-heading" id="example"><h2>2. Render a component</h2><p>Initialize GPUI Kit before using the controls.</p></div>' +
    '<div class="code-panel">' + codeBox('use gpui_antd::AntButton;\\nuse gpui_kit::*;\\n\\n// Inside your impl Render for MyView:\nAntButton::new("save", "Save changes").primary()') + '</div>' +
    '<div class="doc-section-heading" id="preview"><h2>3. Run the gallery</h2><p>The same Gallery view is compiled for desktop and WASM, without a DOM imitation.</p></div>' +
    '<div class="code-panel">' + codeBox('cargo run -p gpui-antd-gallery\\n\\n# Build WebAssembly + docs locally:\nbash scripts/build-wasm.sh\\ncd website && npm install && npm run dev') + '</div>' +
    '<div class="tip-box"><b>Platform requirements</b><p>Native GPUI requires the Rust nightly toolchain pinned in this repository and the GPUI Kit native system libraries. WASM needs a modern browser with WebGPU or WebGL2.</p></div>' +
    '<div class="doc-section-heading" id="architecture"><h2>Architecture</h2></div>' +
    '<div class="arch-graph"><div><b>gpui-antd</b><span>Components + Ant tokens</span></div><span>↓</span><div><b>gpui-kit / gpui-base</b><span>Behavior, focus, input, rendering</span></div><span>↓</span><div><b>GPUI</b><span>Native desktop + WASM canvas</span></div></div>' +
    '</div>';
  toc.innerHTML = '<span class="toc-title">ON THIS PAGE</span><a href="#install" data-anchor>Installation</a><a href="#example" data-anchor>First component</a><a href="#preview" data-anchor>Gallery</a><a href="#architecture" data-anchor>Architecture</a>';
}
function route() {
  const path = decodeURIComponent(location.hash.slice(1) || '/');
  if (path.startsWith('/components/')) {
    const name = path.slice('/components/'.length);
    if (byName[name]) docs(byName[name]); else home();
  } else if (path === '/getting-started') gettingStarted(); else home();
  document.querySelectorAll('[data-nav]').forEach(node => {
    const id = path.startsWith('/components/') ? path.slice('/components/'.length) : path === '/getting-started' ? 'getting-started' : 'home';
    node.classList.toggle('active', node.dataset.nav === id);
  });
  document.querySelectorAll('[data-top]').forEach(node => {
    const active = path.startsWith('/components/') ? 'components' : path === '/getting-started' ? 'guide' : 'home';
    node.classList.toggle('current', node.dataset.top === active);
  });
  sidebar.classList.remove('mobile-open');
  window.scrollTo(0,0);
}
addEventListener('hashchange',route);
route();

document.getElementById('theme-btn').addEventListener('click', () => {
  dark = !dark;
  localStorage.setItem('gpui-antd-dark', dark ? '1' : '0');
  document.documentElement.classList.toggle('dark', dark);
  route();
});
document.getElementById('menu-btn').addEventListener('click', () => sidebar.classList.toggle('mobile-open'));
document.addEventListener('click', async event => {
  const copyButton = event.target.closest('[data-copy]');
  if (copyButton) {
    const text = copyButton.closest('.code-panel').querySelector('code').textContent;
    try { await navigator.clipboard.writeText(text); copyButton.textContent = 'Copied ✓'; setTimeout(()=>copyButton.textContent='Copy code',1600); }
    catch { copyButton.textContent = 'Select code to copy'; }
  }
  const anchor = event.target.closest('[data-anchor]');
  if (anchor) {
    event.preventDefault();
    document.querySelector(anchor.getAttribute('href'))?.scrollIntoView({behavior:'smooth',block:'start'});
  }
});
const overlay = document.getElementById('search-overlay');
const searchInput = document.getElementById('search-input');
const results = document.getElementById('search-results');
function searchItems(value) {
  const term = value.trim().toLowerCase();
  const matches = ITEMS.filter(item => (item.name + ' ' + item.group + ' ' + item.summary).toLowerCase().includes(term));
  results.innerHTML = matches.length ? matches.map(item =>
    '<a class="search-hit" href="' + link(item.name) + '"><span class="card-icon">' + item.icon + '</span><span><strong>' + item.name + '</strong><small>' + item.summary + '</small></span><span>↵</span></a>'
  ).join('') : '<div class="empty-search">No matching components.</div>';
  results.querySelectorAll('a').forEach(a => a.addEventListener('click',closeSearch));
}
function openSearch() { overlay.hidden=false; searchInput.value=''; searchItems(''); requestAnimationFrame(()=>searchInput.focus()); }
function closeSearch() { overlay.hidden=true; }
document.querySelector('#search-shortcut').addEventListener('click',openSearch);
document.querySelectorAll('[data-close-search]').forEach(el => el.addEventListener('click',closeSearch));
searchInput.addEventListener('input', e => searchItems(e.target.value));
document.addEventListener('keydown', event => {
  const input = event.target.closest('input, textarea, [contenteditable]');
  if (event.key==='Escape' && !overlay.hidden) {event.preventDefault();closeSearch();}
  if ((event.key==='/' && !input) || ((event.metaKey||event.ctrlKey) && event.key==='k')) {event.preventDefault();openSearch();}
  if (event.key==='Enter' && !overlay.hidden && document.activeElement===searchInput) {
    const first=results.querySelector('a'); if(first) {location.hash=first.getAttribute('href');closeSearch();}
  }
});
