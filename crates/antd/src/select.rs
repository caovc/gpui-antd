//! Ant Design Select built on GPUI Kit **base**, not `component::select::Select`.
//!
//! # Architecture
//! - [`gpui_kit::base::Popover`] (`BasePopover`) owns placement, Escape, outside
//!   dismissal, focus capture/restoration and deferred overlay registration —
//!   same pattern as [`crate::menu::AntMenu`] flyouts. No absolute fake popup.
//! - Trigger and option list chrome are owned here so hover borders, elevated
//!   popup surfaces, selected checks and the Ant Design down arrow are real.
//! - **`gpui_kit::base::Select` is not nested.** That root needs external
//!   `FocusHandle`s for trigger/content and its own open/confirm/dismiss path;
//!   nesting it with BasePopover double-owns focus and Escape. Instead the
//!   trigger uses `Role::ComboBox` + `aria_expanded`, and the popup body uses
//!   `Role::ListBox` / `Role::ListBoxOption`, matching Menu's explicit roles.
//!
//! # Controlled state
//! Open, selected value and (optional) search query are **parent-owned**:
//! ```ignore
//! AntSelect::new("lang")
//!     .options(["Rust", "Go"])
//!     .selected(Some("Rust".into()))
//!     .open(open)
//!     .on_open_change(|open, _, cx| entity.update(cx, |s, cx| { s.open = open; cx.notify(); }))
//!     .on_select(|value, _, cx| { /* commit */ })
//! ```
//! `.open` defaults to `false`. Without `on_open_change` the popover cannot stay
//! open (controlled sync resets it). Highlight index while open is element-local
//! keyed state (no `cx.new` in render).
//!
//! # Searchable
//! Parent-controlled `.query` + `on_query_change`. No `InputState` entity is
//! created in render. When searchable and open, character keys / Backspace
//! update the query; filtering uses [`select_matches`].
//!
//! # Remaining gaps
//! - Multiple / tags mode (single-select only).
//! - Option groups / disabled options.
//! - Virtualised long lists.
//! - Typeahead without parent-owned query when `searchable` is false.

use std::rc::Rc;

use gpui_kit::{
    Anchor, App, ElementId, Hsla, InteractiveElement as _, IntoElement, KeyDownEvent,
    MouseButton, ParentElement as _, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, Styled as _, Transformation, Window, div, px,
    percentage,
};
use gpui_kit::base::{Popover as BasePopover, StyledExt as _};
use gpui_kit::component::{Icon as KitIcon, Size, Sizable as _};
use gpui_kit::prelude::FluentBuilder as _;

use crate::ControlSize;
use crate::design_tokens::AntComponent;
use crate::icons::AntIconName;
use crate::input::{InputStatus, InputVariant};
use crate::tokens::{AntTheme, color};

type SelectHandler = Rc<dyn Fn(Option<SharedString>, &mut Window, &mut App)>;
type OpenHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type QueryHandler = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;

/// Resolved trigger chrome. Pure data — no GPUI context.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectChrome {
    pub height: f32,
    pub font_size: f32,
    pub background: Hsla,
    pub border: Hsla,
    pub text: Hsla,
    pub radius: f32,
    pub framed: bool,
    /// Hover border when interactive (applied on our hoverable trigger).
    pub hover_border: Option<Hsla>,
}

