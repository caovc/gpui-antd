//! Ant Design application shell primitives: Layout, Header, Sider, Content,
//! and Footer. No HTML layout is used; native GPUI flex layout is shared
//! between the desktop renderer and GPUI WebAssembly.
//
// The shell is compositional: AntLayout is a row or column, nested layouts
// are ordinary children. Header/Footer, Sider and Content own their
// presentation; app state (including Sider.collapsed) remains controlled.

use std::rc::Rc;

use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::component::{Placement, WindowExt as _};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::design_tokens::AntComponent;
use crate::{AntIcon, AntIconName, AntTheme, GridBreakpoint};

type CollapseChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type DrawerBody = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;

/// Ant Design Layout. Vertical by default, `.horizontal(true)` for Sider +
/// nested page layout. Nested layouts grow into their parent flex axis.
#[derive(IntoElement)]
pub struct AntLayout {
    children: Vec<AnyElement>,
    horizontal: bool,
    height: Option<f32>,
    theme: AntTheme,
}
impl Default for AntLayout {
    fn default() -> Self {
        Self::new()
    }
}
impl AntLayout {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            horizontal: false,
            height: None,
            theme: AntTheme::default(),
        }
    }
    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }
    /// Set an explicit height, useful for a self-contained dashboard or demo.
    /// Otherwise the layout flexes to fill the height available from its parent.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height.max(0.));
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    pub fn children(mut self, children: impl IntoIterator<Item = AnyElement>) -> Self {
        self.children.extend(children);
        self
    }
}
impl RenderOnce for AntLayout {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .w_full()
            .min_w_0()
            .min_h(px(0.))
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
impl Default for AntHeader {
    fn default() -> Self {
        Self::new()
    }
}
impl AntHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            theme: AntTheme::default(),
            height: 64.,
            padding: 24.,
            dark: true,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(0.);
        self
    }
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.);
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}
impl RenderOnce for AntHeader {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_none()
            .items_center()
            .w_full()
            .h(px(self.height))
            .px(px(self.padding))
            .bg(if self.dark {
                self.theme.layout_dark
            } else {
                self.theme.surface
            })
            .text_color(if self.dark {
                self.theme.layout_dark_text
            } else {
                self.theme.text
            })
            .children(self.children)
    }
}

