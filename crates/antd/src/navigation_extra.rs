//! Additional native Ant Design navigation primitives. All state is controlled.
use std::rc::Rc;
use gpui_kit::{
    App, ElementId, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, div, px,
};
use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntButton, AntIcon, AntIconName, AntTheme, ControlSize, color};

type PageChange = Rc<dyn Fn(usize, usize, &mut Window, &mut App)>;
type StepChange = Rc<dyn Fn(usize, &mut Window, &mut App)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSlot { Number(usize), JumpBack(usize), JumpForward(usize) }

/// Overflow-safe page count. An empty list still displays one page.
pub fn page_count(total: usize, page_size: usize) -> usize {
    let size = page_size.max(1);
    (total / size + usize::from(total % size != 0)).max(1)
}

/// Condensed numeric page list with jump-back and jump-forward controls.
pub fn page_slots(current: usize, count: usize) -> Vec<PageSlot> {
    let count = count.max(1);
    let current = current.clamp(1, count);
    if count <= 7 {
        return (1..=count).map(PageSlot::Number).collect();
    }
    let (start, end) = if current <= 4 { (2, 5) }
        else if current >= count - 3 { (count - 4, count - 1) }
        else { (current - 1, current + 1) };
    let mut slots = vec![PageSlot::Number(1)];
    if start > 2 { slots.push(PageSlot::JumpBack(current.saturating_sub(5).max(1))); }
    slots.extend((start..=end).map(PageSlot::Number));
    if end < count - 1 {
        slots.push(PageSlot::JumpForward(current.saturating_add(5).min(count)));
    }
    slots.push(PageSlot::Number(count));
    slots
}

