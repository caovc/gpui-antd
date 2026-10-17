//! Ant Design-inspired radio controls built on GPUI Base behavior primitives.
//!
//! Selection is controlled by the parent. A RadioGroup manages a one-based
//! accessibility position, a single Tab stop, and arrow/Home/End navigation.
//! Disabled items are skipped. The same Rust renderer works in GPUI Web.

use std::rc::Rc;
use gpui_kit::{
    App, Axis, ElementId, FocusHandle, InteractiveElement as _, IntoElement,
    KeyDownEvent, ParentElement as _, RenderOnce, SharedString,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};
use gpui_kit::base::{Radio as BaseRadio, RadioGroup as BaseRadioGroup, StyledExt as _};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntTheme, ControlSize, color};

type RadioChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type GroupChange = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Single accessible radio item with Ant Design 16px indicator and optional
/// radio-button presentation. Pass `.checked` from the application state.
#[derive(IntoElement)]
pub struct AntRadio {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    button: bool,
    tab_stop: bool,
    position: Option<(usize, usize)>,
    theme: AntTheme,
    size: ControlSize,
    on_change: Option<RadioChange>,
}

impl AntRadio {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            disabled: false,
            button: false,
            tab_stop: true,
            position: None,
            theme: AntTheme::default(),
            size: ControlSize::Middle,
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self { self.checked = checked; self }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
    pub fn button_style(mut self, button: bool) -> Self { self.button = button; self }
    pub fn size(mut self, size: ControlSize) -> Self { self.size = size; self }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn tab_stop(mut self, enabled: bool) -> Self { self.tab_stop = enabled; self }
    pub fn position(mut self, position: usize, size: usize) -> Self {
        self.position = Some((position, size));
        self
    }
    pub fn on_change(mut self, listener: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(listener));
        self
    }
}

impl RenderOnce for AntRadio {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx).clone();
        let focused = focus.is_focused(window);
        let t = self.theme;
        let checked = self.checked;
        let disabled = self.disabled;
        let button = self.button;
        let height = self.size.height();
        let outer_bg = if disabled { t.disabled_bg } else if button && checked { t.primary } else { t.surface };
        let foreground = if disabled {
            t.disabled_text
        } else if button && checked {
            color(0xffffff)
        } else {
            t.text
        };
        let border = if checked { t.primary } else { t.border };
        let indicator = div()
            .size(px(16.))
            .rounded_full()
            .border_1()
            .border_color(if disabled { t.disabled_text } else { border })
            .bg(if disabled { t.disabled_bg } else { t.surface })
            .flex().items_center().justify_center()
            .child(
                div()
                    .size(px(8.))
                    .rounded_full()
                    .bg(if checked { t.primary } else { t.surface })
            );

        BaseRadio::new(self.id)
            .checked(checked)
            .disabled(disabled)
            .accessibility_label(self.label.clone())
            .track_focus(&focus)
            .tab_stop(self.tab_stop)
            .when_some(self.position, |radio, (index, count)| radio.set_position(index, count))
            .flex()
            .items_center()
            .gap(px(if button { 0. } else { 8. }))
            .h(px(if button { height } else { height.max(28.) }))
            .px(px(if button { self.size.horizontal_padding() } else { 0. }))
            .rounded(px(if button { t.border_radius } else { 4. }))
            .when(button, |radio| radio.border_1().border_color(if focused { t.primary } else { border }).bg(outer_bg))
            .when(!button && focused, |radio| radio.bg(t.info_bg))
            .when(!disabled, |radio| radio.cursor_pointer())
            .text_size(px(if self.size == ControlSize::Large { 16. } else { 14. }))
            .text_color(foreground)
            .when(!button, |radio| radio.child(indicator))
            .child(self.label)
            .when_some(self.on_change, |radio, listener| {
                radio.on_change(move |next, _, window, cx| listener(next, window, cx))
            })
    }
}

/// Select the next enabled index, wrapping and skipping disabled items.
/// `None` means the list has no enabled choices.
pub fn next_enabled_index(enabled: &[bool], current: usize, forward: bool) -> Option<usize> {
    if enabled.is_empty() || !enabled.iter().any(|enabled| *enabled) {
        return None;
    }
    let len = enabled.len();
    let current = current % len;
    for step in 1..=len {
        let index = if forward { (current + step) % len }
        else { (current + len - (step % len)) % len };
        if enabled[index] {
            return Some(index);
        }
    }
    None
}

/// Ant Design radio group. Owns controlled selection and a keyboard-accessible
/// RadioGroup role, but does not store application values.
///
/// Use one distinct ID per child; selection changes arrive as a zero-based
/// index through `.on_change`, and the parent must update `.selected_index`.
#[derive(IntoElement)]
pub struct AntRadioGroup {
    id: ElementId,
    radios: Vec<AntRadio>,
    selected_index: Option<usize>,
    axis: Axis,
    disabled: bool,
    button_style: bool,
    size: ControlSize,
    theme: AntTheme,
    label: Option<SharedString>,
    on_change: Option<GroupChange>,
}

