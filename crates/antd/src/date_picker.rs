//! Ant Design DatePicker on GPUI Kit **base** popover + DatePickerState model.
//!
//! - Model: `Entity<DatePickerState>` only (`date` / `set_date` / `date_time`).
//! - Trigger chrome is ours (height, variant, status, hover, tokens).
//! - Popup is `gpui_kit::base::Popover` (placement, Escape, outside close).
//! - Month grid is drawn here; selected value always comes from DatePickerState.
//! - Month browse cursor is either parent-controlled (`.visible_month` +
//!   `.on_visible_month_change`) or element-local via `use_keyed_state`
//!   (view state, not a second date model). Never a process static.
//!
//! ## Gaps
//! - `DatePickerState::disabled_matcher` is not publicly readable — disabled
//!   days cannot be skipped from this wrapper.
//! - `DateRangePreset` fields are private — presets builder is kept for API
//!   compatibility but presets are not rendered.
//! - Popup datetime segment editing still lives on Kit when the state was
//!   built with `time_precision`; this panel is date-first.
//! - Kit calendar cell tokens beyond primary fill/today border are not used.

use std::rc::Rc;

use chrono::{Datelike, Local, NaiveDate};
use gpui_kit::base::{Button as BaseButton, Popover as BasePopover, StyledExt as _};
use gpui_kit::component::calendar::Date;
use gpui_kit::component::date_picker::{DatePickerState, DateRangePreset};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    Anchor, App, Entity, Hsla, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement as _, RenderOnce, SharedString, StatefulInteractiveElement as _,
    Styled as _, Window, div, px,
};
use crate::design_tokens::AntComponent;
use crate::{
    AntIcon, AntIconName, AntTheme, ControlSize, InputStatus, InputVariant, color,
};

type OpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type VisibleMonthChange = Rc<dyn Fn(NaiveDate, &mut Window, &mut App)>;

/// Pure field chrome for the date trigger.
#[derive(Debug, Clone, Copy, PartialEq)]
struct DatePickerChrome {
    bg: Hsla,
    border: Hsla,
    text: Hsla,
    hover_border: Hsla,
    radius: f32,
    height: f32,
    bordered: bool,
}

fn date_picker_chrome(
    theme: &AntTheme,
    size: ControlSize,
    variant: InputVariant,
    status: InputStatus,
    disabled: bool,
    borderless: bool,
) -> DatePickerChrome {
    let tokens = theme.component(AntComponent::DatePicker);
    let effective = if borderless {
        InputVariant::Borderless
    } else {
        variant
    };
    let bordered = effective.bordered();
    let bg = if disabled {
        theme.disabled_bg
    } else if !effective.framed() {
        color(0).opacity(0.)
    } else if effective == InputVariant::Filled {
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
    DatePickerChrome {
        bg,
        border,
        text,
        hover_border,
        radius: tokens.border_radius.unwrap_or(theme.border_radius),
        height: tokens.control_height.unwrap_or(size.height()),
        bordered,
    }
}

/// Element-local month browse cursor (view state only).
struct MonthCursor {
    year: i32,
    month: u32,
}

impl MonthCursor {
    fn from_date(date: NaiveDate) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
        }
    }

    fn first_day(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year, self.month, 1)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
    }

    fn step(&mut self, delta_months: i32) {
        let mut m = self.month as i32 + delta_months;
        let mut y = self.year;
        while m < 1 {
            m += 12;
            y -= 1;
        }
        while m > 12 {
            m -= 12;
            y += 1;
        }
        self.year = y;
        self.month = m as u32;
    }
}

/// Days of `year-month` laid out Sun→Sat; `None` = padding outside the month.
fn month_day_cells(year: i32, month: u32) -> Vec<Option<u32>> {
    let first = match NaiveDate::from_ymd_opt(year, month, 1) {
        Some(d) => d,
        None => return Vec::new(),
    };
    let pad = first.weekday().num_days_from_sunday() as usize;
    let days = days_in_month(year, month);
    let mut cells = Vec::with_capacity(42);
    cells.extend(std::iter::repeat(None).take(pad));
    for day in 1..=days {
        cells.push(Some(day));
    }
    while cells.len() % 7 != 0 {
        cells.push(None);
    }
    cells
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let (ny, nm) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    match (
        NaiveDate::from_ymd_opt(year, month, 1),
        NaiveDate::from_ymd_opt(ny, nm, 1),
    ) {
        (Some(a), Some(b)) => (b - a).num_days() as u32,
        _ => 30,
    }
}

