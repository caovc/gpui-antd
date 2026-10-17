//! Data-entry and display widgets missing from the initial Ant Design catalog.
//! All activation is GPUI-native; no HTML controls or locally stored model state.

use std::{collections::BTreeSet, rc::Rc};
use gpui_kit::{
    AnyElement, App, ElementId, IntoElement, InteractiveElement as _, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px, relative,
};
use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntIcon, AntIconName, AntTheme};

type RateChange = Rc<dyn Fn(usize, &mut Window, &mut App)>;
type CollapseChange = Rc<dyn Fn(BTreeSet<String>, &mut Window, &mut App)>;

/// Compute the next whole-star rating, supporting click-again-to-clear.
pub fn next_rate(current: usize, clicked: usize, count: usize, allow_clear: bool) -> usize {
    let clicked=clicked.clamp(1,count.max(1));
    if allow_clear && current==clicked {0} else {clicked}
}

/// Controlled, whole-star rating. Half stars and hover preview are future work.
#[derive(IntoElement)]
pub struct AntRate {
    id:ElementId,
    count:usize,
    value:usize,
    disabled:bool,
    allow_clear:bool,
    theme:AntTheme,
    on_change:Option<RateChange>,
}
impl AntRate {
    pub fn new(id:impl Into<ElementId>)->Self{
        Self{id:id.into(),count:5,value:0,disabled:false,allow_clear:true,
            theme:AntTheme::default(),on_change:None}
    }
    pub fn count(mut self,count:usize)->Self{self.count=count.clamp(1,20);self}
    pub fn value(mut self,value:usize)->Self{self.value=value;self}
    pub fn disabled(mut self,disabled:bool)->Self{self.disabled=disabled;self}
    pub fn allow_clear(mut self,allow:bool)->Self{self.allow_clear=allow;self}
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
    pub fn on_change(mut self,f:impl Fn(usize,&mut Window,&mut App)+'static)->Self{
        self.on_change=Some(Rc::new(f));self
    }
}
impl RenderOnce for AntRate {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let count=self.count;
        let selected=self.value.min(count);
        let theme=self.theme;
        let disabled=self.disabled;
        let allow_clear=self.allow_clear;
        let prefix=format!("{:?}",self.id);
        div().flex().items_center().gap(px(6.))
            .children((1..=count).map(|index|{
                let active=index<=selected;
                let callback=self.on_change.clone();
                BaseButton::new(format!("{prefix}-star-{index}"))
                    .disabled(disabled)
                    .accessibility_label(format!("Rate {index} of {count}"))
                    .p(px(2.)).rounded(px(4.))
                    .when(!disabled,|this|this.cursor_pointer().hover(|this|this.bg(theme.surface_secondary)))
                    .child(AntIcon::new(if active{AntIconName::StarFilled}
                        else{AntIconName::StarOutlined})
                        .theme(theme)
                        .color(if active{theme.warning}else{theme.disabled_text})
                        .size(24.))
                    .when_some(callback,|this,callback|this.on_click(move|_,window,cx|{
                        callback(next_rate(selected,index,count,allow_clear),window,cx)
                    }))
            }))
    }
}

pub struct AntCollapseItem {
    key:SharedString,
    label:SharedString,
    children:Vec<AnyElement>,
    disabled:bool,
}
impl AntCollapseItem {
    pub fn new(key:impl Into<SharedString>,label:impl Into<SharedString>)->Self{
        Self{key:key.into(),label:label.into(),children:Vec::new(),disabled:false}
    }
    pub fn child(mut self,element:impl IntoElement)->Self{
        self.children.push(element.into_any_element());self
    }
    pub fn disabled(mut self,disabled:bool)->Self{self.disabled=disabled;self}
}

/// Return the requested set of open panel keys. Accordion closes all others.
pub fn toggled_collapse_keys(open:&BTreeSet<String>,key:&str,accordion:bool)->BTreeSet<String>{
    let mut next=if accordion{BTreeSet::new()}else{open.clone()};
    if !open.contains(key){next.insert(key.to_owned());}
    else if !accordion{next.remove(key);}
    next
}

