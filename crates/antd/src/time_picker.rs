//! Native keyboard-editable Ant Design TimePicker surface.
//!
//! GPUI Kit's segmented TimeFieldState owns hour/minute/second editing,
//! 12/24-hour cycles and TimeFieldEvent::Change. Applications create the state
//! once and subscribe to it; this wrapper never reconstructs time on redraw.
//! A dropdown clock panel, ranges and clear-to-empty still need parity work.

use gpui_kit::{
    App, Entity, IntoElement, ParentElement as _, RenderOnce, Styled as _,
    Window, div, px,
};
use gpui_kit::component::{
    Disableable as _, Sizable as _, Size,
    time_field::{TimeField, TimeFieldState},
};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntTheme, AntIcon, AntIconName, ControlSize, InputStatus};

#[derive(IntoElement)]
pub struct AntTimePicker {
    state:Entity<TimeFieldState>,
    theme:AntTheme,
    size:ControlSize,
    disabled:bool,
    status:InputStatus,
    show_icon:bool,
}
impl AntTimePicker{
    pub fn new(state:&Entity<TimeFieldState>)->Self{
        Self{state:state.clone(),theme:AntTheme::default(),size:ControlSize::Middle,
            disabled:false,status:InputStatus::Normal,show_icon:true}
    }
    pub fn theme(mut self,t:AntTheme)->Self{self.theme=t;self}
    pub fn size(mut self,s:ControlSize)->Self{self.size=s;self}
    pub fn disabled(mut self,d:bool)->Self{self.disabled=d;self}
    pub fn status(mut self,status:InputStatus)->Self{self.status=status;self}
    pub fn show_icon(mut self,icon:bool)->Self{self.show_icon=icon;self}
}
impl RenderOnce for AntTimePicker {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement{
        let theme=self.theme;
        let kit_size=match self.size {
            ControlSize::Small=>Size::Small,
            ControlSize::Middle=>Size::Medium,
            ControlSize::Large=>Size::Large,
        };
        let border=match self.status{
            InputStatus::Normal=>theme.border,
            InputStatus::Error=>theme.error,
            InputStatus::Warning=>theme.warning,
        };
        div().flex().items_center().gap(px(8.))
            .child(TimeField::new(&self.state)
                .with_size(kit_size)
                .disabled(self.disabled)
                .invalid(self.status==InputStatus::Error)
                .rounded(px(theme.border_radius))
                .border_color(border)
                .bg(if self.disabled{theme.disabled_bg}else{theme.surface})
                .text_color(if self.disabled{theme.disabled_text}else{theme.text}))
            .when(self.show_icon,|this|this.child(
                AntIcon::new(AntIconName::ClockCircleOutlined)
                    .theme(theme).size(16.)
                    .color(if self.disabled{theme.disabled_text}else{theme.text_secondary})))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_status_shared_with_input_validation(){
        assert_ne!(InputStatus::Error,InputStatus::Warning);
        assert_eq!(InputStatus::default(),InputStatus::Normal);
    }
}
