//! Native pointer dispatch and executor-clock regressions for controlled menus.
extern crate gpui_kit as gpui;

use std::{collections::BTreeSet, time::Duration};

use gpui_antd::{AntMenu, AntMenuItem, AntMenuMode, AntMenuTrigger, AntTheme};
use gpui_kit::base::Button;
use gpui_kit::test::TestWindowExt;
use gpui_kit::{
    AppContext as _, Bounds, Context, Entity, IntoElement, ParentElement as _, Render,
    SharedString, Styled as _, TestAppContext, Window, WindowBounds, WindowOptions, div, px, size,
};

struct MenuHarness {
    open: BTreeSet<String>,
    changes: Vec<(String, bool)>,
    selections: Vec<String>,
    mode: AntMenuMode,
    collapsed: bool,
    trigger: AntMenuTrigger,
    disabled: bool,
    mounted: bool,
    accept_requests: bool,
    dark: bool,
}
impl Render for MenuHarness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.weak_entity();
        let selection = owner.clone();
        let menu = self.mounted.then(|| {
            AntMenu::new("hover-nav")
                .mode(self.mode)
                .inline_collapsed(self.collapsed)
                .theme(if self.dark {
                    AntTheme::dark()
                } else {
                    AntTheme::light()
                })
                .submenu_trigger(self.trigger)
                .submenu_open_delay(Duration::from_millis(100))
                .submenu_close_delay(Duration::from_millis(200))
                .open_keys(self.open.iter())
                .item(
                    AntMenuItem::submenu("reports", "Reports")
                        .disabled(self.disabled)
                        .child(AntMenuItem::new("daily", "Daily report")),
                )
                .item(
                    AntMenuItem::submenu("locked", "Restricted")
                        .disabled(true)
                        .child(AntMenuItem::new("hidden", "Hidden")),
                )
                .on_open_change(move |key, open, _, cx| {
                    owner
                        .update(cx, |view, cx| {
                            view.changes.push((key.to_string(), open));
                            if view.accept_requests {
                                if open {
                                    view.open.insert(key.to_string());
                                } else {
                                    view.open.remove(key.as_ref());
                                }
                            }
                            cx.notify();
                        })
                        .unwrap();
                })
                .on_select(move |key: SharedString, _, cx| {
                    selection
                        .update(cx, |view, cx| {
                            view.selections.push(key.to_string());
                            cx.notify();
                        })
                        .unwrap();
                })
        });
        div()
            .size_full()
            .p_4()
            .child(
                div()
                    .w(px(if self.collapsed { 76. } else { 240. }))
                    .children(menu),
            )
            .child(
                Button::new("outside")
                    .ml(px(480.))
                    .mt(px(300.))
                    .child("Outside"),
            )
    }
}

const TRIGGER: &str = "ant-menu-Name(\"hover-nav\")-reports";
const DISABLED: &str = "ant-menu-Name(\"hover-nav\")-locked";
const LEAF: &str = "ant-menu-Name(\"ant-menu-flyout-list-Name(\\\"hover-nav\\\")-reports\")-daily";

fn open(
    cx: &mut TestAppContext,
    mode: AntMenuMode,
    collapsed: bool,
    dark: bool,
) -> (gpui_kit::AnyWindowHandle, Entity<MenuHarness>) {
    cx.update(gpui_kit::init);
    cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Default::default(),
                    size(px(800.), px(700.)),
                ))),
                ..Default::default()
            },
            cx,
            |_, cx| {
                cx.new(|_| MenuHarness {
                    open: BTreeSet::new(),
                    changes: vec![],
                    selections: vec![],
                    mode,
                    collapsed,
                    dark,
                    trigger: AntMenuTrigger::Hover,
                    disabled: false,
                    mounted: true,
                    accept_requests: true,
                })
            },
        )
        .unwrap()
    })
}

fn advance(cx: &mut TestAppContext, handle: gpui_kit::AnyWindowHandle, millis: u64) {
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(millis));
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
    })
    .unwrap();
}

#[gpui_kit::test]
fn hover_delays_open_and_close_and_keeps_panel_interactive(cx: &mut TestAppContext) {
    for (mode, collapsed) in [
        (AntMenuMode::Vertical, false),
        (AntMenuMode::Horizontal, false),
        (AntMenuMode::Inline, true),
    ] {
        for dark in [false, true] {
            let (handle, view) = open(cx, mode, collapsed, dark);
            cx.update_window(handle, |_, window, cx| {
                window.render_frame(cx);
                window.hover(TRIGGER, cx);
            })
            .unwrap();
            advance(cx, handle, 99);
            cx.update(|cx| assert!(view.read(cx).changes.is_empty()));
            advance(cx, handle, 1);
            cx.update_window(handle, |_, window, cx| {
                assert!(view.read(cx).open.contains("reports"));
                assert!(view.read(cx).selections.is_empty());
                assert!(window.find(LEAF).visible());
                window.hover(LEAF, cx);
            })
            .unwrap();
            advance(cx, handle, 300);
            cx.update(|cx| assert!(view.read(cx).open.contains("reports")));
            cx.update_window(handle, |_, window, cx| {
                window.hover("outside", cx);
            })
            .unwrap();
            advance(cx, handle, 199);
            cx.update(|cx| assert!(view.read(cx).open.contains("reports")));
            advance(cx, handle, 1);
            cx.update_window(handle, |_, window, cx| {
                assert!(!view.read(cx).open.contains("reports"));
                assert!(window.try_find(LEAF).is_none());
                assert_eq!(
                    view.read(cx).changes,
                    [("reports".into(), true), ("reports".into(), false)]
                );
            })
            .unwrap();
        }
    }
}