/// Controlled collapse panels. Keys belong to the parent entity.
#[derive(IntoElement)]
pub struct AntCollapse {
    id:ElementId,
    items:Vec<AntCollapseItem>,
    open_keys:BTreeSet<String>,
    accordion:bool,
    bordered:bool,
    theme:AntTheme,
    on_change:Option<CollapseChange>,
}
impl AntCollapse {
    pub fn new(id:impl Into<ElementId>)->Self{
        Self{id:id.into(),items:Vec::new(),open_keys:BTreeSet::new(),
            accordion:false,bordered:true,theme:AntTheme::default(),on_change:None}
    }
    pub fn child(mut self,item:AntCollapseItem)->Self{self.items.push(item);self}
    pub fn open_keys<I,S>(mut self,keys:I)->Self
    where I:IntoIterator<Item=S>,S:AsRef<str>{
        self.open_keys=keys.into_iter().map(|key|key.as_ref().to_owned()).collect();self
    }
    pub fn accordion(mut self,accordion:bool)->Self{self.accordion=accordion;self}
    pub fn bordered(mut self,bordered:bool)->Self{self.bordered=bordered;self}
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
    pub fn on_change(mut self,f:impl Fn(BTreeSet<String>,&mut Window,&mut App)+'static)->Self{
        self.on_change=Some(Rc::new(f));self
    }
}
impl RenderOnce for AntCollapse {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let theme=self.theme;
        let prefix=format!("{:?}",self.id);
        let open=self.open_keys;
        let accordion=self.accordion;
        let callback=self.on_change;
        div().flex().flex_col().w_full().overflow_hidden()
            .rounded(px(8.))
            .when(self.bordered,|this|this.border_1().border_color(theme.border))
            .children(self.items.into_iter().enumerate().map(|(index,item)|{
                let expanded=open.contains(item.key.as_ref());
                let next=toggled_collapse_keys(&open,item.key.as_ref(),accordion);
                let notify=callback.clone();
                let head=BaseButton::new(format!("{prefix}-panel-{}",item.key))
                    .disabled(item.disabled)
                    .accessibility_label(item.label.clone())
                    .w_full().flex().items_center().justify_start().gap(px(10.))
                    .px(px(16.)).h(px(46.))
                    .bg(theme.surface_secondary)
                    .when(!item.disabled,|this|this.cursor_pointer()
                        .hover(|this|this.bg(theme.info_bg)))
                    .child(AntIcon::new(if expanded{AntIconName::DownOutlined}
                        else{AntIconName::RightOutlined})
                        .theme(theme).size(13.))
                    .child(div().font_medium().text_size(px(14.))
                        .text_color(if item.disabled{theme.disabled_text}else{theme.text})
                        .child(item.label))
                    .when_some(notify,|this,notify|this.on_click(move|_,window,cx|{
                        notify(next.clone(),window,cx)
                    }));
                div().w_full().flex().flex_col()
                    .when(index>0,|this|this.border_t_1().border_color(theme.border))
                    .child(head)
                    .when(expanded,|this|this.child(div().w_full()
                        .p(px(16.)).bg(theme.surface).text_color(theme.text)
                        .text_size(px(14.)).children(item.children)))
            }))
    }
}

pub struct AntDescriptionItem {
    label:SharedString,
    value:SharedString,
}
impl AntDescriptionItem {
    pub fn new(label:impl Into<SharedString>,value:impl Into<SharedString>)->Self{
        Self{label:label.into(),value:value.into()}
    }
}

/// Native text descriptions. Each cell occupies an equal-width GPUI column.
/// Rowspan, vertical label mode and responsive column maps remain parity work.
#[derive(IntoElement)]
pub struct AntDescriptions {
    title:Option<SharedString>,
    items:Vec<AntDescriptionItem>,
    columns:usize,
    bordered:bool,
    theme:AntTheme,
}
impl AntDescriptions {
    pub fn new()->Self{
        Self{title:None,items:Vec::new(),columns:3,bordered:false,theme:AntTheme::default()}
    }
    pub fn title(mut self,title:impl Into<SharedString>)->Self{self.title=Some(title.into());self}
    pub fn child(mut self,item:AntDescriptionItem)->Self{self.items.push(item);self}
    pub fn columns(mut self,columns:usize)->Self{self.columns=columns.clamp(1,6);self}
    pub fn bordered(mut self,bordered:bool)->Self{self.bordered=bordered;self}
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
}
impl Default for AntDescriptions{fn default()->Self{Self::new()}}
impl RenderOnce for AntDescriptions{
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement{
        let t=self.theme;
        let width=1./self.columns as f32;
        let bordered=self.bordered;
        div().w_full().flex().flex_col().gap(px(14.))
            .when_some(self.title,|this,title|this.child(div().font_semibold()
                .text_color(t.text).text_size(px(16.)).child(title)))
            .child(div().flex().flex_wrap().w_full()
                .children(self.items.into_iter().map(|item|{
                    div().w(relative(width)).flex().flex_col()
                        .when(bordered,|this|this.border_1().border_color(t.border))
                        .child(div().text_size(px(13.)).text_color(t.text_secondary)
                            .when(bordered,|this|this.bg(t.surface_secondary).px(px(12.)).py(px(8.)))
                            .child(item.label))
                        .child(div().text_size(px(14.)).text_color(t.text)
                            .when(bordered,|this|this.px(px(12.)).py(px(10.)))
                            .child(item.value))
                })))
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn rate_value_changes_and_clears(){
        assert_eq!(next_rate(3,3,5,true),0);
        assert_eq!(next_rate(3,3,5,false),3);
        assert_eq!(next_rate(0,8,5,true),5);
    }
    #[test]
    fn collapse_accordion_and_multiple(){
        let open=BTreeSet::from(["a".to_owned(),"b".to_owned()]);
        assert_eq!(toggled_collapse_keys(&open,"a",false),BTreeSet::from(["b".to_owned()]));
        assert!(toggled_collapse_keys(&open,"a",true).is_empty());
        assert_eq!(toggled_collapse_keys(&open,"c",true),BTreeSet::from(["c".to_owned()]));
        assert_eq!(toggled_collapse_keys(&open,"c",false).len(),3);
    }
    #[test]
    fn description_columns_are_bounded(){
        assert_eq!(AntDescriptions::new().columns(0).columns,1);
        assert_eq!(AntDescriptions::new().columns(200).columns,6);
    }
}
