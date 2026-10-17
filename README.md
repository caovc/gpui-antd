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
Empty, Breadcrumb, Tabs, Tooltip, application Layout, Menu, Dropdown, Popover, Modal and Popconfirm, plus Pagination, Steps, Rate, Collapse, Descriptions, Result, Skeleton and Spin.

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
tab icons, scrolling and existing Kit animation. The tab bar has one native
keyboard Tab stop: Left/Right/Home/End change the controlled selection,
wrapping and skipping disabled tabs. Full **individual-tab roving focus**,
tab/tabpanel ARIA linkage and accessibility verification remain parity work.

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
use gpui_antd::{AntIcon, AntIconName, AntTheme, color};
let search_icon = AntIcon::new(AntIconName::SearchOutlined).size(24.);
let success_icon = AntIcon::new(AntIconName::CheckCircleFilled);
let dark_icon = AntIcon::new(AntIconName::CheckCircleTwoTone)
    .theme(AntTheme::dark());
let tinted_icon = AntIcon::new(AntIconName::CheckCircleTwoTone)
    .color(color(0x52c41a));
let custom_icon = AntIcon::new(AntIconName::CheckCircleTwoTone)
    .two_tone_colors(color(0x52c41a), color(0xf6ffed));
let rotating_two_tone = AntIcon::new(AntIconName::CheckCircleTwoTone)
    .two_tone_colors(color(0x52c41a), color(0xf6ffed))
    .spin(true);
