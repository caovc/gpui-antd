//! Ant Design-style text input built around GPUI Kit's production text engine.
//! Cursor, selection, undo, clipboard and IME remain owned by InputState.

use crate::design_tokens::AntComponent;
use gpui_kit::{
    AnyElement, App, Entity, Hsla, IntoElement, RenderOnce, SharedString,
    Styled as _, Window, px,
};
use gpui_kit::component::{Sizable, Size, input::{Input, InputState}};
use gpui_kit::prelude::FluentBuilder as _;
use crate::controls::ControlSize;
use crate::tokens::{AntTheme, color};

/// Ant Design's three common input surface variants.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputVariant {
    #[default]
    Outlined,
    Filled,
    Borderless,
}

impl InputVariant {
    pub fn framed(self) -> bool { self != Self::Borderless }
    pub fn bordered(self) -> bool { self == Self::Outlined }
}

/// Validation status affects the input's border and background semantic colors.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputStatus {
    #[default]
    Normal,
    Error,
    Warning,
}

#[derive(IntoElement)]
pub struct AntInput {
    state: Entity<InputState>,
    theme: AntTheme,
    size: ControlSize,
    variant: InputVariant,
    status: InputStatus,
    disabled: bool,
    readonly: bool,
    clearable: bool,
    password_toggle: bool,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    accessibility_label: Option<SharedString>,
}

impl AntInput {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(), theme: AntTheme::default(),
            size: ControlSize::Middle, variant: InputVariant::Outlined,
            status: InputStatus::Normal, disabled: false, readonly: false,
            clearable: false, password_toggle: false,
            prefix: None, suffix: None, accessibility_label: None,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn size(mut self, size: ControlSize) -> Self { self.size = size; self }
    pub fn variant(mut self, variant: InputVariant) -> Self { self.variant = variant; self }
    pub fn status(mut self, status: InputStatus) -> Self { self.status = status; self }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
    pub fn readonly(mut self, readonly: bool) -> Self { self.readonly = readonly; self }
    pub fn clearable(mut self, clearable: bool) -> Self { self.clearable = clearable; self }
    pub fn password_toggle(mut self, enabled: bool) -> Self { self.password_toggle = enabled; self }
    pub fn prefix(mut self, element: impl IntoElement) -> Self {
        self.prefix = Some(element.into_any_element()); self
    }
    pub fn suffix(mut self, element: impl IntoElement) -> Self {
        self.suffix = Some(element.into_any_element()); self
    }
    pub fn aria_label(mut self, text: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(text.into()); self
    }

    fn palette(&self) -> (Hsla, Hsla) {
        let background = if self.disabled {
            self.theme.disabled_bg
        } else if self.variant == InputVariant::Filled {
            self.theme.surface_secondary
        } else if self.variant == InputVariant::Borderless {
            color(0x000000).opacity(0.)
        } else {
            self.theme.surface
        };
        let border = match self.status {
            InputStatus::Normal => self.theme.border,
            InputStatus::Error => self.theme.error,
            InputStatus::Warning => self.theme.warning,
        };
        (background, border)
    }
}

impl RenderOnce for AntInput {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let size = match self.size {
            ControlSize::Small => Size::Small,
            ControlSize::Middle => Size::Medium,
            ControlSize::Large => Size::Large,
        };
        let (background, border) = self.palette();
        let comp=self.theme.component(AntComponent::Input);
        Input::new(&self.state)
            .with_size(size)
            .disabled(self.disabled)
            .readonly(self.readonly)
            .cleanable(self.clearable)
            .appearance(self.variant.framed())
            .bordered(self.variant.bordered())
            .when(self.password_toggle, |this| this.mask_toggle())
            .when_some(self.prefix, |this, prefix| this.prefix(prefix))
            .when_some(self.suffix, |this, suffix| this.suffix(suffix))
            .when_some(self.accessibility_label, |this, label| this.aria_label(label))
            .rounded(px(comp.border_radius.unwrap_or(self.theme.border_radius)))
            .border_color(comp.color_border.unwrap_or(border))
            .bg(comp.color_bg.unwrap_or(background))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_variant_flags_are_consistent() {
        assert!(InputVariant::Outlined.framed());
        assert!(InputVariant::Outlined.bordered());
        assert!(InputVariant::Filled.framed());
        assert!(!InputVariant::Filled.bordered());
        assert!(!InputVariant::Borderless.framed());
        assert!(!InputVariant::Borderless.bordered());
    }
}