#[gpui_kit::test]
fn rapid_pass_and_disabled_submenus_never_request_open(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, AntMenuMode::Vertical, false, false);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.hover(TRIGGER, cx);
    })
    .unwrap();
    advance(cx, handle, 50);
    cx.update_window(handle, |_, window, cx| {
        window.hover("outside", cx);
    })
    .unwrap();
    advance(cx, handle, 200);
    cx.update_window(handle, |_, window, cx| {
        window.hover(DISABLED, cx);
    })
    .unwrap();
    advance(cx, handle, 300);
    cx.update(|cx| assert!(view.read(cx).changes.is_empty()));
}

#[gpui_kit::test]
fn panel_reentry_cancels_close_and_leaf_selects_once(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, AntMenuMode::Vertical, false, false);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.hover(TRIGGER, cx);
    })
    .unwrap();
    advance(cx, handle, 100);
    cx.update_window(handle, |_, window, cx| {
        window.hover("outside", cx);
    })
    .unwrap();
    advance(cx, handle, 100);
    cx.update_window(handle, |_, window, cx| {
        window.hover(LEAF, cx);
    })
    .unwrap();
    advance(cx, handle, 300);
    cx.update_window(handle, |_, window, cx| {
        assert!(view.read(cx).open.contains("reports"));
        window.click(LEAF, cx);
        assert_eq!(view.read(cx).selections, ["daily"]);
        assert!(!view.read(cx).open.contains("reports"));
    })
    .unwrap();
    advance(cx, handle, 300);
    cx.update(|cx| {
        assert_eq!(
            view.read(cx).changes,
            [("reports".into(), true), ("reports".into(), false)]
        )
    });
}

#[gpui_kit::test]
fn click_and_escape_cancel_delays_without_reopening(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, AntMenuMode::Vertical, false, false);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.hover(TRIGGER, cx);
        window.click(TRIGGER, cx);
        assert!(view.read(cx).open.contains("reports"));
        window.press("escape", cx);
        assert!(!view.read(cx).open.contains("reports"));
    })
    .unwrap();
    advance(cx, handle, 500);
    cx.update(|cx| {
        assert_eq!(
            view.read(cx).changes,
            [("reports".into(), true), ("reports".into(), false)]
        )
    });
    cx.update_window(handle, |_, window, cx| {
        window.hover("outside", cx);
        window.hover(TRIGGER, cx);
    })
    .unwrap();
    advance(cx, handle, 100);
    cx.update_window(handle, |_, window, cx| {
        assert!(view.read(cx).open.contains("reports"));
        window.click("outside", cx);
        assert!(!view.read(cx).open.contains("reports"));
    })
    .unwrap();
}

#[gpui_kit::test]
fn disabling_switching_mode_and_unmounting_cancel_pending_tasks(cx: &mut TestAppContext) {
    for change in 0..3 {
        let (handle, view) = open(cx, AntMenuMode::Vertical, false, false);
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            window.hover(TRIGGER, cx);
        })
        .unwrap();
        advance(cx, handle, 50);
        cx.update_window(handle, |_, window, cx| {
            view.update(cx, |view, cx| {
                match change {
                    0 => view.disabled = true,
                    1 => view.trigger = AntMenuTrigger::Click,
                    _ => view.mounted = false,
                }
                cx.notify();
            });
            window.render_frame(cx);
        })
        .unwrap();
        advance(cx, handle, 300);
        cx.update(|cx| assert!(view.read(cx).changes.is_empty()));
    }
}

#[gpui_kit::test]
fn parent_can_refuse_hover_requests_and_click_mode_stays_click_only(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, AntMenuMode::Vertical, false, false);
    cx.update_window(handle, |_, window, cx| {
        view.update(cx, |view, cx| {
            view.accept_requests = false;
            cx.notify();
        });
        window.render_frame(cx);
        window.hover(TRIGGER, cx);
    })
    .unwrap();
    advance(cx, handle, 100);
    cx.update_window(handle, |_, window, cx| {
        assert_eq!(view.read(cx).changes, [("reports".into(), true)]);
        assert!(view.read(cx).open.is_empty());
        assert!(window.try_find(LEAF).is_none());
        window.hover("outside", cx);
        view.update(cx, |view, cx| {
            view.trigger = AntMenuTrigger::Click;
            view.accept_requests = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.hover(TRIGGER, cx);
    })
    .unwrap();
    advance(cx, handle, 300);
    cx.update_window(handle, |_, window, cx| {
        assert!(view.read(cx).open.is_empty());
        window.click(TRIGGER, cx);
        assert!(view.read(cx).open.contains("reports"));
        window.hover("outside", cx);
    })
    .unwrap();
    advance(cx, handle, 300);
    cx.update(|cx| assert!(view.read(cx).open.contains("reports")));
}

#[gpui_kit::test]
fn inline_menu_ignores_hover_trigger(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, AntMenuMode::Inline, false, false);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.hover(TRIGGER, cx);
    })
    .unwrap();
    advance(cx, handle, 300);
    cx.update(|cx| assert!(view.read(cx).changes.is_empty()));
}

#[gpui_kit::test]
fn keyboard_opens_immediately_and_restores_trigger_focus(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, AntMenuMode::Vertical, false, false);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("outside", cx);
        window.press("shift-tab", cx);
        assert_eq!(window.find(TRIGGER).focused(), Some(true));
        window.press("space", cx);
        assert!(view.read(cx).open.contains("reports"));
        assert!(window.find(LEAF).visible());
        window.press("escape", cx);
        assert!(!view.read(cx).open.contains("reports"));
        assert_eq!(window.find(TRIGGER).focused(), Some(true));
        window.press("right", cx);
        assert!(view.read(cx).open.contains("reports"));
    })
    .unwrap();
}
