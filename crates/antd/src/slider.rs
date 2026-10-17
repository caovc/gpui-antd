//! Native Ant Design slider appearance around GPUI Kit drag/touch slider.
//! GPUI Kit owns pointer interactions and SliderEvent::Change/Release.
//! This wrapper adds focusable arrow/Home/End/Page keys and AntTheme styling.

use gpui_kit::{
    App, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    KeyDownEvent, ParentElement as _, RenderOnce, Role,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};
use gpui_kit::component::slider::{Slider, SliderEvent, SliderState, SliderValue};
use gpui_kit::prelude::FluentBuilder as _;
use crate::AntTheme;

/// Clamp a keyboard increment to an integer or decimal SliderState step.
/// Reject invalid intervals instead of propagating NaN into layout or value.
pub fn slider_key_value(current:f32,min:f32,max:f32,step:f32,key:&str)->Option<f32>{
    if !current.is_finite() || !min.is_finite() || !max.is_finite()
        || !step.is_finite() || step<=0. || max<=min {return None;}
    let value=match key{
        "left"|"down" => current-step,
        "right"|"up" => current+step,
        "pageup" => current+step*10.,
        "pagedown" => current-step*10.,
        "home" => min,
        "end" => max,
        _=>return None,
    }.clamp(min,max);
    let steps=((value-min)/step).round();
    let snapped=(min+steps*step).clamp(min,max);
    Some(if matches!(key,"end") {max} else if matches!(key,"home"){min}
        else {(snapped*10000.).round()/10000.})
}

#[derive(IntoElement)]
pub struct AntSlider {
    state: Entity<SliderState>,
    theme: AntTheme,
    disabled: bool,
    vertical: bool,
    reverse: bool,
    show_value: bool,
    height: f32,
}
impl AntSlider {
    pub fn new(state:&Entity<SliderState>)->Self{
        Self{state:state.clone(),theme:AntTheme::default(),disabled:false,
            vertical:false,reverse:false,show_value:false,height:180.}
    }
    pub fn theme(mut self,t:AntTheme)->Self{self.theme=t;self}
    pub fn disabled(mut self,v:bool)->Self{self.disabled=v;self}
    pub fn vertical(mut self,v:bool)->Self{self.vertical=v;self}
    pub fn reverse(mut self,v:bool)->Self{self.reverse=v;self}
    pub fn show_value(mut self,v:bool)->Self{self.show_value=v;self}
    pub fn height(mut self,v:f32)->Self{self.height=v.max(60.);self}
}
impl RenderOnce for AntSlider {
    fn render(self,window:&mut Window,cx:&mut App)->impl IntoElement{
        let t=self.theme;
        let vertical=self.vertical;
        let disabled=self.disabled;
        let state=self.state;
        let value=state.read(cx).value();
        let id=format!("ant-slider-{:?}",state.entity_id());
        let focus:FocusHandle=window.use_keyed_state(id.clone(),cx,|_,cx|{
            cx.focus_handle().tab_stop(true)
        }).read(cx).clone();
        let key_state=state.clone();
        let key_focus=focus.clone();
        let handle_key=move |event:&KeyDownEvent,window:&mut Window,cx:&mut App|{
            if disabled || !key_focus.is_focused(window)
                || event.keystroke.modifiers.modified()
                || event.keystroke.is_ime_in_progress(){return;}
            let current=key_state.read(cx).value();
            // The range slider continues to use Kit's native pointer model.
            let SliderValue::Single(current)=current else{return;};
            let model=key_state.read(cx);
            let next=slider_key_value(current,model.min_value(),model.max_value(),
                model.step_value(),event.keystroke.key.as_str());
            drop(model);
            if let Some(next)=next{
                if (next-current).abs()>f32::EPSILON{
                    key_state.update(cx,|model,cx|{
                        model.set_value(next,window,cx);
                        cx.emit(SliderEvent::Change(SliderValue::Single(next)));
                    });
                }
                window.prevent_default();cx.stop_propagation();
            }
        };
        let mut slider=Slider::new(&state).disabled(disabled)
            .bg(t.primary).text_color(t.surface).rounded_full();
        if vertical{slider=slider.vertical();}else{slider=slider.horizontal();}
        if self.reverse{slider=slider.reverse();}
        let slider=if vertical{slider.h(px(self.height)).w(px(22.))}
            else{slider.w_full().h(px(26.))};
        div().id(id).role(Role::Slider)
            .track_focus(&focus)
            .on_key_down(handle_key)
            .when(!disabled,|this|this.cursor_pointer())
            .when(vertical,|this|this.flex().items_center().gap(px(12.)))
            .when(!vertical,|this|this.flex().items_center().gap(px(14.)).w_full())
            .child(slider)
            .when(self.show_value,|this|this.child(
                div().text_size(px(13.))
                    .text_color(if disabled{t.disabled_text}else{t.text_secondary})
                    .child(value.to_string())))
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn slider_keyboard_clamps_and_quantizes(){
        assert_eq!(slider_key_value(0.,0.,100.,5.,"left"),Some(0.));
        assert_eq!(slider_key_value(50.,0.,100.,5.,"right"),Some(55.));
        assert_eq!(slider_key_value(50.,0.,100.,5.,"pageup"),Some(100.));
        assert_eq!(slider_key_value(49.,0.,100.,5.,"home"),Some(0.));
        assert_eq!(slider_key_value(49.,0.,100.,5.,"end"),Some(100.));
        assert_eq!(slider_key_value(0.2,0.,1.,0.1,"right"),Some(0.3));
    }
    #[test]
    fn slider_keyboard_rejects_invalid_inputs(){
        assert_eq!(slider_key_value(5.,0.,10.,0.,"up"),None);
        assert_eq!(slider_key_value(f32::NAN,0.,10.,1.,"up"),None);
        assert_eq!(slider_key_value(5.,10.,0.,1.,"up"),None);
        assert_eq!(slider_key_value(5.,0.,10.,1.,"space"),None);
    }
}
