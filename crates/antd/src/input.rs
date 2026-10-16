//! Ant Design-style input frame around GPUI Kit's production text engine.
//! Actual selection, clipboard, IME and editing stay in gpui-component InputState.

use gpui_kit::{App, Entity, IntoElement, RenderOnce, Styled as _, Window, px};
use gpui_kit::component::{Sizable, Size, input::{Input, InputState}};
use crate::controls::ControlSize;
use crate::tokens::AntTheme;

#[derive(IntoElement)]
pub struct AntInput {
    state:Entity<InputState>,
    theme:AntTheme,
    size:ControlSize,
    disabled:bool,
    clearable:bool,
}
impl AntInput {
    pub fn new(state:&Entity<InputState>)->Self {
        Self {state:state.clone(),theme:AntTheme::default(),size:ControlSize::Middle,disabled:false,clearable:false}
    }
    pub fn theme(mut self,t:AntTheme)->Self{self.theme=t;self}
    pub fn size(mut self,size:ControlSize)->Self{self.size=size;self}
    pub fn disabled(mut self,disabled:bool)->Self{self.disabled=disabled;self}
    pub fn clearable(mut self,clearable:bool)->Self{self.clearable=clearable;self}
}
impl RenderOnce for AntInput {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let size=match self.size{ControlSize::Small=>Size::Small,ControlSize::Middle=>Size::Medium,ControlSize::Large=>Size::Large};
        Input::new(&self.state).with_size(size).disabled(self.disabled).cleanable(self.clearable)
            .rounded(px(self.theme.border_radius)).border_color(self.theme.border)
            .bg(if self.disabled{self.theme.disabled_bg}else{self.theme.surface})
    }
}
