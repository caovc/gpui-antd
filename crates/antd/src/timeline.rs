//! Ant Design Timeline: chronological steps with status-colored milestones.
//! Native GPUI composition shared between desktop and WebAssembly.

use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, div, px,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::base::StyledExt as _;
use crate::{AntIcon,AntIconName,AntTheme};
use crate::design_tokens::AntComponent;

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum TimelineMode { #[default] Left, Right, Alternate }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum TimelineTone { #[default] Default, Success, Warning, Error, Processing }

/// Each entry may contain an optional time, caption, real GPUI children,
/// tone and official Ant icon. Item content is consumed once when rendered.
pub struct AntTimelineItem {
    title:SharedString,
    description:Option<SharedString>,
    timestamp:Option<SharedString>,
    tone:TimelineTone,
    icon:Option<AntIconName>,
    children:Vec<AnyElement>,
}
impl AntTimelineItem {
    pub fn new(title:impl Into<SharedString>)->Self{
        Self{title:title.into(),description:None,timestamp:None,
            tone:TimelineTone::Default,icon:None,children:Vec::new()}
    }
    pub fn description(mut self,text:impl Into<SharedString>)->Self{
        self.description=Some(text.into());self
    }
    pub fn timestamp(mut self,value:impl Into<SharedString>)->Self{
        self.timestamp=Some(value.into());self
    }
    pub fn tone(mut self,tone:TimelineTone)->Self{self.tone=tone;self}
    pub fn icon(mut self,icon:AntIconName)->Self{self.icon=Some(icon);self}
    pub fn child(mut self,element:impl IntoElement)->Self{
        self.children.push(element.into_any_element());self
    }
}

/// Timeline is a read-only data display; use callbacks on arbitrary child
/// components for actions. Pending adds the last dot without mutating items.
#[derive(IntoElement)]
pub struct AntTimeline {
    items:Vec<AntTimelineItem>,
    mode:TimelineMode,
    reverse:bool,
    pending:Option<SharedString>,
    theme:AntTheme,
    gap:f32,
}
impl AntTimeline {
    pub fn new()->Self{
        Self{items:Vec::new(),mode:TimelineMode::Left,reverse:false,
            pending:None,theme:AntTheme::default(),gap:8.}
    }
    pub fn child(mut self,item:AntTimelineItem)->Self{self.items.push(item);self}
    pub fn children(mut self,items:impl IntoIterator<Item=AntTimelineItem>)->Self{
        self.items.extend(items);self
    }
    pub fn mode(mut self,mode:TimelineMode)->Self{self.mode=mode;self}
    pub fn reverse(mut self,reverse:bool)->Self{self.reverse=reverse;self}
    pub fn pending(mut self,label:impl Into<SharedString>)->Self{
        self.pending=Some(label.into());self
    }
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
    pub fn gap(mut self,gap:f32)->Self{
        self.gap=if gap.is_finite(){gap.clamp(0.,48.)}else{8.};self
    }
    pub fn item_count(&self)->usize{self.items.len()+usize::from(self.pending.is_some())}
}
impl Default for AntTimeline { fn default()->Self{Self::new()} }

impl RenderOnce for AntTimeline {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let t=self.theme;
        let overrides=t.component(AntComponent::Timeline);
        let stroke=overrides.color_border.unwrap_or(t.border);
        let primary=overrides.color_primary.unwrap_or(t.primary);
        let text=overrides.color_text.unwrap_or(t.text);
        let font_size=overrides.font_size.unwrap_or(t.font_size);
        let gap=self.gap;
        let total=self.item_count();
        let items:Vec<AntTimelineItem>=if self.reverse{
            self.items.into_iter().rev().collect()
        }else{self.items};
        let item_count=items.len();
        let render_item=move |index:usize,item:AntTimelineItem,pending:bool,mode:TimelineMode|{
            let accent=match item.tone{
                TimelineTone::Success=>t.success,
                TimelineTone::Warning=>t.warning,
                TimelineTone::Error=>t.error,
                TimelineTone::Processing=>primary,
                TimelineTone::Default=>stroke,
            };
            let right=match mode{
                TimelineMode::Left=>true,
                TimelineMode::Right=>false,
                TimelineMode::Alternate=>index%2==1,
            };
            let tail=index+1<total;
            let dot=div().size(px(15.)).flex().items_center().justify_center()
                .rounded_full()
                .when(item.icon.is_none()&&!pending,|this|this
                    .border_2().border_color(accent)
                    .bg(t.surface))
                .when_some(item.icon,|this,icon|this.child(
                    AntIcon::new(icon).theme(t).color(accent).size(15.)))
                .when(pending,|this|this.child(
                    AntIcon::loading().theme(t).color(primary).size(15.)));
            let body=div().w_full().flex().flex_col().gap(px(4.))
                .child(div().font_medium().text_size(px(font_size))
                    .text_color(text).child(item.title))
                .when_some(item.timestamp,|this,timestamp|{
                    this.child(div().text_size(px(12.))
                        .text_color(t.text_secondary).child(timestamp))
                })
                .when_some(item.description,|this,description|{
                    this.child(div().text_size(px(font_size-1.))
                        .text_color(t.text_secondary).child(description))
                })
                .children(item.children);
            let track=div().w(px(24.)).flex_none()
                .flex().flex_col().items_center().gap(px(4.))
                .child(dot)
                .when(tail,|this|this.child(
                    div().w(px(2.)).h(px(60.+gap)).bg(stroke)));
            let row=div().w_full().flex().items_start().gap(px(14.));
            if right{row.child(track).child(body)}
            else{row.child(body).child(track)}
        };
        let mut all:Vec<AnyElement>=items.into_iter().enumerate()
            .map(|(index,item)|render_item(index,item,false,self.mode).into_any_element())
            .collect();
        if let Some(label)=self.pending{
            all.push(render_item(item_count,AntTimelineItem::new(label)
                .tone(TimelineTone::Processing),true,self.mode).into_any_element());
        }
        div().w_full().flex().flex_col().gap(px(gap))
            .children(all)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timeline_modes_reverse_and_pending_count(){
        let timeline=AntTimeline::new()
            .mode(TimelineMode::Alternate).reverse(true)
            .child(AntTimelineItem::new("Created").timestamp("09:00"))
            .child(AntTimelineItem::new("Built")
                .tone(TimelineTone::Success)
                .icon(AntIconName::CheckCircleOutlined))
            .pending("Deploying");
        assert_eq!(timeline.item_count(),3);
        assert_eq!(timeline.mode,TimelineMode::Alternate);
        assert!(timeline.reverse);
        assert_eq!(timeline.items[1].tone,TimelineTone::Success);
    }
    #[test]
    fn timeline_gap_rejects_invalid_geometry(){
        assert_eq!(AntTimeline::new().gap(-10.).gap,0.);
        assert_eq!(AntTimeline::new().gap(200.).gap,48.);
        assert_eq!(AntTimeline::new().gap(f32::NAN).gap,8.);
    }
}
