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

Button, Input, Checkbox, Radio/RadioGroup, Switch, Tag, Badge, Alert, Card, Progress and Divider.

The visible appearance of these controls is implemented, but some Ant Design
edge cases (loading spinner animation, dashed borders, icons, advanced form
validation and full hover/focus parity) are not yet implemented.

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

The new RadioGroup supports a controlled selected index, roving Tab stops,
arrows, Home/End navigation and disabled-item skipping. Both layouts and the
radio-button style have examples in the interactive docs.

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

## GitHub Pages

Commits on `main` run the **WASM Docs** workflow: compile the Rust gallery,
generate wasm-bindgen bindings, bundle the docs and deploy `website/dist`.
The official Pages action attempts first-run enablement. If the repository's
Pages setting blocks it, the owner should set
**Settings → Pages → Build and deployment → GitHub Actions**.

## Contributing and roadmap

Next up: accurate hover/focus/disabled style states, accessibility snapshots,
Input variants, Select, Dropdown, Tooltip, Modal, Form and Table. A broader
Ant Design API catalog will be added one tested group at a time.

Upstream frameworks: [GPUI Kit](https://gpui-kit.com/),
[Ant Design](https://ant.design/).
License: MIT. This is a community project and not officially affiliated with
Ant Design or its maintainers.
