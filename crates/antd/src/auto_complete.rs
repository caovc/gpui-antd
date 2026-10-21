//! Native free-text AutoComplete with GPUI Kit editor, typed suggestions,
//! pointer selection and controlled keyboard highlight.
//!
//! The input is NOT restricted to the suggestion values (unlike Select).
//! The suggestion list currently uses an inline GPUI surface but does not
//! yet have Kit popup collision/dismissal handling or full ARIA combobox roles.

use crate::{AntInput, AntTheme, ControlSize};
use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::component::input::InputState;
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, ElementId, Entity, Focusable as _, InteractiveElement as _, IntoElement, KeyDownEvent,
    ParentElement as _, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _,
    Window, div, px,
};
use std::rc::Rc;

type Choose = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type ActiveChange = Rc<dyn Fn(usize, &mut Window, &mut App)>;
type OpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// Filter suggestions without changing their provided order. String
/// matching is Unicode-case-insensitive and permits free-form user entries.
pub fn autocomplete_matches(
    options: &[SharedString],
    query: &str,
    limit: usize,
) -> Vec<SharedString> {
    let query = query.trim().to_lowercase();
    options
        .iter()
        .filter(|value| value.to_lowercase().contains(&query))
        .take(limit)
        .cloned()
        .collect()
}

pub fn next_autocomplete_index(current: usize, count: usize, key: &str) -> Option<usize> {
    if count == 0 {
        return None;
    }
    match key {
        "down" => Some((current + 1) % count),
        "up" => Some((current + count - 1) % count),
        "home" => Some(0),
        "end" => Some(count - 1),
        _ => None,
    }
}

#[derive(IntoElement)]
pub struct AntAutoComplete {
    id: ElementId,
    input: Entity<InputState>,
    options: Vec<SharedString>,
    open: bool,
    active_index: usize,
    max_options: usize,
    size: ControlSize,
    width: f32,
    disabled: bool,
    clearable: bool,
    theme: AntTheme,
    on_select: Option<Choose>,
    on_active_change: Option<ActiveChange>,
    on_open_change: Option<OpenChange>,
}
impl AntAutoComplete {
    pub fn new(id: impl Into<ElementId>, input: &Entity<InputState>) -> Self {
        Self {
            id: id.into(),
            input: input.clone(),
            options: Vec::new(),
            open: false,
            active_index: 0,
            max_options: 8,
            size: ControlSize::Middle,
            width: 300.,
            disabled: false,
            clearable: true,
            theme: AntTheme::default(),
            on_select: None,
            on_active_change: None,
            on_open_change: None,
        }
    }
    pub fn options<I, S>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SharedString>,
    {
        self.options = options.into_iter().map(Into::into).collect();
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }
    pub fn max_options(mut self, max: usize) -> Self {
        self.max_options = max.clamp(1, 50);
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(120.);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(f));
        self
    }
    pub fn on_active_change(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_active_change = Some(Rc::new(f));
        self
    }
    pub fn on_open_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntAutoComplete {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = self.theme;
        let query = self.input.read(cx).value().to_string();
        let suggestions = autocomplete_matches(&self.options, &query, self.max_options);
        let open = self.open && !self.disabled && !suggestions.is_empty();
        let active = self.active_index.min(suggestions.len().saturating_sub(1));
        let key_choices = suggestions.clone();
        let chosen = self.on_select.clone();
        let active_change = self.on_active_change.clone();
        let open_change = self.on_open_change.clone();
        let input_focus = self.input.read(cx).focus_handle(cx);
        let requested_open = self.open;
        let disabled = self.disabled;
        let on_keyboard = move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
            if disabled
                || !input_focus.is_focused(window)
                || event.keystroke.modifiers.modified()
                || event.keystroke.is_ime_in_progress()
            {
                return;
            }
            let key = event.keystroke.key.as_str();
            if open {
                if let Some(next) = next_autocomplete_index(active, key_choices.len(), key) {
                    if next != active {
                        if let Some(callback) = &active_change {
                            callback(next, window, cx);
                        }
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                    return;
                }
                if key == "enter" {
                    if let (Some(value), Some(callback)) = (key_choices.get(active), &chosen) {
                        callback(value.clone(), window, cx);
                    }
                    if let Some(callback) = &open_change {
                        callback(false, window, cx);
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                    return;
                }
            }
            if key == "escape" && requested_open {
                if let Some(callback) = &open_change {
                    callback(false, window, cx);
                }
                window.prevent_default();
                cx.stop_propagation();
            }
        };
        let prefix = format!("{:?}", self.id);
        div()
            .id(self.id)
            .w(px(self.width))
            .flex()
            .flex_col()
            .gap(px(4.))
            .on_key_down(on_keyboard)
            .child(
                AntInput::new(&self.input)
                    .theme(t)
                    .size(self.size)
                    .disabled(self.disabled)
                    .clearable(self.clearable)
                    .aria_label("Autocomplete free-text input"),
            )
            .when(open, |this| {
                this.child(
                    div()
                        .w_full()
                        .max_h(px(256.))
                        .overflow_y_scrollbar()
                        .rounded(px(t.border_radius))
                        .border_1()
                        .border_color(t.border)
                        .bg(t.surface)
                        .p(px(4.))
                        .children(suggestions.into_iter().enumerate().map(|(index, value)| {
                            let is_active = index == active;
                            let click = self.on_select.clone();
                            let close = self.on_open_change.clone();
                            let value_for_click = value.clone();
                            BaseButton::new(format!("{prefix}-suggest-{index}"))
                                .tab_stop(false)
                                .accessibility_label(value.clone())
                                .w_full()
                                .h(px(34.))
                                .px(px(10.))
                                .flex()
                                .items_center()
                                .justify_start()
                                .rounded(px(t.border_radius))
                                .bg(if is_active { t.info_bg } else { t.surface })
                                .text_color(if is_active { t.primary } else { t.text })
                                .text_size(px(14.))
                                .cursor_pointer()
                                .hover(|this| this.bg(t.info_bg).text_color(t.primary))
                                .child(value)
                                .on_click(move |_, window, cx| {
                                    if let Some(callback) = &click {
                                        callback(value_for_click.clone(), window, cx);
                                    }
                                    if let Some(callback) = &close {
                                        callback(false, window, cx);
                                    }
                                })
                        })),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn free_text_suggestions_are_filtered_and_ordered() {
        let list = vec![
            SharedString::from("Rust"),
            SharedString::from("Ruby"),
            SharedString::from("Python"),
            SharedString::from("Java"),
        ];
        assert_eq!(
            autocomplete_matches(&list, " RU ", 10),
            vec![SharedString::from("Rust"), SharedString::from("Ruby")]
        );
        assert!(autocomplete_matches(&list, "nonexistent", 10).is_empty());
        assert_eq!(autocomplete_matches(&list, "", 2).len(), 2);
    }
    #[test]
    fn suggestion_keyboard_navigation_wraps() {
        assert_eq!(next_autocomplete_index(1, 3, "down"), Some(2));
        assert_eq!(next_autocomplete_index(2, 3, "down"), Some(0));
        assert_eq!(next_autocomplete_index(0, 3, "up"), Some(2));
        assert_eq!(next_autocomplete_index(0, 0, "down"), None);
        assert_eq!(next_autocomplete_index(0, 3, "escape"), None);
    }
}
