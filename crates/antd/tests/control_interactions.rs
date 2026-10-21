//! Regressions exercise the production wrappers through real GPUI event dispatch.
extern crate gpui_kit as gpui;

use chrono::{NaiveDate, Timelike};
use gpui_antd::{
    AntDatePicker, AntDropdown, AntDropdownItem, AntInputNumber, AntTheme, AntTimePicker,
    AntTooltip, TooltipPlacement,
};
use gpui_kit::base::Button;
use gpui_kit::component::{
    calendar::Date,
    date_picker::{DatePickerState, DateRangePreset},
    input::InputState,
    time_field::{TimeFieldState, TimePrecision},
};
use gpui_kit::test::TestWindowExt;
use gpui_kit::{
    App, AppContext, Bounds, Context, Entity, Focusable as _, IntoElement, ParentElement as _,
    Render, ScrollDelta, SharedString, Styled as _, TestAppContext, Window, WindowBounds,
    WindowOptions, div, point, px, size,
};
use std::time::Duration;

fn open<V: Render>(
    cx: &mut TestAppContext,
    build: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
) -> (gpui_kit::AnyWindowHandle, Entity<V>) {
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
            build,
        )
        .unwrap()
    })
}
fn day(value: &str) -> NaiveDate {
    value.parse().unwrap()
}

struct Schedule {
    state: Entity<DatePickerState>,
    disabled: bool,
    range: bool,
}
impl Render for Schedule {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_4().child(
            AntDatePicker::new(&self.state)
                .disabled(self.disabled)
                .clearable(true)
                .presets(vec![if self.range {
                    DateRangePreset::range("Release week", day("2026-09-14"), day("2026-09-18"))
                } else {
                    DateRangePreset::single("Release day", day("2026-09-15"))
                }]),
        )
    }
}
#[gpui_kit::test]
fn date_picker_rejects_disabled_days_and_keeps_presets(cx: &mut TestAppContext) {
    let (window, view) = open(cx, |window, cx| {
        cx.new(|cx| {
            let state = cx.new(|cx| {
                let mut state = DatePickerState::new(window, cx)
                    .disabled_matcher(|date: &NaiveDate| *date == day("2026-09-16"));
                state.set_date(day("2026-09-15"), window, cx);
                state
            });
            Schedule {
                state,
                disabled: false,
                range: false,
            }
        })
    });
    let id = cx.update(|cx| ("date-picker", view.read(cx).state.entity_id()));
    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
        window.click(id, cx);
        window.click("calendar-2026-09-16-0-2", cx);
        assert_eq!(
            view.read(cx).state.read(cx).date(),
            Date::Single(Some(day("2026-09-15")))
        );
        window.click("clean", cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
        window.click(id, cx);
        assert!(window.find(("preset", 0usize)).visible());
        assert!(
            window.find(("preset", 0usize)).bounds().top() >= window.find(id).bounds().bottom()
        );
        window.click(("preset", 0usize), cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            view.read(cx).state.read(cx).date(),
            Date::Single(Some(day("2026-09-15")))
        )
    });
}
#[gpui_kit::test]
fn range_presets_and_disabled_picker_keep_their_contract(cx: &mut TestAppContext) {
    let (window, view) = open(cx, |window, cx| {
        cx.new(|cx| Schedule {
            state: cx.new(|cx| DatePickerState::range(window, cx)),
            disabled: false,
            range: true,
        })
    });
    let id = cx.update(|cx| ("date-picker", view.read(cx).state.entity_id()));
    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
        window.click(id, cx);
        window.click(("preset", 0usize), cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            view.read(cx).state.read(cx).date(),
            Date::Range(Some(day("2026-09-14")), Some(day("2026-09-18")))
        );
        view.update(cx, |view, cx| {
            view.disabled = true;
            cx.notify();
        });
    });
    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
        window.click(id, cx);
        assert_eq!(window.find(id).expanded(), Some(false));
    })
    .unwrap();
}
#[gpui_kit::test]
fn date_time_editor_remains_available_after_picking_a_day(cx: &mut TestAppContext) {
    let (window, view) = open(cx, |window, cx| {
        cx.new(|cx| Schedule {
            state: cx.new(|cx| {
                let mut state =
                    DatePickerState::new(window, cx).time_precision(TimePrecision::Second);
                state.set_date_time(
                    NaiveDate::from_ymd_opt(2026, 9, 15)
                        .unwrap()
                        .and_hms_opt(8, 0, 0)
                        .unwrap(),
                    window,
                    cx,
                );
                state
            }),
            disabled: false,
            range: false,
        })
    });
    let id = cx.update(|cx| ("date-picker", view.read(cx).state.entity_id()));
    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
        window.click(id, cx);
        window.click("calendar-2026-09-16-0-2", cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find(id).expanded(), Some(true));
        window.within("time").click("minute", cx);
        window.press("4", cx);
        window.press("5", cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            view.read(cx)
                .state
                .read(cx)
                .date_time()
                .start()
                .unwrap()
                .minute(),
            45
        )
    });
}

