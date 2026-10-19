//! Ant Design Calendar — base GPUI Kit calendar behavior with Ant tokens.
//!
//! Do not use `gpui_kit::component` Calendar. Base `Calendar` owns month/year
//! navigation, focus, keyboard day selection, single/range dates and disabled
//! matching via `CalendarState` (the same type component re-exports, so gallery
//! `Entity<CalendarState>` stays valid). Cell chrome uses the verified
//! `.item(|CalendarItem, CalendarItemState, …|)` renderer.
//!
//! Cell tones (see `calendar_cell_tone`):
//! - selected → primary fill + white text
//! - today (not selected) → primary border
//! - outside-month → `text_secondary`
//! - disabled → `disabled_text`
//!
//! Header arrows are Previous/Next item kinds styled in the same slot.
//! No separate antd header chrome is layered on top.

use gpui_kit::{
    App, Entity, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _,
    Window, px,
};
use gpui_kit::base::{
    Calendar as BaseCalendar, CalendarItemKind, CalendarItemState, CalendarState,
};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntTheme, ControlSize, color};
use crate::design_tokens::AntComponent;

/// Clamp a wide calendar view to a supported number of adjacent months.
pub fn calendar_month_count(months:usize)->usize{months.clamp(1,3)}

/// English month labels for the header / month grid (Kit has no antd i18n hook).
pub fn calendar_month_name(month:i32)->SharedString{
    match month {
        1 => "Jan".into(),
        2 => "Feb".into(),
        3 => "Mar".into(),
        4 => "Apr".into(),
        5 => "May".into(),
        6 => "Jun".into(),
        7 => "Jul".into(),
        8 => "Aug".into(),
        9 => "Sep".into(),
        10 => "Oct".into(),
        11 => "Nov".into(),
        12 => "Dec".into(),
        _ => "".into(),
    }
}

