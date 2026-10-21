# Native AntMenu: Inline, Vertical and Horizontal

`AntMenu` is one controlled Rust/GPUI menu implementation. All three
`AntMenuMode` variants use the same `AntMenuItem` tree,
`selected_key` and `open_keys` callbacks. They do not use a DOM
navigation replica or a separate JavaScript state machine.

## Example

```rust,ignore
use gpui_antd::{AntMenu, AntMenuItem, AntMenuMode, AntIconName};

AntMenu::new("global-navigation")
    .mode(AntMenuMode::Horizontal)
    .theme(theme)
    .selected_key(selected_route)
    .open_keys(open_submenus.iter())
    .on_open_change(|key,open,window,cx|{
        // The owning Entity inserts/removes the key and calls cx.notify().
    })
    .on_select(|key,window,cx|{
        // The owning Entity changes selected_route and calls cx.notify().
    })
    .item(AntMenuItem::new("home","Home")
        .icon(AntIconName::HomeOutlined))
    .item(AntMenuItem::submenu("reports","Reports")
        .icon(AntIconName::BarChartOutlined)
        .child(AntMenuItem::new("daily","Daily"))
        .child(AntMenuItem::new("monthly","Monthly")));
```

Set `.mode(AntMenuMode::Inline)` for nested rows in the same menu. Set
`.mode(AntMenuMode::Vertical)` for full-label vertical rows whose
submenus open to the right. `.inline_collapsed(true)` keeps the icon-only
right-hand flyouts for vertical sidebars; horizontal mode deliberately
ignores icon-only collapse.

| Mode | Root arrangement | Submenu presentation | Root keyboard |
| --- | --- | --- | --- |
| Inline (default) | Vertical | Child rows under expanded header | Up/Down, Right to expand, Left to collapse |
| Vertical | Vertical | Native GPUI Popover, `Anchor::LeftCenter` | Up/Down, Right to open, Escape to close |
| Horizontal | Single scrolling row | Native GPUI Popover, `Anchor::TopLeft` | Left/Right, Down to open, Escape to close |

Home/End move to first/last enabled root item; disabled rows do not
activate or become keyboard navigation targets. Clicking a popup leaf
requests selected route update and closes its parent popup; it never
silently mutates the library's controlled state.

The menu root exposes `Role::Menu` with horizontal/vertical orientation;
GPUI Base Button supplies native `MenuItem`, accessible label, disabled
state, selected state, focus handle, and `aria-expanded` for submenu
headers. Popover owns deferred overlay positioning, Escape, outside click
and focus restoration.

## Optional hover activation

Click remains the default for compatibility. For horizontal, vertical or
collapsed popup menus, enable hover explicitly:

```rust,ignore
use std::time::Duration;
use gpui_antd::{AntMenu, AntMenuMode, AntMenuTrigger};

AntMenu::new("hover-navigation")
    .mode(AntMenuMode::Horizontal)
    .submenu_trigger(AntMenuTrigger::Hover)
    .submenu_open_delay(Duration::from_millis(160))
    .submenu_close_delay(Duration::from_millis(250));
// Add items, controlled open_keys and on_open_change as in the example above.
```

Delay defaults are zero for open and 100 ms for close, following the
[Ant Design Menu delay values](https://ant.design/components/menu/). This
library intentionally retains Click as its default trigger. Hover does not
change inline group activation and never selects a route.

The parent owns open keys. Hover timers only request changes through
`on_open_change`; ignoring a request leaves the popup closed. Leaving before
open cancels the task. Moving from trigger to content, or re-entering during
the close grace period, keeps the panel interactive. Click/keyboard activation
remains immediate. Escape/outside dismissal cancels pending work; exiting and
re-entering is required to start a new hover session. Disable, trigger-mode
changes and unmount also cancel delayed requests.

If the parent applies a hover-open request after the pointer has left, the
newly opened panel starts the close grace period. A new click or keyboard
activation supersedes that stale hover intent.

Descendant menus inside the current flyout use Inline mode: nested titles
expand rows inside the same panel, so hovering those rows keeps the ancestor
open. Separate nested flyouts and their cross-overlay hover coordination
remain future parity work.

Positioning, focus and dismissal still belong to Base Popover. Element-local
state holds only pointer intent and cancellable weak tasks. GPUI retains its
existing popup focus policy; this is not a promise of Ant Design focus parity.

Native UI tests in `crates/antd/tests/menu_hover.rs` use real GPUI pointer
and keyboard events with an executor clock, including light/dark, all three
popup modes, controlled refusal and cancellation cases.

## Native + WASM demo

[Menu documentation](https://caovc.github.io/gpui-antd/#/components/Menu)
has a dedicated `MenuModes` Rust Gallery route with horizontal and vertical
modes, in addition to the `MenuCollapsed` icon-only example and the `MenuHover`
delayed-hover route. All are real Rust Gallery demos. The website
may render the documentation and toolbar through the DOM, but the menu and
its overlays are the same Rust/GPUI source used in native applications.

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test -p gpui-antd --lib
cargo test -p gpui-antd --test control_interactions --test menu_hover
cd website
npm run smoke
```

## Gaps relative to Ant Design

Horizontal overflow is a scrollable single row for now, not Ant Design's
responsive `···` overflow submenu. Keyboard focus handoff to the first popover
leaf in all platforms, nested
popup collision regression tests, RTL key reversal and comprehensive
screen-reader assertions remain outstanding. These are not claimed as
complete visual or WAI-ARIA parity.