fn shift_month(date: NaiveDate, delta: i32) -> NaiveDate {
    let mut c = MonthCursor::from_date(date);
    c.step(delta);
    c.first_day()
}

fn month_title(year: i32, month: u32) -> SharedString {
    let name = match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    };
    format!("{name} {year}").into()
}

fn weekday_short(i: u32) -> SharedString {
    match i {
        0 => "Su".into(),
        1 => "Mo".into(),
        2 => "Tu".into(),
        3 => "We".into(),
        4 => "Th".into(),
        5 => "Fr".into(),
        6 => "Sa".into(),
        _ => "".into(),
    }
}

fn default_visible_from_selection(date: Date) -> NaiveDate {
    date.start()
        .or_else(|| date.end())
        .unwrap_or_else(|| Local::now().date_naive())
}

fn build_trigger(
    is_open: bool,
    theme: AntTheme,
    chrome: DatePickerChrome,
    display: SharedString,
    has_value: bool,
    disabled: bool,
    clearable: bool,
    state: Entity<DatePickerState>,
) -> gpui_kit::AnyElement {
    let clear_state = state.clone();
    BaseButton::new("ant-date-trigger")
        .disabled(disabled)
        .selected(is_open)
        .tab_stop(!disabled)
        .w_full()
        .h(px(chrome.height))
        .px(px(11.))
        .flex()
        .items_center()
        .gap(px(8.))
        .rounded(px(chrome.radius))
        .bg(chrome.bg)
        .text_color(if has_value {
            chrome.text
        } else {
            theme.text_secondary
        })
        .when(chrome.bordered, |b| b.border_1().border_color(chrome.border))
        .when(!chrome.bordered, |b| b.border_0())
        .when(chrome.bordered && !disabled, |b| {
            b.hover(|s| s.border_color(chrome.hover_border))
        })
        .child(
            div()
                .flex_1()
                .min_w_0()
                .overflow_hidden()
                .text_size(px(14.))
                .child(display),
        )
        .when(clearable && has_value && !disabled, |b| {
            b.child(
                div()
                    .id("ant-date-clear")
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        cx.stop_propagation();
                        clear_state.update(cx, |s, cx| {
                            let empty = if s.date().is_single() {
                                Date::Single(None)
                            } else {
                                Date::Range(None, None)
                            };
                            s.set_date(empty, window, cx);
                        });
                    })
                    .child(
                        AntIcon::new(AntIconName::CloseCircleFilled)
                            .theme(theme)
                            .size(12.)
                            .color(theme.text_secondary),
                    ),
            )
        })
        .child(
            AntIcon::new(AntIconName::CalendarOutlined)
                .theme(theme)
                .size(14.)
                .color(if disabled {
                    theme.disabled_text
                } else {
                    theme.text_secondary
                }),
        )
        .into_any_element()
}

/// Native Ant Design-like date picker. Gallery keeps `Entity<DatePickerState>`.
#[derive(IntoElement)]
pub struct AntDatePicker {
    state: Entity<DatePickerState>,
    theme: AntTheme,
    size: ControlSize,
    disabled: bool,
    clearable: bool,
    borderless: bool,
    variant: InputVariant,
    status: InputStatus,
    number_of_months: usize,
    placeholder: Option<SharedString>,
    /// Kept for API compatibility. Kit `DateRangePreset` fields are private, so
    /// presets cannot be rendered from outside gpui-component (see module gaps).
    #[allow(dead_code)]
    presets: Option<Vec<DateRangePreset>>,
    date_format: SharedString,
    visible_month: Option<NaiveDate>,
    open: Option<bool>,
    on_open_change: Option<OpenChange>,
    on_visible_month_change: Option<VisibleMonthChange>,
}

