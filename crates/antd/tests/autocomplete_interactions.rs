//! Exercise the production AutoComplete through native GPUI window dispatch.
extern crate gpui_kit as gpui;

use gpui_antd::{AntAutoComplete, AntTheme, ControlSize, OverlayPlacement};
use gpui_kit::base::{Button, GlobalState};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::test::TestWindowExt as _;
use gpui_kit::{
    AppContext as _, Bounds, Context, Entity, EntityInputHandler as _, Focusable as _, IntoElement,
    ParentElement as _, Render, Role, SharedString, Styled as _, Subscription, TestAppContext,
    Window, WindowBounds, WindowOptions, div, px, size,
};

struct Owner {
    input: Entity<InputState>,
    open: bool,
    active: usize,
    opens: Vec<bool>,
    selections: Vec<String>,
    apply: bool,
    disabled: bool,
    dark: bool,
    size: ControlSize,
    placement: OverlayPlacement,
    edge: bool,
    options: Vec<SharedString>,
    active_requests: Vec<usize>,
    _subscription: Subscription,
}

impl Render for Owner {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let on_open = cx.listener(|this, next: &bool, _, cx| {
            this.opens.push(*next);
            if this.apply {
                this.open = *next;
            }
            cx.notify();
        });
        let on_active = cx.listener(|this, next: &usize, _, cx| {
            this.active_requests.push(*next);
            if this.apply {
                this.active = *next;
            }
            cx.notify();
        });
        let on_select = cx.listener(|this, value: &SharedString, window, cx| {
            this.selections.push(value.to_string());
            if this.apply {
                this.input
                    .update(cx, |input, cx| input.set_value(value.clone(), window, cx));
            }
            cx.notify();
        });
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .when(self.edge, |host| host.justify_end().items_end())
            .child(Button::new("before").child("Before"))
            .child(
                div()
                    .w(px(300.))
                    .h(px(self.size.height()))
                    .overflow_hidden()
                    .child(
                        AntAutoComplete::new("autocomplete", &self.input)
                            .aria_label("Language")
                            .theme(if self.dark {
                                AntTheme::dark()
                            } else {
                                AntTheme::light()
                            })
                            .size(self.size)
                            .disabled(self.disabled)
                            .placement(self.placement)
                            .open(self.open)
                            .active_index(self.active)
                            .max_options(50)
                            .options(self.options.clone())
                            .on_open_change(move |next, window, cx| on_open(&next, window, cx))
                            .on_active_change(move |next, window, cx| on_active(&next, window, cx))
                            .on_select(move |value, window, cx| on_select(&value, window, cx)),
                    ),
            )
            .child(Button::new("after").child("After"))
    }
}

fn open(cx: &mut TestAppContext) -> (gpui_kit::AnyWindowHandle, Entity<Owner>) {
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
            |window, cx| {
                cx.new(|cx| {
                    let input = cx.new(|cx| InputState::new(window, cx));
                    let subscription = cx.subscribe(&input, |_, _, _: &InputEvent, cx| cx.notify());
                    Owner {
                        input,
                        open: false,
                        active: 0,
                        opens: vec![],
                        selections: vec![],
                        apply: true,
                        disabled: false,
                        dark: false,
                        size: ControlSize::Middle,
                        placement: OverlayPlacement::BottomLeft,
                        edge: false,
                        options: ["Rust", "Ruby", "Python"]
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        active_requests: vec![],
                        _subscription: subscription,
                    }
                })
            },
        )
        .unwrap()
    })
}

#[gpui_kit::test]
fn suggestions_do_not_move_following_content(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let before = window.find("after").bounds();
        view.update(cx, |view, cx| {
            view.open = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
        assert!(window.find("Name(\"autocomplete\")-suggest-0").visible());
        assert_eq!(
            window.find("after").bounds(),
            before,
            "popup must not enter normal layout"
        );
    })
    .unwrap();
}

#[gpui_kit::test]
fn pointer_outside_requests_closure(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        view.update(cx, |view, cx| {
            view.open = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("before", cx);
        assert!(!view.read(cx).open, "outside click must request close");
        assert_eq!(view.read(cx).opens, vec![false]);
        assert!(
            window
                .try_find("Name(\"autocomplete\")-suggest-0")
                .is_none()
        );
    })
    .unwrap();
}