/// Compute Ant Design Select trigger colors/geometry from theme + flags.
pub fn select_chrome(
    theme: &AntTheme,
    size: ControlSize,
    variant: InputVariant,
    status: InputStatus,
    disabled: bool,
) -> SelectChrome {
    let comp = theme.component(AntComponent::Select);

    let height = comp.control_height.unwrap_or(match size {
        ControlSize::Small => theme.control_height_sm,
        ControlSize::Middle => theme.control_height,
        ControlSize::Large => theme.control_height_lg,
    });

    let font_size = comp.font_size.unwrap_or(match size {
        ControlSize::Large => 16.,
        _ => theme.font_size,
    });

    let background = if disabled {
        theme.disabled_bg
    } else if variant == InputVariant::Filled {
        theme.surface_secondary
    } else if variant == InputVariant::Borderless {
        color(0).opacity(0.)
    } else {
        theme.surface
    };

    let border = match status {
        InputStatus::Error => theme.error,
        InputStatus::Warning => theme.warning,
        InputStatus::Normal => {
            if variant == InputVariant::Outlined {
                theme.border
            } else {
                color(0).opacity(0.)
            }
        }
    };

    let text = if disabled {
        theme.disabled_text
    } else {
        theme.text
    };

    let hover_border = if disabled {
        None
    } else {
        Some(match status {
            InputStatus::Error => theme.error,
            InputStatus::Warning => theme.warning,
            InputStatus::Normal => theme.primary,
        })
    };

    SelectChrome {
        height,
        font_size,
        background: comp.color_bg.unwrap_or(background),
        border: comp.color_border.unwrap_or(border),
        text: comp.color_text.unwrap_or(text),
        radius: comp.border_radius.unwrap_or(theme.border_radius),
        framed: variant.framed(),
        hover_border,
    }
}

/// Case-insensitive substring filter. Empty/whitespace query keeps every index.
pub fn select_matches(options: &[SharedString], query: &str) -> Vec<usize> {
    let needle = query.trim().to_lowercase();
    if needle.is_empty() {
        return (0..options.len()).collect();
    }
    options
        .iter()
        .enumerate()
        .filter(|(_, option)| option.to_lowercase().contains(&needle))
        .map(|(index, _)| index)
        .collect()
}

/// Keyboard movement over a flat option list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectMove {
    Previous,
    Next,
    First,
    Last,
}

/// Map a key name (`up`/`down`/`home`/`end`) to a list move.
pub fn select_move_for_key(key: &str) -> Option<SelectMove> {
    match key {
        "up" => Some(SelectMove::Previous),
        "down" => Some(SelectMove::Next),
        "home" => Some(SelectMove::First),
        "end" => Some(SelectMove::Last),
        _ => None,
    }
}

/// Advance highlight within `len` filtered rows. Empty list yields `None`.
pub fn move_select_highlight(
    current: Option<usize>,
    len: usize,
    movement: SelectMove,
) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let last = len - 1;
    Some(match movement {
        SelectMove::First => 0,
        SelectMove::Last => last,
        SelectMove::Next => current.map(|index| (index + 1) % len).unwrap_or(0),
        SelectMove::Previous => current
            .map(|index| if index == 0 { last } else { index - 1 })
            .unwrap_or(last),
    })
}

/// Element-local highlight while the popup is open (not application model state).
#[derive(Default)]
struct SelectHighlight {
    /// Index into the **filtered** option list.
    index: Option<usize>,
    /// Last open flag seen, so a fresh open can seed from the selection.
    was_open: bool,
}

/// Parent-owned single-value Select with Ant Design trigger + popup chrome.
#[derive(IntoElement)]
pub struct AntSelect {
    id: ElementId,
    theme: AntTheme,
    size: ControlSize,
    variant: InputVariant,
    status: InputStatus,
    disabled: bool,
    clearable: bool,
    searchable: bool,
    open: bool,
    width: Option<f32>,
    menu_width: Option<f32>,
    placeholder: Option<SharedString>,
    accessibility_label: Option<SharedString>,
    options: Vec<SharedString>,
    selected: Option<SharedString>,
    query: SharedString,
    on_select: Option<SelectHandler>,
    on_open_change: Option<OpenHandler>,
    on_query_change: Option<QueryHandler>,
}

