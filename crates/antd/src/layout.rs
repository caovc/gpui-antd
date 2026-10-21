//! Native Ant Design layout primitives: Space, Flex and 24-column Grid.
//! All geometry is evaluated by GPUI's native/WASM layout engine.

use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window, div, px,
    relative,
};

/// Ant Design Space preset sizes, measured in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpaceSize {
    Small,
    Middle,
    Large,
    Custom(f32),
}
impl Default for SpaceSize {
    fn default() -> Self {
        Self::Small
    }
}
impl SpaceSize {
    pub fn pixels(self) -> f32 {
        match self {
            Self::Small => 8.,
            Self::Middle => 16.,
            Self::Large => 24.,
            Self::Custom(pixels) => pixels.max(0.),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexAlign {
    Start,
    #[default]
    Center,
    End,
    Stretch,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexJustify {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Space: uniform gaps between native GPUI elements, vertical or horizontal.
/// Use the wrapping mode for tag groups and button toolbars.
#[derive(IntoElement)]
pub struct AntSpace {
    children: Vec<AnyElement>,
    size: SpaceSize,
    vertical: bool,
    wrap: bool,
    align: FlexAlign,
}
impl Default for AntSpace {
    fn default() -> Self {
        Self::new()
    }
}
impl AntSpace {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            size: SpaceSize::Small,
            vertical: false,
            wrap: false,
            align: FlexAlign::Center,
        }
    }
    pub fn size(mut self, size: SpaceSize) -> Self {
        self.size = size;
        self
    }
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
    pub fn align(mut self, align: FlexAlign) -> Self {
        self.align = align;
        self
    }
    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.children.push(element.into_any_element());
        self
    }
    pub fn children(mut self, elements: impl IntoIterator<Item = AnyElement>) -> Self {
        self.children.extend(elements);
        self
    }
}
impl RenderOnce for AntSpace {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .flex()
            .gap(px(self.size.pixels()))
            .when(self.vertical, |this| this.flex_col())
            .when(self.wrap, |this| this.flex_wrap())
            .map(|this| match self.align {
                FlexAlign::Start => this.items_start(),
                FlexAlign::Center => this.items_center(),
                FlexAlign::End => this.items_end(),
                FlexAlign::Stretch => this.items_stretch(),
            })
            .children(self.children)
    }
}

/// Flex: configurable layout axis, gap, wrapping, alignment and distribution.
#[derive(IntoElement)]
pub struct AntFlex {
    children: Vec<AnyElement>,
    vertical: bool,
    wrap: bool,
    gap: f32,
    align: FlexAlign,
    justify: FlexJustify,
    fill: bool,
}
impl Default for AntFlex {
    fn default() -> Self {
        Self::new()
    }
}
impl AntFlex {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            vertical: false,
            wrap: false,
            gap: 0.,
            align: FlexAlign::Center,
            justify: FlexJustify::Start,
            fill: false,
        }
    }
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.);
        self
    }
    pub fn align(mut self, align: FlexAlign) -> Self {
        self.align = align;
        self
    }
    pub fn justify(mut self, justify: FlexJustify) -> Self {
        self.justify = justify;
        self
    }
    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }
    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.children.push(element.into_any_element());
        self
    }
}
impl RenderOnce for AntFlex {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .flex()
            .gap(px(self.gap))
            .when(self.vertical, |this| this.flex_col())
            .when(self.wrap, |this| this.flex_wrap())
            .when(self.fill, |this| this.w_full())
            .map(|this| match self.align {
                FlexAlign::Start => this.items_start(),
                FlexAlign::Center => this.items_center(),
                FlexAlign::End => this.items_end(),
                FlexAlign::Stretch => this.items_stretch(),
            })
            .map(|this| match self.justify {
                FlexJustify::Start => this.justify_start(),
                FlexJustify::Center => this.justify_center(),
                FlexJustify::End => this.justify_end(),
                FlexJustify::SpaceBetween => this.justify_between(),
                FlexJustify::SpaceAround => this.justify_around(),
                FlexJustify::SpaceEvenly => this.justify_evenly(),
            })
            .children(self.children)
    }
}

/// Official Ant Design viewport breakpoints in logical pixels.
///
/// GPUI's desktop and WASM viewports both expose logical dimensions, so
/// the breakpoint contract is shared across platforms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridBreakpoint {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}
impl GridBreakpoint {
    pub const ALL: [Self; 6] = [Self::Xs, Self::Sm, Self::Md, Self::Lg, Self::Xl, Self::Xxl];
    pub const fn min_width(self) -> f32 {
        match self {
            Self::Xs => 0.,
            Self::Sm => 576.,
            Self::Md => 768.,
            Self::Lg => 992.,
            Self::Xl => 1200.,
            Self::Xxl => 1600.,
        }
    }
    const fn index(self) -> usize {
        match self {
            Self::Xs => 0,
            Self::Sm => 1,
            Self::Md => 2,
            Self::Lg => 3,
            Self::Xl => 4,
            Self::Xxl => 5,
        }
    }
}

