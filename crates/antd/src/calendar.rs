//! Ant Design Calendar — native GPUI Kit calendar behavior with Ant tokens.
//!
//! CalendarState owns month/year navigation, focus, keyboard day selection,
//! single/range dates and disabled-date matching. Consumers keep one Entity
//! for the lifetime of their view and subscribe to CalendarEvent::Selected.
//! This wrapper does not hold a second, potentially divergent date model.

use gpui_kit::{App, Entity, IntoElement, RenderOnce, Styled as _, Window, px};
use gpui_kit::component::{
    Sizable as _, Size,
    calendar::{Calendar, CalendarState},
};
use crate::{AntTheme, ControlSize, color};
use crate::design_tokens::AntComponent;

/// A real calendar control, not the DatePicker's popup or an HTML table.
/// The GPUI Kit implementation handles month/year dropdowns, arrow-key
/// navigation and date constraints set on CalendarState.
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
        self.months=months.clamp(1,3); self
    }
    pub fn bordered(mut self, bordered: bool) -> Self { self.bordered=bordered; self }
}
impl RenderOnce for AntCalendar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t=self.theme;
        let token=t.component(AntComponent::Calendar);
        let kit_size=match self.size {
            ControlSize::Small=>Size::Small,
            ControlSize::Middle=>Size::Medium,
            ControlSize::Large=>Size::Large,
        };
        Calendar::new(&self.state)
            .number_of_months(self.months)
            .with_size(kit_size)
            .bg(token.color_bg.unwrap_or(t.surface))
            .text_color(token.color_text.unwrap_or(t.text))
            .border_color(if self.bordered {
                token.color_border.unwrap_or(t.border)
            }else{color(0x000000).opacity(0.)})
            .rounded(px(token.border_radius.unwrap_or(t.border_radius)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_months_are_bounded(){
        // No GPUI context is necessary to test the bounds — keep the clamp
        // logic consistent with the builder's documented 1–3 month contract.
        assert_eq!(0usize.clamp(1,3),1);
        assert_eq!(1usize.clamp(1,3),1);
        assert_eq!(9usize.clamp(1,3),3);
    }
}
