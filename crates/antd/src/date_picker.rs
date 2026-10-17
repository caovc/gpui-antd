//! Ant Design-like DatePicker backed by GPUI Kit's native calendar.
//! The same Rust date, range and datetime controls work on desktop and WASM.

use gpui_kit::{App, Entity, IntoElement, RenderOnce, SharedString, Styled as _, Window, px};
use gpui_kit::component::{
    Disableable as _, Sizable as _, Size,
    date_picker::{DatePicker, DatePickerState, DateRangePreset},
};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntTheme, ControlSize};

/// Native date picker: DatePickerState owns selection, calendar and keyboard.
///
/// For single date: `DatePickerState::new(window, cx)`.
/// For range: `DatePickerState::range(window, cx)`.
/// For datetime: `DatePickerState::new(window, cx).time_precision(TimePrecision::Minute)`.
/// Subscribe to `DatePickerEvent::Change` to observe selected values.
#[derive(IntoElement)]
pub struct AntDatePicker {
    state: Entity<DatePickerState>,
    theme: AntTheme,
    size: ControlSize,
    disabled: bool,
    clearable: bool,
    borderless: bool,
    number_of_months: usize,
    placeholder: Option<SharedString>,
    presets: Option<Vec<DateRangePreset>>,
}

impl AntDatePicker {
    pub fn new(state: &Entity<DatePickerState>) -> Self {
        Self {
            state: state.clone(), theme: AntTheme::default(),
            size: ControlSize::Middle, disabled: false, clearable: false,
            borderless: false, number_of_months: 1, placeholder: None, presets: None,
        }
    }

    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn size(mut self, size: ControlSize) -> Self { self.size = size; self }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
    pub fn clearable(mut self, enabled: bool) -> Self { self.clearable = enabled; self }
    pub fn borderless(mut self, enabled: bool) -> Self { self.borderless = enabled; self }
    pub fn number_of_months(mut self, months: usize) -> Self {
        self.number_of_months = months.max(1); self
    }
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = Some(text.into()); self
    }
    pub fn presets(mut self, presets: Vec<DateRangePreset>) -> Self {
        self.presets = Some(presets); self
    }
}

impl RenderOnce for AntDatePicker {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let size = match self.size {
            ControlSize::Small => Size::Small,
            ControlSize::Middle => Size::Medium,
            ControlSize::Large => Size::Large,
        };
        DatePicker::new(&self.state)
            .with_size(size)
            .disabled(self.disabled)
            .cleanable(self.clearable)
            .appearance(!self.borderless)
            .number_of_months(self.number_of_months)
            .when_some(self.placeholder, |picker, text| picker.placeholder(text))
            .when_some(self.presets, |picker, presets| picker.presets(presets))
            .rounded(px(self.theme.border_radius))
            .border_color(self.theme.border)
            .bg(if self.disabled { self.theme.disabled_bg } else { self.theme.surface })
    }
}
