//! Ant Design FloatButton and controlled expandable FloatButton.Group.
//!
//! All controls are GPUI Base buttons; keyboard focus, Enter/Space, disabled
//! state and accessible names remain native. Attach to a relatively positioned
//! GPUI container. There is no browser DOM overlay or fake floating widget.

use std::rc::Rc;
use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, Styled as _, Window, div, px,
};
use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntIcon, AntIconName, AntTheme, color};
use crate::design_tokens::AntComponent;

type ButtonClick = Rc<dyn Fn(&ClickEvent,&mut Window,&mut App)>;
type OpenChange = Rc<dyn Fn(bool,&mut Window,&mut App)>;
type ActionSelect = Rc<dyn Fn(SharedString,&mut Window,&mut App)>;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FloatShape { #[default] Circle, Square }
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FloatVariant { #[default] Default, Primary, Danger }

/// Avoid invalid offsets creating invisible or out-of-bounds elements.
pub fn float_offset(value: f32) -> f32 {
    if value.is_finite() { value.clamp(0.,2048.) } else { 24. }
}

/// Standalone floating action, anchored inside its nearest relative parent.
/// See AntFloatButtonGroup for expanding/collapsing a list of actions.
#[derive(IntoElement)]
pub struct AntFloatButton {
    id: ElementId,
    label: SharedString,
    icon: AntIconName,
    shape: FloatShape,
    variant: FloatVariant,
    theme: AntTheme,
    right: f32,
    bottom: f32,
    disabled: bool,
    badge: Option<usize>,
    on_click: Option<ButtonClick>,
}
impl AntFloatButton {
    pub fn new(id:impl Into<ElementId>,label:impl Into<SharedString>)->Self{
        Self{id:id.into(),label:label.into(),icon:AntIconName::PlusOutlined,
            shape:FloatShape::Circle,variant:FloatVariant::Default,
            theme:AntTheme::default(),right:24.,bottom:24.,disabled:false,
            badge:None,on_click:None}
    }
    pub fn icon(mut self,icon:AntIconName)->Self{self.icon=icon;self}
    pub fn shape(mut self,shape:FloatShape)->Self{self.shape=shape;self}
    pub fn variant(mut self,variant:FloatVariant)->Self{self.variant=variant;self}
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
    pub fn position(mut self,right:f32,bottom:f32)->Self{
        self.right=float_offset(right);self.bottom=float_offset(bottom);self
    }
    pub fn disabled(mut self,disabled:bool)->Self{self.disabled=disabled;self}
    pub fn badge(mut self,count:usize)->Self{self.badge=Some(count.min(99));self}
    pub fn on_click(mut self,callback:impl Fn(&ClickEvent,&mut Window,&mut App)+'static)->Self{
        self.on_click=Some(Rc::new(callback));self
    }
}
impl RenderOnce for AntFloatButton {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement{
        let t=self.theme;
        let o=t.component(AntComponent::FloatButton);
        let primary=o.color_primary.unwrap_or(t.primary);
        let surface=o.color_bg.unwrap_or(t.surface);
        let text=o.color_text.unwrap_or(t.text);
        let border=o.color_border.unwrap_or(t.border);
        let (bg,fg)=match self.variant {
            FloatVariant::Default=>(surface,text),
            FloatVariant::Primary=>(primary,color(0xffffff)),
            FloatVariant::Danger=>(t.error,color(0xffffff)),
        };
        let height=o.control_height.unwrap_or(40.).clamp(24.,72.);
        let radius=if self.shape==FloatShape::Circle{height/2.}
            else{o.border_radius.unwrap_or(t.border_radius)};
        let size=px(height);
        let icon=AntIcon::new(self.icon).theme(t).size(height*0.48).color(fg);
        let click=self.on_click;
        div().absolute().right(px(self.right)).bottom(px(self.bottom))
            .child(div().relative()
                .child(BaseButton::new(self.id)
                    .accessibility_label(self.label)
                    .disabled(self.disabled)
                    .size(size).flex().items_center().justify_center()
                    .border_1().border_color(border)
                    .rounded(px(radius)).bg(bg).text_color(fg)
                    .when(!self.disabled,|this|this.cursor_pointer()
                        .hover(|this|this.bg(t.surface_secondary)))
                    .child(icon)
                    .when_some(click,|this,handler|this.on_click(move|event,window,cx|{
                        handler(event,window,cx);
                    })))
                .when_some(self.badge.filter(|count|*count>0),|this,count|{
                    this.child(div().absolute().top(px(-7.)).right(px(-7.))
                        .min_w(px(18.)).h(px(18.)).px(px(3.))
                        .flex().items_center().justify_center()
                        .rounded_full().bg(t.error).text_color(color(0xffffff))
                        .text_size(px(10.)).child(count.to_string()))
                }))
    }
}

/// A child action in FloatButton.Group, with a stable callback key.
pub struct AntFloatAction {
    key:SharedString,
    label:SharedString,
    icon:AntIconName,
    disabled:bool,
    danger:bool,
}
impl AntFloatAction {
    pub fn new(key:impl Into<SharedString>,label:impl Into<SharedString>,
        icon:AntIconName)->Self{
        Self{key:key.into(),label:label.into(),icon,disabled:false,danger:false}
    }
    pub fn disabled(mut self,yes:bool)->Self{self.disabled=yes;self}
    pub fn danger(mut self,yes:bool)->Self{self.danger=yes;self}
}

/// Controlled FloatButton.Group. The caller owns "open" and must apply
/// on_open_change. Individual action clicks invoke on_select then request
/// closing. The floating group is anchored inside a relative GPUI parent.
#[derive(IntoElement)]
pub struct AntFloatButtonGroup {
    id:ElementId,
    label:SharedString,
    icon:AntIconName,
    actions:Vec<AntFloatAction>,
    open:bool,
    disabled:bool,
    right:f32,
    bottom:f32,
    theme:AntTheme,
    on_open_change:Option<OpenChange>,
    on_select:Option<ActionSelect>,
}
impl AntFloatButtonGroup {
    pub fn new(id:impl Into<ElementId>,label:impl Into<SharedString>)->Self{
        Self{id:id.into(),label:label.into(),icon:AntIconName::AppstoreOutlined,
            actions:Vec::new(),open:false,disabled:false,right:24.,bottom:24.,
            theme:AntTheme::default(),on_open_change:None,on_select:None}
    }
    pub fn icon(mut self,icon:AntIconName)->Self{self.icon=icon;self}
    pub fn action(mut self,action:AntFloatAction)->Self{self.actions.push(action);self}
    pub fn open(mut self,yes:bool)->Self{self.open=yes;self}
    pub fn disabled(mut self,yes:bool)->Self{self.disabled=yes;self}
    pub fn position(mut self,right:f32,bottom:f32)->Self{
        self.right=float_offset(right);self.bottom=float_offset(bottom);self
    }
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
    pub fn on_open_change(mut self,f:impl Fn(bool,&mut Window,&mut App)+'static)->Self{
        self.on_open_change=Some(Rc::new(f));self
    }
    pub fn on_select(mut self,f:impl Fn(SharedString,&mut Window,&mut App)+'static)->Self{
        self.on_select=Some(Rc::new(f));self
    }
}
impl RenderOnce for AntFloatButtonGroup {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement{
        let t=self.theme;
        let token=t.component(AntComponent::FloatButton);
        let prefix=format!("{:?}",self.id);
        let disabled=self.disabled;
        let open=self.open;
        let primary=token.color_primary.unwrap_or(t.primary);
        let surface=token.color_bg.unwrap_or(t.surface);
        let border=token.color_border.unwrap_or(t.border);
        let on_toggle=self.on_open_change;
        let on_select=self.on_select;
        div().absolute().right(px(self.right)).bottom(px(self.bottom))
            .flex().flex_col().items_end().gap(px(10.))
            .when(open,|this|this.children(
                self.actions.into_iter().enumerate().map(|(i,action)|{
                    let key=action.key;
                    let label=action.label;
                    let disabled_action=disabled||action.disabled;
                    let fg=if action.danger{t.error}else{t.text};
                    let chosen=on_select.clone();
                    let close=on_toggle.clone();
                    let action_key=key.clone();
                    let action_text=label.clone();
                    div().flex().items_center().gap(px(8.))
                        .child(div().px(px(8.)).py(px(4.))
                            .rounded(px(t.border_radius))
                            .border_1().border_color(border)
                            .bg(surface).text_size(px(12.)).text_color(t.text_secondary)
                            .child(label))
                        .child(BaseButton::new(format!("{prefix}-action-{i}"))
                            .disabled(disabled_action)
                            .accessibility_label(action_text)
                            .size(px(38.)).rounded_full()
                            .border_1().border_color(border).bg(surface)
                            .when(!disabled_action,|this|this.cursor_pointer()
                                .hover(|this|this.bg(t.surface_secondary)))
                            .child(AntIcon::new(action.icon).theme(t)
                                .color(if disabled_action{t.disabled_text}else{fg}).size(17.))
                            .on_click(move|_,window,cx|{
                                if let Some(callback)=&chosen{callback(action_key.clone(),window,cx);}
                                if let Some(callback)=&close{callback(false,window,cx);}
                            }))
                })
            ))
            .child(BaseButton::new(format!("{prefix}-trigger"))
                .disabled(disabled)
                .accessibility_label(self.label)
                .size(px(42.)).rounded_full().bg(primary).text_color(color(0xffffff))
                .flex().items_center().justify_center()
                .when(!disabled,|this|this.cursor_pointer().hover(|this|this.bg(t.primary_hover)))
                .child(AntIcon::new(if open{AntIconName::CloseOutlined}else{self.icon})
                    .theme(t).color(color(0xffffff)).size(19.))
                .when_some(on_toggle,|this,callback|this.on_click(move|_,window,cx|{
                    callback(!open,window,cx)
                })))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn float_offsets_sanitize_input(){
        assert_eq!(float_offset(-10.),0.);
        assert_eq!(float_offset(30.),30.);
        assert_eq!(float_offset(f32::INFINITY),24.);
        assert_eq!(float_offset(9999.),2048.);
    }
    #[test]
    fn group_actions_keep_keys_and_disabled_state(){
        let action=AntFloatAction::new("archive","Archive",AntIconName::InboxOutlined)
            .disabled(true).danger(true);
        let group=AntFloatButtonGroup::new("group","Actions")
            .action(action).open(true);
        assert!(group.open);
        assert!(group.actions[0].disabled);
        assert_eq!(group.actions[0].key.as_ref(),"archive");
    }
}