struct Menu {
    selected: Vec<SharedString>,
    dark: bool,
}
impl Render for Menu {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.weak_entity();
        div().size_full().p_4().child(
            AntDropdown::new("actions", "Actions")
                .theme(if self.dark {
                    AntTheme::dark()
                } else {
                    AntTheme::light()
                })
                .item(AntDropdownItem::new("first", "First"))
                .item(AntDropdownItem::separator())
                .item(AntDropdownItem::new("disabled", "Disabled").disabled(true))
                .item(AntDropdownItem::new("last", "Last"))
                .on_select(move |key, _, cx| {
                    owner
                        .update(cx, |view, cx| {
                            view.selected.push(key);
                            cx.notify();
                        })
                        .unwrap();
                }),
        )
    }
}
#[gpui_kit::test]
fn dropdown_keyboard_skips_disabled_items_and_dismisses(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, |_, cx| {
        cx.new(|_| Menu {
            selected: vec![],
            dark: true,
        })
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("ant-dropdown-trigger:Name(\"actions\")", cx);
        assert!(
            window.find("ant-dropdown-menu").bounds().top()
                >= window
                    .find("ant-dropdown-trigger:Name(\"actions\")")
                    .bounds()
                    .bottom()
        );
        window.press("down", cx);
        assert_eq!(window.find("ant-dropdown-item:first").focused(), Some(true));
        window.press("down", cx);
        assert_eq!(window.find("ant-dropdown-item:last").focused(), Some(true));
        window.press("home", cx);
        assert_eq!(window.find("ant-dropdown-item:first").focused(), Some(true));
        window.press("end", cx);
        window.press("enter", cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            view.read(cx).selected.as_slice(),
            &[SharedString::from("last")]
        )
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert!(window.try_find("ant-dropdown-item:last").is_none());
        window.click("ant-dropdown-trigger:Name(\"actions\")", cx);
        window.click("ant-dropdown-item:disabled", cx);
        assert!(view.read(cx).selected.len() == 1);
        window.press("home", cx);
        window.press("space", cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            view.read(cx).selected.as_slice(),
            &[SharedString::from("last"), SharedString::from("first")]
        )
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("ant-dropdown-trigger:Name(\"actions\")", cx);
        window.press("down", cx);
        window.press("escape", cx);
        assert!(window.try_find("ant-dropdown-item:first").is_none());
    })
    .unwrap();
}

