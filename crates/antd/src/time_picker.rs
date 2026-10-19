//! Ant Design TimePicker on GPUI Kit base popover + TimeFieldState model.
//!
//! - Model: `Entity<TimeFieldState>` (`time` / `set_time`). No second clock.
//! - Trigger: one Ant frame; Kit `TimeField` sits inside with `border_0` for
//!   segment keyboard editing (not reimplemented).
//! - Popup: `BasePopover` column panel (hour / minute / optional second / period).
//! - Precision and 12h mode are **not** readable from TimeFieldState (private
//!   editor). Mirror them with `.show_second(true)` / `.hour12(true)` on this
//!   wrapper when the state was built that way.
//!
//! ## Gaps
//! - No `clearable` — TimeField has no clear API in 0.7.3.
//! - Selected values are not automatically scrolled into view.
//! - Ranges / time-range picker not implemented.
//! - Gallery must pass `.show_second(true)` / `.hour12(true)` to match states
//!   configured with `TimePrecision::Second` / `HourCycle::H12`.

use std::rc::Rc;

use chrono::{NaiveTime, Timelike};
use gpui_kit::base::{Button as BaseButton, Popover as BasePopover, StyledExt as _, TestSupportExt as _};
use gpui_kit::component::time_field::{TimeField, TimeFieldState};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    Anchor, App, Entity, Hsla, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};
use gpui_kit::component::{Disableable as _, Sizable as _, Size};
use crate::design_tokens::AntComponent;
use crate::{
    AntIcon, AntIconName, AntTheme, ControlSize, InputStatus, InputVariant, color,
};

type OpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// Pure field chrome for the time trigger frame.
#[derive(Debug, Clone, Copy, PartialEq)]
struct TimePickerChrome {
    bg: Hsla,
    border: Hsla,
    text: Hsla,
    hover_border: Hsla,
    radius: f32,
    height: f32,
    bordered: bool,
}

fn time_picker_chrome(
    theme: &AntTheme,
    size: ControlSize,
    variant: InputVariant,
    status: InputStatus,
    disabled: bool,
) -> TimePickerChrome {
    let tokens = theme.component(AntComponent::TimePicker);
    let bordered = variant.bordered();
    let bg = if disabled {
        theme.disabled_bg
    } else if !variant.framed() {
        color(0).opacity(0.)
    } else if variant == InputVariant::Filled {
        tokens.color_bg.unwrap_or(theme.surface_secondary)
    } else {
        tokens.color_bg.unwrap_or(theme.surface)
    };
    let border = match status {
        InputStatus::Error => theme.error,
        InputStatus::Warning => theme.warning,
        InputStatus::Normal => tokens.color_border.unwrap_or(theme.border),
    };
    let hover_border = match status {
        InputStatus::Error => theme.error,
        InputStatus::Warning => theme.warning,
        InputStatus::Normal => theme.primary,
    };
    let text = if disabled {
        theme.disabled_text
    } else {
        tokens.color_text.unwrap_or(theme.text)
    };
    TimePickerChrome {
        bg,
        border,
        text,
        hover_border,
        radius: tokens.border_radius.unwrap_or(theme.border_radius),
        height: tokens.control_height.unwrap_or(size.height()),
        bordered,
    }
}

/// Selected hour column index (0-based) for a given wall time.
/// - 24h: hour 0..=23 → index = hour
/// - 12h: hour 1..=12 display → index = display - 1
pub fn selected_hour_index(time: NaiveTime, hour12: bool) -> usize {
    if hour12 {
        let h = time.hour() % 12;
        let display = if h == 0 { 12 } else { h };
        (display - 1) as usize
    } else {
        time.hour() as usize
    }
}

pub fn selected_minute_index(time: NaiveTime) -> usize {
    time.minute() as usize
}

pub fn selected_second_index(time: NaiveTime) -> usize {
    time.second() as usize
}

pub fn selected_period_index(time: NaiveTime) -> usize {
    if time.hour() >= 12 { 1 } else { 0 }
}