impl AntSelect {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            theme: AntTheme::default(),
            size: ControlSize::Middle,
            variant: InputVariant::Outlined,
            status: InputStatus::Normal,
            disabled: false,
            clearable: false,
            searchable: false,
            open: false,
            width: None,
            menu_width: None,
            placeholder: None,
            accessibility_label: None,
            options: Vec::new(),
            selected: None,
            query: SharedString::default(),
            on_select: None,
            on_open_change: None,
            on_query_change: None,
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
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }
    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    /// Controlled open flag (default `false`). Parent must handle
    /// [`Self::on_open_change`] to keep the popup open.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(0.));
        self
    }
    pub fn menu_width(mut self, width: f32) -> Self {
        self.menu_width = Some(width.max(0.));
        self
    }
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    pub fn aria_label(mut self, text: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(text.into());
        self
    }

    pub fn options(mut self, options: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.options = options.into_iter().map(Into::into).collect();
        self
    }

    /// Committed value. `None` shows the placeholder.
    pub fn selected(mut self, selected: Option<SharedString>) -> Self {
        self.selected = selected;
        self
    }

    /// Parent-owned filter string when [`Self::searchable`] is true.
    pub fn query(mut self, query: impl Into<SharedString>) -> Self {
        self.query = query.into();
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(Option<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    pub fn on_query_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_query_change = Some(Rc::new(handler));
        self
    }

    /// Back-compat helper: `true` → borderless variant.
    pub fn borderless(mut self, borderless: bool) -> Self {
        if borderless {
            self.variant = InputVariant::Borderless;
        } else if self.variant == InputVariant::Borderless {
            self.variant = InputVariant::Outlined;
        }
        self
    }
}

impl RenderOnce for AntSelect {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let chrome = select_chrome(
            &theme,
            self.size,
            self.variant,
            self.status,
            self.disabled,
        );
        let disabled = self.disabled;
        let clearable = self.clearable;
        let searchable = self.searchable;
        let open = self.open && !disabled;
        let options = self.options;
        let selected = self.selected;
        let query = self.query;
        let on_select = self.on_select;
        let on_open = self.on_open_change;
        let on_query = self.on_query_change;
        let placeholder = self
            .placeholder
            .unwrap_or_else(|| SharedString::from("Select"));
        let aria = self.accessibility_label;
        let menu_width = self.menu_width.unwrap_or(220.);
        let width = self.width;
        let id_debug = format!("{:?}", self.id);
        let trigger_id = format!("ant-select-trigger-{id_debug}");
        let popover_id = self.id;
        let highlight_id = format!("ant-select-hl-{id_debug}");

        let filtered = if searchable {
            select_matches(&options, query.as_ref())
        } else {
            (0..options.len()).collect::<Vec<_>>()
        };

        // Seed / clamp highlight when the popup opens or the filter shrinks.
        let highlight_state =
            window.use_keyed_state(highlight_id, cx, |_, _| SelectHighlight::default());
        let selected_filtered = selected.as_ref().and_then(|value| {
            filtered
                .iter()
                .position(|&option_index| options.get(option_index).is_some_and(|o| o == value))
        });
        highlight_state.update(cx, |state, _| {
            if open {
                if !state.was_open {
                    state.index = selected_filtered.or(if filtered.is_empty() {
                        None
                    } else {
                        Some(0)
                    });
                } else if let Some(index) = state.index {
                    if filtered.is_empty() {
                        state.index = None;
                    } else if index >= filtered.len() {
                        state.index = Some(filtered.len() - 1);
                    }
                } else if !filtered.is_empty() {
                    state.index = Some(0);
                }
            } else {
                state.index = None;
            }
            state.was_open = open;
        });
        let highlight = highlight_state.read(cx).index;

        let value_label = selected
            .clone()
            .unwrap_or_else(|| placeholder.clone());
        let showing_placeholder = selected.is_none();
        let value_color = if disabled {
            theme.disabled_text
        } else if showing_placeholder {
            theme.text_secondary
        } else {
            chrome.text
        };

