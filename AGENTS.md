# AGENTS.md — GPUI Ant Design

> **Agent entry point.** Read this document before editing the repository. The
> normative detail is in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md),
> [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md), and
> [docs/COMPONENT_PLAYBOOK.md](docs/COMPONENT_PLAYBOOK.md).
> If a task conflicts with a rule here, explain the conflict rather than
> silently changing the architecture.

## Mission / non-goals

- Build an **open-source, Rust-native desktop component library** whose
  appearance and interaction closely follow **Ant Design**, with a single
  Rust/GPUI implementation usable from native desktop and WebAssembly.
- **Do not substitute a React component, HTML/CSS facsimile, DOM-hosted
  screenshot, or fake canvas for a requested component preview.** The website
  shell and its search controls can use DOM; the component demo itself MUST be
  rendered by Rust GPUI via GPUI Kit Web.
- API parity with `antd` React is **not** a promise: use idiomatic Rust
  builders, controlled application state and GPUI semantics.
- Status: **early alpha**. A successful Rust build or visible WASM canvas is
  NOT evidence of pixel-level Ant Design 1:1 parity, comprehensive accessibility
  or complete keyboard interactions. State what is actually verified.

## Hard constraints

1. **Versions:** Rust edition 2024, nightly toolchain
   (`rust-toolchain.toml`), `gpui-kit = "=0.7.3"` from workspace
   `Cargo.toml`; WASM `wasm-bindgen = "=0.2.121"`. Do not casually upgrade
   the GPUI/GPUI Kit stack: GPUI Kit owns its matching GPUI snapshot. Access
   GPUI types via `gpui_kit` or the crate's `extern crate gpui_kit as gpui;`
   alias, rather than adding a separately versioned GPUI dependency.
2. **Layer boundaries:** `crates/antd` is the reusable component crate.
   `crates/gallery` owns example state, listeners and Rust demo rendering.
   `website` is the documentation/search shell and imports the compiled WASM
   module; it is **not** a second implementation of components.
3. **Behavior before appearance:** reuse `gpui_kit::base` for keyboard,
   focus, semantics and event primitives, and `gpui_kit::component` for
   complex existing infrastructure (editor, calendar, popup, tab bar, etc.).
   Place Ant Design styling in the `gpui-antd` wrapper, not in vendored Kit.
4. **State ownership:** controlled builders report requested next values.
   Their parent entity changes the value and calls `cx.notify()`.
   Do not stash user-facing model state in a stateless `RenderOnce` element.
   For text/date/select, reuse `Entity<InputState>`,
   `Entity<DatePickerState>` and `Entity<SelectState<D>>`.
5. **Tokens:** use `AntTheme::light()/dark()` and semantic token fields.
   Support existing `.theme(t)` conventions and `ControlSize`
   (24/32/40 logical px); avoid magic color hexes throughout new controls.
6. **Icons:** `crates/antd/src/icons.rs`, `crates/antd/assets/antd-icons/**`,
   and `website/public/antd-icons.json` are **generated** from pinned
   ant-design/ant-design-icons commit
   `7f2516ac91226d2b41f93b35cb5197c8d94f7189`.
   Update through `scripts/sync_antd_icons.py` and its workflow; preserve
   the official MIT license and `THIRD_PARTY_NOTICES.md`. There are currently
   848 official icon SVGs (447 outlined, 251 filled, 150 two-tone). Do not
   replace official SVGs with Lucide icons or Unicode characters. Dynamic
   TwoTone colors belong in `crates/antd/src/icon_palette.rs` rather than
   generated SVGs; preserve the generator's `match asset_path` marker.
7. **GitHub Pages:** Vite base is `/gpui-antd/`; production documentation is
   <https://caovc.github.io/gpui-antd/>. Preserve this subpath in browser
   routes and asset URLs. Do not confuse a successful compilation with
   successful Pages deployment.
8. **No ungrounded completion:** Do not say the CI or browser tests passed
   until that run actually reports success. Confirm source commit SHA and
   successful `deploy` job before declaring the updated site live.

## Token Studio development contract

- Theme derivation is Rust-only: `crates/antd/src/design_tokens.rs` implements
  Seed -> Map -> Alias -> Component. Explicit `with_map_overrides` and
  `with_alias_overrides` are limited to supported native token fields.
- `crates/gallery/src/theme_lab.rs` accepts bounded `Map.<name>` /
  `Alias.<name>` route parameters. When adding a token, update the Rust
  builder, parser, JS editor/serializer/Rust generator, tests and
  `docs/THEME_TOKENS.md` in the same change.
- `website/src/theme-designer.js` is an editor only. Its component previews
  must remain **actual Rust GPUI/WASM canvases**, not HTML replicas.
