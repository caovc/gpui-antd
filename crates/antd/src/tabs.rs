//! Ant Design-inspired controlled Tabs over GPUI Kit's native tab bar.
//!
//! Tab selection is owned by the application. GPUI Kit provides pointer
//! activation, native animation, overflow scrolling and tab accessibility.
//! The pinned Kit TabBar paints the animated indicator and exposes each
//! tab's ARIA selected state (Kit 0.7.3 omits the disabled accessibility flag).
//! We attach a stable focus handle to
//! every native KitTab for roving focus and automatic keyboard activation,
//! without replacing the styled tab bar with a DOM or fake visual proxy.

use crate::{AntIconName, AntTheme, ControlSize};
use gpui_kit::component::{
    Icon as KitIcon, Sizable as _, Size,
    tab::{Tab as KitTab, TabBar as KitTabBar, TabVariant as KitTabVariant},
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, InteractiveElement as _, IntoElement,
    KeyDownEvent, ParentElement as _, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};
use std::rc::Rc;

type Change = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Ant Design-like tab bar styles. The underlying Kit tab bar supplies
/// native animation and scrolling; exact Ant Design CSS parity is in progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsVariant {
    /// Line indicator under selected tab (Ant Design default).
    #[default]
    Line,
    /// Bordered tabs over the content panel.
    Card,
    /// Capsule-shaped selection indicator.
    Pills,
    /// Joined segmented selection background.
    Segmented,
}

/// One tab entry with a lazily inserted content panel. The owner rebuilds
/// the `AntTabs` each render; only the selected panel is added to the tree.
pub struct AntTab {
    label: SharedString,
    disabled: bool,
    icon: Option<AntIconName>,
    content: Option<AnyElement>,
}

impl AntTab {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            icon: None,
            content: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn icon(mut self, icon: AntIconName) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

/// A tabs group with a controlled active index and optional native panels.
///
/// Usage:
/// ```ignore
/// AntTabs::new("settings-tabs")
///     .active_index(active_tab)
///     .on_change(|index, _, cx| { /* update owning Entity */ })
///     .child(AntTab::new("Overview").content(div().child("Summary")))
///     .child(AntTab::new("Settings").content(div().child("Options")))
/// ```
///
/// Disabled or unknown indices never invoke `on_change`. The parent must
/// apply the requested index and call `cx.notify()`.
///
/// The selected tab owns the single Tab stop, with an independent focus
/// handle per enabled tab. Left/Right and Home/End move focus and request
/// automatic activation, wrapping while skipping disabled tabs. Enter/Space
/// activate the focused tab; focus inside the selected panel remains separate.
#[derive(IntoElement)]
pub struct AntTabs {
    id: ElementId,
    tabs: Vec<AntTab>,
    active_index: usize,
    variant: TabsVariant,
    size: ControlSize,
    theme: AntTheme,
    on_change: Option<Change>,
    panel_padding: f32,
    keyboard: bool,
}

impl AntTabs {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            active_index: 0,
            variant: TabsVariant::default(),
            size: ControlSize::Middle,
            theme: AntTheme::default(),
            on_change: None,
            panel_padding: 16.,
            keyboard: true,
        }
    }
    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }
    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn panel_padding(mut self, padding: f32) -> Self {
        self.panel_padding = padding.max(0.);
        self
    }
    /// Enable roving per-tab focus and Left/Right/Home/End/Enter/Space.
    /// Enabled by default; `.keyboard(false)` keeps pointer activation.
    pub fn keyboard(mut self, enabled: bool) -> Self {
        self.keyboard = enabled;
        self
    }
    pub fn child(mut self, child: AntTab) -> Self {
        self.tabs.push(child);
        self
    }
    pub fn children(mut self, children: impl IntoIterator<Item = AntTab>) -> Self {
        self.tabs.extend(children);
        self
    }
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Check whether an index can be activated, without modifying any state.
    pub fn can_activate(&self, index: usize) -> bool {
        self.tabs.get(index).is_some_and(|tab| !tab.disabled)
    }
}

