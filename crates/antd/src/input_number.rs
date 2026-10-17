//! Ant Design-style numeric text entry using GPUI Kit's native NumberInput.
//!
//! The host owns Entity<InputState>, including .min/.max/.step, mask,
//! validation and subscription to InputEvent::Change. Native editing, IME,
//! caret, keyboard Up/Down and spin buttons remain in GPUI Kit.

use gpui_kit::{AnyElement, App, Entity, IntoElement, RenderOnce, SharedString, Styled as _, Window, px};
use gpui_kit::component::{
    Disableable as _, Sizable as _, Size,
    input::{InputState, NumberInput},
};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntTheme, ControlSize, InputStatus, InputVariant, color};
use crate::design_tokens::AntComponent;

#[derive(IntoElement)]
pub struct AntInputNumber {
    state: Entity<InputState>,
    theme: AntTheme,
    size: ControlSize,
    variant: InputVariant,
    status: InputStatus,
    disabled: bool,
    placeholder: Option<SharedString>,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
}

impl AntInputNumber {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(), theme: AntTheme::default(),
            size: ControlSize::Middle, variant: InputVariant::Outlined,
            status: InputStatus::Normal, disabled: false, placeholder: None,
            prefix: None, suffix: None,
        }
    }
    pub fn theme(mut self, t: AntTheme) -> Self { self.theme=t; self }
    pub fn size(mut self, s: ControlSize) -> Self { self.size=s; self }
    pub fn variant(mut self, v: InputVariant) -> Self { self.variant=v; self }
    pub fn status(mut self, s: InputStatus) -> Self { self.status=s; self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled=d; self }
    pub fn placeholder(mut self, p: impl Into<SharedString>) -> Self { self.placeholder=Some(p.into()); self }
    pub fn prefix(mut self, el: impl IntoElement) -> Self { self.prefix=Some(el.into_any_element()); self }
    pub fn suffix(mut self, el: impl IntoElement) -> Self { self.suffix=Some(el.into_any_element()); self }
}

impl RenderOnce for AntInputNumber {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let kit_size = match self.size {
            ControlSize::Small => Size::Small,
            ControlSize::Middle => Size::Medium,
            ControlSize::Large => Size::Large,
        };
        let palette = self.theme.component(AntComponent::Input);
        let frame = match self.variant {
            InputVariant::Outlined => self.theme.surface,
            InputVariant::Filled => self.theme.surface_secondary,
            InputVariant::Borderless => color(0).opacity(0.),
        };
        let stroke = match self.status {
            InputStatus::Normal => self.theme.border,
            InputStatus::Error => self.theme.error,
            InputStatus::Warning => self.theme.warning,
        };
        NumberInput::new(&self.state)
            .with_size(kit_size)
            .disabled(self.disabled)
            .appearance(self.variant != InputVariant::Borderless)
            .when_some(self.placeholder, |this, text| this.placeholder(text))
            .when_some(self.prefix, |this, el| this.prefix(el))
            .when_some(self.suffix, |this, el| this.suffix(el))
            .rounded(px(palette.border_radius.unwrap_or(self.theme.border_radius)))
            .border_color(if self.status == InputStatus::Normal {
                palette.color_border.unwrap_or(stroke)
            } else { stroke })
            .bg(if self.disabled { self.theme.disabled_bg }
                else if self.variant == InputVariant::Borderless { frame }
                else { palette.color_bg.unwrap_or(frame) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_variants_follow_shared_input_contract() {
        assert!(InputVariant::Outlined.bordered());
        assert!(!InputVariant::Borderless.framed());
        assert!(InputVariant::Filled.framed());
    }
}