- `scene=single|preview=<name>` in Token Studio resolves ONLY public
  `Gallery::COMPONENTS` names to their existing Rust demo functions.
  Keep the Gallery and website component lists in sync; do not introduce
  stand-in visuals or expose internal icon audit routes through preview.
- Keep JSON import compatible with `gpui-antd-theme-v1`, validating every
  imported numeric and color before preview.

## Files to change for a new public component

1. Add or extend `crates/antd/src/<component>.rs`, with a
   `#[derive(IntoElement)]` `AntXxx` and `RenderOnce` implementation
   wherever appropriate.
2. Export it from `crates/antd/src/lib.rs`.
3. Add the **real Rust** demo and its state/callbacks to
   `crates/gallery/src/lib.rs`, register its name in `COMPONENTS`,
   and map the name to the demo in `impl Render for Gallery`.
4. Add a data-driven entry to `website/src/main.js` `ITEMS` (description,
   Rust code, API), including its navigation category; update sidebar/overview
   counters in `website/index.html` and `website/src/main.js`.
5. Add the component to `website/scripts/smoke.mjs`; make sure the
   `website/demo.html?component=Name` WASM route starts correctly.
6. Add Rust tests for behavior/state transitions, keyboard helper algorithms,
   validation and asset coverage. Test dark/light and disabled states.
7. Update README/docs and, if warranted, a GitHub roadmap issue. Do not
   quietly change public APIs without updating docs and examples.

## Commands and real acceptance checks

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test -p gpui-antd --lib
cargo run -p gpui-antd-gallery          # native preview
bash scripts/build-wasm.sh              # real GPUI -> wasm32-unknown-unknown
cd website
npm install --no-audit --no-fund
npm run build
npx playwright install chromium
npm run smoke                       # Chromium + real WASM canvas
```

WASM prerequisites include nightly with `wasm32-unknown-unknown`,
`wasm-bindgen-cli` **0.2.121**, Node **22+**, and a WebGPU/WebGL2 browser.
Native Linux additionally requires GPUI's system packages; the authoritative
installation example is `.github/workflows/native.yml`. See
[docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) for reproductions and CI links.

## GPUI implementation pitfalls

- Import relevant traits to use fluent styling/events:
  `Styled as _`, `ParentElement as _`, `InteractiveElement as _`,
  `StatefulInteractiveElement as _`,
  `gpui_kit::base::StyledExt as _`, and
  `gpui_kit::prelude::FluentBuilder as _` for `.when` / `.when_some`.
- `RenderOnce` builders move ownership: capture a child's flags **before**
  consuming it in chained builders; do not use moved values afterward.
- `cx.new`, `cx.listener`, `cx.notify()`, and stable per-view `ElementId`
  values are essential for functioning inputs and controlled widgets.
- GPUI Kit Web can temporarily log
  `Wasm assets loading, will be available soon...` while loading its
  **own** SVGs; browser smoke ignores *only this exact known transient
  message*. Do not suppress other errors.
- The internal `IconAudit:<page>` Rust Gallery route and `website/scripts/audit-icons.mjs` must stay geometry-aligned. Pages CI checks all 848 GPUI glyphs on main and samples 2x-DPI boundary pages in PRs. `npm run audit:icons` generates per-icon pixel data and page screenshots. This is not official Ant Design screenshot parity.
- File names for screenshot/artifact upload must be portable (no colon,
  slash, etc.); `smoke.mjs` sanitizes component names.
- Use GPUI Kit's actual overlay and popup infrastructure rather than
  drawing an absolute-positioned popup that bypasses focus, collision
  detection or screen-reader semantics.

## Before marking a task complete

- [ ] Code compiles, Rust tests pass, and new stateful behavior is covered.
- [ ] New demo is rendered **by Rust GPUI**, both native and WASM.
- [ ] Docs page, code sample, exported API and component counts agree.
- [ ] GitHub Actions native + WASM + browser smoke pass for the **HEAD SHA**.
- [ ] If publishing, the Pages **deploy** job reports success.
- [ ] No unintended license removal, generated-icon edit, broken dark theme,
      broken keyboard/focus, or missing semantic label.
- [ ] List remaining parity gaps, test limitations and the relevant issue.

## Immediate priorities / known gaps

Menu horizontal overflow/typeahead, Dropdown/Popover/Modal/Popconfirm advanced parity, complete Layout shell parity (fixed Sider/mobile drawer), Tabs arrow-key navigation, Tooltip keyboard/placement parity, icon visual parity, responsive
Grid offsets/order, richer Typography editing, screenshot comparison and
accessibility. Also consult the repository's live GitHub Issues for the
priorities and avoid creating duplicate work. The icon catalog and its
generator are already in place; do not recreate them.