fn apply_hour(time: NaiveTime, hour_value: u32, hour12: bool) -> NaiveTime {
    let h = if hour12 {
        let pm = time.hour() >= 12;
        let base = hour_value % 12;
        if pm { base + 12 } else { base }
    } else {
        hour_value.min(23)
    };
    time.with_hour(h).unwrap_or(time)
}

fn apply_minute(time: NaiveTime, minute: u32) -> NaiveTime {
    time.with_minute(minute.min(59)).unwrap_or(time)
}

fn apply_second(time: NaiveTime, second: u32) -> NaiveTime {
    time.with_second(second.min(59)).unwrap_or(time)
}

fn apply_period(time: NaiveTime, pm: bool) -> NaiveTime {
    let h = time.hour() % 12;
    let new_h = if pm { h + 12 } else { h };
    time.with_hour(new_h).unwrap_or(time)
}

#[derive(IntoElement)]
pub struct AntTimePicker {
    state: Entity<TimeFieldState>,
    theme: AntTheme,
    size: ControlSize,
    disabled: bool,
    status: InputStatus,
    variant: InputVariant,
    show_icon: bool,
    /// Mirror of TimeFieldState precision when Second was configured (not readable).
    show_second: bool,
    /// Mirror of TimeFieldState hour cycle when H12 was configured (not readable).
    hour12: bool,
    open: Option<bool>,
    on_open_change: Option<OpenChange>,
}

