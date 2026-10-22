//! Exercise the public Tabs wrapper through actual GPUI window event dispatch.
extern crate gpui_kit as gpui;

use gpui_antd::{AntInput, AntTab, AntTabs, AntTheme, ControlSize, TabsVariant};
use gpui_kit::base::Button;
use gpui_kit::component::input::InputState;
use gpui_kit::test::TestWindowExt as _;
use gpui_kit::{
    AppContext as _, Bounds, Context, Entity, InputEvent as _, IntoElement, KeyDownEvent,
    KeyUpEvent, Keystroke, ParentElement as _, Render, Role, Styled as _, TestAppContext, Window,
    WindowBounds, WindowOptions, div, px, size,
};

struct TabsOwner {
    selected: usize,
    requests: Vec<usize>,
    apply: bool,
    keyboard: bool,
    all_disabled: bool,
    panel_clicks: usize,
    variant: TabsVariant,
    size: ControlSize,
    dark: bool,
    editor: Option<Entity<InputState>>,
}

impl Default for TabsOwner {
    fn default() -> Self {
        Self {
            selected: 0,
            requests: vec![],
            apply: true,
            keyboard: true,
            all_disabled: false,
            panel_clicks: 0,
            variant: TabsVariant::Line,
            size: ControlSize::Middle,
            dark: false,
            editor: None,
        }
    }
}

impl Render for TabsOwner {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.weak_entity();
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .child(Button::new("before").child("Before tabs"))
            .child(
                AntTabs::new("tested-tabs")
                    .active_index(self.selected)
                    .keyboard(self.keyboard)
                    .variant(self.variant)
                    .size(self.size)
                    .theme(if self.dark {
                        AntTheme::dark()
                    } else {
                        AntTheme::light()
                    })
                    .on_change(move |index, _, cx| {
                        owner
                            .update(cx, |view, cx| {
                                view.requests.push(index);
                                if view.apply {
                                    view.selected = index;
                                }
                                cx.notify();
                            })
                            .unwrap();
                    })
                    .child(
                        AntTab::new("Overview")
                            .disabled(self.all_disabled)
                            .content(Button::new("overview-action").child("Overview action")),
                    )
                    .child(
                        AntTab::new("Blocked")
                            .disabled(true)
                            .content(Button::new("blocked-action").child("Must stay absent")),
                    )
                    .child(
                        AntTab::new("Settings").disabled(self.all_disabled).content(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(AntInput::new(self.editor.as_ref().unwrap()))
                                .child(
                                    Button::new("settings-action")
                                        .child("Settings action")
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.panel_clicks += 1;
                                            cx.notify();
                                        })),
                                ),
                        ),
                    ),
            )
            .child(Button::new("after").child("After tabs"))
    }
}

fn open(
    cx: &mut TestAppContext,
    mut config: TabsOwner,
) -> (gpui_kit::AnyWindowHandle, Entity<TabsOwner>) {
    cx.update(gpui_kit::init);
    cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Default::default(),
                    size(px(800.), px(600.)),
                ))),
                ..Default::default()
            },
            cx,
            |window, cx| {
                config.editor = Some(cx.new(|cx| InputState::new(window, cx)));
                cx.new(|_| config)
            },
        )
        .unwrap()
    })
}

#[gpui_kit::test]
fn arrows_roam_real_tabs_and_replace_only_the_selected_panel(cx: &mut TestAppContext) {
    for dark in [false, true] {
        for variant in [
            TabsVariant::Line,
            TabsVariant::Card,
            TabsVariant::Pills,
            TabsVariant::Segmented,
        ] {
            for size in [ControlSize::Small, ControlSize::Middle, ControlSize::Large] {
                let (handle, view) = open(
                    cx,
                    TabsOwner {
                        dark,
                        variant,
                        size,
                        ..Default::default()
                    },
                );
                cx.update_window(handle, |_, window, cx| {
                    window.render_frame(cx);
                    assert_eq!(window.find(0usize).selected(), Some(true));
                    assert_eq!(window.find(0usize).role(), Some(Role::Tab));
                    assert_eq!(window.find(0usize).label(), Some("Overview"));
                    // Kit 0.7.3 omits the native disabled flag. Prove
                    // rejection via input; do not infer it from metadata.
                    window.click(1usize, cx);
                    assert!(view.read(cx).requests.is_empty());
                    assert!(window.try_find("blocked-action").is_none());
                    window.click(0usize, cx);
                    let overview_focus = window.focused(cx).expect("clicked tab focus");
                    window.press("right", cx);
                    assert_eq!(view.read(cx).selected, 2);
                    let settings_focus = window.focused(cx).expect("roved tab focus");
                    assert_ne!(overview_focus, settings_focus);
                    assert_eq!(window.find(2usize).selected(), Some(true));
                    assert!(window.try_find("overview-action").is_none());
                    assert!(window.find("settings-action").visible());
                    window.press("right", cx);
                    assert_eq!(view.read(cx).selected, 0);
                    assert_eq!(window.focused(cx), Some(overview_focus));
                    window.press("left", cx);
                    assert_eq!(view.read(cx).selected, 2);
                    window.press("home", cx);
                    assert_eq!(view.read(cx).selected, 0);
                    window.press("end", cx);
                    assert_eq!(view.read(cx).selected, 2);
                    window.press("enter", cx);
                    window.press("space", cx);
                    assert_eq!(view.read(cx).requests, [0, 2, 0, 2, 0, 2]);
                })
                .unwrap();
            }
        }
    }
}

