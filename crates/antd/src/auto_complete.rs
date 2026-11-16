//! Native free-text AutoComplete with GPUI Kit editor, typed suggestions,
//! pointer selection and controlled keyboard highlight.
//!
//! The input is NOT restricted to the suggestion values (unlike Select).
//! Suggestions use a measured, deferred GPUI Base Popup. Text, active index
//! and open state remain parent-owned; the editor keeps typing focus. Native
//! corner placement clamps at window edges; automatic flipping and complete
//! screen-reader relationships are not claimed.

use crate::overlays::{OverlayPlacement, overlay_anchor};
use crate::{AntInput, AntTheme, ControlSize};
use gpui_kit::base::{
    Button as BaseButton, DeferredPopover, GlobalState, Popup, Select as ComboboxRoot,
    TestSupportExt as _,
};
use gpui_kit::component::input::InputState;
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, Bounds, ElementId, Entity, EntityInputHandler as _, Focusable as _,
    InteractiveElement as _, IntoElement, KeystrokeEvent, MouseButton, ParentElement as _, Pixels,
    RenderOnce, Role, ScrollHandle, SharedString, StatefulInteractiveElement as _, Styled as _,
    Subscription, Window, div, px,
};
use std::{cell::Cell, rc::Rc};

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
        "down" => Some(if current.min(count - 1) == count - 1 {
            0
        } else {
            current + 1
        }),
        "up" => Some(if current.min(count - 1) == 0 {
            count - 1
        } else {
            current.min(count - 1) - 1
        }),
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
    label: SharedString,
    placement: OverlayPlacement,
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
            label: "Autocomplete".into(),
            placement: OverlayPlacement::BottomLeft,
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
    /// Name exposed by the native combobox and its editor.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Preferred popup corner. Kit clamps the popup inside the window.
    pub fn placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
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
type Close = Rc<dyn Fn(&mut Window, &mut App)>;
type Keyboard = Rc<dyn Fn(&KeystrokeEvent, &mut Window, &mut App)>;

/// Presentation-only lifetime: a focus subscription, scroll position, and the
/// current frame's dismissal callback. No text/open/selection model lives here.
struct Presentation {
    close: Option<Close>,
    keyboard: Option<Keyboard>,
    _keystrokes: Subscription,
    scroll: ScrollHandle,
    revealed: Option<(usize, Vec<SharedString>, Pixels)>,
    deferred: Option<DeferredPopover>,
    editor_focused: bool,
    _focus_out: Subscription,
}

fn request_close(state: &Entity<Presentation>, window: &mut Window, cx: &mut App) {
    // Mouse-out, Tab and focus-out can describe the same gesture. Take before
    // calling the owner, and refresh on render even when it rejects the request.
    let callback = state.update(cx, |state, cx| {
        cx.notify();
        state.close.take()
    });
    if let Some(callback) = callback {
        callback(window, cx);
    }
}

/// A click or arrow can open before the next frame refreshes `close`. Arm it
/// now so a programmatic blur in the same turn still requests dismissal.
fn arm_close(state: &Entity<Presentation>, callback: &OpenChange, cx: &mut App) {
    state.update(cx, |state, _| {
        state.editor_focused = true;
        if state.close.is_none() {
            let callback = callback.clone();
            state.close = Some(Rc::new(move |window, cx| callback(false, window, cx)));
        }
    });
}

