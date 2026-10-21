//! Native Ant Design slider appearance around GPUI Kit base slider parts.
//!
//! Do not use `gpui_kit::component::slider::Slider`: its single `.bg()` paints
//! both the filled bar and a translucent thumb halo. Instead this wrapper
//! composes base `Slider` + `SliderTrack` + `SliderIndicator` + `SliderThumb`
//! against the same `Entity<SliderState>` gallery and callers already hold
//! (component re-exports that base state type).
//!
//! Painted parts:
//! - rail 4px, full radius, `surface_secondary` (disabled → `disabled_bg`)
//! - fill `theme.primary` (disabled → `theme.border`)
//! - thumb white + 2px primary border, ~14px circle
//!
//! Base Slider has no `reverse` API; reverse is applied only to fill geometry
//! for single-value sliders. Marks and drag-value tooltips are omitted (no
//! base hooks). Keyboard handling and SliderEvent::Change/Release stay.

use crate::design_tokens::AntComponent;
use crate::{AntTheme, color};
use gpui_kit::base::slider::{SliderEvent, SliderState, SliderValue};
use gpui_kit::base::{
    AxisExt as _, Slider as BaseSlider, SliderIndicator, SliderThumb, SliderTrack,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, Axis, Entity, FocusHandle, InteractiveElement as _, IntoElement, KeyDownEvent,
    ParentElement as _, RenderOnce, Role, StatefulInteractiveElement as _, Styled as _, Window,
    div, px, relative,
};

/// Clamp a keyboard increment to an integer or decimal SliderState step.
/// Reject invalid intervals instead of propagating NaN into layout or value.
pub fn slider_key_value(current: f32, min: f32, max: f32, step: f32, key: &str) -> Option<f32> {
    if !current.is_finite()
        || !min.is_finite()
        || !max.is_finite()
        || !step.is_finite()
        || step <= 0.
        || max <= min
    {
        return None;
    }
    let value = match key {
        "left" | "down" => current - step,
        "right" | "up" => current + step,
        "pageup" => current + step * 10.,
        "pagedown" => current - step * 10.,
        "home" => min,
        "end" => max,
        _ => return None,
    }
    .clamp(min, max);
    let steps = ((value - min) / step).round();
    let snapped = (min + steps * step).clamp(min, max);
    Some(if matches!(key, "end") {
        max
    } else if matches!(key, "home") {
        min
    } else {
        (snapped * 10000.).round() / 10000.
    })
}

/// Ant Design rail / fill / thumb-border colors for the current disabled state.
///
/// Returns `(rail, fill, thumb_border)`.
pub fn slider_ant_colors(
    theme: AntTheme,
    disabled: bool,
) -> (gpui_kit::Hsla, gpui_kit::Hsla, gpui_kit::Hsla) {
    let token = theme.component(AntComponent::Slider);
    let fill = if disabled {
        theme.border
    } else {
        token.color_primary.unwrap_or(theme.primary)
    };
    let rail = if disabled {
        theme.disabled_bg
    } else {
        token.color_bg.unwrap_or(theme.surface_secondary)
    };
    let thumb_border = if disabled {
        theme.disabled_text
    } else {
        token.color_border.unwrap_or(fill)
    };
    (rail, fill, thumb_border)
}