#[gpui_kit::test]
fn home_and_end_request_selection_when_focus_is_ahead_of_the_owner(cx: &mut TestAppContext) {
    let (handle, view) = open(
        cx,
        TabsOwner {
            apply: false,
            ..Default::default()
        },
    );
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click(0usize, cx);
        window.press("right", cx);
        assert_eq!(view.read(cx).selected, 0);
        let settings_focus = window.focused(cx).expect("roved tab focus");
        window.press("end", cx);
        assert_eq!(window.focused(cx), Some(settings_focus));
        assert_eq!(view.read(cx).requests, [0, 2, 2]);
        window.press("enter", cx);
        window.press("space", cx);
        assert_eq!(view.read(cx).requests, [0, 2, 2, 2, 2]);
        view.update(cx, |view, cx| {
            view.selected = 2;
            cx.notify();
        });
        window.render_frame(cx);
        window.press("left", cx);
        let overview_focus = window.focused(cx).expect("roved tab focus");
        window.press("home", cx);
        assert_eq!(window.focused(cx), Some(overview_focus));
        assert_eq!(view.read(cx).requests, [0, 2, 2, 2, 2, 0, 0]);
    })
    .unwrap();
}

#[gpui_kit::test]
fn confirmation_activates_on_release_and_cancels_when_focus_moves(cx: &mut TestAppContext) {
    for key in ["enter", "space"] {
        let (handle, view) = open(
            cx,
            TabsOwner {
                apply: false,
                ..Default::default()
            },
        );
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            window.click(0usize, cx);
            window.press("right", cx);
            let keystroke = Keystroke::parse(key).unwrap();
            let down = KeyDownEvent {
                keystroke: keystroke.clone(),
                is_held: false,
                prefer_character_input: false,
            };
            window.dispatch_event(down.clone().to_platform_input(), cx);
            window.render_frame(cx);
            assert_eq!(view.read(cx).requests, [0, 2]);
            window.dispatch_event(
                KeyUpEvent {
                    keystroke: keystroke.clone(),
                }
                .to_platform_input(),
                cx,
            );
            window.render_frame(cx);
            assert_eq!(view.read(cx).requests, [0, 2, 2]);
            window.dispatch_event(down.to_platform_input(), cx);
            window.render_frame(cx);
            window.press("left", cx);
            window.dispatch_event(KeyUpEvent { keystroke }.to_platform_input(), cx);
            window.render_frame(cx);
            assert_eq!(view.read(cx).requests, [0, 2, 2]);
        })
        .unwrap();
    }
}

#[gpui_kit::test]
fn tab_order_and_panel_controls_preserve_selection_and_focus(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, TabsOwner::default());
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("before", cx);
        window.press("tab", cx);
        let overview_focus = window.focused(cx).expect("Tab enters tab list");
        window.press("end", cx);
        let settings_focus = window.focused(cx).expect("End focuses last tab");
        assert_ne!(overview_focus, settings_focus);
        window.press("tab", cx);
        let editor = view.read(cx).editor.as_ref().unwrap().clone();
        assert_eq!(
            window.find(("input", editor.entity_id())).focused(),
            Some(true)
        );
        window.input("draft", cx);
        window.press("home", cx);
        window.press("end", cx);
        window.press("left", cx);
        assert_eq!(view.read(cx).selected, 2);
        assert_eq!(view.read(cx).requests, [2]);
        assert_eq!(editor.read(cx).value().as_ref(), "draft");
        window.press("tab", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).panel_clicks, 1);
        assert_eq!(window.find("settings-action").focused(), Some(true));
        window.press("tab", cx);
        assert_eq!(window.find("after").focused(), Some(true));
        window.press("shift-tab", cx);
        assert_eq!(window.find("settings-action").focused(), Some(true));
        window.press("shift-tab", cx);
        assert_eq!(
            window.find(("input", editor.entity_id())).focused(),
            Some(true)
        );
        window.press("shift-tab", cx);
        assert_eq!(window.focused(cx), Some(settings_focus));
        window.press("shift-tab", cx);
        assert_eq!(window.find("before").focused(), Some(true));
    })
    .unwrap();
}