struct Number {
    state: Entity<InputState>,
    hint: Option<SharedString>,
}
impl Render for Number {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let mut input = AntInputNumber::new(&self.state);
        if let Some(hint) = &self.hint {
            input = input.placeholder(hint.clone());
        }
        div().size_full().p_4().child(input)
    }
}
#[gpui_kit::test]
fn number_placeholder_updates_without_overwriting_the_value(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, |window, cx| {
        cx.new(|cx| Number {
            state: cx.new(|cx| InputState::new(window, cx).placeholder("State hint")),
            hint: Some("Amount".into()),
        })
    });
    cx.update_window(handle, |_, window, cx| window.render_frame(cx))
        .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            view.read(cx)
                .state
                .read(cx)
                .presentation()
                .placeholder()
                .as_ref(),
            "Amount"
        );
        view.update(cx, |view, cx| {
            view.hint = Some("Quantity".into());
            cx.notify();
        });
    });
    cx.update_window(handle, |_, window, cx| {
        view.read(cx)
            .state
            .clone()
            .update(cx, |state, cx| state.set_value("12", window, cx));
        window.render_frame(cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            view.read(cx)
                .state
                .read(cx)
                .presentation()
                .placeholder()
                .as_ref(),
            "Quantity"
        );
        assert_eq!(view.read(cx).state.read(cx).value().as_ref(), "12");
        view.update(cx, |view, cx| {
            view.hint = None;
            cx.notify();
        });
    });
    cx.update_window(handle, |_, window, cx| window.render_frame(cx))
        .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            view.read(cx)
                .state
                .read(cx)
                .presentation()
                .placeholder()
                .as_ref(),
            "Quantity"
        )
    });
}

struct Clock {
    state: Entity<TimeFieldState>,
}
impl Render for Clock {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_4()
            .child(AntTimePicker::new(&self.state).show_second(true).open(true))
    }
}
#[gpui_kit::test]
fn time_columns_scroll_to_late_values_and_commit_them(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, |window, cx| {
        cx.new(|cx| Clock {
            state: cx.new(|cx| TimeFieldState::new(window, cx).precision(TimePrecision::Second)),
        })
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        assert!(!window.find("ant-time-cell-Min-45").visible());
        window.scroll(
            "ant-time-column-Min",
            ScrollDelta::Pixels(point(px(0.), px(-1270.))),
            cx,
        );
        assert!(window.find("ant-time-cell-Min-45").visible());
        window.click("ant-time-cell-Min-45", cx);
        window.scroll(
            "ant-time-column-Sec",
            ScrollDelta::Pixels(point(px(0.), px(-1560.))),
            cx,
        );
        assert!(window.find("ant-time-cell-Sec-55").visible());
        window.click("ant-time-cell-Sec-55", cx);
    })
    .unwrap();
    cx.update(|cx| {
        let value = view.read(cx).state.read(cx).time();
        assert_eq!(value.minute(), 45);
        assert_eq!(value.second(), 55);
    });
}

struct Tip {
    placement: TooltipPlacement,
}
impl Render for Tip {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().pt(px(300.)).pl(px(300.)).child(
            AntTooltip::new("tip", "Helpful text")
                .placement(self.placement)
                .child(
                    Button::new("tip-trigger")
                        .w(px(80.))
                        .h(px(32.))
                        .child("Help"),
                ),
        )
    }
}
#[gpui_kit::test]
fn tooltip_side_matches_requested_placement(cx: &mut TestAppContext) {
    for placement in [
        TooltipPlacement::Top,
        TooltipPlacement::TopLeft,
        TooltipPlacement::TopRight,
        TooltipPlacement::Bottom,
        TooltipPlacement::BottomLeft,
        TooltipPlacement::BottomRight,
        TooltipPlacement::Left,
        TooltipPlacement::Right,
    ] {
        let (handle, _) = open(cx, |_, cx| cx.new(|_| Tip { placement }));
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            window.hover("tip-trigger", cx);
        })
        .unwrap();
        cx.background_executor
            .advance_clock(Duration::from_millis(120));
        cx.run_until_parked();
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            window.render_frame(cx);
            let trigger = window.find("tip-trigger").bounds();
            let bubble = window
                .within("ant-tooltip-body-Name(\"tip\")")
                .find("ant-tooltip-text")
                .bounds();
            match placement {
                TooltipPlacement::Top | TooltipPlacement::TopLeft | TooltipPlacement::TopRight => {
                    assert!(bubble.bottom() <= trigger.top());
                }
                TooltipPlacement::Bottom
                | TooltipPlacement::BottomLeft
                | TooltipPlacement::BottomRight => {
                    assert!(bubble.top() >= trigger.bottom());
                }
                TooltipPlacement::Left => assert!(bubble.right() <= trigger.left()),
                TooltipPlacement::Right => assert!(bubble.left() >= trigger.right()),
            }
        })
        .unwrap();
    }
}