impl AntRadioGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(), radios: Vec::new(), selected_index: None,
            axis: Axis::Horizontal, disabled: false, button_style: false,
            size: ControlSize::Middle, theme: AntTheme::default(),
            label: None, on_change: None,
        }
    }
    pub fn horizontal(id: impl Into<ElementId>) -> Self { Self::new(id) }
    pub fn vertical(id: impl Into<ElementId>) -> Self { Self::new(id).axis(Axis::Vertical) }
    pub fn axis(mut self, axis: Axis) -> Self { self.axis = axis; self }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self { self.label = Some(label.into()); self }
    pub fn child(mut self, radio: AntRadio) -> Self { self.radios.push(radio); self }
    pub fn children(mut self, radios: impl IntoIterator<Item=AntRadio>) -> Self {
        self.radios.extend(radios); self
    }
    pub fn selected_index(mut self, index: Option<usize>) -> Self { self.selected_index = index; self }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
    pub fn button_style(mut self, button: bool) -> Self { self.button_style = button; self }
    pub fn size(mut self, size: ControlSize) -> Self { self.size = size; self }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler)); self
    }
}

impl RenderOnce for AntRadioGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let selected = self.selected_index;
        let axis = self.axis;
        let total = self.radios.len();
        let enabled: Vec<bool> = self.radios.iter().map(|radio| !radio.disabled && !self.disabled).collect();
        let first_enabled = enabled.iter().position(|enabled| *enabled);
        let tab_index = selected.filter(|index| enabled.get(*index).copied().unwrap_or(false))
            .or(first_enabled);
        let handles: Vec<FocusHandle> = self.radios.iter().map(|radio| {
            window.use_keyed_state(radio.id.clone(), cx, |_, cx| cx.focus_handle())
                .read(cx).clone()
        }).collect();
        let group_focus = window.use_keyed_state(self.id.clone(), cx, |_, cx| {
            cx.focus_handle().tab_stop(false)
        }).read(cx).clone();

        let group_handler = self.on_change.clone();
        let keyboard_handler = self.on_change.clone();
        let focus_handles = handles;
        let enabled_keys = enabled;
        let disabled = self.disabled;
        let on_keyboard = move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
            if disabled || event.keystroke.modifiers.modified() || event.keystroke.is_ime_in_progress() {
                return;
            }
            let direction = match event.keystroke.key.as_str() {
                "right" if axis == Axis::Horizontal => Some(true),
                "left" if axis == Axis::Horizontal => Some(false),
                "down" if axis == Axis::Vertical => Some(true),
                "up" if axis == Axis::Vertical => Some(false),
                _ => None,
            };
            let next = if let Some(forward) = direction {
                let origin = focus_handles.iter().position(|handle| handle.is_focused(window))
                    .or(selected)
                    .or_else(|| enabled_keys.iter().position(|enabled| *enabled))
                    .unwrap_or(0);
                next_enabled_index(&enabled_keys, origin, forward)
            } else if event.keystroke.key == "home" {
                enabled_keys.iter().position(|enabled| *enabled)
            } else if event.keystroke.key == "end" {
                enabled_keys.iter().rposition(|enabled| *enabled)
            } else { return };
            if let Some(index) = next {
                window.focus(&focus_handles[index], cx);
                if selected != Some(index) {
                    if let Some(listener) = &keyboard_handler { listener(index, window, cx); }
                }
                window.prevent_default();
                cx.stop_propagation();
            }
        };

        let radios = self.radios.into_iter().enumerate().map(|(index, radio)| {
            let listener = group_handler.clone();
            let mut radio = radio
                .checked(selected == Some(index))
                .disabled(disabled || radio.disabled)
                .button_style(self.button_style || radio.button)
                .size(self.size)
                .theme(self.theme)
                .tab_stop(tab_index == Some(index))
                .position(index + 1, total);
            if let Some(listener) = listener {
                radio = radio.on_change(move |_, window, cx| listener(index, window, cx));
            }
            radio
        });
        BaseRadioGroup::new(self.id)
            .axis(axis)
            .track_focus(&group_focus)
            .when_some(self.label, |group, label| group.aria_label(label))
            .on_key_down(on_keyboard)
            .flex()
            .when(axis == Axis::Vertical, |group| group.flex_col())
            .when(axis == Axis::Horizontal, |group| group.flex_wrap().items_center())
            .gap(px(if self.button_style { 4. } else { 16. }))
            .children(radios)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_wraps_and_skips_disabled_radios() {
        assert_eq!(next_enabled_index(&[true, false, true], 0, true), Some(2));
        assert_eq!(next_enabled_index(&[true, false, true], 2, true), Some(0));
        assert_eq!(next_enabled_index(&[true, false, true], 0, false), Some(2));
        assert_eq!(next_enabled_index(&[false, false, false], 0, true), None);
        assert_eq!(next_enabled_index(&[], 0, false), None);
        assert_eq!(next_enabled_index(&[true], 0, true), Some(0));
        assert_eq!(next_enabled_index(&[false, true], 9, false), Some(1));
    }

    #[test]
    fn group_builders_support_controlled_selection() {
        let group = AntRadioGroup::horizontal("choices")
            .selected_index(Some(1))
            .child(AntRadio::new("opt-a", "Alpha"))
            .child(AntRadio::new("opt-b", "Beta").disabled(true));
        assert_eq!(group.selected_index, Some(1));
        assert_eq!(group.radios.len(), 2);
        assert!(group.radios[1].disabled);
    }
}
