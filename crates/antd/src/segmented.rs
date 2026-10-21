//! Ant Design Segmented: native single-choice switch with controlled index.
//! Roving focus and Arrow/Home/End navigation skip disabled options.

use crate::{AntIcon, AntIconName, AntTheme, ControlSize};
use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, ElementId, FocusHandle, InteractiveElement as _, IntoElement, KeyDownEvent,
    ParentElement as _, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    Styled as _, Window, div, px,
};
use std::rc::Rc;

type SegmentChange = Rc<dyn Fn(usize, &mut Window, &mut App)>;

pub struct AntSegmentedOption {
    label: SharedString,
    icon: Option<AntIconName>,
    disabled: bool,
}
impl AntSegmentedOption {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            disabled: false,
        }
    }
    pub fn icon(mut self, icon: AntIconName) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// The next enabled option, wrapping past the end and skipping disabled keys.
pub fn next_segmented_index(enabled: &[bool], current: usize, key: &str) -> Option<usize> {
    let first = enabled.iter().position(|v| *v)?;
    match key {
        "home" => Some(first),
        "end" => enabled.iter().rposition(|v| *v),
        "left" | "right" => {
            if !enabled.get(current).copied().unwrap_or(false) {
                return Some(first);
            }
            let len = enabled.len();
            for step in 1..=len {
                let candidate = if key == "right" {
                    (current + step) % len
                } else {
                    (current + len - (step % len)) % len
                };
                if enabled[candidate] {
                    return Some(candidate);
                }
            }
            None
        }
        _ => None,
    }
}

#[derive(IntoElement)]
pub struct AntSegmented {
    id: ElementId,
    items: Vec<AntSegmentedOption>,
    selected_index: usize,
    size: ControlSize,
    block: bool,
    disabled: bool,
    theme: AntTheme,
    on_change: Option<SegmentChange>,
}
impl AntSegmented {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected_index: 0,
            size: ControlSize::Middle,
            block: false,
            disabled: false,
            theme: AntTheme::default(),
            on_change: None,
        }
    }
    pub fn child(mut self, option: AntSegmentedOption) -> Self {
        self.items.push(option);
        self
    }
    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
    pub fn block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntSegmented {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = self.theme;
        let selected = self.selected_index;
        let disabled = self.disabled;
        let active: Vec<bool> = self
            .items
            .iter()
            .map(|o| !o.disabled && !disabled)
            .collect();
        let tab_stop = selected
            .checked_sub(0)
            .filter(|i| active.get(*i) == Some(&true))
            .or_else(|| active.iter().position(|v| *v));
        let prefix = format!("{:?}", self.id);
        let focus_handles: Vec<FocusHandle> = (0..self.items.len())
            .map(|index| {
                window
                    .use_keyed_state(format!("ant-segment-{prefix}-{index}"), cx, |_, cx| {
                        cx.focus_handle()
                    })
                    .read(cx)
                    .clone()
            })
            .collect();
        let keyboard_handles = focus_handles.clone();
        let keyboard_enabled = active.clone();
        let keyboard_change = self.on_change.clone();
        let on_keyboard = move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
            if disabled
                || event.keystroke.modifiers.modified()
                || event.keystroke.is_ime_in_progress()
            {
                return;
            }
            let Some(origin) = keyboard_handles.iter().position(|f| f.is_focused(window)) else {
                return;
            };
            if let Some(next) =
                next_segmented_index(&keyboard_enabled, origin, event.keystroke.key.as_str())
            {
                window.focus(&keyboard_handles[next], cx);
                if selected != next {
                    if let Some(cb) = &keyboard_change {
                        cb(next, window, cx);
                    }
                }
                window.prevent_default();
                cx.stop_propagation();
            }
        };
        div()
            .id(self.id)
            .role(Role::TabList)
            .flex()
            .items_center()
            .gap(px(3.))
            .bg(t.surface_secondary)
            .p(px(3.))
            .rounded(px(t.border_radius + 2.))
            .when(self.block, |this| this.w_full())
            .on_key_down(on_keyboard)
            .children(
                self.items
                    .into_iter()
                    .enumerate()
                    .map(move |(index, item)| {
                        let chosen = index == selected;
                        let fg = if disabled || item.disabled {
                            t.disabled_text
                        } else if chosen {
                            t.primary
                        } else {
                            t.text
                        };
                        let callback = self.on_change.clone();
                        BaseButton::new(format!("ant-segment-{prefix}-{index}"))
                            .role(Role::Tab)
                            .selected(chosen)
                            .disabled(disabled || item.disabled)
                            .accessibility_label(item.label.clone())
                            .track_focus(&focus_handles[index])
                            .tab_stop(tab_stop == Some(index))
                            .h(px(self.size.height()))
                            .px(px(self.size.horizontal_padding()))
                            .when(self.block, |this| this.flex_1())
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(6.))
                            .rounded(px(t.border_radius))
                            .bg(if chosen {
                                t.surface
                            } else {
                                t.surface_secondary
                            })
                            .text_color(fg)
                            .text_size(px(if self.size == ControlSize::Large {
                                16.
                            } else {
                                14.
                            }))
                            .when(!disabled && !item.disabled, |this| {
                                this.cursor_pointer().hover(|this| {
                                    this.bg(if chosen { t.surface } else { t.info_bg })
                                })
                            })
                            .when_some(item.icon, |this, icon| {
                                this.child(AntIcon::new(icon).theme(t).color(fg).size(16.))
                            })
                            .child(item.label)
                            .when_some(callback, |this, callback| {
                                this.on_click(move |_, window, cx| {
                                    if selected != index {
                                        callback(index, window, cx);
                                    }
                                })
                            })
                    }),
            )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roving_skips_disabled_and_wraps() {
        assert_eq!(
            next_segmented_index(&[true, false, true], 0, "right"),
            Some(2)
        );
        assert_eq!(
            next_segmented_index(&[true, false, true], 2, "right"),
            Some(0)
        );
        assert_eq!(
            next_segmented_index(&[true, false, true], 0, "left"),
            Some(2)
        );
        assert_eq!(
            next_segmented_index(&[false, true, true], 0, "home"),
            Some(1)
        );
        assert_eq!(
            next_segmented_index(&[true, false, true], 0, "end"),
            Some(2)
        );
        assert_eq!(next_segmented_index(&[false, false], 0, "right"), None);
        assert_eq!(next_segmented_index(&[true], 0, "escape"), None);
    }
    #[test]
    fn selected_index_is_owned_by_parent() {
        let group = AntSegmented::new("view")
            .selected_index(1)
            .child(AntSegmentedOption::new("Rows"))
            .child(AntSegmentedOption::new("Cards").disabled(true));
        assert_eq!(group.selected_index, 1);
        assert!(group.items[1].disabled);
    }
}
