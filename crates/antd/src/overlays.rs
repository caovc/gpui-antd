//! Native Ant Design-inspired overlays powered by GPUI Kit 0.7.3.
//!
//! Dropdown uses GPUI Kit's accessible PopupMenu and keyboard dispatcher.
//! Popover and Popconfirm use GPUI Base's focus/dismissal/placement state.
//! Modal uses GPUI Kit's Dialog host with focus trap, Escape, overlay and
//! stacked z-ordering. No HTML or manually absolutely-positioned overlays.
//!
//! All APIs are alpha: not every Ant Design placement, visual token, or
//! animated transition has been implemented.

use std::rc::Rc;
use gpui_kit::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::base::StyledExt as _;
use gpui_kit::component::{
    Disableable as _, Sizable as _, Size,
    WindowExt as _,
    button::Button as KitButton,
    dialog::{
        Dialog as KitDialog, DialogHeader, DialogTitle,
        DialogDescription, DialogFooter,
    },
    menu::{DropdownMenu as _, PopupMenuItem},
    popover::Popover as KitPopover,
};
use crate::{AntButton, AntIcon, AntIconName, AntTheme};

type MenuSelect = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type OpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type Action = Rc<dyn Fn(&mut Window, &mut App) -> bool>;

/// Menu item with a stable key. Separators are non-interactive.
#[derive(Clone)]
pub struct AntDropdownItem {
    key: SharedString,
    label: SharedString,
    disabled: bool,
    checked: bool,
    separator: bool,
}
impl AntDropdownItem {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key:key.into(),label:label.into(),disabled:false,
            checked:false,separator:false,
        }
    }
    pub fn separator() -> Self {
        Self {
            key:"".into(),label:"".into(),disabled:true,checked:false,separator:true,
        }
    }
    pub fn disabled(mut self, disabled:bool)->Self {self.disabled=disabled;self}
    pub fn checked(mut self, checked:bool)->Self {self.checked=checked;self}
    pub fn key(&self)->&str {self.key.as_ref()}
}

/// Dropdown menu. GPUI Kit owns menu focus, Escape/Enter/arrows,
/// anchoring, scroll handling and dismissal; the Rust app owns selection.
///
/// The trigger currently uses a styled GPUI Kit Button because the Kit
/// `DropdownMenu` trait requires a `Selectable` trigger. Arbitrary
/// custom triggers and item-danger colors remain future parity work.
#[derive(IntoElement)]
pub struct AntDropdown {
    id: ElementId,
    label: SharedString,
    entries: Vec<AntDropdownItem>,
    theme: AntTheme,
    disabled: bool,
    min_width: f32,
    on_select: Option<MenuSelect>,
    on_open_change: Option<OpenChange>,
}
impl AntDropdown {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id:id.into(),label:label.into(),entries:Vec::new(),
            theme:AntTheme::default(),disabled:false,min_width:160.,
            on_select:None,on_open_change:None,
        }
    }
    pub fn theme(mut self, theme:AntTheme)->Self {self.theme=theme;self}
    pub fn disabled(mut self, disabled:bool)->Self {self.disabled=disabled;self}
    pub fn min_width(mut self, width:f32)->Self {self.min_width=width.max(96.);self}
    pub fn item(mut self, entry:AntDropdownItem)->Self {self.entries.push(entry);self}
    pub fn items(mut self, entries:impl IntoIterator<Item=AntDropdownItem>)->Self {
        self.entries.extend(entries);self
    }
    pub fn on_select(mut self, handler:impl Fn(SharedString,&mut Window,&mut App)+'static)->Self {
        self.on_select=Some(Rc::new(handler));self
    }
    pub fn on_open_change(mut self,handler:impl Fn(bool,&mut Window,&mut App)+'static)->Self {
        self.on_open_change=Some(Rc::new(handler));self
    }
    pub fn can_select(&self, key:&str)->bool {
        self.entries.iter().any(|e|!e.separator&&!e.disabled&&e.key.as_ref()==key)
    }
}
impl RenderOnce for AntDropdown {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let entries=self.entries;
        let callback=self.on_select;
        let trigger=KitButton::new(self.id)
            .label(self.label).outline().with_size(Size::Medium)
            .disabled(self.disabled).dropdown_caret(true)
            .bg(self.theme.surface).text_color(self.theme.text);
        trigger.dropdown_menu(move |menu, _, _| {
            entries.iter().fold(menu.min_w(px(self.min_width)), |menu, item| {
                if item.separator {return menu.item(PopupMenuItem::separator());}
                let key=item.key.clone();
                let callback=callback.clone();
                menu.item(
                    PopupMenuItem::new(item.label.clone())
                        .checked(item.checked)
                        .disabled(item.disabled)
                        .on_click(move |_, window, cx| {
                            if let Some(callback)=&callback {
                                callback(key.clone(),window,cx);
                            }
                        })
                )
            })
        })
        .when_some(self.on_open_change, |this, callback| {
            this.on_open_change(move |open, window, cx| callback(*open,window,cx))
        })
    }
}

