# Pinned icon source links — Issue #4

The icon explorer will expose a single **View upstream SVG** link beside the
selected icon. Selecting any search result updates the link and its accessible
name together with the existing Rust GPUI/WASM preview. Filtering without
selection keeps the current source link; color, size and spin do not change
provenance. The link opens a new tab and announces that behavior.

Use the existing generated manifest's source repository, 40-digit revision
and asset path. Convert `antd-icons/<style>/<slug>.svg` to the upstream
`packages/icons-svg/svg/<style>/<slug>.svg` path. Do not infer slugs from Rust
enum names, duplicate URLs in generated assets, or target the mutable main
branch. Invalid provenance metadata leaves the link hidden without an href.

An individual source link on every result would repeat an action 848 times;
a single selected-icon link keeps the search grid focused. Embedding a source
URL in every generated record would duplicate metadata already present.

Validation: check every catalog entry against local bundled files and all
paths in the upstream Git tree at the pin. Cover irregular names
and invalid metadata in Node tests. Chromium smoke exercises the initial
selection, each style, keyboard focus, empty search and control edits, with
real GPUI canvases in desktop light and mobile dark. Keep Native and full
WASM/browser/icon CI gates; verify main deploy before claiming publication.
