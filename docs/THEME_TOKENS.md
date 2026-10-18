# Design Token Studio — GPUI-native theme contract

This project follows Ant Design's **Seed → Map → Alias → Component** token
architecture. The reference is [Ant Design Customize Theme](https://ant.design/docs/react/customize-theme/);
the official [Theme Editor](https://ant-design.antgroup.com/theme-editor) is
the interaction/design target. Our current native algorithm is an **approximation**,
not a port of Ant Design's JavaScript color derivation and not a ConfigProvider.

## Public Rust API

```rust
use gpui_antd::{
    AntTheme, AntThemeAlgorithm, AntSeedTokens, AntComponent,
    AntComponentToken, AntMapTokenOverrides, AntAliasTokenOverrides,
    AntTokenLayer, AntTokenValue, color,
};

let mut seed = AntSeedTokens::default();
seed.color_primary = 0x635bdf;
seed.color_info = 0x1677ff; // independent from the brand color
seed.color_link = 0x635bdf;
seed.border_radius = 8.;
seed.font_size = 15.;

let theme = AntTheme::from_seed(seed, AntThemeAlgorithm::Compact)
    .with_map_overrides(AntMapTokenOverrides::default()
        .color_primary_bg(color(0xe9e7ff)).control_height_sm(22.))
    .with_alias_overrides(AntAliasTokenOverrides::default()
        .color_link(color(0x5852ba)).color_text_disabled(color(0x888888)))
    .with_component(
        AntComponent::Card,
        AntComponentToken::default().padding(32.).border_radius(12.),
    )
    .with_component(
        AntComponent::Tag,
        AntComponentToken::default().color_bg(color(0xe6f4ff)),
    );

// Direct native GPUI values (no CSS variables involved).
let map = theme.map_tokens();
let alias = theme.alias_tokens();
assert_eq!(map.control_height, 28.);
assert_eq!(map.control_height_sm, 22.);
assert_eq!(alias.font_size, 15.);
assert_eq!(alias.color_link, color(0x5852ba));
for resolved in theme.resolved_tokens() {
    match resolved.value {
        AntTokenValue::Color(color) => { /* paint a GPUI swatch */ }
        AntTokenValue::Number(number) => { /* show measurement */ }
        AntTokenValue::Boolean(enabled) => { /* show feature flag */ }
    }
    assert!(matches!(resolved.layer, AntTokenLayer::Map | AntTokenLayer::Alias));
}
```

`AntSeedTokens` contains base brand, status, link, neutral text/background,
control height, border radius, font size, spacing unit and motion values.
`AntThemeAlgorithm` supports Default, Dark, Compact and DarkCompact.
`AntTheme::from_seed` recomputes colors, density and aliases; static
`AntTheme::light()` / `AntTheme::dark()` remain supported.

Global themes are **copyable Rust values**, passed via each component's
`.theme(theme)`. The owning Entity changes its theme and calls
`cx.notify()`. The library deliberately does not imply that nested
`ConfigProvider` inheritance or live CSS variables already exist.

## Derived Map and Alias override layers

`AntTheme::from_seed` resolves a Seed into native GPUI Map and Alias tokens.
Optional `with_map_overrides(AntMapTokenOverrides)` and
`with_alias_overrides(AntAliasTokenOverrides)` builders apply after derivation,
and component namespaces remain the most specific level.

| Layer | Supported editable tokens |
| --- | --- |
| Map | colorPrimaryHover, colorPrimaryActive, colorPrimaryBg, colorPrimaryBorder, colorBgElevated, colorBgLayout, controlHeightSM, controlHeightLG |
| Alias | colorLink, colorTextDisabled, colorSuccess, colorWarning, colorError |

Fields not overridden retain the original derived value. Changes affect
real Rust theme fields, not a second JavaScript renderer. The APIs cover
a **limited subset** of Ant Design's Map/Alias fields; sibling gradients
are not automatically rederived after a direct Map override.

The Token Studio supports these two optional layers in both the Components
and Resolved Tokens WASM scenes. Backward-compatible
`gpui-antd-theme-v1` JSON now optionally contains `map` and `alias`
objects, and Rust generation emits the matching builder chains. Colors
and numeric values are bounded and validated on both sides of the bridge.

## Component Token coverage

`AntComponent::ALL` registers 51 independent namespaces. `AntComponentToken`
has a shared typed shape, but not every field is consumed by every component.

| Component | Live fields in native renderer |
| --- | --- |
| Button | colorPrimary, colorBg, colorText, colorBorder, colorBgHover, borderRadius, controlHeight, fontSize, padding |
| Card | colorBg, colorText, colorBorder, colorHeaderBg, borderRadius, fontSize, padding |
| Input | colorBg, colorBorder, borderRadius |
| Menu | colorBg, colorText, colorBgHover, colorBgSelected, borderRadius, fontSize, controlHeight |
| Layout (Sider) | colorBg, colorText, colorBgHover |
| Checkbox | colorPrimary, colorBg, colorText, colorBorder, borderRadius, controlHeight, fontSize |
| Switch | colorPrimary, colorBg, controlHeight |
| Tag | colorBg, colorText, colorBorder, borderRadius, controlHeight, padding, fontSize |
| Badge | colorBg, colorText, controlHeight, padding, fontSize |
| Alert | colorPrimary, colorBg, colorText, colorBorder, borderRadius, fontSize, padding |
| Progress | colorPrimary, colorBg, colorText, controlHeight, fontSize |
| Divider | colorBorder, controlHeight |
| Typography | colorPrimary, colorBg, colorText, colorBorder, borderRadius, fontSize |
| Tooltip | colorBg, colorText, colorBorder, borderRadius |
| Tree | colorPrimary, colorBg, colorText, colorBgHover, colorBgSelected, borderRadius, controlHeight, fontSize |
| Table | colorBg, colorText, colorBorder, colorHeaderBg, colorBgSelected, borderRadius, fontSize |
| Timeline | colorPrimary, colorText, colorBorder, fontSize |

This matrix states actual **property consumption**, not visual Ant Design
pixel parity. Other namespaces are serialized for forward compatibility but
do not yet alter component-local rendering. The Studio marks unimplemented
fields as **Planned** and disables the editing input rather than pretending
it updates the preview.

## Token Studio

Browse **[Token Studio](https://caovc.github.io/gpui-antd/#/theme)** once
the change has passed CI and been deployed. The page offers:

1. Global Seed Token editor with independently configurable Info and Link
   colors; presets and four light/dark/compact algorithms.
2. Component Token overrides restricted to fields with real Rust consumers.
3. **Overview** view for a curated group of widgets, **Selected component**
   view for any of the 51 registered native GPUI Gallery components (including
   Tabs, Menu, Table, Tree, and Timeline), and **Resolved Tokens** view
   for native Rust Map and Alias swatches.
4. JSON import/export using `gpui-antd-theme-v1`, validated before use,
   and a copyable Rust configuration generated from the same input.
5. GPU readiness/error feedback from the embedded demo instead of using
   iframe `load` as a false renderer-success signal.

Choosing a name in the **Component Token** editor automatically switches the
preview to **Selected component**, except while inspecting resolved tokens.
All 51 component names use the **same actual Rust demo** as the public
component docs; per-component overrides and global Seed/Map/Alias colors are
passed through `ThemeLabSpec`. Unimplemented token fields remain labeled
Planned; rendering a component is not proof of full component-token coverage.
The preview route accepts `scene=single|preview=Tabs` or another registered
name, rejects unknown names, and does not grant access to internal icon
audit routes. Browser CI checks the real Tabs and Menu GPUI/WASM scenes.

The toolbar includes a **Viewport** selector for Desktop (fluid),
Tablet (768 logical CSS pixels), and Mobile (390px). It changes the
**actual Rust/GPUI WASM iframe viewport** without restarting the preview;
the selected viewport is a local editor preference, not an exported token.
Browser smoke asserts the real iframe width and unchanged WASM source URL.

The toolbar includes a **Copy preview URL** action. The copied URL opens the
same real Rust/GPUI WASM scene with Seed, Map, Alias, and Component overrides;
it does not render an HTML screenshot or replicate the design in the DOM.

The editor panels are HTML DOM, but **every UI component and token swatch
inside the preview iframe is painted by the Rust Gallery compiled to GPUI
WebAssembly**. No React/CSS replica is substituted for the component preview.

### Architecture

```text
Token Studio editor (DOM input controls only)
    -> validated seed/map/alias/component JSON
    -> same-origin URL params on GPUI demo.html
    -> ThemeLabSpec parser (Rust, bounded/clamped)
    -> AntTheme::from_seed() + Map/Alias/Component overrides
    -> Overview / Resolved Tokens / validated single-component route
    -> actual Gallery::theme_lab_demo / theme_tokens_demo /
       existing Gallery::<component>_demo
    -> GPUI WebAssembly canvas and readiness postMessage
```

## Compatibility limits and next milestones

- Unchanged Ant Design light/dark reference seed colors preserve the canonical
  built-in palette even when only typography or geometry changes. Custom
  brands still use a deterministic native RGB-mixing approximation.
- Native color mixing is a deterministic approximation. It is not the official
  Ant Design perceptual palette algorithm; screenshot and token-value parity
  need quantitative reference comparisons.
- The public Map/Alias inspector intentionally covers a useful subset of
  official tokens. Extend it only together with actual consumers.
- There is no global/nested ConfigProvider, component-specific derivation
  algorithm, persistent theme context, RTL density mode, CSS variables,
  motion system parity, or font-family configurator.
- Theme Studio currently reloads the WASM preview after a debounced change.
  A stable Rust message bridge for in-place state updates remains planned.
- Hidden/intermediate component namespaces must not be described as live.
  Tabs/Menu overlay and responsive Drawer interaction milestones remain
  separately tracked in repository Issues #6, #8 and #7.

## Checks

```bash
cargo check --workspace
cargo test -p gpui-antd --lib
cargo test -p gpui-antd-gallery --lib
cd website
npm run test:tokens
npm run build
npm run smoke
```

The browser smoke checks the **actual Rust canvas** and the Token Studio
editor-to-WASM route. It does not prove exact screenshot parity or screen-reader
compatibility.