/// Weekday short labels starting Sunday = 0 (matches Kit's default week origin).
pub fn calendar_weekday_name(day:i32)->SharedString{
    match day {
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

/// Cell size in px for the given control size (antd-ish day hit targets).
pub fn calendar_cell_size(size:ControlSize)->f32{
    match size {
        ControlSize::Small => 24.,
        ControlSize::Middle => 28.,
        ControlSize::Large => 32.,
    }
}

/// Visual role of a calendar cell for Ant Design painting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarCellTone {
    /// Selected day / month / year (primary fill).
    Selected,
    /// Today marker when not also selected.
    Today,
    /// Day inside a range selection but not an endpoint.
    InRange,
    /// Outside-month day (muted).
    OutsideMonth,
    /// Disabled by matcher.
    Disabled,
    /// Default in-month day or chrome (weekday, nav).
    Default,
}

/// Resolve Ant cell tone from base `CalendarItemState` flags.
///
/// Priority: disabled > selected > today > in-range > outside-month > default.
pub fn calendar_cell_tone(state:&CalendarItemState)->CalendarCellTone{
    if state.is_disabled() {
        CalendarCellTone::Disabled
    } else if state.is_active() {
        CalendarCellTone::Selected
    } else if state.is_today() {
        CalendarCellTone::Today
    } else if state.is_in_range() {
        CalendarCellTone::InRange
    } else if state.is_muted() {
        CalendarCellTone::OutsideMonth
    } else {
        CalendarCellTone::Default
    }
}

/// A real calendar control, not the DatePicker's popup or an HTML table.
#[derive(IntoElement)]
pub struct AntCalendar {
    state: Entity<CalendarState>,
    theme: AntTheme,
    size: ControlSize,
    months: usize,
    bordered: bool,
}
impl AntCalendar {
    pub fn new(state: &Entity<CalendarState>) -> Self {
        Self { state: state.clone(), theme: AntTheme::default(),
            size: ControlSize::Middle, months: 1, bordered: true }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme=theme; self }
    pub fn size(mut self, size: ControlSize) -> Self { self.size=size; self }
    /// Show 1–3 adjacent months. Larger counts are deliberately capped for
    /// the desktop/WASM gallery; app-specific wide layouts can stack views.
    pub fn months(mut self, months: usize) -> Self {
        self.months=calendar_month_count(months); self
    }
    pub fn bordered(mut self, bordered: bool) -> Self { self.bordered=bordered; self }
}
impl RenderOnce for AntCalendar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t=self.theme;
        let token=t.component(AntComponent::Calendar);
        let months=self.months;
        let cell=calendar_cell_size(self.size);
        let radius=px(token.border_radius.unwrap_or(t.border_radius));
        let panel_pad=token.padding.unwrap_or(12.);
        let bg=token.color_bg.unwrap_or(t.surface);
        let fg=token.color_text.unwrap_or(t.text);
        let border=if self.bordered {
            token.color_border.unwrap_or(t.border)
        } else {
            color(0x000000).opacity(0.)
        };
        let primary=token.color_primary.unwrap_or(t.primary);
        let selected_bg=token.color_bg_selected.unwrap_or(primary);
        let hover_bg=token.color_bg_hover.unwrap_or(t.primary_bg);
        let muted=t.text_secondary;
        let disabled_text=t.disabled_text;
        let white=color(0xffffff);
        let width=match self.size {
            ControlSize::Small => 220. * months as f32,
            ControlSize::Middle => 248. * months as f32,
            ControlSize::Large => 304. * months as f32,
        };

        BaseCalendar::new(("ant-calendar", self.state.entity_id()), &self.state)
            .number_of_months(months)
            .label(|kind, value| match kind {
                CalendarItemKind::Previous => "‹".into(),
                CalendarItemKind::Next => "›".into(),
                CalendarItemKind::MonthToggle | CalendarItemKind::Month => {
                    calendar_month_name(value)
                }
                CalendarItemKind::Weekday => calendar_weekday_name(value),
                _ => value.to_string().into(),
            })
            .item(move |item, state: CalendarItemState, _, _| {
                let kind=state.kind();
                let tone=calendar_cell_tone(&state);
                item
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(
                        matches!(
                            kind,
                            CalendarItemKind::Day
                                | CalendarItemKind::Previous
                                | CalendarItemKind::Next
                                | CalendarItemKind::Year
                        ),
                        |this| this.size(px(cell)).rounded(radius),
                    )
                    .when(
                        matches!(
                            kind,
                            CalendarItemKind::Month
                                | CalendarItemKind::MonthToggle
                                | CalendarItemKind::YearToggle
                        ),
                        |this| {
                            this.h(px(cell))
                                .px(px(8.))
                                .rounded(radius)
                                .when(kind == CalendarItemKind::Month, |this| {
                                    this.w_full().my(px(2.))
                                })
                        },
                    )
                    .when(kind == CalendarItemKind::Weekday, |this| {
                        this.size(px(cell))
                            .text_size(px(12.))
                            .text_color(muted)
                    })
                    .when(kind != CalendarItemKind::Weekday, |this| {
                        this.text_size(px(if cell <= 24. { 12. } else { 14. }))
                    })
                    .map(|this| match tone {
                        CalendarCellTone::Disabled => {
                            this.text_color(disabled_text).opacity(0.55)
                        }
                        CalendarCellTone::Selected => {
                            this.bg(selected_bg).text_color(white)
                        }
                        CalendarCellTone::Today => {
                            this.border_1().border_color(primary).text_color(primary)
                        }
                        CalendarCellTone::InRange => {
                            this.bg(hover_bg).text_color(primary)
                        }
                        CalendarCellTone::OutsideMonth => {
                            this.text_color(muted)
                        }
                        CalendarCellTone::Default => this,
                    })
                    .when(
                        kind != CalendarItemKind::Weekday
                            && !matches!(
                                tone,
                                CalendarCellTone::Disabled | CalendarCellTone::Selected
                            ),
                        |this| this.hover(|this| this.bg(hover_bg).text_color(fg)),
                    )
                    .into_any_element()
            })
            .w(px(width))
            .bg(bg)
            .text_color(fg)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .p(px(panel_pad))
            .gap(px(2.))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_months_are_bounded(){
        assert_eq!(calendar_month_count(0),1);
        assert_eq!(calendar_month_count(1),1);
        assert_eq!(calendar_month_count(2),2);
        assert_eq!(calendar_month_count(9),3);
    }
    #[test]
    fn month_and_weekday_labels_are_stable(){
        assert_eq!(calendar_month_name(1).as_ref(),"Jan");
        assert_eq!(calendar_month_name(12).as_ref(),"Dec");
        assert_eq!(calendar_month_name(0).as_ref(),"");
        assert_eq!(calendar_weekday_name(0).as_ref(),"Su");
        assert_eq!(calendar_weekday_name(6).as_ref(),"Sa");
    }
    #[test]
    fn cell_size_tracks_control_size(){
        assert_eq!(calendar_cell_size(ControlSize::Small),24.);
        assert_eq!(calendar_cell_size(ControlSize::Middle),28.);
        assert_eq!(calendar_cell_size(ControlSize::Large),32.);
    }
    #[test]
    fn cell_tone_priority_matches_antd_rules(){
        let day=CalendarItemKind::Day;
        assert_eq!(
            calendar_cell_tone(&CalendarItemState::new(day).disabled(true).active(true)),
            CalendarCellTone::Disabled
        );
        assert_eq!(
            calendar_cell_tone(&CalendarItemState::new(day).active(true).today(true)),
            CalendarCellTone::Selected
        );
        assert_eq!(
            calendar_cell_tone(&CalendarItemState::new(day).today(true).muted(true)),
            CalendarCellTone::Today
        );
        assert_eq!(
            calendar_cell_tone(&CalendarItemState::new(day).in_range(true)),
            CalendarCellTone::InRange
        );
        assert_eq!(
            calendar_cell_tone(&CalendarItemState::new(day).muted(true)),
            CalendarCellTone::OutsideMonth
        );
        assert_eq!(
            calendar_cell_tone(&CalendarItemState::new(day)),
            CalendarCellTone::Default
        );
    }
}