#[derive(IntoElement)]
pub struct AntSlider {
    state: Entity<SliderState>,
    theme: AntTheme,
    disabled: bool,
    vertical: bool,
    /// When true, single-value fill runs from the thumb toward max (base has no reverse API).
    reverse: bool,
    show_value: bool,
    height: f32,
}
impl AntSlider {
    pub fn new(state: &Entity<SliderState>) -> Self {
        Self {
            state: state.clone(),
            theme: AntTheme::default(),
            disabled: false,
            vertical: false,
            reverse: false,
            show_value: false,
            height: 180.,
        }
    }
    pub fn theme(mut self, t: AntTheme) -> Self {
        self.theme = t;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
    pub fn vertical(mut self, v: bool) -> Self {
        self.vertical = v;
        self
    }
    /// Reverse the filled side for single-value sliders (fill geometry only).
    pub fn reverse(mut self, v: bool) -> Self {
        self.reverse = v;
        self
    }
    pub fn show_value(mut self, v: bool) -> Self {
        self.show_value = v;
        self
    }
    pub fn height(mut self, v: f32) -> Self {
        self.height = v.max(60.);
        self
    }
}
impl RenderOnce for AntSlider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = self.theme;
        let vertical = self.vertical;
        let disabled = self.disabled;
        let reverse = self.reverse;
        let state = self.state;
        let value = state.read(cx).value();
        let percentage = state.read(cx).percentage();
        let is_range = value.is_range();
        let (rail_color, fill_color, thumb_border) = slider_ant_colors(t, disabled);
        let thumb_fill = color(0xffffff);
        let axis = if vertical {
            Axis::Vertical
        } else {
            Axis::Horizontal
        };
        // Base has no reverse flag; invert fill ends for single-value only.
        let (bar_start, bar_end) = if reverse && !is_range {
            (relative(percentage.end), relative(0.))
        } else {
            (relative(percentage.start), relative(1. - percentage.end))
        };
        let id = format!("ant-slider-{:?}", state.entity_id());
        let focus: FocusHandle = window
            .use_keyed_state(id.clone(), cx, |_, cx| cx.focus_handle().tab_stop(true))
            .read(cx)
            .clone();
        let key_state = state.clone();
        let key_focus = focus.clone();
        let handle_key = move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
            if disabled
                || !key_focus.is_focused(window)
                || event.keystroke.modifiers.modified()
                || event.keystroke.is_ime_in_progress()
            {
                return;
            }
            let current = key_state.read(cx).value();
            let SliderValue::Single(current) = current else {
                return;
            };
            let model = key_state.read(cx);
            let next = slider_key_value(
                current,
                model.min_value(),
                model.max_value(),
                model.step_value(),
                event.keystroke.key.as_str(),
            );
            drop(model);
            if let Some(next) = next {
                if (next - current).abs() > f32::EPSILON {
                    key_state.update(cx, |model, cx| {
                        model.set_value(next, window, cx);
                        cx.emit(SliderEvent::Change(SliderValue::Single(next)));
                    });
                }
                window.prevent_default();
                cx.stop_propagation();
            }
        };

        // 14px circular thumb centered on the 4px rail ((14-4)/2 = 5).
        let thumb = |position: gpui_kit::DefiniteLength, start: bool| {
            SliderThumb::new(&state)
                .axis(axis)
                .start(start)
                .disabled(disabled)
                .absolute()
                .when(axis.is_horizontal(), |this| {
                    this.top(px(-5.)).left(position).ml(px(-7.))
                })
                .when(axis.is_vertical(), |this| {
                    this.bottom(position).left(px(-5.)).mb(px(-7.))
                })
                .flex_shrink_0()
                .size(px(14.))
                .rounded_full()
                .bg(thumb_fill)
                .border_2()
                .border_color(thumb_border)
                .when(!disabled, |this| this.cursor_pointer())
        };

        let slider = BaseSlider::new(&state)
            .axis(axis)
            .disabled(disabled)
            .flex()
            .items_center()
            .justify_center()
            .when(vertical, |this| this.h(px(self.height)).w(px(22.)))
            .when(!vertical, |this| this.w_full().h(px(26.)))
            .child(
                SliderTrack::new(&state)
                    .axis(axis)
                    .disabled(disabled)
                    .flex()
                    .when(axis.is_horizontal(), |this| {
                        this.items_center().h(px(26.)).w_full()
                    })
                    .when(axis.is_vertical(), |this| {
                        this.justify_center().w(px(22.)).h_full()
                    })
                    .flex_shrink_0()
                    .child(
                        SliderIndicator::new(&state)
                            .relative()
                            .when(axis.is_horizontal(), |this| this.w_full().h(px(4.)))
                            .when(axis.is_vertical(), |this| this.h_full().w(px(4.)))
                            .bg(rail_color)
                            .rounded_full()
                            .child(
                                div()
                                    .absolute()
                                    .when(axis.is_horizontal(), |this| {
                                        this.h_full().left(bar_start).right(bar_end)
                                    })
                                    .when(axis.is_vertical(), |this| {
                                        this.w_full().bottom(bar_start).top(bar_end)
                                    })
                                    .bg(fill_color)
                                    .rounded_full(),
                            )
                            .when(is_range, |this| {
                                this.child(thumb(relative(percentage.start), true))
                            })
                            .child(thumb(relative(percentage.end), false)),
                    ),
            );

        div()
            .id(id)
            .role(Role::Slider)
            .track_focus(&focus)
            .on_key_down(handle_key)
            .when(!disabled, |this| this.cursor_pointer())
            .when(vertical, |this| this.flex().items_center().gap(px(12.)))
            .when(!vertical, |this| {
                this.flex().items_center().gap(px(14.)).w_full()
            })
            .child(slider)
            .when(self.show_value, |this| {
                this.child(
                    div()
                        .text_size(px(13.))
                        .text_color(if disabled {
                            t.disabled_text
                        } else {
                            t.text_secondary
                        })
                        .child(value.to_string()),
                )
            })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn slider_keyboard_clamps_and_quantizes() {
        assert_eq!(slider_key_value(0., 0., 100., 5., "left"), Some(0.));
        assert_eq!(slider_key_value(50., 0., 100., 5., "right"), Some(55.));
        assert_eq!(slider_key_value(50., 0., 100., 5., "pageup"), Some(100.));
        assert_eq!(slider_key_value(49., 0., 100., 5., "home"), Some(0.));
        assert_eq!(slider_key_value(49., 0., 100., 5., "end"), Some(100.));
        assert_eq!(slider_key_value(0.2, 0., 1., 0.1, "right"), Some(0.3));
    }
    #[test]
    fn slider_keyboard_rejects_invalid_inputs() {
        assert_eq!(slider_key_value(5., 0., 10., 0., "up"), None);
        assert_eq!(slider_key_value(f32::NAN, 0., 10., 1., "up"), None);
        assert_eq!(slider_key_value(5., 10., 0., 1., "up"), None);
        assert_eq!(slider_key_value(5., 0., 10., 1., "space"), None);
    }
    #[test]
    fn slider_ant_colors_follow_disabled_state() {
        let t = AntTheme::light();
        let (rail, fill, border) = slider_ant_colors(t, false);
        assert_eq!(rail, t.surface_secondary);
        assert_eq!(fill, t.primary);
        assert_eq!(border, t.primary);
        let (rail_d, fill_d, border_d) = slider_ant_colors(t, true);
        assert_eq!(rail_d, t.disabled_bg);
        assert_eq!(fill_d, t.border);
        assert_eq!(border_d, t.disabled_text);
    }
}
