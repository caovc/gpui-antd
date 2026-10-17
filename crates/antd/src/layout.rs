//! Native Ant Design layout primitives: Space, Flex and 24-column Grid.
//! All geometry is evaluated by GPUI's native/WASM layout engine.

use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement as _, RenderOnce, Window, div, px, relative,
};
use gpui_kit::prelude::FluentBuilder as _;

/// Ant Design Space preset sizes, measured in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpaceSize {
    Small,
    Middle,
    Large,
    Custom(f32),
}
impl Default for SpaceSize { fn default() -> Self { Self::Small } }
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
impl Default for AntSpace { fn default() -> Self { Self::new() } }
impl AntSpace {
    pub fn new() -> Self {
        Self { children: Vec::new(), size: SpaceSize::Small,
            vertical: false, wrap: false, align: FlexAlign::Center }
    }
    pub fn size(mut self, size: SpaceSize) -> Self { self.size = size; self }
    pub fn vertical(mut self, vertical: bool) -> Self { self.vertical = vertical; self }
    pub fn wrap(mut self, wrap: bool) -> Self { self.wrap = wrap; self }
    pub fn align(mut self, align: FlexAlign) -> Self { self.align = align; self }
    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.children.push(element.into_any_element()); self
    }
    pub fn children(mut self, elements: impl IntoIterator<Item=AnyElement>) -> Self {
        self.children.extend(elements); self
    }
}
impl RenderOnce for AntSpace {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().flex().gap(px(self.size.pixels()))
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
impl Default for AntFlex { fn default() -> Self { Self::new() } }
impl AntFlex {
    pub fn new() -> Self {
        Self { children:Vec::new(),vertical:false,wrap:false,gap:0.,
            align:FlexAlign::Center,justify:FlexJustify::Start,fill:false }
    }
    pub fn vertical(mut self, vertical: bool) -> Self { self.vertical=vertical; self }
    pub fn wrap(mut self, wrap: bool) -> Self { self.wrap=wrap; self }
    pub fn gap(mut self, gap: f32) -> Self { self.gap=gap.max(0.); self }
    pub fn align(mut self, align: FlexAlign) -> Self { self.align=align; self }
    pub fn justify(mut self, justify: FlexJustify) -> Self { self.justify=justify; self }
    pub fn fill(mut self, fill: bool) -> Self { self.fill=fill; self }
    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.children.push(element.into_any_element()); self
    }
}
impl RenderOnce for AntFlex {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().flex().gap(px(self.gap))
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

/// One column in Ant Design's 24-unit grid.
pub struct AntCol {
    span: u8,
    offset: u8,
    children: Vec<AnyElement>,
}
impl AntCol {
    pub fn new(span: u8) -> Self {
        Self { span:span.min(24),offset:0,children:Vec::new() }
    }
    pub fn span(mut self, span: u8) -> Self { self.span=span.min(24);self }
    pub fn offset(mut self, offset: u8) -> Self { self.offset=offset.min(24);self }
    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.children.push(element.into_any_element());self
    }
    fn render_with_gutter(self, gutter: f32) -> AnyElement {
        div().flex_none()
            .w(relative(self.span as f32 / 24.))
            .ml(relative(self.offset as f32 / 24.))
            .min_w_0()
            .px(px(gutter/2.))
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
impl Default for AntRow { fn default() -> Self { Self::new() } }
impl AntRow {
    pub fn new() -> Self { Self {columns:Vec::new(),gutter:0.,vertical_gutter:0.} }
    pub fn gutter(mut self, gutter: f32) -> Self { self.gutter=gutter.max(0.);self }
    pub fn vertical_gutter(mut self, gap: f32) -> Self { self.vertical_gutter=gap.max(0.);self }
    pub fn child(mut self, col: AntCol) -> Self {self.columns.push(col);self}
    pub fn children(mut self, cols: impl IntoIterator<Item=AntCol>) -> Self {
        self.columns.extend(cols);self
    }
}
impl RenderOnce for AntRow {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().w_full().flex().flex_wrap()
            .gap_y(px(self.vertical_gutter))
            .children(self.columns.into_iter().map(|col| col.render_with_gutter(self.gutter)))
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
    fn grid_spans_clamp_to_24() {
        assert_eq!(AntCol::new(12).span, 12);
        assert_eq!(AntCol::new(30).span, 24);
        assert_eq!(AntCol::new(7).offset(30).offset, 24);
        assert_eq!(AntCol::new(24).span(0).span, 0);
    }
}
