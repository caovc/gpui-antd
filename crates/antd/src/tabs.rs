//! Ant Design-inspired controlled Tabs over GPUI Kit's native tab bar.
//!
//! Tab selection is owned by the application. GPUI Kit provides pointer
//! activation, native animation, overflow scrolling and tab accessibility.
//! Arrow/Home/End roving focus is a known gap in the pinned Kit version;
//! do not claim full keyboard parity until it is implemented and tested.

use std::rc::Rc;
use gpui_kit::{
    AnyElement, App, ElementId, IntoElement, RenderOnce, SharedString,
    Styled as _, Window, div, px,
};
use gpui_kit::component::{
    Sizable as _, Size,
    Icon as KitIcon,
    tab::{Tab as KitTab, TabBar as KitTabBar, TabVariant as KitTabVariant},
};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntIconName, AntTheme, ControlSize};

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
        Self { label:label.into(),disabled:false,icon:None,content:None }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled=disabled;self
    }
    pub fn icon(mut self, icon: AntIconName) -> Self {
        self.icon=Some(icon);self
    }
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content=Some(content.into_any_element());self
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
/// apply the requested index and call `cx.notify()`. Keyboard semantics are
/// inherited from the pinned GPUI Kit tab primitives; arrow-key roving focus
/// is not yet implemented upstream.
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
}

impl AntTabs {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),tabs:Vec::new(),active_index:0,
            variant:TabsVariant::default(),size:ControlSize::Middle,
            theme:AntTheme::default(),on_change:None,panel_padding:16.,
        }
    }
    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index=index;self
    }
    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant=variant;self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size=size;self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme=theme;self
    }
    pub fn panel_padding(mut self, padding: f32) -> Self {
        self.panel_padding=padding.max(0.);self
    }
    pub fn child(mut self, child: AntTab) -> Self {
        self.tabs.push(child);self
    }
    pub fn children(mut self, children: impl IntoIterator<Item=AntTab>) -> Self {
        self.tabs.extend(children);self
    }
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change=Some(Rc::new(handler));self
    }

    /// Check whether an index can be activated, without modifying any state.
    pub fn can_activate(&self, index: usize) -> bool {
        self.tabs.get(index).is_some_and(|tab| !tab.disabled)
    }
}

impl RenderOnce for AntTabs {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let selected=self.active_index;
        let allowed=self.tabs.iter().map(|t|!t.disabled).collect::<Vec<_>>();
        let size=match self.size {
            ControlSize::Small=>Size::Small,
            ControlSize::Middle=>Size::Medium,
            ControlSize::Large=>Size::Large,
        };
        let kit_variant=match self.variant {
            TabsVariant::Line=>KitTabVariant::Underline,
            TabsVariant::Card=>KitTabVariant::Tab,
            TabsVariant::Pills=>KitTabVariant::Pill,
            TabsVariant::Segmented=>KitTabVariant::Segmented,
        };
        let bar=self.tabs.iter().fold(
            KitTabBar::new(self.id)
                .with_variant(kit_variant)
                .with_size(size)
                .selected_index(selected)
                .w_full()
                .text_color(self.theme.text),
            |bar, tab| {
                let item=KitTab::new()
                    .label(tab.label.clone())
                    .disabled(tab.disabled)
                    .when_some(tab.icon, |item, icon| {
                        item.icon(KitIcon::default().path(icon.path()))
                    });
                bar.child(item)
            },
        );
        let bar=bar.when_some(self.on_change, |bar, callback| {
            bar.on_click(move |index, window, cx| {
                if allowed.get(*index).copied()==Some(true) {
                    callback(*index, window, cx);
                }
            })
        });
        let panel=self.tabs.into_iter().nth(selected)
            .filter(|tab|!tab.disabled)
            .and_then(|tab|tab.content);
        div().w_full().flex().flex_col().gap(px(0.))
            .child(bar)
            .when_some(panel, |this, panel| {
                this.child(
                    div().w_full().min_w_0().p(px(self.panel_padding))
                        .text_color(self.theme.text)
                        .child(panel)
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_is_controlled_and_rejects_disabled_or_unknown_indices() {
        let tabs=AntTabs::new("test")
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
    fn builders_store_variants_sizes_and_panels() {
        let tabs=AntTabs::new("settings")
            .variant(TabsVariant::Card)
            .size(ControlSize::Small)
            .panel_padding(-5.)
            .child(AntTab::new("Overview").icon(AntIconName::HomeOutlined));
        assert_eq!(tabs.variant,TabsVariant::Card);
        assert_eq!(tabs.size,ControlSize::Small);
        assert_eq!(tabs.panel_padding,0.);
        assert_eq!(tabs.tabs.len(),1);
        assert_eq!(tabs.tabs[0].icon,Some(AntIconName::HomeOutlined));
    }
}
