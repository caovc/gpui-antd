//! Ant Design application shell primitives: Layout, Header, Sider, Content,
//! and Footer. No HTML layout is used; native GPUI flex layout is shared
//! between the desktop renderer and GPUI WebAssembly.
//
// The shell is compositional: AntLayout is a row or column, nested layouts
// are ordinary children. Header/Footer, Sider and Content own their
// presentation; app state (including Sider.collapsed) remains controlled.

use std::rc::Rc;

use gpui_kit::{
    AnyElement, App, ElementId, IntoElement, InteractiveElement as _, ParentElement as _,
    RenderOnce, Styled as _, Window, div, px,
};
use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::prelude::FluentBuilder as _;

use crate::{
    AntIcon, AntIconName, AntTheme, GridBreakpoint,
};

type CollapseChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// Ant Design Layout. Vertical by default, `.horizontal(true)` for Sider +
/// nested page layout. Nested layouts grow into their parent flex axis.
#[derive(IntoElement)]
pub struct AntLayout {
    children: Vec<AnyElement>,
    horizontal: bool,
    height: Option<f32>,
    theme: AntTheme,
}
impl Default for AntLayout { fn default() -> Self { Self::new() } }
impl AntLayout {
    pub fn new() -> Self {
        Self { children:Vec::new(),horizontal:false,height:None,theme:AntTheme::default() }
    }
    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal=horizontal;self
    }
    /// Set an explicit height, useful for a self-contained dashboard or demo.
    /// Otherwise the layout flexes to fill the height available from its parent.
    pub fn height(mut self, height: f32) -> Self {
        self.height=Some(height.max(0.));self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme=theme;self }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());self
    }
    pub fn children(mut self, children: impl IntoIterator<Item=AnyElement>) -> Self {
        self.children.extend(children);self
    }
}
impl RenderOnce for AntLayout {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().flex().flex_col().flex_1().w_full().min_w_0().min_h(px(0.))
            .bg(self.theme.layout_body)
            .when(self.horizontal, |this| this.flex_row())
            .when_some(self.height, |this, height| this.h(px(height)).flex_none())
            .children(self.children)
    }
}

/// Top app bar. Ant Design's classic dark (#001529) header is the default;
/// use `.dark(false)` for an application-surface header.
#[derive(IntoElement)]
pub struct AntHeader {
    children: Vec<AnyElement>,
    theme: AntTheme,
    height: f32,
    padding: f32,
    dark: bool,
}
impl Default for AntHeader { fn default() -> Self { Self::new() } }
impl AntHeader {
    pub fn new() -> Self {
        Self {children:Vec::new(),theme:AntTheme::default(),height:64.,padding:24.,dark:true}
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {self.theme=theme;self}
    pub fn height(mut self, height: f32) -> Self {self.height=height.max(0.);self}
    pub fn padding(mut self, padding: f32) -> Self {self.padding=padding.max(0.);self}
    pub fn dark(mut self, dark: bool) -> Self {self.dark=dark;self}
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());self
    }
}
impl RenderOnce for AntHeader {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().flex().flex_none().items_center().w_full()
            .h(px(self.height)).px(px(self.padding))
            .bg(if self.dark {self.theme.layout_dark} else {self.theme.surface})
            .text_color(if self.dark {self.theme.layout_dark_text} else {self.theme.text})
            .children(self.children)
    }
}