        let open_for_trigger = open;
        let on_open_trigger = on_open.clone();
        let trigger = {
            let clear_selected = selected.clone();
            let on_clear = on_select.clone();
            let hover_border = chrome.hover_border;
            let border = chrome.border;
            let bg = chrome.background;
            let framed = chrome.framed;

            div()
                .id(trigger_id)
                .role(Role::ComboBox)
                .aria_expanded(open_for_trigger)
                .when_some(aria, |this, label| this.aria_label(label))
                .when_some(selected.clone(), |this, value| this.aria_value(value))
                .when_some(width, |this, w| this.w(px(w)))
                .when(width.is_none(), |this| this.w_full())
                .h(px(chrome.height))
                .px(px(11.))
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.))
                .text_size(px(chrome.font_size))
                .when(framed, |this| {
                    this.border_1()
                        .border_color(border)
                        .rounded(px(chrome.radius))
                        .bg(bg)
                })
                .when(!framed, |this| this.bg(bg))
                .when(!disabled, |this| this.cursor_pointer())
                .when(!disabled && framed, |this| {
                    this.when_some(hover_border, |this, hover| {
                        this.hover(|style| style.border_color(hover))
                    })
                })
                .when(disabled, |this| this.cursor_not_allowed())
                .child(
                    div()
                        .id("ant-select-value")
                        .flex_1()
                        .min_w_0()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .text_color(value_color)
                        .child(value_label),
                )
                .child(
                    div()
                        .id("ant-select-suffix")
                        .flex()
                        .items_center()
                        .gap(px(4.))
                        .flex_shrink_0()
                        .when(clearable && selected.is_some() && !disabled, |this| {
                            let on_clear = on_clear.clone();
                            this.child(
                                div()
                                    .id("ant-select-clear")
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation();
                                    })
                                    .on_click(move |_, window, cx| {
                                        cx.stop_propagation();
                                        if let Some(handler) = on_clear.as_ref() {
                                            handler(None, window, cx);
                                        }
                                    })
                                    .child(
                                        KitIcon::default()
                                            .path(AntIconName::CloseCircleFilled.path())
                                            .with_size(Size::Size(px(12.)))
                                            .text_color(theme.text_secondary),
                                    ),
                            )
                        })
                        .child({
                            let arrow = KitIcon::default()
                                .path(AntIconName::DownOutlined.path())
                                .with_size(Size::Size(px(12.)))
                                .text_color(if disabled {
                                    theme.disabled_text
                                } else {
                                    theme.text_secondary
                                });
                            if open_for_trigger {
                                // 180° when open (Ant Design expand affordance).
                                arrow
                                    .transform(Transformation::rotate(percentage(0.5)))
                                    .into_any_element()
                            } else {
                                arrow.into_any_element()
                            }
                        }),
                )
                // Arrow-down / Enter on a closed trigger request open (popover
                // already toggles on pointer + space/enter via its Confirm).
                .when(!disabled, |this| {
                    let on_open = on_open_trigger.clone();
                    this.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.modifiers.modified()
                            || event.keystroke.is_ime_in_progress()
                        {
                            return;
                        }
                        let key = event.keystroke.key.as_str();
                        if !open_for_trigger && (key == "down" || key == "up") {
                            if let Some(handler) = on_open.as_ref() {
                                handler(true, window, cx);
                            }
                            window.prevent_default();
                            cx.stop_propagation();
                        }
                    })
                })
                .into_any_element()
        };

        let filtered_for_content = filtered.clone();
        let options_for_content = options.clone();
        let selected_for_content = selected.clone();
        let on_select_content = on_select.clone();
        let on_open_content = on_open.clone();
        let on_query_content = on_query.clone();
        let query_for_content = query.clone();
        let highlight_entity = highlight_state.clone();
        let highlight_now = highlight;
        let parent_view = window.current_view();

        BasePopover::new(popover_id)
            .anchor(Anchor::BottomLeft)
            .offset(px(4.))
            .open(open)
            .overlay_closable(true)
            .on_open_change(move |is_open, window, cx| {
                if disabled {
                    return;
                }
                if let Some(handler) = on_open.as_ref() {
                    handler(*is_open, window, cx);
                }
            })
            .trigger_with(move |_, _, _| trigger)
            .content(move |_, _, _| {
                // Snapshot from the parent render — do not capture `&mut App`.
                let filtered = filtered_for_content;
                let options = options_for_content;
                let highlight = highlight_now;
                let on_select = on_select_content;
                let on_open = on_open_content;
                let on_query = on_query_content;
                let query = query_for_content;
                let selected = selected_for_content;
                let filtered_for_keys = filtered.clone();
                let options_for_keys = options.clone();

                div()
                    .id("ant-select-popup")
                    .role(Role::ListBox)
                    .min_w(px(menu_width))
                    .max_h(px(256.))
                    .overflow_y_scroll()
                    .py(px(4.))
                    .bg(theme.bg_elevated)
                    .border_1()
                    .border_color(theme.border)
                    .rounded(px(8.))
                    .shadow_lg()
                    .when(searchable, |this| {
                        this.child(
                            div()
                                .id("ant-select-query")
                                .px(px(12.))
                                .py(px(6.))
                                .text_size(px(12.))
                                .text_color(theme.text_secondary)
                                .child(if query.is_empty() {
                                    SharedString::from("Type to filter…")
                                } else {
                                    query.clone()
                                }),
                        )
                    })
                    .on_key_down({
                        let on_select = on_select.clone();
                        let on_open = on_open.clone();
                        let on_query = on_query.clone();
                        let query = query.clone();
                        let highlight_entity = highlight_entity.clone();
                        move |event: &KeyDownEvent, window, cx| {
                            if event.keystroke.modifiers.modified()
                                || event.keystroke.is_ime_in_progress()
                            {
                                return;
                            }
                            let key = event.keystroke.key.as_str();
                            let len = filtered_for_keys.len();

                            if let Some(movement) = select_move_for_key(key) {
                                let current = highlight_entity.read(cx).index;
                                let next = move_select_highlight(current, len, movement);
                                highlight_entity.update(cx, |state, cx| {
                                    state.index = next;
                                    cx.notify();
                                });
                                cx.notify(parent_view);
                                window.prevent_default();
                                cx.stop_propagation();
                                return;
                            }

                            if key == "enter" {
                                let current = highlight_entity.read(cx).index;
                                if let Some(filtered_index) = current {
                                    if let Some(&option_index) =
                                        filtered_for_keys.get(filtered_index)
                                    {
                                        if let Some(value) = options_for_keys.get(option_index) {
                                            if let Some(handler) = on_select.as_ref() {
                                                handler(Some(value.clone()), window, cx);
                                            }
                                        }
                                    }
                                }
                                if let Some(handler) = on_open.as_ref() {
                                    handler(false, window, cx);
                                }
                                window.prevent_default();
                                cx.stop_propagation();
                                return;
                            }

                            if searchable {
                                if key == "backspace" {
                                    let mut next = query.to_string();
                                    next.pop();
                                    if let Some(handler) = on_query.as_ref() {
                                        handler(next.into(), window, cx);
                                    }
                                    window.prevent_default();
                                    cx.stop_propagation();
                                    return;
                                }
                                // Printable single-character typeahead into the
                                // parent-owned query (no InputState entity).
                                if key.len() == 1
                                    && !event.keystroke.modifiers.modified()
                                    && !key.chars().next().is_some_and(|ch| ch.is_control())
                                {
                                    let mut next = query.to_string();
                                    next.push_str(key);
                                    if let Some(handler) = on_query.as_ref() {
                                        handler(next.into(), window, cx);
                                    }
                                    window.prevent_default();
                                    cx.stop_propagation();
                                }
                            }
                        }
                    })
                    .when(filtered.is_empty(), |this| {
                        this.child(
                            div()
                                .id("ant-select-empty")
                                .h(px(32.))
                                .px(px(12.))
                                .flex()
                                .items_center()
                                .text_size(px(13.))
                                .text_color(theme.text_secondary)
                                .child("No data"),
                        )
                    })
                    .children(filtered.into_iter().enumerate().map(
                        move |(filtered_index, option_index)| {
                            let value = options[option_index].clone();
                            let is_selected = selected.as_ref() == Some(&value);
                            let is_active = highlight == Some(filtered_index);
                            let row_id =
                                format!("ant-select-opt-{}-{}", id_debug, value.as_ref());
                            let on_select = on_select.clone();
                            let on_open = on_open.clone();
                            let commit = value.clone();

                            div()
                                .id(row_id)
                                .role(Role::ListBoxOption)
                                .aria_selected(is_selected)
                                .h(px(32.))
                                .px(px(12.))
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap(px(8.))
                                .cursor_pointer()
                                .text_size(px(theme.font_size))
                                .bg(if is_active {
                                    theme.surface_secondary
                                } else {
                                    color(0).opacity(0.)
                                })
                                .text_color(if is_selected {
                                    theme.primary
                                } else {
                                    theme.text
                                })
                                .hover(|style| style.bg(theme.surface_secondary))
                                .on_click(move |_, window, cx| {
                                    if let Some(handler) = on_select.as_ref() {
                                        handler(Some(commit.clone()), window, cx);
                                    }
                                    if let Some(handler) = on_open.as_ref() {
                                        handler(false, window, cx);
                                    }
                                })
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .overflow_hidden()
                                        .text_ellipsis()
                                        .child(value.clone()),
                                )
                                .when(is_selected, |this| {
                                    this.child(
                                        KitIcon::default()
                                            .path(AntIconName::CheckOutlined.path())
                                            .with_size(Size::Size(px(12.)))
                                            .text_color(theme.primary),
                                    )
                                })
                        },
                    ))
            })
            .when_some(width, |this, w| this.w(px(w)))
            .when(width.is_none(), |this| this.w_full())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outlined_middle_uses_control_height_and_surface() {
        let theme = AntTheme::light();
        let chrome = select_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Outlined,
            InputStatus::Normal,
            false,
        );
        assert_eq!(chrome.height, theme.control_height);
        assert_eq!(chrome.font_size, theme.font_size);
        assert_eq!(chrome.background, theme.surface);
        assert_eq!(chrome.border, theme.border);
        assert_eq!(chrome.text, theme.text);
        assert!(chrome.framed);
        assert_eq!(chrome.hover_border, Some(theme.primary));
    }

    #[test]
    fn size_ladder_is_24_32_40() {
        let theme = AntTheme::light();
        assert_eq!(
            select_chrome(
                &theme,
                ControlSize::Small,
                InputVariant::Outlined,
                InputStatus::Normal,
                false
            )
            .height,
            24.
        );
        assert_eq!(
            select_chrome(
                &theme,
                ControlSize::Middle,
                InputVariant::Outlined,
                InputStatus::Normal,
                false
            )
            .height,
            32.
        );
        assert_eq!(
            select_chrome(
                &theme,
                ControlSize::Large,
                InputVariant::Outlined,
                InputStatus::Normal,
                false
            )
            .height,
            40.
        );
        assert_eq!(
            select_chrome(
                &theme,
                ControlSize::Large,
                InputVariant::Outlined,
                InputStatus::Normal,
                false
            )
            .font_size,
            16.
        );
    }

    #[test]
    fn filled_and_borderless_chrome() {
        let theme = AntTheme::light();
        let filled = select_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Filled,
            InputStatus::Normal,
            false,
        );
        assert_eq!(filled.background, theme.surface_secondary);
        assert!(filled.framed);
        assert_eq!(filled.border.a, 0.);

        let borderless = select_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Borderless,
            InputStatus::Normal,
            false,
        );
        assert!(!borderless.framed);
        assert_eq!(borderless.background.a, 0.);
        assert_eq!(borderless.border.a, 0.);
    }

    #[test]
    fn status_and_disabled_override_border_and_hover() {
        let theme = AntTheme::light();
        let err = select_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Outlined,
            InputStatus::Error,
            false,
        );
        assert_eq!(err.border, theme.error);
        assert_eq!(err.hover_border, Some(theme.error));

        let warn = select_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Outlined,
            InputStatus::Warning,
            false,
        );
        assert_eq!(warn.border, theme.warning);
        assert_eq!(warn.hover_border, Some(theme.warning));

        let disabled = select_chrome(
            &theme,
            ControlSize::Middle,
            InputVariant::Outlined,
            InputStatus::Error,
            true,
        );
        assert_eq!(disabled.background, theme.disabled_bg);
        assert_eq!(disabled.text, theme.disabled_text);
        assert_eq!(disabled.border, theme.error);
        assert_eq!(disabled.hover_border, None);
    }

    #[test]
    fn component_tokens_override_defaults() {
        use crate::design_tokens::AntComponentToken;
        let theme = AntTheme::light().with_component(
            AntComponent::Select,
            AntComponentToken::default()
                .control_height(28.)
                .font_size(13.)
                .border_radius(2.)
                .color_bg(color(0x112233))
                .color_border(color(0x445566))
                .color_text(color(0x778899)),
        );
        let chrome = select_chrome(
            &theme,
            ControlSize::Large,
            InputVariant::Outlined,
            InputStatus::Normal,
            false,
        );
        assert_eq!(chrome.height, 28.);
        assert_eq!(chrome.font_size, 13.);
        assert_eq!(chrome.radius, 2.);
        assert_eq!(chrome.background, color(0x112233));
        assert_eq!(chrome.border, color(0x445566));
        assert_eq!(chrome.text, color(0x778899));
    }

    #[test]
    fn select_matches_filters_case_insensitively() {
        let options: Vec<SharedString> = ["Rust", "Go", "Ruby", "TypeScript"]
            .into_iter()
            .map(SharedString::from)
            .collect();
        assert_eq!(select_matches(&options, ""), vec![0, 1, 2, 3]);
        assert_eq!(select_matches(&options, "  "), vec![0, 1, 2, 3]);
        assert_eq!(select_matches(&options, "ru"), vec![0, 2]);
        assert_eq!(select_matches(&options, "SCRIPT"), vec![3]);
        assert!(select_matches(&options, "zzz").is_empty());
    }

    #[test]
    fn highlight_movement_wraps_and_handles_empty() {
        assert_eq!(
            move_select_highlight(None, 0, SelectMove::Next),
            None
        );
        assert_eq!(
            move_select_highlight(None, 3, SelectMove::Next),
            Some(0)
        );
        assert_eq!(
            move_select_highlight(Some(2), 3, SelectMove::Next),
            Some(0)
        );
        assert_eq!(
            move_select_highlight(Some(0), 3, SelectMove::Previous),
            Some(2)
        );
        assert_eq!(
            move_select_highlight(Some(1), 3, SelectMove::First),
            Some(0)
        );
        assert_eq!(
            move_select_highlight(Some(1), 3, SelectMove::Last),
            Some(2)
        );
        assert_eq!(select_move_for_key("up"), Some(SelectMove::Previous));
        assert_eq!(select_move_for_key("down"), Some(SelectMove::Next));
        assert_eq!(select_move_for_key("home"), Some(SelectMove::First));
        assert_eq!(select_move_for_key("end"), Some(SelectMove::Last));
        assert_eq!(select_move_for_key("enter"), None);
    }
}
