//! Ant Design-style numeric text entry.
//!
//! **Presentation is owned here.** Kit `Input` is mounted with `appearance(false)`
//! so it contributes only the editing engine (caret, IME, selection, undo, mask).
//! Height, 1px border, radius, padding, hover/focus border, disabled fill, status
//! icon, addon boxes and the stacked handler column are drawn by this wrapper
//! using `AntTheme` / `AntComponent::InputNumber` tokens and `AntIcon`.
//!
//! **Stepping:** `InputState::apply_number_step` is crate-private in gpui-base.
//! The verified public path is `gpui_kit::base::NumberInput` with
//! `controls_right()`, whose default `on_step` calls that private method (and
//! respects host-configured `.step` / `.min` / `.max` on `InputState`). We
//! decorate those buttons with `UpOutlined` / `DownOutlined` — we do **not** use
//! `component::NumberInput`, which always paints Kit's own ± pair and cannot
//! hide them via `appearance(false)`.
//!
//! **Gaps (not implemented):** formatter, parser, precision, string-mode keyboard
//! edge cases beyond Kit's built-in number mask / step.
//!
//! Placeholder: set on the parent-owned `InputState` (`InputState::placeholder` /
//! `set_placeholder`). The `.placeholder` builder is kept for API compatibility
//! and synchronizes an explicitly supplied hint only when it changes.

use crate::design_tokens::AntComponent;
use crate::icons::{AntIcon, AntIconName};
use crate::input::{
    InputStatus, InputVariant, input_control_height, input_field_background, input_field_border,
    input_font_size, input_hover_border, input_paints_border, input_radius, input_status_icon,
    input_status_icon_color, should_inject_status_suffix,
};
use crate::{AntTheme, ControlSize};
use gpui_kit::base::NumberInput as BaseNumberInput;
use gpui_kit::component::{
    Sizable as _, Size,
    input::{Input, InputState},
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, Entity, Focusable as _, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _,
    Window, div, px,
};

/// Whether the antd stacked handler column is composed (independent of disabled;
/// disabled greys the buttons out rather than removing them).
pub(crate) fn number_shows_controls(controls: bool) -> bool {
    controls
}

/// Icon size for half-height handler buttons.
pub(crate) fn number_handler_icon_size(font_size: f32) -> f32 {
    (font_size - 2.).clamp(10., 14.)
}

#[derive(IntoElement)]
pub struct AntInputNumber {
    state: Entity<InputState>,
    theme: AntTheme,
    size: ControlSize,
    variant: InputVariant,
    status: InputStatus,
    disabled: bool,
    controls: bool,
    placeholder: Option<SharedString>,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    addon_before: Option<AnyElement>,
    addon_after: Option<AnyElement>,
}

