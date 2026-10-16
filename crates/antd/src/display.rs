//! Ant Design display primitives.

use gpui_kit::{AnyElement, IntoElement, ParentElement, RenderOnce, SharedString, Styled as _, App, Window, div, px, relative};
use crate::tokens::{AntTheme, color};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Tone { #[default] Default, Blue, Green, Gold, Red, Purple }

fn tone_colors(tone: Tone, t: AntTheme) -> (gpui_kit::Hsla, gpui_kit::Hsla, gpui_kit::Hsla) {
    match tone {
        Tone::Default => (t.surface_secondary, t.text_secondary, t.border),
        Tone::Blue => (t.info_bg, t.primary, color(0x91caff)),
        Tone::Green => (t.success_bg, t.success, color(0xb7eb8f)),
        Tone::Gold => (t.warning_bg, color(0xd48806), color(0xffe58f)),
        Tone::Red => (t.error_bg, t.error, color(0xffccc7)),
        Tone::Purple => (color(0xf9f0ff), color(0x722ed1), color(0xd3adf7)),
    }
}

#[derive(IntoElement)]
pub struct AntTag {
    text: SharedString, tone: Tone, theme: AntTheme, bordered: bool,
}
impl AntTag {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self{text:text.into(),tone:Tone::Default,theme:AntTheme::default(),bordered:true}
    }
    pub fn tone(mut self,tone:Tone)->Self{self.tone=tone;self}
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
    pub fn bordered(mut self,yes:bool)->Self{self.bordered=yes;self}
}
impl RenderOnce for AntTag {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let (bg,fg,border)=tone_colors(self.tone,self.theme);
        div().flex().items_center().justify_center()
            .rounded(px(4.)).px(px(7.)).h(px(24.))
            .border_1().border_color(if self.bordered{border}else{bg})
            .bg(bg).text_color(fg).text_size(px(12.))
            .child(self.text)
    }
}

/// Compact standalone number badge.
#[derive(IntoElement)]
pub struct AntBadge {
    count:usize, dot:bool, color:gpui_kit::Hsla, theme:AntTheme,
}
impl AntBadge {
    pub fn count(count:usize)->Self {
        Self{count,dot:false,color:color(0xff4d4f),theme:AntTheme::default()}
    }
    pub fn dot(mut self)->Self{self.dot=true;self}
    pub fn theme(mut self,t:AntTheme)->Self{self.theme=t;self}
    pub fn color(mut self,c:gpui_kit::Hsla)->Self{self.color=c;self}
}
impl RenderOnce for AntBadge {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let label=if self.count>99{"99+".to_owned()}else{self.count.to_string()};
        div().flex().items_center().justify_center()
            .h(px(if self.dot{8.}else{20.}))
            .min_w(px(if self.dot{8.}else{20.}))
            .px(px(if self.dot{0.}else{5.}))
            .rounded_full().bg(self.color).text_color(color(0xffffff))
            .text_size(px(12.))
            .child(if self.dot{String::new()}else{label})
    }
}

/// Card presentation. Pass Rust GPUI elements as children.
#[derive(IntoElement)]
pub struct AntCard {
    title:Option<SharedString>,
    children:Vec<AnyElement>,
    theme:AntTheme,
    bordered:bool,
}
impl AntCard {
    pub fn new()->Self{Self{title:None,children:Vec::new(),theme:AntTheme::default(),bordered:true}}
    pub fn title(mut self,title:impl Into<SharedString>)->Self{self.title=Some(title.into());self}
    pub fn theme(mut self,t:AntTheme)->Self{self.theme=t;self}
    pub fn bordered(mut self,v:bool)->Self{self.bordered=v;self}
    pub fn child(mut self,element:impl IntoElement)->Self{self.children.push(element.into_any_element());self}
}
impl Default for AntCard {fn default()->Self{Self::new()}}
impl RenderOnce for AntCard {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let t=self.theme;
        div().flex().flex_col().rounded(px(8.))
            .border_1().border_color(if self.bordered{t.border}else{t.surface})
            .bg(t.surface)
            .when_some(self.title, |this,title| {
                this.child(div().h(px(56.)).flex().items_center()
                    .px(px(24.)).font_semibold().text_size(px(16.))
                    .text_color(t.text).border_b_1().border_color(t.border)
                    .child(title))
            })
            .child(div().p(px(24.)).text_size(px(14.)).text_color(t.text).children(self.children))
    }
}

#[derive(IntoElement)]
pub struct AntAlert {
    text:SharedString, tone:Tone, theme:AntTheme,
}
impl AntAlert {
    pub fn new(text:impl Into<SharedString>)->Self{Self{text:text.into(),tone:Tone::Blue,theme:AntTheme::default()}}
    pub fn tone(mut self,tone:Tone)->Self{self.tone=tone;self}
    pub fn theme(mut self,t:AntTheme)->Self{self.theme=t;self}
}
impl RenderOnce for AntAlert {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let (bg,fg,border)=tone_colors(self.tone,self.theme);
        let symbol=match self.tone {Tone::Green=>"✓",Tone::Gold=>"!",Tone::Red=>"×",_=>"i"};
        div().flex().items_center().gap(px(10.))
            .px(px(15.)).py(px(10.))
            .rounded(px(8.)).border_1().border_color(border).bg(bg)
            .child(div().flex().items_center().justify_center()
                .size(px(16.)).rounded_full().bg(fg).text_color(color(0xffffff))
                .text_size(px(12.)).font_bold().child(symbol))
            .child(div().text_color(self.theme.text).text_size(px(14.)).child(self.text))
    }
}

#[derive(IntoElement)]
pub struct AntDivider{theme:AntTheme}
impl AntDivider {
    pub fn new()->Self{Self{theme:AntTheme::default()}}
    pub fn theme(mut self,t:AntTheme)->Self{self.theme=t;self}
}
impl Default for AntDivider {fn default()->Self{Self::new()}}
impl RenderOnce for AntDivider {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        div().w_full().h(px(1.)).bg(self.theme.border)
    }
}

#[derive(IntoElement)]
pub struct AntProgress{
    percent:f32, theme:AntTheme, tone:Tone, show_info:bool
}
impl AntProgress {
    pub fn new(percent:f32)->Self{Self{percent:percent.clamp(0.,100.),theme:AntTheme::default(),tone:Tone::Blue,show_info:true}}
    pub fn tone(mut self,tone:Tone)->Self{self.tone=tone;self}
    pub fn theme(mut self,t:AntTheme)->Self{self.theme=t;self}
    pub fn show_info(mut self,v:bool)->Self{self.show_info=v;self}
}
impl RenderOnce for AntProgress {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let color=match self.tone {Tone::Green=>self.theme.success,Tone::Gold=>self.theme.warning,Tone::Red=>self.theme.error,_=>self.theme.primary};
        div().flex().items_center().gap(px(12.)).w_full()
            .child(div().flex_1().h(px(8.)).rounded_full().bg(self.theme.disabled_bg)
                .child(div().w(relative(self.percent/100.)).h_full().rounded_full().bg(color)))
            .when(self.show_info,|this|this.child(div().w(px(40.)).text_size(px(14.)).text_color(self.theme.text_secondary).child(format!("{:.0}%",self.percent))))
    }
}