/// One column in Ant Design's 24-unit grid.
///
/// Configure desktop/mobile spans with `.xs(24).sm(12).md(8).lg(6)`.
/// The last applicable declared breakpoint overrides the base span.
/// Responsive spans are evaluated against `Window::viewport_size()` at
/// render time, so resizing native or browser windows updates the layout.
pub struct AntCol {
    span: u8,
    offset: u8,
    responsive: [Option<u8>; 6],
    children: Vec<AnyElement>,
}
impl AntCol {
    pub fn new(span: u8) -> Self {
        Self {
            span: span.min(24),
            offset: 0,
            responsive: [None; 6],
            children: Vec::new(),
        }
    }
    pub fn span(mut self, span: u8) -> Self {
        self.span = span.min(24);
        self
    }
    pub fn offset(mut self, offset: u8) -> Self {
        self.offset = offset.min(24);
        self
    }
    pub fn breakpoint(mut self, breakpoint: GridBreakpoint, span: u8) -> Self {
        self.responsive[breakpoint.index()] = Some(span.min(24));
        self
    }
    pub fn xs(self, span: u8) -> Self {
        self.breakpoint(GridBreakpoint::Xs, span)
    }
    pub fn sm(self, span: u8) -> Self {
        self.breakpoint(GridBreakpoint::Sm, span)
    }
    pub fn md(self, span: u8) -> Self {
        self.breakpoint(GridBreakpoint::Md, span)
    }
    pub fn lg(self, span: u8) -> Self {
        self.breakpoint(GridBreakpoint::Lg, span)
    }
    pub fn xl(self, span: u8) -> Self {
        self.breakpoint(GridBreakpoint::Xl, span)
    }
    pub fn xxl(self, span: u8) -> Self {
        self.breakpoint(GridBreakpoint::Xxl, span)
    }
    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.children.push(element.into_any_element());
        self
    }
    /// Resolve the effective column width from the viewport breakpoints.
    pub fn span_at(&self, width: gpui_kit::Pixels) -> u8 {
        let mut result = self.span;
        for breakpoint in GridBreakpoint::ALL {
            if width >= px(breakpoint.min_width()) {
                if let Some(value) = self.responsive[breakpoint.index()] {
                    result = value;
                }
            }
        }
        result
    }
    fn render_with_gutter(self, gutter: f32, viewport_width: gpui_kit::Pixels) -> AnyElement {
        let span = self.span_at(viewport_width);
        if span == 0 {
            return div().into_any_element();
        }
        div()
            .flex_none()
            .w(relative(span as f32 / 24.))
            .ml(relative(self.offset as f32 / 24.))
            .min_w_0()
            .px(px(gutter / 2.))
            .children(self.children)
            .into_any_element()
    }
}

/// Ant Design 24-column grid. Each Col supplies an independent span/offset.
/// Gutter is split across the columns rather than added as a Flex gap, so
/// e.g. spans 6+6+6+6 still fill exactly one logical row.
#[derive(IntoElement)]
pub struct AntRow {
    columns: Vec<AntCol>,
    gutter: f32,
    vertical_gutter: f32,
}
impl Default for AntRow {
    fn default() -> Self {
        Self::new()
    }
}
impl AntRow {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            gutter: 0.,
            vertical_gutter: 0.,
        }
    }
    pub fn gutter(mut self, gutter: f32) -> Self {
        self.gutter = gutter.max(0.);
        self
    }
    pub fn vertical_gutter(mut self, gap: f32) -> Self {
        self.vertical_gutter = gap.max(0.);
        self
    }
    pub fn child(mut self, col: AntCol) -> Self {
        self.columns.push(col);
        self
    }
    pub fn children(mut self, cols: impl IntoIterator<Item = AntCol>) -> Self {
        self.columns.extend(cols);
        self
    }
}
impl RenderOnce for AntRow {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        // Match CSS media-query breakpoints using the logical GPUI viewport.
        let viewport_width = window.viewport_size().width;
        div()
            .w_full()
            .flex()
            .flex_wrap()
            .gap_y(px(self.vertical_gutter))
            .children(
                self.columns
                    .into_iter()
                    .map(|col| col.render_with_gutter(self.gutter, viewport_width)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn space_sizes_and_custom_clamping() {
        assert_eq!(SpaceSize::default().pixels(), 8.);
        assert_eq!(SpaceSize::Middle.pixels(), 16.);
        assert_eq!(SpaceSize::Large.pixels(), 24.);
        assert_eq!(SpaceSize::Custom(-3.).pixels(), 0.);
        assert_eq!(SpaceSize::Custom(13.).pixels(), 13.);
    }
    #[test]
    fn responsive_breakpoints_match_ant_design() {
        let col = AntCol::new(24).sm(12).md(8).lg(6).xxl(4);
        for (width, expected) in [
            (0., 24),
            (575., 24),
            (576., 12),
            (767., 12),
            (768., 8),
            (991., 8),
            (992., 6),
            (1599., 6),
            (1600., 4),
        ] {
            assert_eq!(col.span_at(px(width)), expected, "viewport={width}");
        }
        // Sparse declarations inherit the latest eligible override.
        let sparse = AntCol::new(18).md(8);
        assert_eq!(sparse.span_at(px(650.)), 18);
        assert_eq!(sparse.span_at(px(2000.)), 8);
    }

    #[test]
    fn zero_span_and_out_of_range_values_are_safe() {
        let col = AntCol::new(24).xs(0).md(200);
        assert_eq!(col.span_at(px(300.)), 0);
        assert_eq!(col.span_at(px(900.)), 24);
        assert_eq!(GridBreakpoint::Sm.min_width(), 576.);
    }

    #[test]
    fn grid_spans_clamp_to_24() {
        assert_eq!(AntCol::new(12).span, 12);
        assert_eq!(AntCol::new(30).span, 24);
        assert_eq!(AntCol::new(7).offset(30).offset, 24);
        assert_eq!(AntCol::new(24).span(0).span, 0);
    }
}