impl AntInputNumber {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            theme: AntTheme::default(),
            size: ControlSize::Middle,
            variant: InputVariant::Outlined,
            status: InputStatus::Normal,
            disabled: false,
            controls: true,
            placeholder: None,
            prefix: None,
            suffix: None,
            addon_before: None,
            addon_after: None,
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
    pub fn variant(mut self, v: InputVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn status(mut self, s: InputStatus) -> Self {
        self.status = s;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    /// Show the antd stacked Up/Down handler column (default true).
    pub fn controls(mut self, show: bool) -> Self {
        self.controls = show;
        self
    }
    /// Override the hint in the parent-owned InputState. Omit this builder to
    /// keep the placeholder configured on the state.
    pub fn placeholder(mut self, p: impl Into<SharedString>) -> Self {
        self.placeholder = Some(p.into());
        self
    }
    pub fn prefix(mut self, el: impl IntoElement) -> Self {
        self.prefix = Some(el.into_any_element());
        self
    }
    pub fn suffix(mut self, el: impl IntoElement) -> Self {
        self.suffix = Some(el.into_any_element());
        self
    }
    pub fn addon_before(mut self, el: impl IntoElement) -> Self {
        self.addon_before = Some(el.into_any_element());
        self
    }
    pub fn addon_after(mut self, el: impl IntoElement) -> Self {
        self.addon_after = Some(el.into_any_element());
        self
    }
}

impl RenderOnce for AntInputNumber {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // Capture before moving adornments into the kit control.
        let has_caller_suffix = self.suffix.is_some();
        let inject_status = should_inject_status_suffix(self.status, has_caller_suffix);
        let disabled = self.disabled;
        let show_handlers = number_shows_controls(self.controls);
        let variant = self.variant;
        let status = self.status;
        let size = self.size;
        let theme = self.theme;
        let state = self.state;
        if let Some(placeholder) = self.placeholder {
            // Kit Input reads its hint from InputState. Sync only on a change,
            // after rendering, so this cannot create a perpetual redraw loop.
            if state.read(cx).presentation().placeholder() != &placeholder {
                let hint_state = state.clone();
                window.defer(cx, move |window, cx| {
                    hint_state.update(cx, |state, cx| {
                        if state.presentation().placeholder() != &placeholder {
                            state.set_placeholder(placeholder, window, cx);
                        }
                    });
                });
            }
        }
        let prefix = self.prefix;
        let mut suffix = self.suffix;
        let addon_before = self.addon_before;
        let addon_after = self.addon_after;

        let kit_size = match size {
            ControlSize::Small => Size::Small,
            ControlSize::Middle => Size::Medium,
            ControlSize::Large => Size::Large,
        };
        let comp = theme.component(AntComponent::InputNumber);
        let height = input_control_height(size, &theme, &comp);
        let font_size = input_font_size(size, &theme, &comp);
        let radius = input_radius(&theme, &comp);
        let background = input_field_background(disabled, variant, &theme, &comp);
        let rest_border = input_field_border(status, variant, &theme, &comp);
        let active_border = input_hover_border(status, &theme, &comp);
        let paints_border = input_paints_border(variant);
        let has_addons = addon_before.is_some() || addon_after.is_some();
        let icon_size = number_handler_icon_size(font_size);

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

        // Editing engine only — no Kit frame (appearance false).
        let editor = Input::new(&state)
            .with_size(kit_size)
            .disabled(disabled)
            .appearance(false)
            .bordered(false)
            .focus_bordered(false)
            .when_some(prefix, |this, el| this.prefix(el))
            .when_some(suffix, |this, el| this.suffix(el))
            .w_full()
            .text_size(px(font_size))
            .text_color(if disabled {
                theme.disabled_text
            } else {
                theme.text
            });

        // Antd stacked handlers via verified base NumberInput::controls_right.
        // Button faces are ours (AntIcon); stepping is Kit's default on_step.
        let body = if show_handlers {
            let theme_up = theme;
            let theme_down = theme;
            let stroke = border_now;
            BaseNumberInput::new(&state)
                .disabled(disabled)
                .controls_right()
                .increment_button(move |btn| {
                    btn.flex_1()
                        .min_h_0()
                        .w(px(22.))
                        .focusable(false)
                        .disabled(disabled)
                        .flex()
                        .items_center()
                        .justify_center()
                        .border_l_1()
                        .border_b_1()
                        .border_color(stroke)
                        .text_color(if disabled {
                            theme_up.disabled_text
                        } else {
                            theme_up.text_secondary
                        })
                        .when(!disabled, |this| {
                            this.hover(|this| this.bg(theme_up.surface_secondary))
                        })
                        .child(
                            AntIcon::new(AntIconName::UpOutlined)
                                .theme(theme_up)
                                .size(icon_size)
                                .color(if disabled {
                                    theme_up.disabled_text
                                } else {
                                    theme_up.text_secondary
                                }),
                        )
                })
                .decrement_button(move |btn| {
                    btn.flex_1()
                        .min_h_0()
                        .w(px(22.))
                        .focusable(false)
                        .disabled(disabled)
                        .flex()
                        .items_center()
                        .justify_center()
                        .border_l_1()
                        .border_color(stroke)
                        .text_color(if disabled {
                            theme_down.disabled_text
                        } else {
                            theme_down.text_secondary
                        })
                        .when(!disabled, |this| {
                            this.hover(|this| this.bg(theme_down.surface_secondary))
                        })
                        .child(
                            AntIcon::new(AntIconName::DownOutlined)
                                .theme(theme_down)
                                .size(icon_size)
                                .color(if disabled {
                                    theme_down.disabled_text
                                } else {
                                    theme_down.text_secondary
                                }),
                        )
                })
                .input(editor)
                .size_full()
                .into_any_element()
        } else {
            editor.into_any_element()
        };

        let field = if has_addons {
            div()
                .id(("ant-input-number-group", state.entity_id()))
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
                        .child(body),
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
                .id(("ant-input-number", state.entity_id()))
                .flex()
                .items_center()
                .h(px(height))
                .w_full()
                .rounded(px(radius))
                .when(!show_handlers, |this| this.px(px(11.)))
                .when(show_handlers, |this| this.pl(px(11.)))
                .when(paints_border, |this| {
                    this.border_1().border_color(border_now)
                })
                .when(paints_border && !disabled && !focused, |this| {
                    this.hover(|this| this.border_color(active_border))
                })
                .bg(background)
                .child(body)
                .into_any_element()
        };

        field
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::format_show_count;

    #[test]
    fn number_variants_follow_shared_input_contract() {
        assert!(InputVariant::Outlined.bordered());
        assert!(!InputVariant::Borderless.framed());
        assert!(InputVariant::Filled.framed());
    }

    #[test]
    fn controls_visibility_rules() {
        assert!(number_shows_controls(true));
        assert!(!number_shows_controls(false));
    }

    #[test]
    fn handler_icon_size_is_bounded() {
        assert_eq!(number_handler_icon_size(14.), 12.);
        assert_eq!(number_handler_icon_size(8.), 10.);
        assert_eq!(number_handler_icon_size(20.), 14.);
    }

    #[test]
    fn number_height_matches_input_ladder() {
        let theme = AntTheme::light();
        let comp = theme.component(AntComponent::InputNumber);
        assert_eq!(input_control_height(ControlSize::Small, &theme, &comp), 24.);
        assert_eq!(
            input_control_height(ControlSize::Middle, &theme, &comp),
            32.
        );
        assert_eq!(input_control_height(ControlSize::Large, &theme, &comp), 40.);
    }

    #[test]
    fn addon_and_status_chrome_helpers_shared_with_input() {
        assert!(should_inject_status_suffix(InputStatus::Error, false));
        assert!(!should_inject_status_suffix(InputStatus::Error, true));
        assert_eq!(format_show_count(1, Some(5)), "1 / 5");
        let theme = AntTheme::light();
        assert!(input_paints_border(InputVariant::Outlined));
        assert!(!input_paints_border(InputVariant::Borderless));
        assert_eq!(
            input_hover_border(
                InputStatus::Error,
                &theme,
                &theme.component(AntComponent::InputNumber)
            ),
            theme.error
        );
    }
}
