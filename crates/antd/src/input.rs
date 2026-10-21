//! Ant Design-style text input.
//!
//! **Presentation is owned here.** Kit `Input` is always mounted with
//! `appearance(false)` (+ `bordered(false)`) so Kit drops its border, background
//! and radius. This wrapper is the only frame: 1px border, token radius, height
//! 24/32/40, horizontal padding, hover border (primary or status), focus border,
//! disabled fill, status suffix icon, addon before/after (attached, shared outer
//! border, no gap, outer corners only), and optional `show_count`.
//!
//! Kit still owns caret, IME, selection, undo, mask, clear and password toggle
//! via parent-owned `Entity<InputState>`. No entities are constructed in render.
//!
//! **Gaps:** formatter/parser, max-length enforcement (`.max_length_hint` is
//! display-only beside `show_count`; use InputState validation if you need a hard
//! cap), and textarea/multi-line.

use crate::controls::ControlSize;
use crate::design_tokens::{AntComponent, AntComponentToken};
use crate::icons::{AntIcon, AntIconName};
use crate::tokens::{AntTheme, color};
use gpui_kit::component::{
    Sizable, Size,
    input::{Input, InputState},
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, Entity, Focusable as _, Hsla, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _,
    Window, div, px,
};

/// Ant Design's three common input surface variants.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputVariant {
    #[default]
    Outlined,
    Filled,
    Borderless,
}

impl InputVariant {
    pub fn framed(self) -> bool {
        self != Self::Borderless
    }
    pub fn bordered(self) -> bool {
        self == Self::Outlined
    }
}

/// Validation status affects the input's border and background semantic colors.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputStatus {
    #[default]
    Normal,
    Error,
    Warning,
}

/// Resolve control height: component token override, else theme size ladder (24/32/40).
pub(crate) fn input_control_height(
    size: ControlSize,
    theme: &AntTheme,
    component: &AntComponentToken,
) -> f32 {
    component.control_height.unwrap_or(match size {
        ControlSize::Small => theme.control_height_sm,
        ControlSize::Middle => theme.control_height,
        ControlSize::Large => theme.control_height_lg,
    })
}

/// Font size from component token or theme, with Large bump matching buttons.
pub(crate) fn input_font_size(
    size: ControlSize,
    theme: &AntTheme,
    component: &AntComponentToken,
) -> f32 {
    component
        .font_size
        .unwrap_or(if size == ControlSize::Large {
            theme.font_size + 2.
        } else {
            theme.font_size
        })
}

/// Surface fill for the field chrome.
pub(crate) fn input_field_background(
    disabled: bool,
    variant: InputVariant,
    theme: &AntTheme,
    component: &AntComponentToken,
) -> Hsla {
    if disabled {
        theme.disabled_bg
    } else if variant == InputVariant::Borderless {
        color(0).opacity(0.)
    } else if variant == InputVariant::Filled {
        component.color_bg.unwrap_or(theme.surface_secondary)
    } else {
        component.color_bg.unwrap_or(theme.surface)
    }
}

/// Resting border color (status wins over the normal token border).
pub(crate) fn input_field_border(
    status: InputStatus,
    variant: InputVariant,
    theme: &AntTheme,
    component: &AntComponentToken,
) -> Hsla {
    if !variant.framed() {
        return color(0).opacity(0.);
    }
    match status {
        InputStatus::Normal => component.color_border.unwrap_or(theme.border),
        InputStatus::Error => theme.error,
        InputStatus::Warning => theme.warning,
    }
}

/// Hover/focus border: primary when normal, else the status color. No hover when disabled.
pub(crate) fn input_hover_border(
    status: InputStatus,
    theme: &AntTheme,
    component: &AntComponentToken,
) -> Hsla {
    match status {
        InputStatus::Normal => component.color_primary.unwrap_or(theme.primary),
        InputStatus::Error => theme.error,
        InputStatus::Warning => theme.warning,
    }
}

/// Ant Design status glyph used as the default suffix when the caller did not set one.
pub(crate) fn input_status_icon(status: InputStatus) -> Option<AntIconName> {
    match status {
        InputStatus::Normal => None,
        InputStatus::Error => Some(AntIconName::CloseCircleFilled),
        InputStatus::Warning => Some(AntIconName::ExclamationCircleFilled),
    }
}

pub(crate) fn input_status_icon_color(status: InputStatus, theme: &AntTheme) -> Option<Hsla> {
    match status {
        InputStatus::Normal => None,
        InputStatus::Error => Some(theme.error),
        InputStatus::Warning => Some(theme.warning),
    }
}

/// Inject a status suffix only when the caller left suffix empty.
pub(crate) fn should_inject_status_suffix(status: InputStatus, caller_set_suffix: bool) -> bool {
    !caller_set_suffix && input_status_icon(status).is_some()
}