struct CatalogPicker {
    open: bool,
    path: Vec<String>,
    dark: bool,
}
impl Render for CatalogPicker {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use gpui_antd::{AntCascader, AntCascaderOption};
        let owner = cx.weak_entity();
        let selection = owner.clone();
        div()
            .size_full()
            .p_4()
            .child(
                AntCascader::new("catalog-cascader")
                    .theme(if self.dark {
                        AntTheme::dark()
                    } else {
                        AntTheme::light()
                    })
                    .open(self.open)
                    .path(self.path.clone())
                    .option(
                        AntCascaderOption::new("province", "Province")
                            .child(AntCascaderOption::new("city", "City"))
                            .child(AntCascaderOption::new("disabled", "Disabled").disabled(true)),
                    )
                    .on_open_change(move |open, _, cx| {
                        owner
                            .update(cx, |view, cx| {
                                view.open = open;
                                cx.notify();
                            })
                            .unwrap();
                    })
                    .on_change(move |path, _, _, cx| {
                        selection
                            .update(cx, |view, cx| {
                                view.path = path;
                                cx.notify();
                            })
                            .unwrap();
                    }),
            )
            .child(Button::new("outside").child("Outside"))
    }
}
#[gpui_kit::test]
fn cascader_places_below_trigger_selects_and_dismisses(cx: &mut TestAppContext) {
    for dark in [false, true] {
        let (handle, view) = open(cx, |_, cx| {
            cx.new(|_| CatalogPicker {
                open: false,
                path: vec![],
                dark,
            })
        });
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let trigger = "cascader-trigger-Name(\"catalog-cascader\")";
            window.click(trigger, cx);
            assert!(
                window.find("cascader-0-province").bounds().top()
                    >= window.find(trigger).bounds().bottom()
            );
            window.click("cascader-0-province", cx);
            window.click("cascader-1-disabled", cx);
            assert_eq!(view.read(cx).path, ["province"]);
            window.click("cascader-1-city", cx);
            assert_eq!(view.read(cx).path, ["province", "city"]);
            assert!(!view.read(cx).open);
            assert!(window.try_find("cascader-0-province").is_none());
            window.click(trigger, cx);
            window.press("escape", cx);
            assert!(!view.read(cx).open);
            window.click(trigger, cx);
            window.click("outside", cx);
            assert!(!view.read(cx).open);
        })
        .unwrap();
    }
}

struct MentionField {
    input: Entity<InputState>,
    _subscription: gpui_kit::Subscription,
}
impl Render for MentionField {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use gpui_antd::{AntInput, AntMentionOption, AntMentions};
        let text = self.input.read(cx).value().to_string();
        div()
            .size_full()
            .p_4()
            .child(
                AntMentions::new("mentions")
                    .track_focus(&self.input.read(cx).focus_handle(cx))
                    .text(text.clone(), text.len())
                    .trigger(AntInput::new(&self.input))
                    .option(AntMentionOption::new("ada", "Ada Lovelace")),
            )
            .child(Button::new("outside").child("Outside"))
    }
}
#[gpui_kit::test]
fn mentions_dismissal_survives_rerender_and_query_changes(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, |window, cx| {
        cx.new(|cx| {
            let input = cx.new(|cx| InputState::new(window, cx));
            let subscription = cx.subscribe(
                &input,
                |_, _, _: &gpui_kit::component::input::InputEvent, cx| cx.notify(),
            );
            MentionField {
                input,
                _subscription: subscription,
            }
        })
    });
    cx.update_window(handle, |_, window, cx| {
        view.read(cx)
            .input
            .clone()
            .update(cx, |input, cx| input.set_value("@a", window, cx));
        window.render_frame(cx);
        assert!(window.find("mention-ada").visible());
        window.press("escape", cx);
        assert!(window.try_find("mention-ada").is_none());
        view.update(cx, |_, cx| cx.notify());
        window.render_frame(cx);
        assert!(window.try_find("mention-ada").is_none());
    })
    .unwrap();
    cx.update_window(handle, |_, window, cx| {
        view.read(cx)
            .input
            .clone()
            .update(cx, |input, cx| input.set_value("@ad", window, cx));
        window.render_frame(cx);
        assert!(window.find("mention-ada").visible());
        window.click("outside", cx);
        window.render_frame(cx);
        assert!(window.try_find("mention-ada").is_none());
    })
    .unwrap();
}

