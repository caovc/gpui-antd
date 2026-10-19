//! Ant Design date field backed by Kit's complete date/range/datetime picker.
//!
//! DatePickerState owns selection, disabled matchers, month navigation, time
//! editing and keyboard behavior. These cannot be read through Kit 0.7.3's
//! public API, so retain its calendar and preset panel rather than bypassing
//! their validation. The outer field uses Ant tokens; popup chrome remains Kit's.

use crate::design_tokens::AntComponent;
use crate::{AntTheme, ControlSize, InputStatus, InputVariant, color};
use gpui_kit::component::{
    Disableable as _, Sizable as _, Size,
    date_picker::{DatePicker, DatePickerState, DateRangePreset},
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, Entity, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

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

/// DatePickerState owns the date, calendar, focus and datetime editor.
/// Subscribe to Kit DatePickerEvent::Change to observe selections. Configure
/// date formatting, disabled matchers and time precision on that state.
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
    presets: Option<Vec<DateRangePreset>>,
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
        self.number_of_months = months.max(1);
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
}
impl RenderOnce for AntDatePicker {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let chrome = date_picker_chrome(
            &self.theme,
            self.size,
            self.variant,
            self.status,
            self.disabled,
            self.borderless,
        );
        let size = match self.size {
            ControlSize::Small => Size::Small,
            ControlSize::Middle => Size::Medium,
            ControlSize::Large => Size::Large,
        };
        div()
            .id(("ant-date-frame", self.state.entity_id()))
            .w_full()
            .h(px(chrome.height))
            .rounded(px(chrome.radius))
            .bg(chrome.bg)
            .text_color(chrome.text)
            .when(chrome.bordered, |this| {
                this.border_1().border_color(chrome.border)
            })
            .when(chrome.bordered && !self.disabled, |this| {
                this.hover(|this| this.border_color(chrome.hover_border))
            })
            .child(
                DatePicker::new(&self.state)
                    .with_size(size)
                    .disabled(self.disabled)
                    .cleanable(self.clearable)
                    .appearance(false)
                    .number_of_months(self.number_of_months)
                    .when_some(self.placeholder, |picker, text| picker.placeholder(text))
                    .when_some(self.presets, |picker, presets| picker.presets(presets)),
            )
    }
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
}