#[gpui_kit::test]
fn native_semantics_and_corner_placement_work_in_both_themes_and_sizes(cx: &mut TestAppContext) {
    for dark in [false, true] {
        for control_size in [ControlSize::Small, ControlSize::Middle, ControlSize::Large] {
            for placement in [OverlayPlacement::BottomLeft, OverlayPlacement::TopRight] {
                let (handle, view) = open(cx);
                cx.update_window(handle, |_, window, cx| {
                    view.update(cx, |view, cx| {
                        view.open = true;
                        view.dark = dark;
                        view.size = control_size;
                        view.placement = placement;
                        view.edge = placement == OverlayPlacement::TopRight;
                        cx.notify();
                    });
                    window.render_frame(cx);
                    window.render_frame(cx);
                    let root = window.find("autocomplete");
                    assert_eq!(root.role(), Some(Role::ComboBox));
                    assert_eq!(root.label(), Some("Language"));
                    assert_eq!(root.expanded(), Some(true));
                    assert_eq!(root.bounds().size.height, px(control_size.height()));
                    let panel = window.find("Name(\"autocomplete\")-suggestions");
                    assert_eq!(panel.role(), Some(Role::ListBox));
                    let option = window.find("Name(\"autocomplete\")-suggest-0");
                    assert_eq!(option.role(), Some(Role::ListBoxOption));
                    assert_eq!(option.label(), Some("Rust"));
                    assert_eq!(option.selected(), Some(false));
                    if placement == OverlayPlacement::TopRight {
                        assert!(panel.bounds().bottom() <= root.bounds().top());
                    } else {
                        assert!(panel.bounds().top() >= root.bounds().bottom());
                    }
                    assert!(panel.bounds().left() >= px(8.));
                    assert!(panel.bounds().right() <= px(792.));
                    assert!(panel.bounds().top() >= px(8.));
                    assert!(panel.bounds().bottom() <= px(692.));
                })
                .unwrap();
            }
        }
    }
}

#[gpui_kit::test]
fn editor_keeps_typing_space_caret_clicks_and_pointer_selection(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let input = view.read(cx).input.clone();
        let id = ("input", input.entity_id());
        window.click(id, cx);
        window.input("Ru", cx);
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        assert_eq!(
            window.find("Name(\"autocomplete\")-suggest-1").label(),
            Some("Ruby")
        );
        window.click(id, cx); // Editor caret click must not toggle the popup.
        assert!(view.read(cx).open);
        window.press("space", cx);
        assert_eq!(input.read(cx).value().as_ref(), "Ru ");
        assert!(view.read(cx).open);
        window.click("Name(\"autocomplete\")-suggest-1", cx);
        assert_eq!(view.read(cx).selections, vec!["Ruby"]);
        assert_eq!(input.read(cx).value().as_ref(), "Ruby");
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        assert!(!view.read(cx).open);
        assert_eq!(window.find("autocomplete").value(), Some("Ruby"));
    })
    .unwrap();
    cx.run_until_parked();
    assert_eq!(cx.update(|cx| view.read(cx).selections.len()), 1);
}

#[gpui_kit::test]
fn navigation_and_confirm_do_not_change_text_until_owner_accepts(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let input = view.read(cx).input.clone();
        window.click(("input", input.entity_id()), cx);
        window.input("Ru", cx);
        window.press("down", cx);
        assert_eq!(view.read(cx).active, 1);
        assert_eq!(input.read(cx).value().as_ref(), "Ru");
        window.press("home", cx);
        assert_eq!(view.read(cx).active, 0);
        window.press("end", cx);
        assert_eq!(view.read(cx).active, 1);
        window.press("secondary-enter", cx);
        assert!(view.read(cx).selections.is_empty());
        assert!(view.read(cx).open);
        window.press("enter", cx);
        assert_eq!(view.read(cx).selections, vec!["Ruby"]);
        assert_eq!(input.read(cx).value().as_ref(), "Ruby");
        assert!(!view.read(cx).open);
        window.press("down", cx);
        assert!(view.read(cx).open);
        window.press("escape", cx);
        assert!(!view.read(cx).open);
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
    })
    .unwrap();
}

#[gpui_kit::test]
fn tab_shift_tab_and_programmatic_blur_close_without_selecting(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("before", cx);
        let before = window.focused(cx).unwrap();
        window.press("tab", cx);
        let input = view.read(cx).input.clone();
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        window.press("down", cx);
        assert!(view.read(cx).open);
        window.press("tab", cx);
        assert!(!view.read(cx).open);
        assert!(!input.read(cx).focus_handle(cx).is_focused(window));
        window.press("shift-tab", cx);
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        window.press("down", cx);
        window.press("shift-tab", cx);
        assert_eq!(window.focused(cx), Some(before.clone()));
        assert!(!view.read(cx).open);
        window.click(("input", input.entity_id()), cx);
        assert!(view.read(cx).open);
        before.focus(window, cx);
        window.render_frame(cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert!(!view.read(cx).open);
        assert!(view.read(cx).selections.is_empty());
    })
    .unwrap();
}

