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
    App, AppContext, Bounds, Context, Entity, IntoElement, ParentElement as _, Render, ScrollDelta,
    SharedString, Styled as _, TestAppContext, Window, WindowBounds, WindowOptions, div, point, px,
    size,
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
        TooltipPlacement::Bottom,
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
                TooltipPlacement::Top => assert!(bubble.bottom() <= trigger.top()),
                TooltipPlacement::Bottom => assert!(bubble.top() >= trigger.bottom()),
                TooltipPlacement::Left => assert!(bubble.right() <= trigger.left()),
                TooltipPlacement::Right => assert!(bubble.left() >= trigger.right()),
            }
        })
        .unwrap();
    }
}
