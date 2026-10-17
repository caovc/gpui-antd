# Component design and implementation playbook

This document sets the acceptance standard for new `gpui-antd` widgets.
It is intended for code agents, maintainers and contributors.

## 1. Study the actual Ant Design behavior

For each named component, compare the official Ant Design demos and document:

1. anatomy (trigger, popup, menu/panel, labels, prefixes and suffixes),
2. variants, sizes, light/dark visual tokens and disabled/loading states,
3. interaction contract (click, pointer hover, Enter, Space, arrows, Home/End,
   Escape, Tab, dismissal and focus restoration),
4. accessibility role/label/value/selected/expanded/disabled semantics,
5. control ownership (parent-controlled values, event callbacks, optional
   internal state), and
6. complexity/performance constraints on desktop and WebAssembly.

This comparison defines *parity goals*. Clearly document any gaps and don't
invent unsupported methods or publish empty placeholders as complete APIs.

## 2. Choose behavior primitives first

Use pinned `gpui-kit = "=0.7.3"`:

| Class of component | Starting point |
| --- | --- |
| Button, Checkbox, Switch, Radio | `gpui_kit::base` behavior elements |
| Focusable links, tab list, tooltip semantics | `gpui_kit::base` / appropriate Kit component |
| Input/IME/selection/caret | `gpui_kit::component::input::InputState` |
| Searchable select | `gpui_kit::component::select::{SelectState,Select}` |
| Date/range picker | `gpui_kit::component::date_picker` |
| Tabs | `gpui_kit::component::tab::{TabBar,Tab,TabVariant}` |
| Tooltip overlay | Kit/Base tooltip overlay infrastructure; avoid ad hoc popups |
| Modal, Dropdown | GPUI Kit's registered Root overlays/menu behavior |
| Layout, typography, icons | Native GPUI elements, semantic AntTheme tokens |

If a Kit component's styles cannot be fully overridden, record the mismatch
instead of impersonating an exact Ant Design look. Avoid importing a different
GPUI version to reach unavailable APIs.

## 3. Rust component API conventions

```rust
#[derive(IntoElement)]
pub struct AntExample {
    id: ElementId,
    selected: bool,
    disabled: bool,
    theme: AntTheme,
    on_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
}

impl AntExample {
    pub fn new(id: impl Into<ElementId>) -> Self { /* defaults */ }
    pub fn selected(mut self, next: bool) -> Self { self.selected = next; self }
    pub fn disabled(mut self, next: bool) -> Self { self.disabled = next; self }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn on_change(
        mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler)); self
    }
}
```

- Use `Ant` prefixes to avoid collisions with Kit types. Expose named
  variants such as `TabVariant` only after checking existing exported names.
- Prefer `RenderOnce` for view-only elements. Keep user state in a GPUI
  `Entity`, with data copied into controlled builders at render time.
- Event listeners signal **requested** state; the parent must update and
  notify. Disabled callbacks must remain inert; loading states must disable
  duplicate activation when appropriate.
- Stable `ElementId` values avoid losing focus/animation states between
  frames; do not use a new random ID during every render.
- Keep public structs extensible with private fields and builder methods.
- Avoid excessive type-erasure or `Box<dyn Any>` when a small typed enum
  suffices. Test deterministic state logic without requiring GPU startup.

## 4. Styling and layout

- Baseline light/dark theme from `crate::tokens::AntTheme`.
- `ControlSize::Small/Middle/Large` is 24/32/40 logical pixels.
- GPUI's `Styled` trait provides `.bg`, `.border_color`, `.gap`,
  `.text_color`, `.rounded`, etc. Use `px` deliberately for fixed sizes
  and relative GPUI sizing for responsive allocation.
- Prefer existing Ant Design semantic tokens to ad hoc literal colors.
- Ensure hover, focus, active, disabled, danger, primary and ghost states
  have legible contrast on light and dark backgrounds.
- Keep native keyboard semantics even when a visual skin changes.

## 5. Tabs, tooltips and overlay discipline

Tabs:
- Controlled selection `selected_index`; on tab activation report the index,
  with the parent applying it and calling `cx.notify()`.
- Disabled tabs cannot activate. Their accessible state should be announced.
- Prefer GPUI Kit's TabBar for scrolling tabs, tab indicator motion, overflow
  and foundation semantics; add Rust-owned panel rendering.
- `AntTabs` attaches a stable `FocusHandle` to each native Kit Tab;
  only the selected enabled tab is in the Tab order. Left/Right/Home/End
  rove focus and request controlled selection, skipping disabled tabs,
  and Enter/Space activate the focused tab. Keyboard handling is attached
  to individual tabs, not the selected panel, so panel controls keep
  their own key behavior.
