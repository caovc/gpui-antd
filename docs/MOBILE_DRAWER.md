# Mobile Layout / Drawer

`AntMobileDrawer` is a narrow-viewport navigation trigger which opens the
**native GPUI Kit Sheet**, not an HTML overlay or an absolutely positioned
fake panel. It complements `AntSider::mobile_hidden(true)`:

```rust
use gpui_antd::*;
use gpui_kit::component::WindowExt as _;

// Child of the same AntLayout::new().horizontal(true) container:
let sider = AntSider::new("nav-sider")
    .width(200.)
    .collapsed_width(64.)
    .breakpoint(GridBreakpoint::Md)
    .mobile_hidden(true)
    .child(desktop_navigation);

// In the AntHeader next to the workspace title:
// The closure must rebuild elements. Capture an Entity/WeakEntity or an
// existing listener outside it; do not move a one-time AnyElement into it.
let drawer = AntMobileDrawer::new("mobile-nav")
    .title("Navigation")
    .theme(theme)
    .breakpoint(GridBreakpoint::Md)
    .width(280.)
    .mask_closable(true)
    .on_open_change(|next, window, cx| {
        // Optional: owner records the requested Sheet lifecycle state.
    })
    .content(|window, cx| {
        AntMenu::new("mobile-menu")
            .theme(theme)
            .item(AntMenuItem::new("home", "Home"))
            .on_select(|_, window, cx| window.close_sheet(cx))
    });
```

## Behavior

- Under the chosen `GridBreakpoint`, the Sider has logical width `0`
  and no interactive descendants; the Drawer trigger appears in the header.
- At the breakpoint and above, the Sider resumes its previous independent
  desktop collapse state and the Drawer trigger disappears.
- Clicking the native trigger opens a left-hand **Sheet**, whose title bar,
  backdrop dismissal, Escape handling, focus restoration and 150ms entrance
  animation are handled by the pinned GPUI Kit, not by the wrapper.
- The Sheet body closure may be invoked multiple times while open. Preserve
  persistent GPUI Entity handles instead of creating editor state in the body.
- A navigation leaf may call `window.close_sheet(cx)` after handling
  `on_select`. The `on_open_change` listener receives `true` on open and
  `false` on close, without overriding any parent selection state.
- `mask_closable(false)` disables backdrop dismissal while retaining the
  close button and keyboard dismissal, just like the native Sheet model.
- Width is constrained to 120–520 logical pixels; default width is 280.
  This milestone does not yet implement all Drawer placements, nested push,
  footer slots, focus trap assertions for every platform, or Ant Design's
  exact sliding/overscroll visual values.

## Preview / validation

The [Layout docs](https://caovc.github.io/gpui-antd/#/components/Layout)
contain both the original interactive Sider demo and a dedicated **480px
`LayoutDrawer`** gallery route. It is rendered by Rust GPUI/WebAssembly.

```bash
cargo test -p gpui-antd --lib
cargo check --workspace
cd website && npm run smoke
```

Chromium browser smoke opens the *actual* GPUI Sheet and tests Escape and
backdrop dismissal through screenshot pixel changes. These tests verify
the visual open/close cycle, not the complete screen-reader accessibility
tree or pixel-level equivalence to React Ant Design.
