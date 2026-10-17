# GPUI Ant Design

**An open-source, Ant Design-inspired component library for native Rust GPUI.**

[![Native crate CI](https://github.com/caovc/gpui-antd/actions/workflows/native.yml/badge.svg)](https://github.com/caovc/gpui-antd/actions/workflows/native.yml)
[![WASM Docs](https://github.com/caovc/gpui-antd/actions/workflows/pages.yml/badge.svg)](https://github.com/caovc/gpui-antd/actions/workflows/pages.yml)

**[Documentation and real WASM component previews](https://caovc.github.io/gpui-antd/)**

> **Early alpha (0.1.0).** The goal is visual and interaction fidelity with
> Ant Design, NOT compatibility with the React component API. This is the first
> end-to-end milestone, not the completed Ant Design catalog.

## Contributor / coding agent guide

This repository includes a canonical [AGENTS.md](AGENTS.md) entry point
for code agents and maintainers, with detailed
[architecture](docs/ARCHITECTURE.md),
[development & CI](docs/DEVELOPMENT.md), and
[component playbook](docs/COMPONENT_PLAYBOOK.md) documents. Read these
before implementing components or changing the WASM gallery. See
[CONTRIBUTING.md](CONTRIBUTING.md) for contribution expectations.

## Design philosophy

- **Rust-native:** components are GPUI elements. They run on desktop GPUs.
- **One source of truth:** the documentation gallery compiles the **same Rust
  controls** to WASM through GPUI Kit Web. No imitation HTML UI.
- **Behavior before paint:** Button, Checkbox, Radio and Switch reuse GPUI Base
  keyboard, focus, semantic accessibility and controlled-state behavior.
- **Design tokens:** Ant Design-inspired light/dark colors and 24/32/40 px
  control sizes, with a 6 px base radius.
- **Open source:** independent implementations of Ant Design visual guidance.

## First milestone component set

Button, Input, Select, DatePicker, Form, Checkbox, Radio/RadioGroup, Switch, Tag, Badge, Alert, Card, Progress, Divider, Icons, Typography, Space, Flex, Grid, Avatar, Statistic,
Empty, Breadcrumb, Tabs, Tooltip, application Layout, Dropdown, Popover, Modal and Popconfirm.

Input now supports outlined, filled and borderless variants, warning/error status,
prefix/suffix decorations, password reveal, sizes and read-only/disabled
states by wrapping GPUI Kit's native editing engine.

The visible appearance of these controls is implemented, but some Ant Design
edge cases (icon-only button modes, advanced form validation and full
hover/focus/a11y parity) are not yet implemented.

## Repository layout

```
crates/antd/     # Reusable gpui-antd Rust component crate
crates/gallery/  # One Rust gallery for native and browser GPUI Web
website/         # Docs, API examples, search, actual WASM iframe
scripts/         # Reproducible wasm-bindgen build pipeline
.github/workflows/pages.yml    # GitHub Pages (WASM + Vite)
.github/workflows/native.yml   # Native Linux build and tests
```

## Native quick start

Rust nightly and GPUI Kit's system prerequisites are required. Follow the
[GPUI Kit installation guide](https://gpui-kit.com/docs/installation/).

```sh
git clone https://github.com/caovc/gpui-antd.git
cd gpui-antd
cargo run -p gpui-antd-gallery
```

To consume from another GPUI app:

```toml
[dependencies]
gpui-antd = { git = "https://github.com/caovc/gpui-antd" }
gpui-kit = "=0.7.3"
```

```rust
use gpui_antd::{AntButton, ControlSize};

let save = AntButton::new("save", "Save")
    .primary()
    .size(ControlSize::Middle)
    .on_click(|_, _, _| println!("saved"));
```

Initialize GPUI Kit (`gpui_kit::init(cx)`) before creating component views.

FormSchema provides deterministic Rust validation (required, length, email and
matching fields). AntForm and AntFormItem use the same native GPUI input
components and show inline validation errors. The form state is application
owned; no HTML form controls are rendered.

DatePicker is a Rust wrapper around GPUI Kit's single-date/range/datetime
calendar controls; the gallery exercises each mode through real GPUI/WASM.

Select is a typed single-value wrapper around GPUI Kit's searchable SelectState.
It supports filtering, clearable values, popup width, placeholders and disabled
states; multiple selection remains a future milestone.

The new RadioGroup supports a controlled selected index, roving Tab stops,
arrows, Home/End navigation and disabled-item skipping. Both layouts and the
radio-button style have examples in the interactive docs.

## Tabs and Tooltip

`AntTabs` wraps the GPUI Kit native TabBar with controlled selection,
line/card/pill/segmented styling, selected Rust content panels, disabled tabs,
tab icons, scrolling and existing Kit animation. Full roving keyboard focus
is still a known upstream parity gap.

`AntTooltip` composes a theme-aware tooltip with GPUI Kit's native
registered provider rather than absolute-positioned HTML. Hover and disabled
triggers are in the WASM gallery; keyboard focus and all Ant Design placement
options remain parity work.

## Ant Design Icons

The alpha library now includes **848 genuine Ant Design SVG icons**: 447
Outlined, 251 Filled and 150 TwoTone variants. They are embedded from the
official MIT-licensed ant-design-icons repository and render through GPUI
on macOS, Windows, Linux and browser WASM (not an icon font). Outlined/Filled
icons use theme-tinted masks; TwoTone icons use full-color SVG images so the
blue shapes and light interior details remain distinct.

```rust
use gpui_antd::{AntIcon, AntIconName};
let search_icon = AntIcon::new(AntIconName::SearchOutlined).size(24.);
let success_icon = AntIcon::new(AntIconName::CheckCircleFilled);
let two_tone_icon = AntIcon::new(AntIconName::CheckCircleTwoTone);
let loading_icon = AntIcon::loading();
```

`.color(...)` and `.spin(...)` currently affect only monochrome icons;
TwoTone images retain the official palette and remain static. The bundled
TwoTone artwork keeps its 1024-unit viewBox while the asset loader gives it a
128px intrinsic image size for efficient full-color rasterization.

Search the full icon catalog in the [Icons documentation](https://caovc.github.io/gpui-antd/#/components/Icons). The docs display icon names as searchable text; the selected icon is always painted by a real Rust/GPUI WASM canvas. The website manifest and Rust enum are generated by the pinned `scripts/sync_antd_icons.py` updater.

Register `gpui_antd::AntIconAssets` as the application's GPUI `AssetSource`.
If GPUI Kit icons are also used, **compose** `AntIconAssets` and
`gpui_kit::assets::Assets` (the Gallery has a cross-platform example in
`crates/gallery/src/lib.rs`). The latter alone cannot load Ant icons. The
official icon license and pinned source revision are documented in
`THIRD_PARTY_NOTICES.md` and `crates/antd/assets/LICENSE-ANT-DESIGN-ICONS`.

## Typography and layout

`AntTitle`, `AntText`, `AntParagraph`, `AntLink`, and `AntTypography`
provide the Ant Design type scale, status colors, strong/emphasized/code/
marked text, single- and multi-line truncation with controlled expand/collapse, clipboard copy, and accessible links.

`AntSpace` and `AntFlex` provide fixed/custom gaps, orientation, wrapping,
alignment and distribution. `AntRow`/`AntCol` implement the familiar
24-column grid with span, offset and gutter. Avatar, Statistic, Empty and
Breadcrumb provide additional native data-display, feedback and navigation
primitives. Responsive xs/sm/md/lg/xl/xxl breakpoints now follow Ant Design's viewport widths (0/576/768/992/1200/1600px). Responsive offsets, fully editable rich text remains future work.

## Application Layout

The native application shell is available as `AntLayout`, `AntHeader`,
`AntSider`, `AntContent`, and `AntFooter`. Nested Layout elements form
horizontal or vertical GPUI flex trees. The sidebar supports controlled
collapse, light/dark surfaces, configured widths and automatic collapse
below an optional viewport breakpoint. The Gallery shows an interactive
dashboard with a collapsible sidebar using real GPUI/WASM.

`AntSider::breakpoint` is currently a viewport-based auto-collapse rule:
it does not update application state or create a mobile drawer by itself.
Sider menu items and route navigation belong to the consuming app.

## Icons preview visibility

The three official-icon Gallery samples are horizontal scroll strips instead
of tall clipped grids; the documentation catalogue wraps full icon names
rather than truncating them to ellipses. Load more results to browse all 848
icons or search for a specific name. The standalone preview renders the
chosen icon with real Rust/GPUI.


## Dropdown, Popover, Modal and Popconfirm (alpha)

Native Kit overlay infrastructure is wrapped in \`crates/antd/src/overlays.rs\`:
- \`AntDropdown\` uses GPUI Kit PopupMenu with stable item keys, checked /
  disabled / separator states, keyboard menu navigation and callbacks.
- \`AntPopover\` owns an anchored GPU-native content surface with a trigger,
  optional arrow and controlled or uncontrolled open state.
- \`AntModal\` wraps GPUI Kit Dialog/Root for modal focus, backdrop, Escape,
  stacking and OK/Cancel callbacks returning \`bool\` to allow veto.
- \`AntPopconfirm\` is an **anchored Popover**, not a generic modal alert;
  it uses native Rust buttons and dismisses via GPUI Base PopoverState.

These are first-stage APIs. Dropdown/Popover triggers currently use Kit
buttons, Modal supports plain-text content (richer form panels are pending),
and automatic Ant Design placement/animation parity is not complete. Browser
smoke verifies the canvas starts; full pointer/keyboard interaction tests
and screenshot comparisons must still be added.


## Browser gallery (real GPUI → WASM)

The browser showcases GPUI's real `wasm32-unknown-unknown` backend.
You need Rust nightly, the WASM target, `wasm-bindgen-cli` 0.2.121,
Node.js 22+, and a browser with WebGPU or WebGL2 support.

```sh
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install wasm-bindgen-cli --version 0.2.121 --locked
bash scripts/build-wasm.sh
cd website
npm install
npm run dev
```

Production build: `cd website && npm run build`. Vite is configured with
GitHub Pages project path `/gpui-antd/`.

The first WASM build downloads a ~49 KB IBM Plex Sans subset font from the
pinned upstream GPUI Kit gallery revision. It is needed because GPUI Web does
not supply a system font. See [notices](THIRD_PARTY_NOTICES.md).

## Browser smoke verification

The Pages CI opens the compiled Rust/WASM demos for core controls, Icons,
Typography and layout primitives in headless Chromium (SwiftShader software rendering). It verifies
that GPUI creates a visible canvas and clears its startup screen; screenshots
are retained as a CI artifact. This is a startup smoke test, not yet visual
pixel-parity certification against React Ant Design.

Locally: `cd website && npm install && npx playwright install chromium && npm run smoke`
after running `npm run build` with the WASM module in place.

## GitHub Pages

Commits on `main` run the **WASM Docs** workflow: compile the Rust gallery,
generate wasm-bindgen bindings, bundle the docs and deploy `website/dist`.
The official Pages action attempts first-run enablement. If the repository's
Pages setting blocks it, the owner should set
**Settings → Pages → Build and deployment → GitHub Actions**.

## Contributing and roadmap

Next up: responsive Grid offsets, advanced icon customization,
visual/screenshot regression, accessibility snapshots, Dropdown, Tooltip,
Modal, multi-select and high-performance Table. A broader
Ant Design API catalog will be added one tested group at a time.

Upstream frameworks: [GPUI Kit](https://gpui-kit.com/),
[Ant Design](https://ant.design/).
License: MIT. This is a community project and not officially affiliated with
Ant Design or its maintainers.
