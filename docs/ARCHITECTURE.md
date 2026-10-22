# Architecture and project background

## Why this repository exists

`caovc/gpui-antd` is an **independent, community-maintained** Ant
Design-inspired Rust component library for native desktop applications
written with [GPUI](https://github.com/zed-industries/zed) and
[GPUI Kit](https://github.com/longbridge/gpui-kit). The goal is
**high visual and interaction fidelity** to the Ant Design design language,
not to transpile React or embed the DOM into a desktop application.

A second goal is documentation with a **real working GPUI WebAssembly
gallery**, analogous in spirit to the GPUI Kit component docs. The same Rust
`Gallery` view is run on native desktop and compiled to
`wasm32-unknown-unknown` in the documentation site. GitHub Pages publishes
the Vite docs shell plus generated `wasm-bindgen` files.

This is alpha v0.1.0, so the current demos illustrate a growing subset of
Ant Design. Claims of visual 1:1 parity require dedicated screenshot
comparison and accessibility/keyboard acceptance tests not yet implemented.

## End-to-end dependency and runtime graph

```text
                Cargo workspace (Rust 2024 + nightly)
   ┌─────────────────────────────────────────────────┐
   │ crates/antd  (gpui-antd, reusable library)       │
   │   AntTheme light/dark + ControlSize              │
   │   GPUI RenderOnce wrappers, state callbacks      │
   │   AntIcon / AntIconAssets (official SVG assets)   │
   └──────────────────────┬──────────────────────────┘
                          │
   ┌──────────────────────▼──────────────────────────┐
   │ crates/gallery (gpui-antd-gallery)               │
   │   Gallery: Entity state + Rust rendering         │
   │   GalleryAssets: AntIconAssets + GPUI Kit assets │
   └───────────────────┬───────────────┬─────────────┘
                       │               │
                  Native GPU       Rust GPUI Web
                       │               │
            cargo run -p ...     cargo rustc --target wasm32
                                       │
                                wasm-bindgen 0.2.121
                                       │
                                website/src/wasm/*
                                       │
                                Vite build (website/)
                                       │
                   HTML docs ── iframe → demo.html
                                      │
                             website/src/demo.js
                             loads real Rust/WASM,
                             waits for GPU canvas
                                       │
                             GitHub Pages deployment
                              /gpui-antd/
```

Dependencies: `gpui-kit = "=0.7.3"` is pinned in root Cargo.toml and
provides the GPUI-compatible widget and behavior ecosystem. App code should
not declare an independent `gpui` crate dependency. WASM bridge uses pinned
`wasm-bindgen = "=0.2.121"`. Gallery is unpublished
(`publish = false`); `gpui-antd` is the consumable crate.

## Component ownership and state

`crates/antd/src/` currently contains:

| File | Purpose |
| --- | --- |
| `controls.rs` | Button, Checkbox, Switch, 24/32/40 px control sizes |
| `input.rs` | Text-editor appearance wrapping GPUI Kit InputState |
| `radio.rs` | Single radio, controlled group, roving key navigation |
| `select.rs` | Controlled single-value Select; Base Popover list, antd trigger |
| `date_picker.rs` | GPUI Kit native calendar/range/datetime |
| `form.rs` | Form presentation and pure Rust validation schema |
| `display.rs` | Tag, Badge, Alert, Card, Progress, Divider |
| `list.rs` | List items, actions, separators, loading and empty presentation |
| `typography.rs` | Title, Text, Paragraph (line clamp), Link |
| `layout.rs` | Space, Flex, 24-column Row/Col with breakpoints |
| `app_layout.rs` | App shell Layout/Header/Sider/Content/Footer and responsive collapse |
| `menu.rs` | Native controlled flat navigation Menu and accessible Sider actions |
| `overlays.rs` | Native PopupMenu, Popover, Dialog and Popconfirm wrappers |
| `basics.rs` | Avatar, Statistic, Empty, Breadcrumb |
| `tabs.rs` | Controlled TabBar + selected native Rust tab panels |
| `tooltip.rs` | Theme-aware native GPUI tooltip over an arbitrary trigger |
| `icons.rs` | Generated 848-icon catalog/embedded asset dispatch + AntIcon |
| `tokens.rs` | AntTheme and light/dark semantic tokens |
| `lib.rs` | Exported public API surface |

Examples of state ownership:

- A checkbox emits `on_change(bool)`; its parent persists that bool.
- A radio group emits a zero-based `usize`; the parent updates selection
  and calls `cx.notify()`.
- Text editing is implemented by GPUI Kit's `Entity<InputState>`; do not
  recreate a DOM text field, caret, IME or clipboard handling.
- Select and date pickers own their GPUI Kit state entities; the Ant wrapper
  styles their rendered element.
- Form `FormSchema` works on pure Rust key/value data and returns
  field-specific validation results; widgets don't secretly manage
  application submission.

For more controls, prefer **behavior primitives** from
`gpui_kit::base`, **rich compositional components** from
`gpui_kit::component`, and state owned in `crates/gallery` or the consuming
application. Refer to the pinned upstream source before inventing GPUI APIs.

## Design tokens and measurements

`AntTheme` currently supplies primary/hover/active colors, text/muted text,
surface/secondary surface, border, success/warning/error colors,
disabled styling, state backgrounds, control height and border radius.
It has `light()` and `dark()` constructors; both are values, not a global
React-style context. Pass `.theme(t)` to each wrapper until a tested theme
provider is introduced.
For Kit-backed interiors, applications explicitly call `t.apply_to_kit(cx)`
after Kit initialization and on theme changes. This synchronizes the Kit
application appearance/shared colors, background tokens and Base projection
across all windows. Gallery does so before creating its widget state. The
operation belongs to application updates; stateless renders do not mutate the
global theme. Full scoped ConfigProvider inheritance remains open (#3).

Baseline:
- primary light blue `#1677ff`, middle control height **32 px**;
- small/middle/large control heights **24/32/40 px**;
- standard border radius **6 px**;
- Typography heading sizes **38/30/24/20/16 px**;
- Space gaps **8/16/24 px**;
- Grid breakpoints: `xs=0, sm=576, md=768, lg=992, xl=1200,
  xxl=1600` logical pixels.

These are approximations of Ant Design's public design language, not a
guarantee that the whole set matches a specific React Ant Design release.

## Icon asset architecture

The `scripts/sync_antd_icons.py` generator reads the official MIT SVGs at
pinned `ant-design/ant-design-icons` Git commit
`7f2516ac91226d2b41f93b35cb5197c8d94f7189`:

```text
upstream SVGs
    ├── crates/antd/assets/antd-icons/{outlined,filled,twotone}/*.svg
    ├── crates/antd/src/icons.rs   (AntIconName registry + include_bytes!)
    └── website/public/antd-icons.json   (name/style search metadata)
```

848 assets: 447 Outlined, 251 Filled, 150 TwoTone. The SVG bytes are loaded
from the Rust asset source on both native and WASM, without making every icon
a network request. Outlined/Filled SVGs render through Kit Icon's single-color
alpha mask, while TwoTone SVGs render through GPUI `img()` as full-color images.
GPUI `svg()` discards interior fill colors, which made the TwoTone light layer
look like a solid black shape. The asset loader injects a 128px intrinsic size
for TwoTone rasterization (the 1024-unit viewBox and original path/fill art
remain unchanged) to avoid huge cached textures. `.spin(true)` uses two native SVG alpha masks rotated in sync, with the
same theme colors. Static TwoTone keeps full-color `img()` painting.
For TwoTone, `.color(primary)` overrides
the primary fill and `.two_tone_colors(primary, secondary)` overrides both fills;
otherwise colors follow `theme.primary` and `theme.info_bg`, including dark
mode. The runtime palette parser in `crates/antd/src/icon_palette.rs`
generates validated virtual asset keys, preserving image-cache isolation and
official SVG bytes. `GalleryAssets` in `crates/gallery/src/lib.rs` composes
`AntIconAssets` with GPUI Kit's own `Assets` source. On browser WASM, Kit's
source may fetch its original icons asynchronously. The docs icon explorer is
only a **DOM index**; selecting a name loads a *single actual* Rust/GPUI canvas
(`Icon:SomeOutlined`). The sample Icons gallery is intentionally bounded,
rather than drawing 848 icons in one viewport.

Don't directly hand-edit generated variants or SVGs. Re-run the pinned
generator and review the diff. The original SVG MIT license is preserved in
`crates/antd/assets/LICENSE-ANT-DESIGN-ICONS` and in
`THIRD_PARTY_NOTICES.md`. Font licensing: the browser builds download
IBM Plex Sans subset from GPUI Kit (SIL OFL).

## Web + GitHub Pages path contract

`website/vite.config.js` fixes the site base to `/gpui-antd/`. The Vite
entrypoints are:

- `website/index.html` + `website/src/main.js`: docs navigation, search,
  API tables, overview, theme switch, and component cards.
- `website/src/icons-explorer.js`: generated icon-name search and style
  filter; uses real GPUI iframe for the selected SVG.
- `website/src/icon-preview.js`: selected-icon navigation lifecycle. The
  requested route is separate from the acknowledged real GPUI document;
  loading/error states hide stale canvases and allow explicit recovery.
- `website/demo.html` + `website/src/demo.js`: wasm-bindgen initialization;
  reads `?component=Name&dark=0|1`, invokes `wasm.run(name,dark)`, then waits
  for a non-zero GPUI canvas and the actual hidden IME textarea. The canvas
  can precede keyboard startup; early ready messages would end host focus
  protection before GPUI's initial focus. A JS return alone doesn't prove paint.
- `crates/gallery/src/lib.rs`: chooses a *Rust* demo based on component name
  (including `Icon:Name` for individual official icons).
- `website/scripts/smoke.mjs`: Playwright Chromium startup checks (headless
  SwiftShader) for real canvas and docs icon search; saves screenshots.

**No duplicate HTML implementation** of a component in the docs. Markdown and
HTML only explain, frame and select the Rust/WASM preview.

## Deployment and known limitations

`.github/workflows/native.yml` validates `cargo check --workspace` and
`cargo test -p gpui-antd --lib`. `.github/workflows/pages.yml` performs
WASM build, Vite build, browser smoke, artifact upload and GitHub Pages deploy.
`.github/workflows/icons-sync.yml` performs pinned upstream icon generation
and commits its results to `main`. The official docs URL is
<https://caovc.github.io/gpui-antd/>.

Do not interpret a successful build as validation of icon *rendering pixels*,
pointer navigation, focus/ARIA quality, drag/drop or performance. Remaining
gaps include visual reference regression, complex overlays, full Ant Design
APIs, detailed high-DPI behavior and internationalization.