/// Sidebar with controlled collapse, optional breakpoint-based auto-collapse,
/// dark/light appearance and an accessible native collapse trigger.
///
/// Automatically collapses below the named breakpoint without mutating the
/// desktop collapsed state. A narrow viewport is independently controllable
/// through `responsive_open`, allowing users to expand the sidebar even
/// while the breakpoint is active. The on_collapse callback reports the NEXT
/// effective collapsed state and application code owns both flags.
#[derive(IntoElement)]
pub struct AntSider {
    id: ElementId,
    children: Vec<AnyElement>,
    theme: AntTheme,
    width: f32,
    collapsed_width: f32,
    collapsed: bool,
    responsive_open: bool,
    mobile_hidden: bool,
    breakpoint: Option<GridBreakpoint>,
    dark: bool,
    collapsible: bool,
    on_collapse: Option<CollapseChange>,
}
impl AntSider {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            theme: AntTheme::default(),
            width: 200.,
            collapsed_width: 80.,
            collapsed: false,
            responsive_open: false,
            mobile_hidden: false,
            breakpoint: None,
            dark: true,
            collapsible: false,
            on_collapse: None,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(0.);
        self
    }
    pub fn collapsed_width(mut self, width: f32) -> Self {
        self.collapsed_width = width.max(0.);
        self
    }
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
    /// Control whether the sidebar is expanded BELOW its responsive breakpoint.
    /// Separate from the desktop `collapsed` state; defaults to false.
    /// On user click, apply `responsive_open(!next_collapsed)` in narrow mode.
    pub fn responsive_open(mut self, open: bool) -> Self {
        self.responsive_open = open;
        self
    }
    pub fn breakpoint(mut self, breakpoint: GridBreakpoint) -> Self {
        self.breakpoint = Some(breakpoint);
        self
    }
    /// Hide the inline Sider below its breakpoint, so a mobile Sheet can own
    /// navigation without taking permanent width from the main content.
    /// Defaults to false for backwards compatibility.
    pub fn mobile_hidden(mut self, enabled: bool) -> Self {
        self.mobile_hidden = enabled;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }
    pub fn on_collapse(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_collapse = Some(Rc::new(handler));
        self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    /// Whether the current viewport is below the configured breakpoint.
    pub fn is_responsive_at(&self, viewport: gpui_kit::Pixels) -> bool {
        self.breakpoint
            .is_some_and(|b| viewport < px(b.min_width()))
    }
    /// Effective state: responsive and desktop toggles are independent.
    /// The user can expand a collapsed narrow sidebar with `responsive_open`.
    pub fn is_collapsed_at(&self, viewport: gpui_kit::Pixels) -> bool {
        if self.is_responsive_at(viewport) {
            !self.responsive_open
        } else {
            self.collapsed
        }
    }
    /// True only when responsive mobile hiding was explicitly requested.
    pub fn is_hidden_at(&self, viewport: gpui_kit::Pixels) -> bool {
        self.mobile_hidden && self.is_responsive_at(viewport)
    }
    /// Effective sidebar width at the supplied logical viewport.
    pub fn width_at(&self, viewport: gpui_kit::Pixels) -> f32 {
        if self.is_hidden_at(viewport) {
            return 0.;
        }
        if self.is_collapsed_at(viewport) {
            self.collapsed_width
        } else {
            self.width
        }
    }
}
impl RenderOnce for AntSider {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let collapsed = self.is_collapsed_at(window.viewport_size().width);
        let hidden = self.is_hidden_at(window.viewport_size().width);
        let width = self.width_at(window.viewport_size().width);
        let comp = self.theme.component(AntComponent::Layout);
        let fg = comp.color_text.unwrap_or(if self.dark {
            self.theme.layout_dark_text
        } else {
            self.theme.text
        });
        let bg = comp.color_bg.unwrap_or(if self.dark {
            self.theme.layout_dark
        } else {
            self.theme.surface
        });
        let button_fg = fg;
        div()
            .flex()
            .flex_col()
            .flex_none()
            .min_h(px(0.))
            .w(px(width))
            .bg(bg)
            .text_color(fg)
            .when(!hidden, |this| {
                this.child(
                    div()
                        .flex_1()
                        .min_h(px(0.))
                        .flex()
                        .flex_col()
                        .children(self.children),
                )
            })
            .when(self.collapsible && !hidden, |this| {
                let requested = !collapsed;
                this.child(
                    BaseButton::new(self.id)
                        .accessibility_label(if collapsed {
                            "Expand sidebar"
                        } else {
                            "Collapse sidebar"
                        })
                        .w_full()
                        .h(px(40.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(bg)
                        .text_color(button_fg)
                        .hover(|this| {
                            this.bg(comp.color_bg_hover.unwrap_or(self.theme.layout_sider_hover))
                        })
                        .child(
                            AntIcon::new(if collapsed {
                                AntIconName::MenuUnfoldOutlined
                            } else {
                                AntIconName::MenuFoldOutlined
                            })
                            .size(16.)
                            .color(button_fg)
                            .theme(self.theme),
                        )
                        .when(!collapsed, |this| this.child("Collapse"))
                        .when_some(self.on_collapse, |this, listener| {
                            this.on_click(move |_, window, cx| {
                                listener(requested, window, cx);
                            })
                        }),
                )
            })
    }
}

/// A mobile navigation Drawer built on GPUI Kit's native, focus-managed Sheet.
///
/// This is intentionally a trigger plus a **rebuildable content factory**:
/// GPUI Kit can rebuild a Sheet multiple times while open. Consumers should
/// capture existing Entity state rather than moving a one-shot AnyElement into
/// the factory. The title bar, Escape, mask dismissal and focus restoration
/// belong to the native GPUI Sheet infrastructure, not an HTML overlay.
///
/// Pair with `AntSider::mobile_hidden(true).breakpoint(GridBreakpoint::Md)`.
#[derive(IntoElement)]
pub struct AntMobileDrawer {
    id: ElementId,
    title: SharedString,
    theme: AntTheme,
    breakpoint: GridBreakpoint,
    width: f32,
    mask_closable: bool,
    content: Option<DrawerBody>,
    on_open_change: Option<CollapseChange>,
}
impl AntMobileDrawer {
    /// Creates a left-hand mobile drawer; only shows the trigger below `Md`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: "Navigation".into(),
            theme: AntTheme::default(),
            breakpoint: GridBreakpoint::Md,
            width: 280.,
            mask_closable: true,
            content: None,
            on_open_change: None,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }
    pub fn breakpoint(mut self, breakpoint: GridBreakpoint) -> Self {
        self.breakpoint = breakpoint;
        self
    }
    /// GPUI logical pixels. Clamped to a usable 120..520 range.
    pub fn width(mut self, width: f32) -> Self {
        self.width = if width.is_finite() {
            width.clamp(120., 520.)
        } else {
            280.
        };
        self
    }
    pub fn mask_closable(mut self, closable: bool) -> Self {
        self.mask_closable = closable;
        self
    }
    /// Rebuild the Sheet's children on demand. Capture persistent GPUI Entity
    /// handles outside this callback; do not allocate fresh editors inside it.
    pub fn content<F, E>(mut self, factory: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> E + 'static,
        E: IntoElement,
    {
        self.content = Some(Rc::new(move |window, cx| {
            factory(window, cx).into_any_element()
        }));
        self
    }
    /// Reports `true` when Sheet opens, `false` on mask, X or Escape close.
    /// The Sheet itself owns open state; consumer callbacks may update app UI.
    pub fn on_open_change(
        mut self,
        callback: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(callback));
        self
    }
    pub fn is_visible_at(&self, viewport: gpui_kit::Pixels) -> bool {
        viewport < px(self.breakpoint.min_width())
    }
}
impl RenderOnce for AntMobileDrawer {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let visible = self.is_visible_at(window.viewport_size().width);
        let tokens = self.theme.component(AntComponent::Layout);
        let theme = self.theme;
        let width = self.width;
        let title = self.title;
        let body = self.content;
        let mask_closable = self.mask_closable;
        let on_open_change = self.on_open_change;
        div().when(visible, |this| {
            this.child(
                BaseButton::new(self.id)
                    .accessibility_label("Open navigation drawer")
                    .w(px(34.))
                    .h(px(34.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(tokens.border_radius.unwrap_or(theme.border_radius)))
                    .bg(tokens.color_bg.unwrap_or(theme.surface))
                    .text_color(tokens.color_text.unwrap_or(theme.text))
                    .hover(|button| button.bg(tokens.color_bg_hover.unwrap_or(theme.info_bg)))
                    .child(
                        AntIcon::new(AntIconName::MenuOutlined)
                            .size(19.)
                            .color(tokens.color_text.unwrap_or(theme.text))
                            .theme(theme),
                    )
                    .on_click(move |_, window, cx| {
                        let title = title.clone();
                        let body = body.clone();
                        let close_handler = on_open_change.clone();
                        window.open_sheet_at(Placement::Left, cx, move |sheet, window, cx| {
                            let on_close = close_handler.clone();
                            let mut sheet = sheet
                                .title(title.clone())
                                .size(px(width))
                                .resizable(false)
                                .overlay(true)
                                .overlay_closable(mask_closable)
                                .bg(theme.surface)
                                .text_color(theme.text)
                                .border_color(theme.border)
                                .on_close(move |_, window, cx| {
                                    if let Some(callback) = &on_close {
                                        callback(false, window, cx);
                                    }
                                });
                            if let Some(factory) = &body {
                                sheet = sheet.child(factory(window, cx));
                            }
                            sheet
                        });
                        if let Some(callback) = &on_open_change {
                            callback(true, window, cx);
                        }
                    }),
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
    scrollable: bool,
    scroll_id: ElementId,
}
impl Default for AntContent {
    fn default() -> Self {
        Self::new()
    }
}
impl AntContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            theme: AntTheme::default(),
            padding: 24.,
            surface: false,
            scrollable: false,
            scroll_id: "ant-content-scroll".into(),
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.);
        self
    }
    pub fn surface(mut self, surface: bool) -> Self {
        self.surface = surface;
        self
    }
    /// Scroll Content independently of Header/Footer/Sider in an explicit-height Layout.
    /// Only content scrolls; use `scroll_id` if multiple scrollable bodies exist.
    pub fn scrollable(mut self, enabled: bool) -> Self {
        self.scrollable = enabled;
        self
    }
    /// Stable native GPUI scroll element identity, unique in the current view.
    pub fn scroll_id(mut self, id: impl Into<ElementId>) -> Self {
        self.scroll_id = id.into();
        self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}
impl RenderOnce for AntContent {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id(self.scroll_id)
            .flex_1()
            .min_h(px(0.))
            .min_w_0()
            .p(px(self.padding))
            .bg(if self.surface {
                self.theme.surface
            } else {
                self.theme.layout_body
            })
            .text_color(self.theme.text)
            .when(self.scrollable, |this| this.overflow_y_scroll())
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
impl Default for AntFooter {
    fn default() -> Self {
        Self::new()
    }
}
impl AntFooter {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            theme: AntTheme::default(),
            padding: 20.,
            center: true,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.);
        self
    }
    pub fn center(mut self, center: bool) -> Self {
        self.center = center;
        self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}
impl RenderOnce for AntFooter {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_none()
            .items_center()
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
        let sider = AntSider::new("sidebar")
            .width(240.)
            .collapsed_width(64.)
            .breakpoint(GridBreakpoint::Md);
        assert_eq!(sider.width_at(px(767.)), 64.);
        assert_eq!(sider.width_at(px(768.)), 240.);
        assert_eq!(sider.width_at(px(2000.)), 240.);
        assert!(!sider.collapsed);
        let forced = sider.collapsed(true);
        assert_eq!(forced.width_at(px(1000.)), 64.);
    }
    #[test]
    fn responsive_manual_expansion_is_separate_from_desktop_collapse() {
        let sider = AntSider::new("sidebar")
            .width(220.)
            .collapsed_width(64.)
            .collapsed(true)
            .breakpoint(GridBreakpoint::Md);
        assert!(sider.is_responsive_at(px(767.)));
        assert!(!sider.is_responsive_at(px(768.)));
        assert!(sider.is_collapsed_at(px(767.)));
        assert!(sider.is_collapsed_at(px(768.)));
        let mobile_open = sider.responsive_open(true);
        assert_eq!(mobile_open.width_at(px(600.)), 220.);
        assert_eq!(mobile_open.width_at(px(992.)), 64.);
        assert!(!mobile_open.is_collapsed_at(px(600.)));
        assert!(mobile_open.is_collapsed_at(px(992.)));
    }
    #[test]
    fn sider_width_and_header_height_clamp_to_nonnegative() {
        let sider = AntSider::new("sidebar").width(-20.).collapsed_width(-12.);
        assert_eq!(sider.width_at(px(1100.)), 0.);
        assert_eq!(sider.collapsed(true).width_at(px(1100.)), 0.);
        assert_eq!(AntHeader::new().height(-200.).height, 0.);
        assert_eq!(AntFooter::new().padding(-1.).padding, 0.);
    }
    #[test]
    fn mobile_hidden_sider_and_drawer_breakpoint_are_independent() {
        let sider = AntSider::new("sidebar")
            .width(240.)
            .collapsed_width(64.)
            .breakpoint(GridBreakpoint::Md)
            .mobile_hidden(true);
        assert!(sider.is_hidden_at(px(767.)));
        assert!(!sider.is_hidden_at(px(768.)));
        assert_eq!(sider.width_at(px(375.)), 0.);
        assert_eq!(sider.width_at(px(1200.)), 240.);
        assert!(!AntSider::new("legacy").mobile_hidden);
        let drawer = AntMobileDrawer::new("mobile")
            .width(280.)
            .breakpoint(GridBreakpoint::Md);
        assert!(drawer.is_visible_at(px(767.)));
        assert!(!drawer.is_visible_at(px(768.)));
        assert_eq!(drawer.width, 280.);
        assert_eq!(drawer.width(900.).width, 520.);
        assert_eq!(AntMobileDrawer::new("tiny").width(-10.).width, 120.);
        assert_eq!(AntMobileDrawer::new("nan").width(f32::NAN).width, 280.);
    }

    #[test]
    fn nested_layout_direction_and_dimensions() {
        let layout = AntLayout::new().horizontal(true).height(420.);
        assert!(layout.horizontal);
        assert_eq!(layout.height, Some(420.));
        assert_eq!(AntContent::new().padding(-3.).padding, 0.);
        let content = AntContent::new()
            .scrollable(true)
            .scroll_id("dashboard-content");
        assert!(content.scrollable);
    }
}