/// A parent-controlled Pagination. on_change reports (page, page_size).
/// Size changer and direct input jumper are future API work.
#[derive(IntoElement)]
pub struct AntPagination {
    id: ElementId,
    total: usize,
    page_size: usize,
    current: usize,
    size: ControlSize,
    disabled: bool,
    simple: bool,
    show_total: bool,
    hide_on_single_page: bool,
    theme: AntTheme,
    on_change: Option<PageChange>,
}
impl AntPagination {
    pub fn new(id: impl Into<ElementId>, total: usize) -> Self {
        Self { id:id.into(), total, page_size:10, current:1, size:ControlSize::Middle,
            disabled:false, simple:false, show_total:false, hide_on_single_page:false,
            theme:AntTheme::default(), on_change:None }
    }
    pub fn current(mut self, value:usize)->Self{self.current=value;self}
    pub fn page_size(mut self, value:usize)->Self{self.page_size=value.max(1);self}
    pub fn size(mut self, value:ControlSize)->Self{self.size=value;self}
    pub fn disabled(mut self, value:bool)->Self{self.disabled=value;self}
    pub fn simple(mut self, value:bool)->Self{self.simple=value;self}
    pub fn show_total(mut self, value:bool)->Self{self.show_total=value;self}
    pub fn hide_on_single_page(mut self, value:bool)->Self{self.hide_on_single_page=value;self}
    pub fn theme(mut self, value:AntTheme)->Self{self.theme=value;self}
    pub fn pages(&self)->usize{page_count(self.total,self.page_size)}
    pub fn effective_current(&self)->usize{self.current.clamp(1,self.pages())}
    pub fn on_change(mut self, callback:impl Fn(usize,usize,&mut Window,&mut App)+'static)->Self{
        self.on_change=Some(Rc::new(callback));self
    }
}
impl RenderOnce for AntPagination {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let t=self.theme;
        let count=self.pages();
        let current=self.effective_current();
        let size=self.size;
        let disabled=self.disabled;
        let per_page=self.page_size;
        let prefix=format!("{:?}",self.id);
        let callback=self.on_change;
        let button=|key:String, text:String, target:usize, active:bool, blocked:bool|{
            let listener=callback.clone();
            AntButton::new(key,text).theme(t).size(size)
                .disabled(disabled||blocked)
                .when(active,|this|this.primary())
                .when_some(listener,|this,listener|this.on_click(move|_,window,cx|{
                    listener(target,per_page,window,cx)
                }))
        };
        let mut items=Vec::new();
        if !(self.hide_on_single_page && count==1) {
            if self.show_total {
                items.push(div().text_size(px(13.)).text_color(t.text_secondary)
                    .child(format!("Total {} items",self.total)).into_any_element());
            }
            items.push(button(format!("{prefix}-prev"),"‹ Previous".into(),
                current.saturating_sub(1).max(1),false,current==1).into_any_element());
            if self.simple {
                items.push(div().text_size(px(14.)).text_color(t.text)
                    .child(format!("{current} / {count}")).into_any_element());
            } else {
                for slot in page_slots(current,count) {
                    let (target,label,active,key)=match slot {
                        PageSlot::Number(page)=>(page,page.to_string(),page==current,format!("page-{page}")),
                        PageSlot::JumpBack(page)=>(page,"•••".into(),false,"jump-back".into()),
                        PageSlot::JumpForward(page)=>(page,"•••".into(),false,"jump-forward".into()),
                    };
                    items.push(button(format!("{prefix}-{key}"),label,target,active,false).into_any_element());
                }
            }
            items.push(button(format!("{prefix}-next"),"Next ›".into(),
                current.saturating_add(1).min(count),false,current==count).into_any_element());
        }
        div().flex().flex_wrap().items_center().gap(px(8.)).children(items)
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StepStatus { Wait, Process, Finish, Error }

pub struct AntStep {
    title:SharedString,
    description:Option<SharedString>,
    status:Option<StepStatus>,
    disabled:bool,
}
impl AntStep {
    pub fn new(title:impl Into<SharedString>)->Self{
        Self{title:title.into(),description:None,status:None,disabled:false}
    }
    pub fn description(mut self,description:impl Into<SharedString>)->Self{
        self.description=Some(description.into());self
    }
    pub fn status(mut self,status:StepStatus)->Self{self.status=Some(status);self}
    pub fn disabled(mut self,disabled:bool)->Self{self.disabled=disabled;self}
}

/// Steps: controlled current step, optional vertical mode and clickable labels.
#[derive(IntoElement)]
pub struct AntSteps {
    id:ElementId,
    steps:Vec<AntStep>,
    current:usize,
    vertical:bool,
    size:ControlSize,
    theme:AntTheme,
    on_change:Option<StepChange>,
}
impl AntSteps {
    pub fn new(id:impl Into<ElementId>)->Self{
        Self{id:id.into(),steps:Vec::new(),current:0,vertical:false,
            size:ControlSize::Middle,theme:AntTheme::default(),on_change:None}
    }
    pub fn child(mut self,step:AntStep)->Self{self.steps.push(step);self}
    pub fn current(mut self,current:usize)->Self{self.current=current;self}
    pub fn vertical(mut self,value:bool)->Self{self.vertical=value;self}
    pub fn size(mut self,size:ControlSize)->Self{self.size=size;self}
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
    pub fn on_change(mut self,f:impl Fn(usize,&mut Window,&mut App)+'static)->Self{
        self.on_change=Some(Rc::new(f));self
    }
}
impl RenderOnce for AntSteps {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let t=self.theme;
        let vertical=self.vertical;
        let total=self.steps.len();
        let prefix=format!("{:?}",self.id);
        let mut rows=Vec::with_capacity(total);
        for (index,step) in self.steps.into_iter().enumerate(){
            let status=step.status.unwrap_or(if index<self.current {StepStatus::Finish}
                else if index==self.current {StepStatus::Process} else {StepStatus::Wait});
            let color=match status {
                StepStatus::Finish|StepStatus::Process=>t.primary,
                StepStatus::Error=>t.error,
                StepStatus::Wait=>t.disabled_text,
            };
            let mark=if matches!(status,StepStatus::Finish|StepStatus::Error){
                AntIcon::new(if status==StepStatus::Finish {
                    AntIconName::CheckOutlined
                }else{AntIconName::CloseOutlined})
                    .theme(t).color(color).size(20.).into_any_element()
            }else{
                div().size(px(28.)).flex().items_center().justify_center()
                    .rounded_full().border_1().border_color(color)
                    .bg(if status==StepStatus::Process{t.primary}else{t.surface})
                    .text_color(if status==StepStatus::Process{color(0xffffff)}else{color})
                    .text_size(px(13.)).child((index+1).to_string()).into_any_element()
            };
            let caption=div().flex().flex_col().gap(px(3.))
                .child(div().text_color(if step.disabled{t.disabled_text}else{t.text})
                    .font_medium().text_size(px(if self.size==ControlSize::Small{13.}else{15.}))
                    .child(step.title.clone()))
                .when_some(step.description,|this,desc|{
                    this.child(div().text_size(px(12.)).text_color(t.text_secondary).child(desc))
                });
            let mut button=BaseButton::new(format!("{prefix}-step-{index}"))
                .disabled(step.disabled).accessibility_label(step.title)
                .p(px(6.)).flex().items_start().gap(px(10.))
                .bg(t.surface).rounded(px(6.))
                .when(!step.disabled,|this|this.cursor_pointer().hover(|this|this.bg(t.surface_secondary)))
                .child(mark).child(caption);
            if let Some(callback)=self.on_change.clone(){
                button=button.on_click(move|_,window,cx|callback(index,window,cx));
            }
            rows.push(div().flex().items_center().gap(px(6.))
                .when(!vertical && index+1<total,|this|this.flex_1())
                .child(button)
                .when(!vertical && index+1<total,|this|{
                    this.child(div().flex_1().h(px(1.)).bg(t.border))
                }).into_any_element());
        }
        div().flex().gap(px(if vertical{12.}else{6.}))
            .when(vertical,|this|this.flex_col())
            .when(!vertical,|this|this.items_start().w_full())
            .children(rows)
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn page_math_boundaries(){
        assert_eq!(page_count(0,10),1);
        assert_eq!(page_count(101,10),11);
        assert_eq!(page_count(usize::MAX,1),usize::MAX);
        assert_eq!(page_count(10,0),10);
        assert_eq!(AntPagination::new("p",50).current(100).effective_current(),5);
    }
    #[test]
    fn page_slots_are_bounded(){
        assert_eq!(page_slots(1,5),(1..=5).map(PageSlot::Number).collect::<Vec<_>>());
        assert!(page_slots(10,20).contains(&PageSlot::JumpBack(5)));
        assert!(page_slots(10,20).contains(&PageSlot::JumpForward(15)));
        for slot in page_slots(200,10){
            let n=match slot{PageSlot::Number(n)|PageSlot::JumpBack(n)|PageSlot::JumpForward(n)=>n};
            assert!((1..=10).contains(&n));
        }
    }
    #[test]
    fn steps_keep_disabled_and_status(){
        let step=AntStep::new("Review").disabled(true).status(StepStatus::Error);
        assert!(step.disabled);
        assert_eq!(step.status,Some(StepStatus::Error));
    }
}