impl AntDatePicker {
    pub fn new(state: &Entity<DatePickerState>) -> Self {
        Self {
            state: state.clone(),
            theme: AntTheme::default(),
            size: ControlSize::Middle,
            disabled: false,
            clearable: false,
            borderless: false,
            variant: InputVariant::Outlined,
            status: InputStatus::Normal,
            number_of_months: 1,
            placeholder: None,
            presets: None,
            date_format: "%Y/%m/%d".into(),
            visible_month: None,
            open: None,
            on_open_change: None,
            on_visible_month_change: None,
        }
    }

    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn clearable(mut self, enabled: bool) -> Self {
        self.clearable = enabled;
        self
    }
    pub fn borderless(mut self, enabled: bool) -> Self {
        self.borderless = enabled;
        self
    }
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn status(mut self, status: InputStatus) -> Self {
        self.status = status;
        self
    }
    pub fn number_of_months(mut self, months: usize) -> Self {
        self.number_of_months = months.max(1).min(3);
        self
    }
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    pub fn presets(mut self, presets: Vec<DateRangePreset>) -> Self {
        self.presets = Some(presets);
        self
    }
    pub fn date_format(mut self, format: impl Into<SharedString>) -> Self {
        self.date_format = format.into();
        self
    }
    /// Parent-owned browse month (any day in the month). Without this, the
    /// wrapper keeps a local month cursor in element state.
    pub fn visible_month(mut self, month: NaiveDate) -> Self {
        self.visible_month = Some(month);
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
    /// Fired when the user presses prev/next. Required for controlled
    /// `.visible_month`; optional when the local element cursor is used.
    pub fn on_visible_month_change(
        mut self,
        f: impl Fn(NaiveDate, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_visible_month_change = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for AntDatePicker {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let size = self.size;
        let disabled = self.disabled;
        let clearable = self.clearable;
        let borderless = self.borderless;
        let variant = self.variant;
        let status = self.status;
        let months = self.number_of_months;
        let placeholder = self
            .placeholder
            .unwrap_or_else(|| SharedString::from("Select date"));
        let date_format = self.date_format;
        let controlled_month = self.visible_month;
        let open = self.open;
        let on_open = self.on_open_change;
        let on_month = self.on_visible_month_change;
        let state = self.state;

        let chrome = date_picker_chrome(&theme, size, variant, status, disabled, borderless);
        let selection = state.read(cx).date();
        let display = selection
            .format(date_format.as_ref())
            .unwrap_or_else(|| placeholder.clone());
        let has_value = selection.is_some();
        let today = Local::now().date_naive();

        let seed = controlled_month.unwrap_or_else(|| default_visible_from_selection(selection));
        let cursor = window.use_keyed_state(
            ("ant-date-month", state.entity_id()),
            cx,
            |_, _| MonthCursor::from_date(seed),
        );
        let base_month = if let Some(m) = controlled_month {
            MonthCursor::from_date(m)
        } else {
            let c = cursor.read(cx);
            MonthCursor {
                year: c.year,
                month: c.month,
            }
        };

        let popover_id = format!("ant-date-picker-{}", state.entity_id());
        let cell = 28.0_f32;

        if disabled {
            return build_trigger(
                false, theme, chrome, display, has_value, true, false, state,
            );
        }

        let trigger_state = state.clone();
        BasePopover::new(popover_id)
            .anchor(Anchor::BottomLeft)
            .offset(px(4.))
            .overlay_closable(true)
            .when_some(open, |this, o| this.open(o))
            .when_some(on_open, |this, cb| {
                this.on_open_change(move |open, window, cx| cb(*open, window, cx))
            })
            .trigger_with(move |is_open, _, _| {
                build_trigger(
                    is_open,
                    theme,
                    chrome,
                    display.clone(),
                    has_value,
                    false,
                    clearable,
                    trigger_state.clone(),
                )
            })
            .content(move |_popover_state, _window, cx| {
                // Entity handle so day clicks can dismiss after a complete single pick.
                let popover_entity = cx.entity().clone();
                let state = state.clone();
                let cursor = cursor.clone();
                let on_month = on_month.clone();
                let controlled = controlled_month;
                let selection = state.read(cx).date();

                div()
                    .flex()
                    .flex_col()
                    .p(px(12.))
                    .bg(theme.surface)
                    .border_1()
                    .border_color(theme.border)
                    .rounded(px(theme.border_radius))
                    .shadow_lg()
                    .child({
                        let title = month_title(base_month.year, base_month.month);
                        let cursor_prev = cursor.clone();
                        let on_month_prev = on_month.clone();
                        let cursor_next = cursor.clone();
                        let on_month_next = on_month.clone();
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .mb(px(8.))
                            .child(nav_btn("ant-date-prev", "‹", theme, move |window, cx| {
                                let from = if let Some(m) = controlled {
                                    m
                                } else {
                                    cursor_prev.read(cx).first_day()
                                };
                                let next = shift_month(from, -1);
                                if let Some(cb) = &on_month_prev {
                                    cb(next, window, cx);
                                } else if controlled.is_none() {
                                    cursor_prev.update(cx, |c, cx| {
                                        c.step(-1);
                                        cx.notify();
                                    });
                                }
                            }))
                            .child(
                                div()
                                    .flex_1()
                                    .text_center()
                                    .text_color(theme.text)
                                    .text_size(px(14.))
                                    .child(title),
                            )
                            .child(nav_btn("ant-date-next", "›", theme, move |window, cx| {
                                let from = if let Some(m) = controlled {
                                    m
                                } else {
                                    cursor_next.read(cx).first_day()
                                };
                                let next = shift_month(from, 1);
                                if let Some(cb) = &on_month_next {
                                    cb(next, window, cx);
                                } else if controlled.is_none() {
                                    cursor_next.update(cx, |c, cx| {
                                        c.step(1);
                                        cx.notify();
                                    });
                                }
                            }))
                    })
                    .child({
                        let mut row = div().flex().gap(px(16.));
                        for offset in 0..months {
                            let mut mc = MonthCursor {
                                year: base_month.year,
                                month: base_month.month,
                            };
                            mc.step(offset as i32);
                            row = row.child(month_grid(
                                mc.year,
                                mc.month,
                                cell,
                                theme,
                                today,
                                selection,
                                state.clone(),
                                popover_entity.clone(),
                            ));
                        }
                        row
                    })
                    .into_any_element()
            })
            .w_full()
            .into_any_element()
    }
}

fn nav_btn(
    id: impl Into<gpui_kit::ElementId>,
    label: &'static str,
    theme: AntTheme,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    BaseButton::new(id)
        .w(px(28.))
        .h(px(28.))
        .rounded(px(theme.border_radius))
        .text_color(theme.text)
        .hover(|s| s.bg(theme.surface_secondary))
        .on_click(move |_, window, cx| on_click(window, cx))
        .child(label)
}

fn month_grid(
    year: i32,
    month: u32,
    cell: f32,
    theme: AntTheme,
    today: NaiveDate,
    selection: Date,
    state: Entity<DatePickerState>,
    popover: Entity<gpui_kit::base::PopoverState>,
) -> impl IntoElement {
    let cells = month_day_cells(year, month);
    div()
        .flex()
        .flex_col()
        .gap(px(4.))
        .child(
            div()
                .text_size(px(12.))
                .text_color(theme.text_secondary)
                .mb(px(2.))
                .child(month_title(year, month)),
        )
        .child({
            let mut head = div().flex();
            for i in 0..7u32 {
                head = head.child(
                    div()
                        .w(px(cell))
                        .h(px(cell * 0.7))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.))
                        .text_color(theme.text_secondary)
                        .child(weekday_short(i)),
                );
            }
            head
        })
        .child({
            let mut body = div().flex().flex_col();
            for week in cells.chunks(7) {
                let mut row = div().flex();
                for day in week {
                    row = row.child(day_cell(
                        year,
                        month,
                        *day,
                        cell,
                        theme,
                        today,
                        selection,
                        state.clone(),
                        popover.clone(),
                    ));
                }
                body = body.child(row);
            }
            body
        })
}

fn day_cell(
    year: i32,
    month: u32,
    day: Option<u32>,
    cell: f32,
    theme: AntTheme,
    today: NaiveDate,
    selection: Date,
    state: Entity<DatePickerState>,
    popover: Entity<gpui_kit::base::PopoverState>,
) -> gpui_kit::AnyElement {
    let Some(day) = day else {
        return div().w(px(cell)).h(px(cell)).into_any_element();
    };
    let Some(date) = NaiveDate::from_ymd_opt(year, month, day) else {
        return div().w(px(cell)).h(px(cell)).into_any_element();
    };
    let is_selected = selection.is_active(&date);
    let in_range = selection.is_in_range(&date);
    let is_today = date == today;
    let label: SharedString = day.to_string().into();

    let bg = if is_selected {
        theme.primary
    } else if in_range {
        theme.primary_bg
    } else {
        color(0).opacity(0.)
    };
    let fg = if is_selected {
        color(0xffffff)
    } else {
        theme.text
    };

    BaseButton::new(format!("ant-day-{year}-{month}-{day}"))
        .w(px(cell))
        .h(px(cell))
        .rounded(px(theme.border_radius_sm))
        .bg(bg)
        .text_color(fg)
        .text_size(px(13.))
        .when(is_today && !is_selected, |b| {
            b.border_1().border_color(theme.primary)
        })
        .when(!is_selected, |b| b.hover(|s| s.bg(theme.surface_secondary)))
        .on_click(move |_, window, cx| {
            let mut should_close = false;
            state.update(cx, |s, cx| {
                let current = s.date();
                let next = match current {
                    Date::Single(_) => {
                        should_close = true;
                        Date::Single(Some(date))
                    }
                    Date::Range(start, end) => {
                        if start.is_none() || (start.is_some() && end.is_some()) {
                            Date::Range(Some(date), None)
                        } else if let Some(a) = start {
                            should_close = true;
                            if date < a {
                                Date::Range(Some(date), Some(a))
                            } else {
                                Date::Range(Some(a), Some(date))
                            }
                        } else {
                            Date::Range(Some(date), None)
                        }
                    }
                };
                s.set_date(next, window, cx);
            });
            if should_close {
                popover.update(cx, |p, cx| p.dismiss(window, cx));
            }
        })
        .child(label)
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrome_error_warning_disabled_filled_borderless_height() {
        let theme = AntTheme::light();
        let err = date_picker_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Outlined,
            InputStatus::Error,
            false,
            false,
        );
        assert_eq!(err.border, theme.error);
        assert!(err.bordered);
        assert_eq!(err.height, 32.);

        let disabled = date_picker_chrome(
            &theme,
            ControlSize::Large,
            InputVariant::Outlined,
            InputStatus::Normal,
            true,
            false,
        );
        assert_eq!(disabled.bg, theme.disabled_bg);
        assert_eq!(disabled.text, theme.disabled_text);
        assert!(disabled.bordered);
        assert_eq!(disabled.height, 40.);

        let filled = date_picker_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Filled,
            InputStatus::Normal,
            false,
            false,
        );
        assert_eq!(filled.bg, theme.surface_secondary);
        assert!(!filled.bordered);

        let borderless = date_picker_chrome(
            &theme,
            ControlSize::Small,
            InputVariant::Outlined,
            InputStatus::Normal,
            false,
            true,
        );
        assert!(!borderless.bordered);
        assert_eq!(borderless.bg.a, 0.);
        assert_eq!(borderless.height, 24.);
    }

    #[test]
    fn month_day_cells_pad_to_full_weeks() {
        let cells = month_day_cells(2024, 1);
        assert!(cells.len() >= 28 && cells.len() % 7 == 0);
        assert_eq!(cells.iter().filter(|c| c.is_some()).count(), 31);
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2023, 2), 28);
    }

    #[test]
    fn month_cursor_steps_across_year() {
        let mut c = MonthCursor {
            year: 2024,
            month: 1,
        };
        c.step(-1);
        assert_eq!((c.year, c.month), (2023, 12));
        c.step(2);
        assert_eq!((c.year, c.month), (2024, 2));
    }
}