impl AntTimePicker {
    pub fn new(state: &Entity<TimeFieldState>) -> Self {
        Self {
            state: state.clone(),
            theme: AntTheme::default(),
            size: ControlSize::Middle,
            disabled: false,
            status: InputStatus::Normal,
            variant: InputVariant::Outlined,
            show_icon: true,
            show_second: false,
            hour12: false,
            open: None,
            on_open_change: None,
        }
    }
    pub fn theme(mut self, t: AntTheme) -> Self {
        self.theme = t;
        self
    }
    pub fn size(mut self, s: ControlSize) -> Self {
        self.size = s;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn status(mut self, status: InputStatus) -> Self {
        self.status = status;
        self
    }
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn show_icon(mut self, icon: bool) -> Self {
        self.show_icon = icon;
        self
    }
    /// Show a seconds column. Pass true when the state used `TimePrecision::Second`.
    pub fn show_second(mut self, show: bool) -> Self {
        self.show_second = show;
        self
    }
    /// 12-hour columns + AM/PM. Pass true when the state used `HourCycle::H12`.
    pub fn hour12(mut self, hour12: bool) -> Self {
        self.hour12 = hour12;
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }
    pub fn on_open_change(
        mut self,
        f: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for AntTimePicker {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let size = self.size;
        let disabled = self.disabled;
        let status = self.status;
        let variant = self.variant;
        let show_icon = self.show_icon;
        let show_second = self.show_second;
        let hour12 = self.hour12;
        let open = self.open;
        let on_open = self.on_open_change;
        let state = self.state;

        // Register the hosting view as a TimeFieldState observer so column
        // selection re-renders after set_time.
        let _ = state.read(cx).time();

        let kit_size = match size {
            ControlSize::Small => Size::Small,
            ControlSize::Middle => Size::Medium,
            ControlSize::Large => Size::Large,
        };
        let chrome = time_picker_chrome(&theme, size, variant, status, disabled);
        let icon_color = if disabled {
            theme.disabled_text
        } else {
            theme.text_secondary
        };
        let frame_id = format!("ant-time-frame-{:?}", state.entity_id());
        let build_frame = {
            let state = state.clone();
            move |is_open: bool| -> gpui_kit::AnyElement {
                div()
                    .id(frame_id.clone())
                    .flex()
                    .items_center()
                    .h(px(chrome.height))
                    .rounded(px(chrome.radius))
                    .bg(chrome.bg)
                    .when(chrome.bordered, |this| {
                        this.border_1().border_color(if is_open {
                            chrome.hover_border
                        } else {
                            chrome.border
                        })
                    })
                    .when(!chrome.bordered, |this| this.border_0())
                    .when(chrome.bordered && !disabled && !is_open, |this| {
                        this.hover(|style| style.border_color(chrome.hover_border))
                    })
                    .when(show_icon, |this| this.pr(px(8.)))
                    .child(
                        TimeField::new(&state)
                            .with_size(kit_size)
                            .disabled(disabled)
                            .invalid(status == InputStatus::Error)
                            .border_0()
                            .bg(color(0).opacity(0.))
                            .text_color(chrome.text)
                            .h(px(chrome.height)),
                    )
                    .when(show_icon, |this| {
                        this.child(
                            AntIcon::new(AntIconName::ClockCircleOutlined)
                                .theme(theme)
                                .size(14.)
                                .color(icon_color),
                        )
                    })
                    .into_any_element()
            }
        };

        if disabled {
            return build_frame(false);
        }

        let popover_id = format!("ant-time-picker-{}", state.entity_id());
        let panel_state = state.clone();

        BasePopover::new(popover_id)
            .anchor(Anchor::TopLeft)
            .offset(px(4.))
            .overlay_closable(true)
            .when_some(open, |this, o| this.open(o))
            .when_some(on_open, |this, cb| {
                this.on_open_change(move |open, window, cx| cb(*open, window, cx))
            })
            .trigger_with(move |is_open, _, _| build_frame(is_open))
            .content(move |_, _, cx| {
                let time = panel_state.read(cx).time();
                let hour_sel = selected_hour_index(time, hour12);
                let minute_sel = selected_minute_index(time);
                let second_sel = selected_second_index(time);
                let period_sel = selected_period_index(time);

                let mut columns = div().flex().gap(px(4.)).p(px(8.));

                // Hours
                let hours: Vec<u32> = if hour12 {
                    (1..=12).collect()
                } else {
                    (0..24).collect()
                };
                columns = columns.child(time_column(
                    "Hour",
                    hours
                        .iter()
                        .enumerate()
                        .map(|(i, h)| {
                            let label: SharedString = format!("{h:02}").into();
                            (i, *h, label, i == hour_sel)
                        })
                        .collect(),
                    theme,
                    panel_state.clone(),
                    move |t, value| apply_hour(t, value, hour12),
                ));

                // Minutes
                columns = columns.child(time_column(
                    "Min",
                    (0..60u32)
                        .map(|m| {
                            let label: SharedString = format!("{m:02}").into();
                            (m as usize, m, label, m as usize == minute_sel)
                        })
                        .collect(),
                    theme,
                    panel_state.clone(),
                    apply_minute,
                ));

                if show_second {
                    columns = columns.child(time_column(
                        "Sec",
                        (0..60u32)
                            .map(|s| {
                                let label: SharedString = format!("{s:02}").into();
                                (s as usize, s, label, s as usize == second_sel)
                            })
                            .collect(),
                        theme,
                        panel_state.clone(),
                        apply_second,
                    ));
                }

                if hour12 {
                    let items = vec![
                        (0usize, 0u32, SharedString::from("AM"), period_sel == 0),
                        (1usize, 1u32, SharedString::from("PM"), period_sel == 1),
                    ];
                    columns = columns.child(time_column(
                        "",
                        items,
                        theme,
                        panel_state.clone(),
                        |t, value| apply_period(t, value == 1),
                    ));
                }

                div()
                    .bg(theme.surface)
                    .border_1()
                    .border_color(theme.border)
                    .rounded(px(theme.border_radius))
                    .shadow_lg()
                    .child(columns)
                    .into_any_element()
            })
            .into_any_element()
    }
}

fn time_column(
    heading: &'static str,
    items: Vec<(usize, u32, SharedString, bool)>,
    theme: AntTheme,
    state: Entity<TimeFieldState>,
    apply: impl Fn(NaiveTime, u32) -> NaiveTime + Clone + 'static,
) -> impl IntoElement {
    let mut col = div()
        .flex()
        .flex_col()
        .w(px(56.))
        .max_h(px(220.))
        .overflow_hidden();
    if !heading.is_empty() {
        col = col.child(
            div()
                .text_size(px(11.))
                .text_color(theme.text_secondary)
                .text_center()
                .mb(px(4.))
                .child(heading),
        );
    }
    let mut list = div()
        .id(format!("ant-time-column-{heading}"))
        .test_support()
        .min_h_0()
        .max_h(px(200.))
        .overflow_y_scroll()
        .flex().flex_col().gap(px(2.));
    for (idx, value, label, selected) in items {
        let apply = apply.clone();
        let st = state.clone();
        list = list.child(
            BaseButton::new(format!("ant-time-cell-{heading}-{idx}"))
                .w_full()
                .h(px(28.)).flex_shrink_0()
                .rounded(px(theme.border_radius_sm))
                .text_size(px(13.))
                .text_color(if selected {
                    color(0xffffff)
                } else {
                    theme.text
                })
                .bg(if selected {
                    theme.primary
                } else {
                    color(0).opacity(0.)
                })
                .when(!selected, |b| b.hover(|s| s.bg(theme.surface_secondary)))
                .on_click(move |_, window, cx| {
                    st.update(cx, |s, cx| {
                        let next = apply(s.time(), value);
                        s.set_time(next, window, cx);
                    });
                })
                .child(label),
        );
    }
    col.child(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrome_error_warning_disabled_filled_borderless_height() {
        let theme = AntTheme::light();
        let err = time_picker_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Outlined,
            InputStatus::Error,
            false,
        );
        assert_eq!(err.border, theme.error);
        assert_eq!(err.hover_border, theme.error);
        assert!(err.bordered);
        assert_eq!(err.height, 32.);

        let disabled = time_picker_chrome(
            &theme,
            ControlSize::Large,
            InputVariant::Outlined,
            InputStatus::Normal,
            true,
        );
        assert_eq!(disabled.bg, theme.disabled_bg);
        assert_eq!(disabled.text, theme.disabled_text);
        assert!(disabled.bordered);
        assert_eq!(disabled.height, 40.);

        let filled = time_picker_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Filled,
            InputStatus::Normal,
            false,
        );
        assert_eq!(filled.bg, theme.surface_secondary);
        assert!(!filled.bordered);

        let borderless = time_picker_chrome(
            &theme,
            ControlSize::Small,
            InputVariant::Borderless,
            InputStatus::Normal,
            false,
        );
        assert!(!borderless.bordered);
        assert_eq!(borderless.bg.a, 0.);
        assert_eq!(borderless.height, 24.);
    }

    #[test]
    fn column_indices_match_naive_time() {
        let t = NaiveTime::from_hms_opt(15, 30, 45).unwrap();
        assert_eq!(selected_hour_index(t, false), 15);
        assert_eq!(selected_hour_index(t, true), 2); // 3 PM → index 2 (1-based hour 3)
        assert_eq!(selected_minute_index(t), 30);
        assert_eq!(selected_second_index(t), 45);
        assert_eq!(selected_period_index(t), 1);

        let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        assert_eq!(selected_hour_index(midnight, false), 0);
        assert_eq!(selected_hour_index(midnight, true), 11); // 12 AM
        assert_eq!(selected_period_index(midnight), 0);

        let noon = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert_eq!(selected_hour_index(noon, true), 11); // 12 PM
        assert_eq!(selected_period_index(noon), 1);
    }

    #[test]
    fn apply_hour_minute_period_round_trip() {
        let t = NaiveTime::from_hms_opt(10, 5, 0).unwrap();
        assert_eq!(apply_hour(t, 15, false).hour(), 15);
        assert_eq!(apply_minute(t, 42).minute(), 42);
        assert_eq!(apply_period(t, true).hour(), 22);
        assert_eq!(apply_period(apply_period(t, true), false).hour(), 10);
        let h12 = apply_hour(t, 3, true); // keep AM → 3
        assert_eq!(h12.hour(), 3);
        let h12pm = apply_hour(apply_period(t, true), 3, true); // 3 PM
        assert_eq!(h12pm.hour(), 15);
    }
}
