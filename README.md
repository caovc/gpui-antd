# GPUI Ant Design

**An open-source, Ant Design-inspired component library for native Rust GPUI.**

[![Native crate CI](https://github.com/caovc/gpui-antd/actions/workflows/native.yml/badge.svg)](https://github.com/caovc/gpui-antd/actions/workflows/native.yml)
[![WASM Docs](https://github.com/caovc/gpui-antd/actions/workflows/pages.yml/badge.svg)](https://github.com/caovc/gpui-antd/actions/workflows/pages.yml)

**[Documentation and real WASM component previews](https://caovc.github.io/gpui-antd/)**

> **Early alpha (0.1.0).** The goal is visual and interaction fidelity with
> Ant Design, NOT compatibility with the React component API. This is the first
> end-to-end milestone, not the completed Ant Design catalog.

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

Button, Input, Select, DatePicker, Form, Checkbox, Radio/RadioGroup, Switch, Tag, Badge, Alert, Card, Progress, Divider, Icons, Typography, Space, Flex, and Grid.

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

## Ant Design Icons

The alpha library now includes **78 genuine Ant Design SVG icons**: 58
Outlined, 12 Filled and 8 TwoTone variants. They are embedded from the
official MIT-licensed ant-design-icons repository and render through GPUI's
SVG painter on macOS, Windows, Linux and browser WASM (not an icon font).

```rust
use gpui_antd::{AntIcon, AntIconName};
let search_icon = AntIcon::new(AntIconName::SearchOutlined).size(24.);
let success_icon = AntIcon::new(AntIconName::CheckCircleFilled);
let loading_icon = AntIcon::loading();
```

Register `gpui_antd::AntIconAssets` as the application's GPUI `AssetSource`.
If GPUI Kit icons are also used, **compose** `AntIconAssets` and
`gpui_kit::assets::Assets` (the Gallery has a cross-platform example in
`crates/gallery/src/lib.rs`). The latter alone cannot load Ant icons. The
official icon license and pinned source revision are documented in
`THIRD_PARTY_NOTICES.md` and `crates/antd/assets/LICENSE-ANT-DESIGN-ICONS`.

## Typography and layout

`AntTitle`, `AntText`, `AntParagraph`, `AntLink`, and `AntTypography`
provide the Ant Design type scale, status colors, strong/emphasized/code/
marked text, single-line truncation, clipboard copy, and accessible links.

`AntSpace` and `AntFlex` provide fixed/custom gaps, orientation, wrapping,
alignment and distribution. `AntRow`/`AntCol` implement the familiar
24-column grid with span, offset and gutter. Responsive named breakpoints,
selectable rich text and multi-line ellipsis need later work.

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

Next up: responsive Grid breakpoints, the full Ant Design icon catalog,
visual/screenshot regression, accessibility snapshots, Dropdown, Tooltip,
Modal, multi-select and high-performance Table. A broader
Ant Design API catalog will be added one tested group at a time.

Upstream frameworks: [GPUI Kit](https://gpui-kit.com/),
[Ant Design](https://ant.design/).
License: MIT. This is a community project and not officially affiliated with
Ant Design or its maintainers.