impl RenderOnce for AntAutoComplete {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = self.theme;
        let query = self.input.read(cx).value().to_string();
        let suggestions = autocomplete_matches(&self.options, &query, self.max_options);
        let can_suggest = !self.disabled && !suggestions.is_empty();
        let open = self.open && can_suggest;
        let active = self.active_index.min(suggestions.len().saturating_sub(1));
        let input_focus = self.input.read(cx).focus_handle(cx);
        let prefix = format!("{:?}", self.id);
        // Base Popup keeps an 8px window inset. Bound oversized content to that
        // viewport too; corner positioning owns snapping, not content resizing.
        let panel_width = px(self.width).min((window.viewport_size().width - px(16.)).max(px(1.)));
        let panel_height = px(256.).min((window.viewport_size().height - px(16.)).max(px(1.)));
        let observer_focus = input_focus.clone();
        let presentation = window.use_keyed_state(
            SharedString::from(format!(
                "autocomplete-state-{prefix}-{}",
                self.input.entity_id()
            )),
            cx,
            move |window, cx| {
                let weak = cx.entity().downgrade();
                let subscription =
                    window.on_focus_out(&observer_focus, cx, move |_, window, cx| {
                        if let Some(state) = weak.upgrade() {
                            request_close(&state, window, cx);
                        }
                    });
                let weak = cx.entity().downgrade();
                let keystrokes = cx.intercept_keystrokes(move |event, window, cx| {
                    if let Some(state) = weak.upgrade() {
                        let callback = state.read(cx).keyboard.clone();
                        if let Some(callback) = callback {
                            callback(event, window, cx);
                        }
                    }
                });
                Presentation {
                    close: None,
                    keyboard: None,
                    _keystrokes: keystrokes,
                    scroll: ScrollHandle::new(),
                    revealed: None,
                    deferred: None,
                    editor_focused: false,
                    _focus_out: subscription,
                }
            },
        );
        let close = self.on_open_change.clone().map(|callback| {
            Rc::new(move |window: &mut Window, cx: &mut App| callback(false, window, cx)) as Close
        });
        let scroll = presentation.update(cx, |state, cx| {
            if open && state.deferred.is_none() {
                state.deferred = Some(GlobalState::register_deferred_popover(cx));
            } else if !open {
                state.deferred = None;
            }
            state.close = if self.open && !self.disabled {
                close
            } else {
                None
            };
            let signature = open.then(|| (active, suggestions.clone(), panel_height));
            if signature != state.revealed {
                if open {
                    state.scroll.scroll_to_item(active);
                }
                state.revealed = signature;
            }
            state.scroll.clone()
        });
        let focused = !self.disabled && input_focus.is_focused(window);
        let lost_focus = presentation.update(cx, |state, _| {
            let lost = state.editor_focused && !focused && self.open && !self.disabled;
            state.editor_focused = focused;
            lost
        });
        if lost_focus {
            request_close(&presentation, window, cx);
        }
        let keyboard_close = presentation.downgrade();
        let key_choices = suggestions.clone();
        let chosen = self.on_select.clone();
        let active_change = self.on_active_change.clone();
        let open_change = self.on_open_change.clone();
        let keyboard_focus = input_focus.clone();
        let keyboard_input = self.input.clone();
        let requested_open = self.open;
        let disabled = self.disabled;
        let on_keyboard = move |event: &KeystrokeEvent, window: &mut Window, cx: &mut App| {
            if disabled
                || !keyboard_focus.is_focused(window)
                || (event.keystroke.is_ime_in_progress()
                    && !matches!(event.keystroke.key.as_str(), "enter" | "tab"))
            {
                return;
            }
            if keyboard_input.update(cx, |input, cx| {
                input.marked_text_range(window, cx).is_some()
            }) {
                return; // The editor owns confirmation/navigation during IME composition.
            }
            let Some(keyboard_close) = keyboard_close.upgrade() else {
                return;
            };
            let key = event.keystroke.key.as_str();
            if key == "tab" {
                if requested_open {
                    request_close(&keyboard_close, window, cx);
                }
                return; // Keep native Tab and Shift-Tab traversal.
            }
            if event.keystroke.modifiers.modified() {
                return;
            }
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
                    request_close(&keyboard_close, window, cx);
                    window.prevent_default();
                    cx.stop_propagation();
                    return;
                }
            } else if can_suggest && matches!(key, "down" | "up") {
                if let Some(callback) = &open_change {
                    arm_close(&keyboard_close, callback, cx);
                    callback(true, window, cx);
                }
                window.prevent_default();
                cx.stop_propagation();
                return;
            }
            if key == "escape" && requested_open {
                request_close(&keyboard_close, window, cx);
                window.prevent_default();
                cx.stop_propagation();
            }
        };
        presentation.update(cx, |state, _| state.keyboard = Some(Rc::new(on_keyboard)));
        let trigger_bounds = Rc::new(Cell::new(None::<Bounds<Pixels>>));
        let measured_bounds = trigger_bounds.clone();
        let outside_close = presentation.clone();
        let list = div()
            .id(SharedString::from(format!("{prefix}-suggestions")))
            .test_support()
            .role(Role::ListBox)
            .aria_label(SharedString::from(format!("{} suggestions", self.label)))
            .w(panel_width)
            .max_h(panel_height)
            .overflow_y_scroll()
            .track_scroll(&scroll)
            .rounded(px(t.border_radius))
            .border_1()
            .border_color(t.border)
            .bg(t.bg_elevated)
            .p(px(4.))
            .on_mouse_down_out(move |event, window, cx| {
                // A caret/clear-button click belongs to the editor, not to a
                // popup trigger toggle. Kit supplies the actual trigger bounds.
                if !trigger_bounds
                    .get()
                    .is_some_and(|bounds| bounds.contains(&event.position))
                {
                    request_close(&outside_close, window, cx);
                }
            })
            .children(suggestions.into_iter().enumerate().map(|(index, value)| {
                let click = self.on_select.clone();
                let close = presentation.clone();
                let focus = input_focus.clone();
                let selected = query.as_str() == value.as_ref();
                let value_for_click = value.clone();
                BaseButton::new(format!("{prefix}-suggest-{index}"))
                    .focusable(false)
                    .tab_stop(false)
                    .role(Role::ListBoxOption)
                    .aria_selected(selected)
                    .when(index == active, |row| row.aria_active_descendant())
                    .accessibility_label(value.clone())
                    .w_full()
                    .h(px(t.control_height.max(24.)))
                    .px(px(t.padding * 0.75))
                    .flex()
                    .items_center()
                    .justify_start()
                    .rounded(px(t.border_radius))
                    .bg(if index == active {
                        t.info_bg
                    } else {
                        t.bg_elevated
                    })
                    .text_color(if index == active { t.primary } else { t.text })
                    .text_size(px(t.font_size))
                    .cursor_pointer()
                    .hover(|row| row.bg(t.info_bg).text_color(t.primary))
                    .child(value)
                    .on_click(move |_, window, cx| {
                        focus.focus(window, cx);
                        if let Some(callback) = &click {
                            callback(value_for_click.clone(), window, cx);
                        }
                        request_close(&close, window, cx);
                    })
            }))
            .vertical_scrollbar(&scroll);
        let click_open = self.on_open_change.clone();
        let click_close = presentation.clone();
        let click_focus = input_focus.clone();
        let input = div()
            .w_full()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                click_focus.focus(window, cx);
                if can_suggest && !requested_open {
                    if let Some(callback) = &click_open {
                        arm_close(&click_close, callback, cx);
                        callback(true, window, cx);
                    }
                }
            })
            .child(
                AntInput::new(&self.input)
                    .theme(t)
                    .size(self.size)
                    .disabled(self.disabled)
                    .clearable(self.clearable)
                    .aria_label(self.label.clone()),
            );
        let semantic_close = presentation.clone();
        // Base Select is also Base Combobox's semantic foundation. The editor
        // owns the only focus handle/Tab stop. Guarded raw keys above preserve
        // IME and modified editor commands rather than binding select actions.
        ComboboxRoot::new(self.id)
            .key_context("AntAutoComplete")
            .accessibility_label(self.label)
            .accessibility_value(query)
            .open(open)
            .disabled(self.disabled)
            .w(px(self.width))
            .on_open_change(move |next, window, cx| {
                if next && can_suggest {
                    if let Some(callback) = &self.on_open_change {
                        arm_close(&semantic_close, callback, cx);
                        callback(true, window, cx);
                    }
                    input_focus.focus(window, cx);
                } else if !next {
                    request_close(&semantic_close, window, cx);
                }
            })
            .child(
                Popup::new(
                    SharedString::from(format!("autocomplete-popup-{prefix}")),
                    input,
                )
                .w_full()
                .anchor(overlay_anchor(self.placement))
                .offset(px(4.))
                .on_position(move |_, bounds| measured_bounds.set(Some(bounds)))
                .when(open, |popup| popup.content(list)),
            )
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