/// Find the next enabled tab for keyboard navigation.
///
/// The selection remains controlled: callers use the returned index to
/// request an update. Out-of-range or disabled selections fall back to the
/// first enabled tab, without ever indexing outside the tab collection.
fn next_enabled_tab(enabled: &[bool], selected: usize, key: &str) -> Option<usize> {
    let first = enabled.iter().position(|value| *value)?;
    match key {
        "home" => Some(first),
        "end" => enabled.iter().rposition(|value| *value),
        "right" | "left" => {
            if !enabled.get(selected).copied().unwrap_or(false) {
                return Some(first);
            }
            let len = enabled.len();
            for step in 1..=len {
                let candidate = if key == "right" {
                    (selected + step) % len
                } else {
                    (selected + len - (step % len)) % len
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

impl RenderOnce for AntTabs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let selected = self.active_index;
        let allowed = self.tabs.iter().map(|t| !t.disabled).collect::<Vec<_>>();
        let nav_id = format!("ant-tabs-navigation:{:?}", self.id);
        // Keyed per-index identity is stable under ordinary controlled
        // selection changes. A changed tab order also changes active_index,
        // which is the public controlled identity in this API.
        let tab_focus: Vec<FocusHandle> = self
            .tabs
            .iter()
            .enumerate()
            .map(|(index, _)| {
                window
                    .use_keyed_state(format!("{nav_id}:tab:{index}"), cx, |_, cx| {
                        cx.focus_handle()
                    })
                    .read(cx)
                    .clone()
            })
            .collect();
        let tab_stop = allowed
            .get(selected)
            .filter(|enabled| **enabled)
            .map(|_| selected)
            .or_else(|| allowed.iter().position(|enabled| *enabled));
        let size = match self.size {
            ControlSize::Small => Size::Small,
            ControlSize::Middle => Size::Medium,
            ControlSize::Large => Size::Large,
        };
        let kit_variant = match self.variant {
            TabsVariant::Line => KitTabVariant::Underline,
            TabsVariant::Card => KitTabVariant::Tab,
            TabsVariant::Pills => KitTabVariant::Pill,
            TabsVariant::Segmented => KitTabVariant::Segmented,
        };
        let all_handles = tab_focus.clone();
        let keyboard_handler = self.on_change.clone();
        let enabled_keys = allowed.clone();
        let keyboard = self.keyboard;
        let bar = self.tabs.iter().enumerate().fold(
            KitTabBar::new(self.id)
                .with_variant(kit_variant)
                .with_size(size)
                .selected_index(selected)
                .w_full()
                .text_color(self.theme.text),
            |bar, (index, tab)| {
                let focus = all_handles[index].clone();
                let focused = focus.clone();
                let handles = all_handles.clone();
                let enabled = enabled_keys.clone();
                let on_change = keyboard_handler.clone();
                let activate = keyboard_handler.clone();
                let pointer_focus = focus.clone();
                let item = KitTab::new()
                    .label(tab.label.clone())
                    .aria_label(tab.label.clone())
                    .disabled(tab.disabled)
                    .track_focus(&focus.tab_stop(keyboard && tab_stop == Some(index)))
                    .on_click(move |event, window, cx| {
                        if keyboard {
                            window.focus(&pointer_focus, cx);
                        }
                        // Kit/GPUI synthesizes Enter/Space clicks on key-up.
                        // Keep one activation path and avoid reporting a
                        // no-op keyboard selection as another owner change.
                        if index != selected || !matches!(event, ClickEvent::Keyboard(_)) {
                            if let Some(callback) = &activate {
                                callback(index, window, cx);
                            }
                        }
                    })
                    .when(keyboard && !tab.disabled, |item| {
                        item.on_key_down(
                            move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                                // Do not steal shortcuts or IME composition from
                                // the surrounding page or selected tab panel.
                                if !focused.is_focused(window)
                                    || event.keystroke.modifiers.modified()
                                    || event.keystroke.is_ime_in_progress()
                                {
                                    return;
                                }
                                let key = event.keystroke.key.as_str();
                                if let Some(next) = next_enabled_tab(&enabled, index, key) {
                                    if next != index {
                                        window.focus(&handles[next], cx);
                                    }
                                    // A controlled owner may not have applied
                                    // the last request yet. Home/End still
                                    // activate their target when focus is
                                    // already there but selection is stale.
                                    if next != selected {
                                        if let Some(callback) = &on_change {
                                            callback(next, window, cx);
                                        }
                                    }
                                    window.prevent_default();
                                    cx.stop_propagation();
                                }
                            },
                        )
                    })
                    .when_some(tab.icon, |item, icon| {
                        item.icon(KitIcon::default().path(icon.path()))
                    });
                bar.child(item)
            },
        );
        let active_label = self
            .tabs
            .get(selected)
            .filter(|tab| !tab.disabled)
            .map(|tab| tab.label.clone());
        let panel = self
            .tabs
            .into_iter()
            .nth(selected)
            .filter(|tab| !tab.disabled)
            .and_then(|tab| tab.content);
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(0.))
            // Kit's inner Tabs already has Role::TabList. Do NOT create a
            // nested second TabList: each KitTab supplies Role::Tab and
            // aria-selected through GPUI Base (disabled flag is missing).
            .child(div().child(bar))
            .when_some(panel, |this, panel| {
                this.child(
                    div()
                        .id(format!("{nav_id}:panel:{selected}"))
                        .role(Role::TabPanel)
                        .aria_label(active_label.unwrap_or_else(|| "Tab content".into()))
                        .w_full()
                        .min_w_0()
                        .p(px(self.panel_padding))
                        .text_color(self.theme.text)
                        .child(panel),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_is_controlled_and_rejects_disabled_or_unknown_indices() {
        let tabs = AntTabs::new("test")
            .active_index(2)
            .child(AntTab::new("One"))
            .child(AntTab::new("Blocked").disabled(true))
            .child(AntTab::new("Three"));
        assert_eq!(tabs.active_index, 2);
        assert!(tabs.can_activate(0));
        assert!(!tabs.can_activate(1));
        assert!(tabs.can_activate(2));
        assert!(!tabs.can_activate(100));
    }

    #[test]
    fn native_arrow_navigation_wraps_and_skips_disabled_tabs() {
        assert_eq!(next_enabled_tab(&[true, false, true], 0, "right"), Some(2));
        assert_eq!(next_enabled_tab(&[true, false, true], 2, "right"), Some(0));
        assert_eq!(next_enabled_tab(&[true, false, true], 0, "left"), Some(2));
        assert_eq!(next_enabled_tab(&[false, false], 0, "left"), None);
        assert_eq!(next_enabled_tab(&[], 0, "right"), None);
        assert_eq!(next_enabled_tab(&[false, true, false], 0, "home"), Some(1));
        assert_eq!(next_enabled_tab(&[false, true, false], 0, "end"), Some(1));
        assert_eq!(next_enabled_tab(&[true, false, true], 99, "right"), Some(0));
        assert_eq!(next_enabled_tab(&[true, false, true], 2, "pageup"), None);
    }
    #[test]
    fn roving_focus_uses_focused_tab_not_stale_controlled_selection() {
        // Focus can move ahead of an asynchronously applied controlled value.
        // Arrow keys must follow the focused tab, not the old active index.
        let enabled = [true, false, true, true];
        assert_eq!(next_enabled_tab(&enabled, 2, "right"), Some(3));
        assert_eq!(next_enabled_tab(&enabled, 3, "left"), Some(2));
        assert_eq!(next_enabled_tab(&enabled, 3, "right"), Some(0));
        assert_eq!(next_enabled_tab(&enabled, 0, "home"), Some(0));
        assert_eq!(next_enabled_tab(&enabled, 2, "end"), Some(3));
        assert_eq!(next_enabled_tab(&enabled, 2, "space"), None);
    }
    #[test]
    fn keyboard_navigation_can_be_disabled_by_consumer() {
        assert!(AntTabs::new("default").keyboard);
        assert!(!AntTabs::new("manual").keyboard(false).keyboard);
    }
    #[test]
    fn builders_store_variants_sizes_and_panels() {
        let tabs = AntTabs::new("settings")
            .variant(TabsVariant::Card)
            .size(ControlSize::Small)
            .panel_padding(-5.)
            .child(AntTab::new("Overview").icon(AntIconName::HomeOutlined));
        assert_eq!(tabs.variant, TabsVariant::Card);
        assert_eq!(tabs.size, ControlSize::Small);
        assert_eq!(tabs.panel_padding, 0.);
        assert_eq!(tabs.tabs.len(), 1);
        assert_eq!(tabs.tabs[0].icon, Some(AntIconName::HomeOutlined));
    }
}