/// Controlled or uncontrolled native Popover with themed text and content.
///
/// The trigger is currently a GPUI Kit button; interactive children inside
/// the popup render as actual Rust/GPUI components. Kit/Base manage viewport
/// positioning, outside dismissal, and the popup focus lifecycle.
#[derive(IntoElement)]
pub struct AntPopover {
    id: ElementId,
    trigger_label: SharedString,
    title: SharedString,
    children: Vec<AnyElement>,
    theme: AntTheme,
    width: f32,
    arrow: bool,
    disabled: bool,
    default_open: bool,
    open: Option<bool>,
    on_open_change: Option<OpenChange>,
}
impl AntPopover {
    pub fn new(id:impl Into<ElementId>,trigger_label:impl Into<SharedString>) -> Self {
        Self {
            id:id.into(),trigger_label:trigger_label.into(),title:"".into(),
            children:Vec::new(),theme:AntTheme::default(),width:260.,
            arrow:false,disabled:false,default_open:false,open:None,on_open_change:None,
        }
    }
    pub fn title(mut self,title:impl Into<SharedString>)->Self {self.title=title.into();self}
    pub fn theme(mut self,theme:AntTheme)->Self {self.theme=theme;self}
    pub fn width(mut self,width:f32)->Self {self.width=width.max(120.);self}
    pub fn arrow(mut self,arrow:bool)->Self {self.arrow=arrow;self}
    pub fn disabled(mut self,disabled:bool)->Self {self.disabled=disabled;self}
    pub fn default_open(mut self,open:bool)->Self {self.default_open=open;self}
    /// Must be accompanied by `on_open_change` for a controlled popup.
    pub fn open(mut self,open:bool)->Self {self.open=Some(open);self}
    pub fn on_open_change(mut self,handler:impl Fn(bool,&mut Window,&mut App)+'static)->Self {
        self.on_open_change=Some(Rc::new(handler));self
    }
    pub fn child(mut self,element:impl IntoElement)->Self {
        self.children.push(element.into_any_element());self
    }
}
impl RenderOnce for AntPopover {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme=self.theme;
        // Each trigger must retain a distinct GPUI identity for focus,
        // hover and keyed popup state in a Gallery with multiple popovers.
        let trigger_id=SharedString::from(format!("ant-popover-trigger:{:?}",self.id));
        KitPopover::new(self.id)
            .trigger(
                KitButton::new(trigger_id)
                    .label(self.trigger_label)
                    .outline().with_size(Size::Medium)
                    .disabled(self.disabled)
            )
            .arrow(self.arrow)
            .default_open(self.default_open)
            .when_some(self.open, |this, open| this.open(open))
            .when_some(self.on_open_change, |this, callback| {
                this.on_open_change(move |open, window, cx| callback(*open,window,cx))
            })
            .w(px(self.width))
            .bg(theme.surface).text_color(theme.text)
            .border_1().border_color(theme.border)
            .rounded(px(theme.border_radius))
            .child(div().flex().flex_col().gap(px(12.))
                .when(!self.title.is_empty(), |this| {
                    this.child(div().font_semibold().text_color(theme.text)
                        .child(self.title))
                })
                .children(self.children))
    }
}