- The pinned Kit TabBar already supplies one `Role::TabList` around the
  native `Role::Tab` children and their selected/disabled state; do not
  wrap it in another TabList. Selected native content uses `Role::TabPanel`
  and the visible tab's label.
- This is **not** yet complete APG parity: `aria-controls` / `aria-labelledby`
  ID relations and real screen-reader audits remain; add visual keyboard
  simulations and browser smoke before claiming verification.

Tooltips:
- Tooltip content should be descriptive, non-interactive and hidden on
  pointer leave/dismiss. Screen-reader role must be `Tooltip`.
- Prefer Kit's tooltip overlay for trigger measurement, delay, collision
  handling, enter/leave grace and dismissal.
- Avoid placing an absolutely-positioned div inside the trigger and calling
  it a finished tooltip: it ignores viewport collision/overlay management.
- Verify keyboard focus, pointer hover and dismissal on both GPU surfaces
  before claiming full parity.

Dropdown/Modal/Popconfirm:
- Use Kit's registered Root overlay, focus trap, dismissal, Escape and
  restoration primitives.
- Constrain popovers to the viewport and avoid trapping inputs incorrectly.

### Native Menu behavior

- `AntMenu` is a controlled flat menu backed by GPUI Base Button semantics.
  Owner Entities update `selected_key` after `on_select(SharedString,...)`.
- Each item must have a unique, stable key. Disabled items stay inert.
  Collapsed icon-only items retain accessible names.
- Use `AntMenu` in Sider and in standalone navigation; do not substitute
  decorative `div` rows with no interaction.
- Inline `AntMenuItem::submenu(key,label).child(...)` nests entries.
  The parent Entity owns expanded keys through `.open_keys(...)` and
  `on_open_change(key, next_open, window, cx)`, separate from route selection.
- Up/Down/Home/End rove focus and skip disabled visible rows; Right expands
  a group and Left/Escape closes one, restoring focus to the group header.
  Enter/Space and pointer activation use GPUI Base Buttons.
- Collapsed submenu flyouts, horizontal menu and typeahead have **not**
  been implemented. Verify keyboard behavior before claiming full a11y parity.

### Responsive Layout behavior

- `AntSider::collapsed` controls the desktop sidebar independently of the
  narrow-viewport `AntSider::responsive_open` value.
- `on_collapse(next_collapsed)` requests the NEXT effective state. Below the
  breakpoint, the application updates `responsive_open(!next_collapsed)`;
  above it, update `collapsed(next_collapsed)`.
- Use `AntContent::scrollable(true).scroll_id(stable_id)` for independent
  Content scrolling in an explicit-height layout. Header/Footer/Sider are
  normal GPUI siblings and remain in place.
- This is not yet Ant Design's sliding mobile drawer/overlay or fixed-position
  global Header/Sider implementation; do not claim those behaviors.

## 6. Live preview / regression requirements

One component means **one Rust implementation**, compiled twice.

- Native `Gallery` component selection: `crates/gallery/src/lib.rs`.
- Browser selection: `website/demo.html?component=<Name>` routes to
  `wasm.run(component,dark)`; no React or HTML mock.
- Documentation: `website/src/main.js` `ITEMS` / API props / Rust code.
- Browser CI `website/scripts/smoke.mjs` launches Chromium with SwiftShader,
  validates a visible canvas, and collects screenshot Artifacts.
- For state changes, write a Rust test asserting transitions and disabled
  behavior. Where possible, simulate pointer/keyboard and verify the expected
  state is rendered. For overlays, test show/close lifecycle.
- For icon previews, the DOM can search `antd-icons.json`, but the selected
  SVG image must be rendered by `AntIcon` within GPUI. All icon data is
  MIT-licensed and pinned; preserve the generator.

Never disable smoke for a new component just because it fails. Fix the code
or clearly record the failure, keeping Pages deployment from advertising a
broken build.

## 7. PR / commit template

```markdown
### Why
Ant Design reference behavior and current missing capability.

### Implementation
- Rust component API and behavior primitive.
- Theme/size/keyboard/a11y states.
- Real GPUI gallery, docs and API example.

### Tests
- Rust library unit tests.
- `cargo check --workspace`.
- WASM build and Chromium smoke.
- Manual keyboard/dark mode notes.

### Parity gaps
Any unsupported Ant Design props/behaviors or unverified visual claims.
```

Prefer cohesive commits that compile. If unavoidable, follow a compiling
intermediate commit quickly and verify the final commit SHA before saying
the branch is green.
