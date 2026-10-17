//! Native Ant Design-inspired Menu, built from GPUI Base button semantics.
//!
//! The same menu renders inside Sider and standalone desktop/WASM views.
//! Selection is controlled by the owning Rust GPUI Entity. This initial
//! milestone implements accessible flat navigation; nested submenu trees,
//! keyboard roving focus and popup modes are future Ant Design parity work.

use std::rc::Rc;
use gpui_kit::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, div, px,
};
use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::prelude::FluentBuilder as _;

use crate::{AntIcon, AntIconName, AntTheme, color};

type MenuSelect = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;

/// One menu action identified by an application-owned stable key.
#[derive(Debug, Clone)]
pub struct AntMenuItem {
    key: SharedString,
    label: SharedString,
    icon: Option<AntIconName>,
    disabled: bool,
    danger: bool,
}
impl AntMenuItem {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key:key.into(),label:label.into(),
            icon:None,disabled:false,danger:false,
        }
    }
    pub fn icon(mut self, icon: AntIconName) -> Self {
        self.icon=Some(icon);self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled=disabled;self
    }
    pub fn danger(mut self, danger: bool) -> Self {
        self.danger=danger;self
    }
    pub fn key(&self) -> &str {self.key.as_ref()}
}

/// Controlled Ant Design-like flat navigation Menu.
///
/// `on_select` reports the stable item key; it does not modify this value
/// until the application calls `.selected_key(...)` on the next render.
/// Each action uses GPUI Base Button for focus, Enter/Space, and semantics.
/// Inline collapsed mode hides labels visually but preserves screen-reader
/// accessible names. This is not the nested/pop-up SubMenu implementation.
#[derive(IntoElement)]
pub struct AntMenu {
    id: ElementId,
    items: Vec<AntMenuItem>,
    selected: Option<SharedString>,
    theme: AntTheme,
    dark: bool,
    inline_collapsed: bool,
    item_height: f32,
    on_select: Option<MenuSelect>,
}
impl AntMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id:id.into(),items:Vec::new(),selected:None,
            theme:AntTheme::default(),dark:false,inline_collapsed:false,
            item_height:40.,on_select:None,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme=theme;self }
    pub fn dark(mut self, dark: bool) -> Self {self.dark=dark;self}
    pub fn inline_collapsed(mut self, collapsed: bool) -> Self {
        self.inline_collapsed=collapsed;self
    }
    pub fn item_height(mut self, height: f32) -> Self {
        self.item_height=height.max(24.);self
    }
    pub fn selected_key(mut self, key: impl Into<SharedString>) -> Self {
        self.selected=Some(key.into());self
    }
    pub fn item(mut self, item: AntMenuItem) -> Self {
        self.items.push(item);self
    }
    pub fn items(mut self, items: impl IntoIterator<Item=AntMenuItem>) -> Self {
        self.items.extend(items);self
    }
    pub fn on_select(mut self, handler: impl Fn(SharedString, &mut Window, &mut App)+'static) -> Self {
        self.on_select=Some(Rc::new(handler));self
    }
    /// Whether a key names an enabled item, for callers validating routes.
    pub fn can_select(&self, key: &str) -> bool {
        self.items.iter().any(|item|item.key.as_ref()==key&&!item.disabled)
    }
}
impl RenderOnce for AntMenu {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme=self.theme;
        let dark=self.dark;
        let collapsed=self.inline_collapsed;
        let selected=self.selected;
        let on_select=self.on_select;
        let item_height=self.item_height;
        let prefix=format!("{:?}",self.id);
        div().w_full().flex().flex_col().gap(px(3.))
            .px(px(7.)).py(px(6.))
            .bg(if dark {theme.layout_dark} else {theme.surface})
            .children(self.items.into_iter().map(move |item| {
                let chosen=selected.as_ref().is_some_and(|key|key==&item.key);
                let label=item.label.clone();
                let key=item.key.clone();
                let bg=if chosen {
                    if dark {theme.primary} else {theme.info_bg}
                } else if dark {theme.layout_dark} else {theme.surface};
                let fg=if item.disabled {theme.disabled_text}
                    else if chosen {if dark {color(0xffffff)} else {theme.primary}}
                    else if item.danger {theme.error}
                    else if dark {theme.layout_dark_text} else {theme.text};
                let hover_bg=if dark {theme.layout_sider_hover} else {theme.info_bg};
                BaseButton::new(format!("ant-menu-{prefix}-{}",item.key))
                    .accessibility_label(label.clone())
                    .disabled(item.disabled)
                    .w_full().h(px(item_height))
                    .px(px(if collapsed {6.} else {12.}))
                    .flex().items_center().justify_start()
                    .gap(px(11.)).rounded(px(theme.border_radius))
                    .bg(bg).text_color(fg).text_size(px(14.))
                    .when(!item.disabled, |this| this.cursor_pointer()
                        .hover(|this| this.bg(hover_bg)))
                    .when_some(item.icon, |this, icon| {
                        this.child(AntIcon::new(icon).theme(theme).size(17.).color(fg))
                    })
                    .when(!collapsed, |this| {
                        this.child(div().min_w_0().child(label))
                    })
                    .when_some(on_select.clone(), |this, cb| {
                        this.on_click(move |_,window,cx| cb(key.clone(),window,cx))
                    })
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_rejects_disabled_and_unknown_route_keys() {
        let menu=AntMenu::new("nav")
            .item(AntMenuItem::new("home","Home").icon(AntIconName::HomeOutlined))
            .item(AntMenuItem::new("locked","Restricted").disabled(true));
        assert!(menu.can_select("home"));
        assert!(!menu.can_select("locked"));
        assert!(!menu.can_select("missing"));
        assert_eq!(menu.items[0].key(),"home");
    }
    #[test]
    fn builder_preserves_controlled_selection_and_collapsed_mode() {
        let menu=AntMenu::new("sidebar").dark(true)
            .inline_collapsed(true).item_height(-7.)
            .selected_key("settings")
            .item(AntMenuItem::new("settings","Settings").danger(true));
        assert!(menu.dark);
        assert!(menu.inline_collapsed);
        assert_eq!(menu.item_height,24.);
        assert_eq!(menu.selected.as_deref(),Some("settings"));
        assert!(menu.items[0].danger);
    }
}