/// Modal dialog launched by a native GPUI Base button. Kit/Dialog owns
/// Escape, focus trapping, stacked modal layers and backdrop dismissal.
///
/// `on_ok` and `on_cancel` return true to dismiss, false to stay open.
/// The body is plain text in this first alpha API; richer arbitrary form
/// content is a follow-up, not a promised React-style API.
#[derive(IntoElement)]
pub struct AntModal {
    id: ElementId,
    trigger_label: SharedString,
    title: SharedString,
    description: SharedString,
    body: SharedString,
    theme: AntTheme,
    width: f32,
    keyboard: bool,
    mask_closable: bool,
    on_ok: Option<Action>,
    on_cancel: Option<Action>,
}
impl AntModal {
    pub fn new(id:impl Into<ElementId>,trigger_label:impl Into<SharedString>,
        title:impl Into<SharedString>)->Self
    {
        Self {
            id:id.into(),trigger_label:trigger_label.into(),title:title.into(),
            description:"".into(),body:"".into(),theme:AntTheme::default(),
            width:520.,keyboard:true,mask_closable:true,on_ok:None,on_cancel:None,
        }
    }
    pub fn description(mut self,description:impl Into<SharedString>)->Self {
        self.description=description.into();self
    }
    pub fn body(mut self,body:impl Into<SharedString>)->Self {self.body=body.into();self}
    pub fn theme(mut self,theme:AntTheme)->Self {self.theme=theme;self}
    pub fn width(mut self,width:f32)->Self {self.width=width.max(260.);self}
    pub fn keyboard(mut self,keyboard:bool)->Self {self.keyboard=keyboard;self}
    pub fn mask_closable(mut self,closable:bool)->Self {self.mask_closable=closable;self}
    pub fn on_ok(mut self,handler:impl Fn(&mut Window,&mut App)->bool+'static)->Self {
        self.on_ok=Some(Rc::new(handler));self
    }
    pub fn on_cancel(mut self,handler:impl Fn(&mut Window,&mut App)->bool+'static)->Self {
        self.on_cancel=Some(Rc::new(handler));self
    }
}
impl RenderOnce for AntModal {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme=self.theme;
        let title=self.title;
        let description=self.description;
        let body=self.body;
        let ok=self.on_ok;
        let cancel=self.on_cancel;
        KitDialog::new(cx)
            .trigger(AntButton::new(self.id,self.trigger_label).theme(theme))
            .width(px(self.width))
            .keyboard(self.keyboard)
            .overlay_closable(self.mask_closable)
            .close_button(true)
            .content(move |content, _, _| {
                let ok=ok.clone();
                let cancel=cancel.clone();
                content
                    .child(
                        DialogHeader::new()
                            .child(DialogTitle::new().child(title.clone()))
                            .when(!description.is_empty(), |this| {
                                this.child(DialogDescription::new().child(description.clone()))
                            })
                    )
                    .child(
                        div().p(px(16.)).text_color(theme.text).child(body.clone())
                    )
                    .child(
                        DialogFooter::new()
                            .child(AntButton::new("ant-modal-cancel","Cancel").theme(theme)
                                .on_click(move |_,window,cx| {
                                    if cancel.as_ref().is_none_or(|cb|cb(window,cx)) {
                                        window.close_dialog(cx);
                                    }
                                }))
                            .child(AntButton::new("ant-modal-ok","OK").theme(theme).primary()
                                .on_click(move |_,window,cx| {
                                    if ok.as_ref().is_none_or(|cb|cb(window,cx)) {
                                        window.close_dialog(cx);
                                    }
                                }))
                    )
            })
    }
}