#[gpui_kit::test]
fn mentions_keeps_typing_in_the_editor_when_suggestions_open(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, |window, cx| {
        cx.new(|cx| {
            let input = cx.new(|cx| InputState::new(window, cx));
            let subscription = cx.subscribe(
                &input,
                |_, _, _: &gpui_kit::component::input::InputEvent, cx| cx.notify(),
            );
            MentionField {
                input,
                _subscription: subscription,
            }
        })
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let id = ("input", view.read(cx).input.entity_id());
        window.click(id, cx);
        window.input("@", cx);
        assert!(window.find("mention-ada").visible());
        window.input("a", cx);
        assert_eq!(view.read(cx).input.read(cx).value().as_ref(), "@a");
        window.press("escape", cx);
        assert!(window.try_find("mention-ada").is_none());
        window.input("d", cx);
        assert_eq!(view.read(cx).input.read(cx).value().as_ref(), "@ad");
        assert!(window.find("mention-ada").visible());
        window.press("escape", cx);
        window.press("secondary-a", cx);
        window.press("backspace", cx);
        window.input("@ad", cx);
        assert!(window.find("mention-ada").visible());
    })
    .unwrap();
}

struct TourCard {
    open: bool,
}
impl Render for TourCard {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        use gpui_antd::{AntTour, AntTourStep};
        div().size_full().child(
            AntTour::new("tour")
                .open(self.open)
                .step(AntTourStep::new("Welcome", "First step")),
        )
    }
}
#[gpui_kit::test]
fn targetless_tour_honors_parent_visibility(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, |_, cx| cx.new(|_| TourCard { open: false }));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert!(window.try_find("Name(\"tour\")-next").is_none());
        view.update(cx, |view, cx| {
            view.open = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert!(window.find("Name(\"tour\")-next").visible());
        view.update(cx, |view, cx| {
            view.open = false;
            cx.notify();
        });
        window.render_frame(cx);
        assert!(window.try_find("Name(\"tour\")-next").is_none());
    })
    .unwrap();
}

struct CollapsedMenu {
    open: bool,
}
impl Render for CollapsedMenu {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use gpui_antd::{AntMenu, AntMenuItem};
        let owner = cx.weak_entity();
        div().size_full().p_4().child(
            div().w(px(76.)).child(
                AntMenu::new("rail")
                    .inline_collapsed(true)
                    .open_keys(if self.open { vec!["reports"] } else { vec![] })
                    .item(
                        AntMenuItem::submenu("reports", "Reports")
                            .child(AntMenuItem::new("daily", "Daily report")),
                    )
                    .on_open_change(move |_, open, _, cx| {
                        owner
                            .update(cx, |view, cx| {
                                view.open = open;
                                cx.notify();
                            })
                            .unwrap();
                    }),
            ),
        )
    }
}
#[gpui_kit::test]
fn collapsed_menu_flyout_is_right_of_rail_and_escape_closes(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, |_, cx| cx.new(|_| CollapsedMenu { open: false }));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let trigger = "ant-menu-Name(\"rail\")-reports";
        let child = "ant-menu-Name(\"ant-menu-flyout-list-Name(\\\"rail\\\")-reports\")-daily";
        window.click(trigger, cx);
        assert!(view.read(cx).open);
        assert!(window.find(child).bounds().left() >= window.find(trigger).bounds().right());
        window.press("escape", cx);
        assert!(!view.read(cx).open);
        assert!(window.try_find(child).is_none());
    })
    .unwrap();
}