#[gpui_kit::test]
fn keyboard_opt_out_preserves_pointer_and_panel_activation(cx: &mut TestAppContext) {
    let (handle, view) = open(
        cx,
        TabsOwner {
            keyboard: false,
            ..Default::default()
        },
    );
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click(2usize, cx);
        assert_eq!(view.read(cx).selected, 2);
        window.press("left", cx);
        window.press("home", cx);
        assert_eq!(view.read(cx).requests, [2]);
        window.click("settings-action", cx);
        assert_eq!(view.read(cx).panel_clicks, 1);
        window.click("before", cx);
        window.press("tab", cx);
        let editor = view.read(cx).editor.as_ref().unwrap();
        assert_eq!(
            window.find(("input", editor.entity_id())).focused(),
            Some(true)
        );
    })
    .unwrap();
}

#[gpui_kit::test]
fn modified_navigation_leaves_owner_selection_untouched(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, TabsOwner::default());
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click(0usize, cx);
        let focus = window.focused(cx);
        for key in [
            "ctrl-right",
            "shift-end",
            "alt-left",
            "secondary-home",
            "ctrl-enter",
            "alt-space",
        ] {
            window.press(key, cx);
            assert_eq!(view.read(cx).selected, 0);
            assert_eq!(view.read(cx).requests, [0]);
            assert_eq!(window.focused(cx), focus);
        }
    })
    .unwrap();
}

struct TwoGroups {
    selected: [usize; 2],
    requests: Vec<(usize, usize)>,
}
impl Render for TwoGroups {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().flex().flex_col().gap_4().children(
            ["first-group", "second-group"]
                .into_iter()
                .enumerate()
                .map(|(group, id)| {
                    let owner = cx.weak_entity();
                    AntTabs::new(id)
                        .active_index(self.selected[group])
                        .child(AntTab::new("One"))
                        .child(AntTab::new("Two"))
                        .on_change(move |index, _, cx| {
                            owner
                                .update(cx, |view, cx| {
                                    view.selected[group] = index;
                                    view.requests.push((group, index));
                                    cx.notify();
                                })
                                .unwrap();
                        })
                }),
        )
    }
}

#[gpui_kit::test]
fn independent_tab_groups_do_not_share_focus_or_requests(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let (handle, view) = cx.update(|cx| {
        gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|_| TwoGroups {
                selected: [0, 0],
                requests: vec![],
            })
        })
        .unwrap()
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.within("first-group").click(0usize, cx);
        let first_focus = window.focused(cx);
        window.press("right", cx);
        assert_eq!(view.read(cx).selected, [1, 0]);
        window.within("second-group").click(0usize, cx);
        assert_ne!(window.focused(cx), first_focus);
        window.press("right", cx);
        assert_eq!(view.read(cx).selected, [1, 1]);
        window.press("tab", cx);
        window.press("home", cx);
        assert_eq!(window.focused(cx), first_focus);
        assert_eq!(view.read(cx).selected, [0, 1]);
        assert_eq!(
            view.read(cx).requests,
            [(0, 0), (0, 1), (1, 0), (1, 1), (0, 0)]
        );
    })
    .unwrap();
}

#[gpui_kit::test]
fn disabled_and_invalid_active_indices_keep_a_safe_tab_order(cx: &mut TestAppContext) {
    for selected in [1, usize::MAX] {
        let (handle, view) = open(
            cx,
            TabsOwner {
                selected,
                ..Default::default()
            },
        );
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            assert!(window.try_find("blocked-action").is_none());
            assert!(window.try_find("overview-action").is_none());
            assert!(window.try_find("settings-action").is_none());
            window.click("before", cx);
            window.press("tab", cx);
            assert!(window.focused(cx).is_some());
            window.press("home", cx);
            assert_eq!(view.read(cx).selected, 0);
            assert_eq!(view.read(cx).requests, [0]);
        })
        .unwrap();
    }
    let (handle, view) = open(
        cx,
        TabsOwner {
            all_disabled: true,
            ..Default::default()
        },
    );
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("before", cx);
        window.press("tab", cx);
        assert_eq!(window.find("after").focused(), Some(true));
        window.click(0usize, cx);
        window.click(2usize, cx);
        assert!(view.read(cx).requests.is_empty());
    })
    .unwrap();
}