/// show_count label: `n` or `n / max` (display only; does not truncate).
pub(crate) fn format_show_count(current_chars: usize, max_hint: Option<usize>) -> String {
    match max_hint {
        Some(max) => format!("{current_chars} / {max}"),
        None => current_chars.to_string(),
    }
}

/// Whether the field should paint its own border (outlined/filled).
pub(crate) fn input_paints_border(variant: InputVariant) -> bool {
    variant.framed()
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
    show_count: bool,
    max_length_hint: Option<usize>,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    addon_before: Option<AnyElement>,
    addon_after: Option<AnyElement>,
    accessibility_label: Option<SharedString>,
}

impl AntInput {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            theme: AntTheme::default(),
            size: ControlSize::Middle,
            variant: InputVariant::Outlined,
            status: InputStatus::Normal,
            disabled: false,
            readonly: false,
            clearable: false,
            password_toggle: false,
            show_count: false,
            max_length_hint: None,
            prefix: None,
            suffix: None,
            addon_before: None,
            addon_after: None,
            accessibility_label: None,
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
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn status(mut self, status: InputStatus) -> Self {
        self.status = status;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }
    pub fn password_toggle(mut self, enabled: bool) -> Self {
        self.password_toggle = enabled;
        self
    }
    /// Show a live character count beside the field (reads [`InputState::value`]).
    pub fn show_count(mut self, show: bool) -> Self {
        self.show_count = show;
        self
    }
    /// Optional max displayed next to the count as `n / max`. Display only — does not truncate.
    pub fn max_length_hint(mut self, max: usize) -> Self {
        self.max_length_hint = Some(max);
        self
    }
    pub fn prefix(mut self, element: impl IntoElement) -> Self {
        self.prefix = Some(element.into_any_element());
        self
    }
    pub fn suffix(mut self, element: impl IntoElement) -> Self {
        self.suffix = Some(element.into_any_element());
        self
    }
    /// Ant Design addonBefore: attached box on the start edge.
    pub fn addon_before(mut self, element: impl IntoElement) -> Self {
        self.addon_before = Some(element.into_any_element());
        self
    }
    /// Ant Design addonAfter: attached box on the end edge.
    pub fn addon_after(mut self, element: impl IntoElement) -> Self {
        self.addon_after = Some(element.into_any_element());
        self
    }
    pub fn aria_label(mut self, text: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(text.into());
        self
    }
}

/// Shared corner radius for the outer chrome frame.
pub(crate) fn input_radius(theme: &AntTheme, component: &AntComponentToken) -> f32 {
    component.border_radius.unwrap_or(theme.border_radius)
}

