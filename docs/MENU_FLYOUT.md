# Menu · collapsed SubMenu flyouts

The existing `AntMenu` already has controlled `selected_key`,
`open_keys`, inline submenus, roving keyboard focus, disabled items and
official Ant Design icons. With `.inline_collapsed(true)`, each enabled
submenu icon now retains its nested children and opens a native **GPUI Base
Popover** to the right of the collapsed sidebar.

```rust
let collapsed_menu = AntMenu::new("collapsed-nav")
    .theme(theme).dark(true)
    .inline_collapsed(true)
    .selected_key(route)
    .open_keys(open_submenu_keys)
    .on_open_change(|key,open,window,cx| {
        // The owner maintains its BTreeSet of open keys, then notifies GPUI.
    })
    .on_select(|key,window,cx| {
        // The owner updates its selected route and notifies GPUI.
    })
    .item(AntMenuItem::submenu("workspace","Workspace")
        .icon(AntIconName::FolderOutlined)
        .child(AntMenuItem::new("settings","Settings")
            .icon(AntIconName::SettingOutlined))
        .child(AntMenuItem::submenu("advanced","Advanced")
            .child(AntMenuItem::new("audit","Audit"))));
```

## Native interaction contract

- Expanded inline menus still behave exactly as before and insert nested
  rows into the parent vertical menu.
- In collapsed mode only top-level icons occupy the Sider's layout width.
  `Popover::trigger_with` reuses each GPUI Base Button, preserving its
  `Role::MenuItem`, accessible text and per-row focus identity.
- Opening a flyout uses the **native deferred popup overlay**, anchored at
  `Anchor::RightCenter`, not a manually positioned absolute `div`.
- GPUI owns outside-click dismissal, keyboard Escape and focus lifecycle.
  `on_open_change` receives the requested next state and updates the
  same application-owned `open_keys` as inline mode. Right Arrow can
  request opening a focused collapsed submenu.
- Popup content uses a *second real AntMenu* with stable identities and
  controlled `open_keys`, so nested submenus, disabled leaves, and
  per-row keyboard navigation reuse the existing implementation.
- Selecting a leaf invokes `on_select` and requests closing the parent
  flyout, preserving route-controlled state.
- Disabled root submenus never open; unknown leaves and disabled ancestors
  cannot be selected.

## Preview and regression

[Menu docs](https://caovc.github.io/gpui-antd/#/components/Menu)
include `MenuCollapsed`, a dedicated real Rust/GPUI WASM preview alongside
the existing `Menu` and `MenuTree` demos.

The browser smoke interacts with the actual rendered GPUI Canvas:
clicking the collapsed icon must paint a flyout outside its sidebar, and
Escape must dismiss it. No React or HTML-rendered menu is used in the
component preview.

## Remaining Ant Design parity work

This adds **click + keyboard** submenu flyouts; Ant Design hover-delay,
`triggerSubMenuAction` modes, horizontal overflow, multiple placement
variations, precise RTL placement, submenu typeahead and exhaustive
screen-reader regression are not yet implemented. Do not describe this as
1:1 visual equivalence to the React library.

Run `cargo test -p gpui-antd --lib`, `cargo check --workspace`,
and `cd website && npm run smoke`; GitHub Actions is authoritative for
native and WASM browser checks.
