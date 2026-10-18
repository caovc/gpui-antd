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
| Vertical | Vertical | Native GPUI Popover, `Anchor::RightCenter` | Up/Down, Right to open, Escape to close |
| Horizontal | Single scrolling row | Native GPUI Popover, `Anchor::BottomLeft` | Left/Right, Down to open, Escape to close |

Home/End move to first/last enabled root item; disabled rows do not
activate or become keyboard navigation targets. Clicking a popup leaf
requests selected route update and closes its parent popup; it never
silently mutates the library's controlled state.

The menu root exposes `Role::Menu` with horizontal/vertical orientation;
GPUI Base Button supplies native `MenuItem`, accessible label, disabled
state, selected state, focus handle, and `aria-expanded` for submenu
headers. Popover owns deferred overlay positioning, Escape, outside click
and focus restoration.

## Native + WASM demo

[Menu documentation](https://caovc.github.io/gpui-antd/#/components/Menu)
has a dedicated `MenuModes` Rust Gallery route with horizontal and vertical
modes, in addition to the `MenuCollapsed` icon-only example. The website
may render the documentation and toolbar through the DOM, but the menu and
its overlays are the same Rust/GPUI source used in native applications.

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test -p gpui-antd --lib
cd website
npm run smoke
```

## Gaps relative to Ant Design

Horizontal overflow is a scrollable single row for now, not Ant Design's
responsive `···` overflow submenu. Pointer hover to open/close with delays,
keyboard focus handoff to the first popover leaf in all platforms, nested
popup collision regression tests, RTL key reversal and comprehensive
screen-reader assertions remain outstanding. These are not claimed as
complete visual or WAI-ARIA parity.