/// Sidebar with controlled collapse, optional breakpoint-based auto-collapse,
/// dark/light appearance and an accessible native collapse trigger.
///
/// Auto-collapse is an effective *view* state while the viewport is narrower
/// than `breakpoint`. It never mutates the application's `collapsed` value:
/// `on_collapse` reports user-triggered toggles only.
#[derive(IntoElement)]
pub struct AntSider {
    id: ElementId,
    children: Vec<AnyElement>,
    theme: AntTheme,
    width: f32,
    collapsed_width: f32,
    collapsed: bool,
    breakpoint: Option<GridBreakpoint>,
    dark: bool,
    collapsible: bool,
    on_collapse: Option<CollapseChange>,
}
impl AntSider {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id:id.into(),children:Vec::new(),theme:AntTheme::default(),
            width:200.,collapsed_width:80.,collapsed:false,breakpoint:None,
            dark:true,collapsible:false,on_collapse:None,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {self.theme=theme;self}
    pub fn width(mut self, width: f32) -> Self {self.width=width.max(0.);self}
    pub fn collapsed_width(mut self, width: f32) -> Self {self.collapsed_width=width.max(0.);self}
    pub fn collapsed(mut self, collapsed: bool) -> Self {self.collapsed=collapsed;self}
    pub fn breakpoint(mut self, breakpoint: GridBreakpoint) -> Self {
        self.breakpoint=Some(breakpoint);self
    }
    pub fn dark(mut self, dark: bool) -> Self {self.dark=dark;self}
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible=collapsible;self
    }
    pub fn on_collapse(mut self, handler: impl Fn(bool, &mut Window, &mut App)+'static) -> Self {
        self.on_collapse=Some(Rc::new(handler));self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());self
    }
    /// Effective collapsed state including the optional viewport breakpoint.
    pub fn is_collapsed_at(&self, viewport: gpui_kit::Pixels) -> bool {
        self.collapsed || self.breakpoint.is_some_and(|b| viewport < px(b.min_width()))
    }
    /// Effective sidebar width at the supplied logical viewport.
    pub fn width_at(&self, viewport: gpui_kit::Pixels) -> f32 {
        if self.is_collapsed_at(viewport) {self.collapsed_width} else {self.width}
    }
}
impl RenderOnce for AntSider {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let collapsed=self.is_collapsed_at(window.viewport_size().width);
        let width=self.width_at(window.viewport_size().width);
        let fg=if self.dark {self.theme.layout_dark_text} else {self.theme.text};
        let bg=if self.dark {self.theme.layout_dark} else {self.theme.surface};
        let button_fg=fg;
        div().flex().flex_col().flex_none().min_h(px(0.))
            .w(px(width)).bg(bg).text_color(fg)
            .child(div().flex_1().min_h(px(0.)).flex().flex_col()
                .children(self.children))
            .when(self.collapsible, |this| {
                let requested=!self.collapsed;
                this.child(
                    BaseButton::new(self.id)
                        .w_full().h(px(40.)).flex().items_center().justify_center()
                        .bg(bg).text_color(button_fg)
                        .hover(|this| this.bg(self.theme.layout_sider_hover))
                        .child(AntIcon::new(if collapsed {
                            AntIconName::MenuUnfoldOutlined
                        } else {
                            AntIconName::MenuFoldOutlined
                        }).size(16.).color(button_fg).theme(self.theme))
                        .when(!collapsed, |this| this.child("Collapse"))
                        .when_some(self.on_collapse, |this, listener| {
                            this.on_click(move |_, window, cx| {
                                listener(requested, window, cx);
                            })
                        })
                )
            })
    }
}

/// Flexible content surface inside AntLayout. Defaults to the familiar
/// neutral body background; `.surface(true)` gives a raised white panel.
#[derive(IntoElement)]
pub struct AntContent {
    children: Vec<AnyElement>,
    theme: AntTheme,
    padding: f32,
    surface: bool,
}
impl Default for AntContent { fn default() -> Self { Self::new() } }
impl AntContent {
    pub fn new() -> Self {
        Self {children:Vec::new(),theme:AntTheme::default(),padding:24.,surface:false}
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {self.theme=theme;self}
    pub fn padding(mut self, padding: f32) -> Self {self.padding=padding.max(0.);self}
    pub fn surface(mut self, surface: bool) -> Self {self.surface=surface;self}
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());self
    }
}
impl RenderOnce for AntContent {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().flex_1().min_h(px(0.)).min_w_0()
            .p(px(self.padding))
            .bg(if self.surface {self.theme.surface} else {self.theme.layout_body})
            .text_color(self.theme.text)
            .children(self.children)
    }
}

/// Footer for page-wide copyright, version and secondary actions.
#[derive(IntoElement)]
pub struct AntFooter {
    children: Vec<AnyElement>,
    theme: AntTheme,
    padding: f32,
    center: bool,
}
impl Default for AntFooter { fn default() -> Self { Self::new() } }
impl AntFooter {
    pub fn new() -> Self {
        Self {children:Vec::new(),theme:AntTheme::default(),padding:20.,center:true}
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {self.theme=theme;self}
    pub fn padding(mut self, padding: f32) -> Self {self.padding=padding.max(0.);self}
    pub fn center(mut self, center: bool) -> Self {self.center=center;self}
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());self
    }
}
impl RenderOnce for AntFooter {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().w_full().flex().flex_none().items_center()
            .when(self.center, |this| this.justify_center())
            .p(px(self.padding))
            .bg(self.theme.layout_body)
            .text_color(self.theme.text_secondary)
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sider_defaults_and_responsive_breakpoint_edges() {
        let sider=AntSider::new("sidebar").width(240.).collapsed_width(64.)
            .breakpoint(GridBreakpoint::Md);
        assert_eq!(sider.width_at(px(767.)),64.);
        assert_eq!(sider.width_at(px(768.)),240.);
        assert_eq!(sider.width_at(px(2000.)),240.);
        assert!(!sider.collapsed);
        let forced=sider.collapsed(true);
        assert_eq!(forced.width_at(px(1000.)),64.);
    }
    #[test]
    fn sider_width_and_header_height_clamp_to_nonnegative() {
        let sider=AntSider::new("sidebar").width(-20.).collapsed_width(-12.);
        assert_eq!(sider.width_at(px(1100.)),0.);
        assert_eq!(sider.collapsed(true).width_at(px(1100.)),0.);
        assert_eq!(AntHeader::new().height(-200.).height,0.);
        assert_eq!(AntFooter::new().padding(-1.).padding,0.);
    }
    #[test]
    fn nested_layout_direction_and_dimensions() {
        let layout=AntLayout::new().horizontal(true).height(420.);
        assert!(layout.horizontal);
        assert_eq!(layout.height,Some(420.));
        assert_eq!(AntContent::new().padding(-3.).padding,0.);
    }
}