#[gpui_kit::test]
fn owner_can_reject_open_active_selection_and_close_requests(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        view.update(cx, |view, cx| {
            view.apply = false;
            cx.notify();
        });
        window.render_frame(cx);
        let input = view.read(cx).input.clone();
        window.click(("input", input.entity_id()), cx);
        assert!(!view.read(cx).open);
        assert_eq!(view.read(cx).opens, vec![true]);
        view.update(cx, |view, cx| {
            view.open = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
        window.input("Ru", cx);
        window.press("down", cx);
        assert_eq!(view.read(cx).active, 0);
        assert_eq!(view.read(cx).active_requests, vec![1]);
        window.press("enter", cx);
        assert_eq!(view.read(cx).selections, vec!["Rust"]);
        assert_eq!(input.read(cx).value().as_ref(), "Ru");
        assert!(view.read(cx).open);
        window.render_frame(cx);
        assert!(window.find("Name(\"autocomplete\")-suggest-0").visible());
        window.press("escape", cx);
        assert_eq!(view.read(cx).opens, vec![true, false, false]);
        assert!(view.read(cx).open);
    })
    .unwrap();
}

#[gpui_kit::test]
fn disabled_empty_and_invalid_active_states_are_safe(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        view.update(cx, |view, cx| {
            view.open = true;
            view.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(window.find("autocomplete").expanded(), Some(false));
        assert!(
            window
                .try_find("Name(\"autocomplete\")-suggest-0")
                .is_none()
        );
        let input = view.read(cx).input.clone();
        window.click(("input", input.entity_id()), cx);
        window.press("down", cx);
        window.press("enter", cx);
        assert!(view.read(cx).opens.is_empty());
        assert!(view.read(cx).selections.is_empty());
        view.update(cx, |view, cx| {
            view.disabled = false;
            view.options.clear();
            cx.notify();
        });
        window.render_frame(cx);
        window.click(("input", input.entity_id()), cx);
        window.input("unrestricted text", cx);
        assert_eq!(input.read(cx).value().as_ref(), "unrestricted text");
        assert_eq!(window.find("autocomplete").expanded(), Some(false));
        window.press("enter", cx);
        assert!(view.read(cx).selections.is_empty());
        window.press("escape", cx);
        assert!(!view.read(cx).open);
        view.update(cx, |view, cx| {
            view.options = vec!["unrestricted text".into()];
            view.open = true;
            view.active = usize::MAX;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).selections, vec!["unrestricted text"]);
    })
    .unwrap();
}

#[gpui_kit::test]
fn keyboard_reveals_options_beyond_the_native_scroll_viewport(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        view.update(cx, |view, cx| {
            view.options = (0..40).map(|i| format!("Language {i:02}").into()).collect();
            cx.notify();
        });
        window.render_frame(cx);
        let input = view.read(cx).input.clone();
        window.click(("input", input.entity_id()), cx);
        window.press("end", cx);
        window.render_frame(cx);
        let panel = window.find("Name(\"autocomplete\")-suggestions").bounds();
        let last = window.find("Name(\"autocomplete\")-suggest-39").bounds();
        assert!(last.top() >= panel.top() && last.bottom() <= panel.bottom());
        window.press("enter", cx);
        assert_eq!(view.read(cx).selections, vec!["Language 39"]);
    })
    .unwrap();
}

#[gpui_kit::test]
fn marked_ime_text_keeps_navigation_and_confirmation_in_the_editor(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let input = view.read(cx).input.clone();
        window.click(("input", input.entity_id()), cx);
        input.update(cx, |input, cx| {
            input.replace_and_mark_text_in_range(None, "Ru", Some(2..2), window, cx);
            assert!(input.marked_text_range(window, cx).is_some());
        });
        window.render_frame(cx);
        window.press("down", cx);
        window.press("enter", cx);
        assert!(view.read(cx).active_requests.is_empty());
        assert!(view.read(cx).selections.is_empty());
        input.update(cx, |input, cx| {
            input.replace_text_in_range(None, "Ru", window, cx);
            assert!(input.marked_text_range(window, cx).is_none());
        });
        window.render_frame(cx);
        window.press("down", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).selections, vec!["Ruby"]);
        assert_eq!(input.read(cx).value().as_ref(), "Ruby");
    })
    .unwrap();
}

#[gpui_kit::test]
fn popup_registration_tracks_the_controlled_visible_surface(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert!(!GlobalState::is_in_deferred_context(cx));
        let input = view.read(cx).input.clone();
        window.click(("input", input.entity_id()), cx);
        assert!(GlobalState::is_in_deferred_context(cx));
        window.press("escape", cx);
        assert!(!GlobalState::is_in_deferred_context(cx));
        window.press("down", cx);
        assert!(GlobalState::is_in_deferred_context(cx));
        view.update(cx, |view, cx| {
            view.options.clear();
            cx.notify();
        });
        window.render_frame(cx);
        assert!(!GlobalState::is_in_deferred_context(cx));
    })
    .unwrap();
}