/// Anchored Popconfirm built on the same native Popover/Popup state machine.
/// Unlike a modal alert, it stays attached to its trigger and uses local
/// Confirm and Cancel buttons. Escape/outside press restore focus via Kit.
#[derive(IntoElement)]
pub struct AntPopconfirm {
    id: ElementId,
    trigger_label: SharedString,
    title: SharedString,
    description: SharedString,
    theme: AntTheme,
    width: f32,
    disabled: bool,
    on_confirm: Option<Action>,
    on_cancel: Option<Action>,
}
impl AntPopconfirm {
    pub fn new(id:impl Into<ElementId>,trigger_label:impl Into<SharedString>,
        title:impl Into<SharedString>)->Self
    {
        Self {
            id:id.into(),trigger_label:trigger_label.into(),title:title.into(),
            description:"".into(),theme:AntTheme::default(),
            width:280.,disabled:false,on_confirm:None,on_cancel:None,
        }
    }
    pub fn description(mut self,description:impl Into<SharedString>)->Self {
        self.description=description.into();self
    }
    pub fn theme(mut self,theme:AntTheme)->Self {self.theme=theme;self}
    pub fn width(mut self,width:f32)->Self {self.width=width.max(160.);self}
    pub fn disabled(mut self,disabled:bool)->Self {self.disabled=disabled;self}
    /// Return true to close; false to keep the confirmation popover open.
    pub fn on_confirm(mut self,handler:impl Fn(&mut Window,&mut App)->bool+'static)->Self {
        self.on_confirm=Some(Rc::new(handler));self
    }
    pub fn on_cancel(mut self,handler:impl Fn(&mut Window,&mut App)->bool+'static)->Self {
        self.on_cancel=Some(Rc::new(handler));self
    }
}
impl RenderOnce for AntPopconfirm {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme=self.theme;
        let title=self.title;
        let desc=self.description;
        let width=self.width;
        let confirm=self.on_confirm;
        let cancel=self.on_cancel;
        let trigger_id=SharedString::from(format!("ant-popconfirm-trigger:{:?}",self.id));
        KitPopover::new(self.id)
            .trigger(
                KitButton::new(trigger_id)
                    .label(self.trigger_label).outline().with_size(Size::Medium)
                    .disabled(self.disabled)
            )
            .arrow(true).overlay_closable(true)
            .content(move |_, _, cx| {
                let close_cancel=cx.weak_entity();
                let close_confirm=cx.weak_entity();
                let cancel=cancel.clone();
                let confirm=confirm.clone();
                div().w(px(width)).flex().flex_col().gap(px(12.))
                    .child(div().flex().items_start().gap(px(8.))
                        .child(AntIcon::new(AntIconName::ExclamationCircleFilled)
                            .theme(theme).size(18.).color(theme.warning))
                        .child(div().flex().flex_col().gap(px(7.))
                            .child(div().font_semibold().child(title.clone()))
                            .when(!desc.is_empty(), |this| {
                                this.child(div().text_size(px(13.))
                                    .text_color(theme.text_secondary).child(desc.clone()))
                            })))
                    .child(div().flex().justify_end().gap(px(8.))
                        .child(AntButton::new("ant-popconfirm-no","Cancel").theme(theme)
                            .on_click(move |_,window,cx| {
                                if cancel.as_ref().is_none_or(|cb|cb(window,cx)) {
                                    let _=close_cancel.update(cx,|state,cx|state.dismiss(window,cx));
                                }
                            }))
                        .child(AntButton::new("ant-popconfirm-yes","Confirm").theme(theme)
                            .primary().size(crate::ControlSize::Small)
                            .on_click(move |_,window,cx| {
                                if confirm.as_ref().is_none_or(|cb|cb(window,cx)) {
                                    let _=close_confirm.update(cx,|state,cx|state.dismiss(window,cx));
                                }
                            })))
            })
            .w(px(width)).bg(theme.surface).text_color(theme.text)
            .border_1().border_color(theme.border)
            .rounded(px(theme.border_radius))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_rejects_disabled_and_separator_selections() {
        let menu=AntDropdown::new("menu","More")
            .item(AntDropdownItem::new("open","Open"))
            .item(AntDropdownItem::new("delete","Delete").disabled(true))
            .item(AntDropdownItem::separator())
            .item(AntDropdownItem::new("marked","Marked").checked(true));
        assert!(menu.can_select("open"));
        assert!(menu.can_select("marked"));
        assert!(!menu.can_select("delete"));
        assert!(!menu.can_select(""));
        assert!(!menu.can_select("missing"));
    }

    #[test]
    fn overlay_size_bounds_and_controlled_open() {
        let popover=AntPopover::new("popover","Show").width(-1.).open(true);
        assert_eq!(popover.width,120.);
        assert_eq!(popover.open,Some(true));
        let modal=AntModal::new("modal","Launch","Title").width(100.);
        assert_eq!(modal.width,260.);
        let confirmation=AntPopconfirm::new("confirm","Delete","Really delete?")
            .width(20.).disabled(true);
        assert_eq!(confirmation.width,160.);
        assert!(confirmation.disabled);
    }
}