impl RenderOnce for AntInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // Capture flags before moving prefix/suffix/addons into children.
        let has_caller_suffix = self.suffix.is_some();
        let inject_status = should_inject_status_suffix(self.status, has_caller_suffix);
        let disabled = self.disabled;
        let readonly = self.readonly;
        let clearable = self.clearable;
        let password_toggle = self.password_toggle;
        let show_count = self.show_count;
        let max_hint = self.max_length_hint;
        let variant = self.variant;
        let status = self.status;
        let size = self.size;
        let theme = self.theme;
        let state = self.state;
        let accessibility_label = self.accessibility_label;
        let prefix = self.prefix;
        let mut suffix = self.suffix;
        let addon_before = self.addon_before;
        let addon_after = self.addon_after;

        let kit_size = match size {
            ControlSize::Small => Size::Small,
            ControlSize::Middle => Size::Medium,
            ControlSize::Large => Size::Large,
        };
        let comp = theme.component(AntComponent::Input);
        let height = input_control_height(size, &theme, &comp);
        let font_size = input_font_size(size, &theme, &comp);
        let radius = input_radius(&theme, &comp);
        let background = input_field_background(disabled, variant, &theme, &comp);
        let rest_border = input_field_border(status, variant, &theme, &comp);
        let active_border = input_hover_border(status, &theme, &comp);
        let paints_border = input_paints_border(variant);
        let has_addons = addon_before.is_some() || addon_after.is_some();

        if inject_status {
            if let (Some(name), Some(tint)) = (
                input_status_icon(status),
                input_status_icon_color(status, &theme),
            ) {
                suffix = Some(
                    AntIcon::new(name)
                        .theme(theme)
                        .size(font_size)
                        .color(tint)
                        .into_any_element(),
                );
            }
        }

        let focused = state.focus_handle(cx).is_focused(window) && !disabled;
        let border_now = if focused && paints_border {
            active_border
        } else {
            rest_border
        };

        // Kit editor without its own frame — wrapper owns antd chrome.
        let editor = Input::new(&state)
            .with_size(kit_size)
            .disabled(disabled)
            .readonly(readonly)
            .cleanable(clearable)
            .appearance(false)
            .bordered(false)
            .focus_bordered(false)
            .when(password_toggle, |this| this.mask_toggle())
            .when_some(prefix, |this, prefix| this.prefix(prefix))
            .when_some(suffix, |this, suffix| this.suffix(suffix))
            .when_some(accessibility_label, |this, label| this.aria_label(label))
            .h(px(height))
            .w_full()
            .text_size(px(font_size))
            .text_color(if disabled {
                theme.disabled_text
            } else {
                theme.text
            });

        let field = if has_addons {
            // Attached addon boxes: shared outer border, secondary fill, square inner edges.
            div()
                .id(("ant-input-group", state.entity_id()))
                .flex()
                .flex_row()
                .items_stretch()
                .h(px(height))
                .w_full()
                .rounded(px(radius))
                .overflow_hidden()
                .when(paints_border, |this| {
                    this.border_1().border_color(border_now)
                })
                .when(paints_border && !disabled && !focused, |this| {
                    this.hover(|this| this.border_color(active_border))
                })
                .bg(background)
                .when_some(addon_before, |this, before| {
                    this.child(
                        div()
                            .flex()
                            .items_center()
                            .px(px(11.))
                            .bg(theme.surface_secondary)
                            .when(paints_border, |this| {
                                this.border_r_1().border_color(border_now)
                            })
                            .text_color(theme.text_secondary)
                            .text_size(px(font_size))
                            .child(before),
                    )
                })
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .px(px(11.))
                        .flex()
                        .items_center()
                        .child(editor),
                )
                .when_some(addon_after, |this, after| {
                    this.child(
                        div()
                            .flex()
                            .items_center()
                            .px(px(11.))
                            .bg(theme.surface_secondary)
                            .when(paints_border, |this| {
                                this.border_l_1().border_color(border_now)
                            })
                            .text_color(theme.text_secondary)
                            .text_size(px(font_size))
                            .child(after),
                    )
                })
                .into_any_element()
        } else {
            div()
                .id(("ant-input", state.entity_id()))
                .flex()
                .items_center()
                .h(px(height))
                .w_full()
                .rounded(px(radius))
                .px(px(11.))
                .when(paints_border, |this| {
                    this.border_1().border_color(border_now)
                })
                .when(paints_border && !disabled && !focused, |this| {
                    this.hover(|this| this.border_color(active_border))
                })
                .bg(background)
                .child(editor)
                .into_any_element()
        };

        // show_count: verified InputState::value reader (display only).
        if show_count {
            let current = state.read(cx).value().chars().count();
            let label = format_show_count(current, max_hint);
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.))
                .w_full()
                .child(div().flex_1().min_w_0().child(field))
                .child(
                    div()
                        .flex_shrink_0()
                        .text_size(px(theme.font_size - 2.).max(px(11.)))
                        .text_color(theme.text_secondary)
                        .child(label),
                )
                .into_any_element()
        } else {
            field
        }
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

    #[test]
    fn control_height_follows_size_ladder() {
        let theme = AntTheme::light();
        let comp = theme.component(AntComponent::Input);
        assert_eq!(input_control_height(ControlSize::Small, &theme, &comp), 24.);
        assert_eq!(
            input_control_height(ControlSize::Middle, &theme, &comp),
            32.
        );
        assert_eq!(input_control_height(ControlSize::Large, &theme, &comp), 40.);
    }

    #[test]
    fn status_suffix_injection_rules() {
        assert!(should_inject_status_suffix(InputStatus::Error, false));
        assert!(!should_inject_status_suffix(InputStatus::Error, true));
        assert!(!should_inject_status_suffix(InputStatus::Normal, false));
        assert_eq!(
            input_status_icon(InputStatus::Error),
            Some(AntIconName::CloseCircleFilled)
        );
        assert_eq!(
            input_status_icon(InputStatus::Warning),
            Some(AntIconName::ExclamationCircleFilled)
        );
    }

    #[test]
    fn show_count_format() {
        assert_eq!(format_show_count(3, None), "3");
        assert_eq!(format_show_count(3, Some(10)), "3 / 10");
    }

    #[test]
    fn hover_border_uses_status_or_primary() {
        let theme = AntTheme::light();
        let comp = theme.component(AntComponent::Input);
        assert_eq!(
            input_hover_border(InputStatus::Normal, &theme, &comp),
            theme.primary
        );
        assert_eq!(
            input_hover_border(InputStatus::Error, &theme, &comp),
            theme.error
        );
        assert_eq!(
            input_hover_border(InputStatus::Warning, &theme, &comp),
            theme.warning
        );
    }

    #[test]
    fn borderless_skips_painted_border() {
        assert!(!input_paints_border(InputVariant::Borderless));
        assert!(input_paints_border(InputVariant::Outlined));
        assert!(input_paints_border(InputVariant::Filled));
    }
}
