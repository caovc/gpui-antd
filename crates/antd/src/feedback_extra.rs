//! Ant Design feedback primitives rendered by the same GPUI tree on desktop and WASM.
use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px, relative,
};
use gpui_kit::base::StyledExt as _;
use gpui_kit::component::{Sizable as _, Size, spinner::Spinner};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntIcon, AntIconName, AntTheme};

#[derive(Clone,Copy,Debug,PartialEq,Eq,Default)]
pub enum ResultStatus {
    #[default] Success, Info, Warning, Error, Forbidden, NotFound, ServerError
}
impl ResultStatus {
    pub fn default_title(self)->&'static str{
        match self {
            Self::Success=>"Successfully Completed",
            Self::Info=>"Information",
            Self::Warning=>"There are some problems",
            Self::Error=>"Submission Failed",
            Self::Forbidden=>"403",
            Self::NotFound=>"404",
            Self::ServerError=>"500",
        }
    }
    fn icon(self)->AntIconName {
        match self {
            Self::Success=>AntIconName::CheckCircleFilled,
            Self::Info=>AntIconName::InfoCircleFilled,
            Self::Warning=>AntIconName::WarningFilled,
            Self::Error=>AntIconName::CloseCircleFilled,
            Self::Forbidden|Self::NotFound|Self::ServerError=>AntIconName::QuestionCircleOutlined,
        }
    }
}

/// Status/empty-result page with optional native GPUI action buttons.
#[derive(IntoElement)]
pub struct AntResult {
    status:ResultStatus,
    title:Option<SharedString>,
    subtitle:Option<SharedString>,
    children:Vec<AnyElement>,
    theme:AntTheme,
}
impl AntResult {
    pub fn new(status:ResultStatus)->Self{
        Self{status,title:None,subtitle:None,children:Vec::new(),theme:AntTheme::default()}
    }
    pub fn title(mut self,title:impl Into<SharedString>)->Self{self.title=Some(title.into());self}
    pub fn subtitle(mut self,subtitle:impl Into<SharedString>)->Self{
        self.subtitle=Some(subtitle.into());self
    }
    pub fn child(mut self,child:impl IntoElement)->Self{
        self.children.push(child.into_any_element());self
    }
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
}
impl RenderOnce for AntResult{
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement{
        let t=self.theme;
        let accent=match self.status {
            ResultStatus::Success=>t.success,
            ResultStatus::Warning=>t.warning,
            ResultStatus::Error=>t.error,
            _=>t.primary,
        };
        let title=self.title.unwrap_or_else(||self.status.default_title().into());
        div().w_full().py(px(34.)).flex().flex_col().items_center().gap(px(13.))
            .child(AntIcon::new(self.status.icon()).theme(t).color(accent).size(64.))
            .child(div().font_medium().text_center().text_size(px(24.))
                .text_color(t.text).child(title))
            .when_some(self.subtitle,|this,subtitle|{
                this.child(div().text_center().text_size(px(14.))
                    .text_color(t.text_secondary).child(subtitle))
            })
            .child(div().flex().items_center().justify_center().gap(px(10.))
                .mt(px(10.)).children(self.children))
    }
}

/// Skeleton placeholder. Appearance is static; shimmer animation and
/// specialised Image/Button/Input skeletons remain future milestones.
#[derive(IntoElement)]
pub struct AntSkeleton {
    avatar:bool,
    title:bool,
    rows:usize,
    theme:AntTheme,
}
impl AntSkeleton {
    pub fn new()->Self{Self{avatar:false,title:true,rows:3,theme:AntTheme::default()}}
    pub fn avatar(mut self,yes:bool)->Self{self.avatar=yes;self}
    pub fn title(mut self,yes:bool)->Self{self.title=yes;self}
    pub fn rows(mut self,rows:usize)->Self{self.rows=rows.min(20);self}
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
}
impl Default for AntSkeleton{fn default()->Self{Self::new()}}
impl RenderOnce for AntSkeleton {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let shade=self.theme.disabled_bg;
        div().w_full().flex().items_start().gap(px(16.))
            .when(self.avatar,|this|this.child(div().size(px(40.))
                .flex_none().rounded_full().bg(shade)))
            .child(div().flex_1().min_w_0().flex().flex_col().gap(px(13.))
                .when(self.title,|this|this.child(div().w(relative(0.42))
                    .h(px(16.)).rounded(px(4.)).bg(shade)))
                .children((0..self.rows).map(|i|{
                    div().w(relative(if i+1==self.rows{0.62}else{1.}))
                        .h(px(13.)).rounded(px(4.)).bg(shade)
                })))
    }
}

/// Animated GPUI Kit spinner, optionally shown above explanatory text.
/// This is a native GPU control, not a CSS or DOM animation.
#[derive(IntoElement)]
pub struct AntSpin {
    spinning:bool,
    tip:Option<SharedString>,
    children:Vec<AnyElement>,
    theme:AntTheme,
}
impl AntSpin {
    pub fn new()->Self{Self{spinning:true,tip:None,children:Vec::new(),theme:AntTheme::default()}}
    pub fn spinning(mut self,spinning:bool)->Self{self.spinning=spinning;self}
    pub fn tip(mut self,tip:impl Into<SharedString>)->Self{self.tip=Some(tip.into());self}
    pub fn child(mut self,child:impl IntoElement)->Self{
        self.children.push(child.into_any_element());self
    }
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
}
impl Default for AntSpin{fn default()->Self{Self::new()}}
impl RenderOnce for AntSpin {
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement {
        let t=self.theme;
        div().flex().flex_col().items_center().gap(px(10.))
            .when(self.spinning,|this|{
                this.child(Spinner::new().with_size(Size::Small).color(t.primary))
                    .when_some(self.tip,|this,tip|{
                        this.child(div().text_size(px(13.)).text_color(t.primary).child(tip))
                    })
            })
            .children(self.children)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn result_status_has_useful_defaults(){
        assert_eq!(ResultStatus::Forbidden.default_title(),"403");
        assert_eq!(ResultStatus::NotFound.default_title(),"404");
        assert_eq!(ResultStatus::ServerError.default_title(),"500");
    }
    #[test]
    fn skeleton_row_count_is_bounded(){
        assert_eq!(AntSkeleton::new().rows(300).rows,20);
        assert_eq!(AntSkeleton::new().rows(0).rows,0);
    }
    #[test]
    fn spin_default_is_visible(){
        assert!(AntSpin::new().spinning);
        assert!(!AntSpin::new().spinning(false).spinning);
    }
}