let loading_icon = AntIcon::loading();
```

`.color(primary)` sets the primary color for both monochrome and TwoTone icons;
`.two_tone_colors(primary, secondary)` customizes both SVG fills. The default
TwoTone palette follows `theme.primary` and `theme.info_bg` in light/dark mode,
while explicit colors take precedence. TwoTone also supports `.spin(true)` through two synchronized native GPUI SVG
color masks. Static icons continue using full-color GPUI `img()` rendering. Official SVGs keep their
1024-unit viewBox; GPUI rasterizes each TwoTone resource at a 128px intrinsic
size. Each palette has its own image cache key, so theme switches reload colors
without modifying any of the 150 official SVG files.

In the searchable icon inspector you can change primary/secondary colors,
size and rotation. These are documentation controls; the actual artwork
still renders entirely through the Rust GPUI/WASM component.

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
below an optional viewport breakpoint. At small widths,
`AntSider::responsive_open(bool)` overrides auto-collapse without altering
the desktop `collapsed` state. `AntContent::scrollable(true)` independently
scrolls the body while its sibling Header and Footer stay in place. The Gallery shows an interactive
dashboard with a collapsible sidebar using real GPUI/WASM.

`AntSider::breakpoint` is currently a viewport-based auto-collapse rule;
it does not create a mobile drawer. `on_collapse` returns the requested
effective collapsed state; applications should update `responsive_open`
in narrow mode and `collapsed` on desktop.
Sider menu items and route navigation belong to the consuming app.

## Automated icon paint audit

The icon explorer supports size, color and spin options while the SVG
is painted by the Rust GPUI/WASM gallery, not HTML/React. A deterministic
visual-audit command additionally renders the entire pinned 848-icon catalog
in 36 native-GPUI gallery grids:

```bash
cd website
npm run audit:icons          # all 848 official icons, screenshot mosaics
npm run audit:icons -- --sample  # four boundary pages, used by Pages CI
```

The audit checks pixel presence in each glyph's GPU canvas region and
stores PNG pages plus a JSON report under `$RUNNER_TEMP/gpui-antd-icon-audit`
(default `/tmp/gpui-antd-icon-audit`). This is a smoke-level visual
regression, not a reference screenshot parity certification.

## Icons preview visibility

The three official-icon Gallery samples are horizontal scroll strips instead
of tall clipped grids; the documentation catalogue wraps full icon names
rather than truncating them to ellipses. Load more results to browse all 848
icons or search for a specific name. The standalone preview renders the
chosen icon with real Rust/GPUI.


## Navigation Menu

`AntMenu` and `AntMenuItem` are GPUI-native controlled flat navigation
components. They provide light/dark styles, selected route keys, SVG icons,
disabled/danger states and icon-only collapsed Sider rendering with accessible
labels. GPUI Base Button handles Tab, Enter/Space, focus and activation. The
Layout gallery uses a **real AntMenu** in its sidebar; no inert icon/text rows.

Nested inline `AntMenuItem::submenu(key,label).child(...)` supports
controlled `AntMenu::open_keys([...])` and
`on_open_change(key, next_open, window, cx)` callbacks. Up/Down/Home/End
move keyboard focus across enabled visible rows; Right opens a submenu
and Left/Escape return to its parent. Pointer, Enter and Space activation
are handled by GPUI Base Button. When inline-collapsed, nested children
are hidden; an icon-only flyout submenu remains future parity work.
Horizontal menu and typeahead also remain future milestones.

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


## New native component families (alpha wave 1)

Eight additional Ant Design-inspired components render through the same
Rust GPUI implementation on native desktop and browser WebAssembly:

- **Navigation:** AntPagination (controlled current page, jump gaps, simple
  mode, disabled/size/total) and AntSteps (controlled index, vertical steps,
  descriptions, statuses and disabled items).
- **Data entry:** AntRate (controlled whole-star rating, clear-on-repeat,
  count and disabled states).
- **Data display:** AntCollapse/AntCollapseItem (controlled set of expanded
  keys, accordion, native panel content), AntDescriptions/AntDescriptionItem
  (labeled text details, equal-width columns, bordered mode).
- **Feedback:** AntResult (semantic status, title/subtitle/actions),
  AntSkeleton (avatar/title/paragraph placeholders), and AntSpin
  (actual GPUI Kit animated spinner with optional tip and content).

Stateful callbacks only request the next value; the host Entity must update
its state and call cx.notify(). The samples in Gallery do this explicitly.

These are **partial Ant Design implementations**, not complete parity:
Pagination does not yet have a quick jumper or page-size select; Rate does not
support half-stars; Collapse lacks motion; Descriptions does not span cells;
Result is not illustration-identical; Skeleton is static; Spin has no
overlay mask for children. See [component coverage matrix](docs/COMPONENT_MATRIX.md)
for the complete comparison and priority backlog.


## Data-entry wave 2 (additional five public components)

InputNumber, Slider, TimePicker, AutoComplete and Segmented now have Rust
APIs, real native/GPU-backed WASM examples and browser startup checks.
They bring the documented alpha component count to **44**.

- AntInputNumber wraps Kit NumberInput; create a persistent InputState with
  .min, .max, .step, .default_value, then use AntInputNumber::new(&state).
  Number keyboard input, IME, caret, clamping on blur and stepper events are
  provided by the upstream editor.
- AntSlider wraps Kit SliderState/Slider. Pointer and touch support single and
  range sliders. Tab focus + arrow, PageUp/Down, Home/End update single values;
  subscribe to SliderEvent::Change and SliderEvent::Release as appropriate.
  Range keyboard manipulation is not complete.
- AntTimePicker wraps Kit TimeFieldState and TimeField, including minute/second
  precision, 12/24-hour editing, segment keyboard navigation and event subscription.
  This is **not yet** the popup selector of the full Ant Design TimePicker.
- AntSegmented adds controlled selections, icons, disabled options, block
  layout and roving keyboard focus. It is independent of Tabs because it does
  not render tabs or content panels.
- AntAutoComplete uses a real InputState (free text) plus filtered suggestion
  actions. Owner state controls open, highlight and selected value; suggestions
  render as an in-flow native GPUI panel, not DOM. Outside click collision
  handling and complete combobox ARIA behavior remain for follow-up.

These are alpha variants, not claims of React API compatibility or pixel parity.
See [coverage and gap matrix](docs/COMPONENT_MATRIX.md).
