//! Ant Design Menu navigation rendered by GPUI Base, on desktop and WASM.
//!
//! Rust Entity state owns selection and expanded submenu keys. GPUI Base
//! supplies real keyboard/pointer activation and accessibility semantics.
//! This implementation supports inline nested submenus and roving keyboard
//! focus, not a separate HTML navigation tree or a fake popup.
//!
//! Still pending: horizontal Menu, icon-only submenu popup, viewport clipping,
//! typeahead and full screen-reader regression tests.

use std::{collections::BTreeSet, rc::Rc};

use gpui_kit::{
    App, ElementId, FocusHandle, InteractiveElement as _, IntoElement, KeyDownEvent,
    ParentElement as _, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};
use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::prelude::FluentBuilder as _;

use crate::{AntIcon, AntIconName, AntTheme, color};
use crate::design_tokens::AntComponent;

type MenuSelect = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type MenuOpenChange = Rc<dyn Fn(SharedString, bool, &mut Window, &mut App)>;

/// One menu action or a parent of nested menu entries, addressed by a stable
/// application key. Submenus are identified by their children or by using
/// `.submenu(key, label)`. Keys must be unique across the full menu tree.
#[derive(Debug, Clone)]
pub struct AntMenuItem {
    key: SharedString,
    label: SharedString,
    icon: Option<AntIconName>,
    disabled: bool,
    danger: bool,
    submenu: bool,
    children: Vec<AntMenuItem>,
}
impl AntMenuItem {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key:key.into(), label:label.into(), icon:None,
            disabled:false, danger:false, submenu:false, children:Vec::new(),
        }
    }
    /// Creates a submenu title with child entries supplied via `.child`.
    pub fn submenu(key:impl Into<SharedString>, label:impl Into<SharedString>) -> Self {
        let mut item=Self::new(key,label);
        item.submenu=true;
        item
    }
    pub fn icon(mut self, icon: AntIconName) -> Self {self.icon=Some(icon);self}
    pub fn disabled(mut self, disabled: bool) -> Self {self.disabled=disabled;self}
    pub fn danger(mut self, danger: bool) -> Self {self.danger=danger;self}
    pub fn child(mut self, child: AntMenuItem) -> Self {
        self.submenu=true;
        self.children.push(child);
        self
    }
    pub fn children(mut self, children: impl IntoIterator<Item=AntMenuItem>) -> Self {
        self.submenu=true;
        self.children.extend(children);
        self
    }
    pub fn key(&self) -> &str {self.key.as_ref()}
    pub fn is_submenu(&self) -> bool {self.submenu || !self.children.is_empty()}
}

/// Keys can be navigated independently from the selected route.
///
/// Arrow keys move focus, not application route state. The route changes
/// only on Enter/Space or pointer activation, keeping selection controlled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MenuMove { Next, Previous, First, Last }

fn next_menu_focus(enabled: &[bool], current:usize, command:MenuMove) -> Option<usize> {
    if enabled.is_empty() || !enabled.iter().any(|item|*item) {return None;}
    let len=enabled.len();
    match command {
        MenuMove::First=>enabled.iter().position(|item|*item),
        MenuMove::Last=>enabled.iter().rposition(|item|*item),
        MenuMove::Next|MenuMove::Previous=>{
            let current=current%len;
            for step in 1..=len {
                let index=if command==MenuMove::Next {
                    (current+step)%len
                } else {
                    (current+len-(step%len))%len
                };
                if enabled[index] {return Some(index);}
            }
            None
        }
    }
}

fn has_selected_descendant(item:&AntMenuItem, selected:Option<&str>) -> bool {
    item.children.iter().any(|child|
        selected==Some(child.key()) || has_selected_descendant(child,selected))
}

/// Snapshot of a visible menu row: descendants are only included when the
/// corresponding controlled key is open (and inline mode isn't collapsed).
struct VisibleMenuRow {
    item: AntMenuItem,
    depth:usize,
    parent:Option<String>,
    expanded:bool,
    selected_descendant:bool,
}

fn flatten_visible(
    items:Vec<AntMenuItem>, open_keys:&BTreeSet<String>,
    selected:Option<&str>, collapsed:bool,
    depth:usize, parent:Option<String>, rows:&mut Vec<VisibleMenuRow>,
) {
    for mut item in items {
        let submenu=item.is_submenu();
        let expanded=submenu && open_keys.contains(item.key());
        let descendant=has_selected_descendant(&item,selected);
        let disabled=item.disabled;
        let key=item.key().to_owned();
        let children=std::mem::take(&mut item.children);
        rows.push(VisibleMenuRow {
            item,depth,parent:parent.clone(),expanded,selected_descendant:descendant,
        });
        if submenu && expanded && !collapsed && !disabled {
            flatten_visible(children,open_keys,selected,collapsed,
                depth+1,Some(key),rows);
        }
    }
}

/// Ant Design-inspired vertical inline menu with controlled selection and
/// expandable, multi-level submenus.
///
/// - `selected_key` is a route key reported by `on_select`.
/// - `open_keys` is the set of submenu keys, updated from
///   `on_open_change(key, next_open, window, cx)`.
/// - Up/Down/Home/End rove focus across visible enabled items, wrapping.
/// - Right opens a focused submenu (or moves into already-open children).
/// - Left/Escape close a focused submenu and return focus to its parent.
/// - Enter and Space use GPUI Base Button native activation.
///
/// Collapsed mode currently hides submenu children instead of opening popup
/// flyouts. That mode, horizontal navigation and typeahead remain roadmap.
#[derive(IntoElement)]
pub struct AntMenu {
    id: ElementId,
    items: Vec<AntMenuItem>,
    selected: Option<SharedString>,
    open_keys:BTreeSet<String>,
    theme:AntTheme,
    dark:bool,
    inline_collapsed:bool,
    item_height:f32,
    on_select:Option<MenuSelect>,
    on_open_change:Option<MenuOpenChange>,
}
impl AntMenu {
    pub fn new(id:impl Into<ElementId>) -> Self {
        Self {
            id:id.into(),items:Vec::new(),selected:None,open_keys:BTreeSet::new(),
            theme:AntTheme::default(),dark:false,inline_collapsed:false,
            item_height:40.,on_select:None,on_open_change:None,
        }
    }
    pub fn theme(mut self, theme:AntTheme)->Self {self.theme=theme;self}
    pub fn dark(mut self, dark:bool)->Self {self.dark=dark;self}
    pub fn inline_collapsed(mut self, collapsed:bool)->Self {
        self.inline_collapsed=collapsed;self
    }
    pub fn item_height(mut self, height:f32)->Self {
        self.item_height=height.max(24.);self
    }
    pub fn selected_key(mut self, key:impl Into<SharedString>)->Self {
        self.selected=Some(key.into());self
    }
    pub fn open_keys<I,S>(mut self, keys:I)->Self
    where I:IntoIterator<Item=S>, S:AsRef<str> {
        self.open_keys=keys.into_iter().map(|key|key.as_ref().to_owned()).collect();
        self
    }
    pub fn item(mut self, item:AntMenuItem)->Self {self.items.push(item);self}
    pub fn items(mut self, items:impl IntoIterator<Item=AntMenuItem>)->Self {
        self.items.extend(items);self
    }
    pub fn on_select(mut self, handler:impl Fn(SharedString,&mut Window,&mut App)+'static)->Self {
        self.on_select=Some(Rc::new(handler));self
    }
    pub fn on_open_change(mut self,
        handler:impl Fn(SharedString,bool,&mut Window,&mut App)+'static)->Self
    {
        self.on_open_change=Some(Rc::new(handler));self
    }
    /// Whether a key names an enabled *leaf* anywhere in the menu tree.
    pub fn can_select(&self, key:&str)->bool {
        fn visit(items:&[AntMenuItem],key:&str)->bool {
            items.iter().any(|item| {
                !item.disabled && if item.is_submenu() {
                    visit(&item.children,key)
                } else {
                    item.key()==key
                }
            })
        }
        visit(&self.items,key)
    }
    /// Whether a key resolves to an enabled expandable menu title.
    pub fn can_expand(&self,key:&str)->bool {
        fn visit(items:&[AntMenuItem],key:&str)->bool {
            items.iter().any(|item| {
                !item.disabled && ((item.key()==key && item.is_submenu()) ||
                    visit(&item.children,key))
            })
        }
        visit(&self.items,key)
    }
}

impl RenderOnce for AntMenu {
    fn render(self, window:&mut Window, cx:&mut App)->impl IntoElement {
        let theme=self.theme;
        let dark=self.dark;
        let collapsed=self.inline_collapsed;
        let selected=self.selected;
        let on_select=self.on_select;
        let on_open_change=self.on_open_change;
        let overrides=theme.component(AntComponent::Menu);
        let item_height=overrides.control_height.unwrap_or(self.item_height);
        let prefix=format!("{:?}",self.id);
        let mut rows=Vec::new();
        flatten_visible(self.items,&self.open_keys,selected.as_deref(),
            collapsed,0,None,&mut rows);

        let focus_handles:Vec<FocusHandle>=rows.iter().map(|row| {
            let id=format!("ant-menu-{prefix}-{}",row.item.key);
            window.use_keyed_state(id,cx,|_,cx|cx.focus_handle())
                .read(cx).clone()
        }).collect();
        let enabled:Vec<bool>=rows.iter().map(|row|!row.item.disabled).collect();
        let active=selected.as_ref().and_then(|key| rows.iter()
            .position(|row|row.item.key==*key&&!row.item.disabled))
            .or_else(||enabled.iter().position(|enabled|*enabled));
        let keys:Vec<String>=rows.iter().map(|row|row.item.key().to_owned()).collect();
        let submenu:Vec<bool>=rows.iter().map(|row|row.item.is_submenu()).collect();
        let expanded:Vec<bool>=rows.iter().map(|row|row.expanded).collect();
        let parents:Vec<Option<String>>=rows.iter().map(|row|row.parent.clone()).collect();
        let open_change_keyboard=on_open_change.clone();

        let row_handles=focus_handles.clone();
        let handle_keyboard=move |event:&KeyDownEvent,window:&mut Window,cx:&mut App| {
            if event.keystroke.modifiers.modified() ||
                event.keystroke.is_ime_in_progress() {
                return;
            }
            let Some(origin)=focus_handles.iter().position(|handle|handle.is_focused(window))
            else {return;};
            let key=event.keystroke.key.as_str();
            let direction=match key {
                "down"=>Some(MenuMove::Next),
                "up"=>Some(MenuMove::Previous),
                "home"=>Some(MenuMove::First),
                "end"=>Some(MenuMove::Last),
                _=>None,
            };
            if let Some(command)=direction {
                if let Some(next)=next_menu_focus(&enabled,origin,command) {
                    window.focus(&focus_handles[next],cx);
                    window.prevent_default();
                    cx.stop_propagation();
                }
                return;
            }
            let focused_key=keys[origin].clone();
            if key=="right" && submenu[origin] && !collapsed {
                if !expanded[origin] {
                    if let Some(listener)=&open_change_keyboard {
                        listener(focused_key.into(),true,window,cx);
                    }
                } else if let Some(index)=((origin+1)..keys.len())
                    .find(|ix|parents[*ix].as_deref()==Some(focused_key.as_str())
                        && enabled[*ix])
                {
                    window.focus(&focus_handles[index],cx);
                }
                window.prevent_default();cx.stop_propagation();
            } else if (key=="left" || key=="escape") && !collapsed {
                if let Some(parent)=parents[origin].as_ref() {
                    if let Some(listener)=&open_change_keyboard {
                        listener(parent.clone().into(),false,window,cx);
                    }
                    if let Some(index)=keys.iter().position(|key|key==parent) {
                        window.focus(&focus_handles[index],cx);
                    }
                    window.prevent_default();cx.stop_propagation();
                } else if submenu[origin] && expanded[origin] {
                    if let Some(listener)=&open_change_keyboard {
                        listener(focused_key.into(),false,window,cx);
                    }
                    window.prevent_default();cx.stop_propagation();
                }
            }
        };

        div().id(self.id).role(Role::Menu)
            .w_full().flex().flex_col().gap(px(3.))
            .px(px(7.)).py(px(6.))
            .bg(overrides.color_bg.unwrap_or(
                if dark {theme.layout_dark} else {theme.surface}))
            .on_key_down(handle_keyboard)
            .children(rows.into_iter().enumerate().map(move |(index,row)| {
                let is_submenu=row.item.is_submenu();
                let chosen=selected.as_ref()
                    .is_some_and(|key|key==&row.item.key) && !is_submenu;
                let highlighted=chosen || row.selected_descendant;
                let label=row.item.label.clone();
                let key=row.item.key.clone();
                let open_key=key.clone();
                let bg=if highlighted {
                    overrides.color_bg_selected.unwrap_or(
                        if dark {theme.primary} else {theme.info_bg})
                } else {
                    overrides.color_bg.unwrap_or(
                        if dark {theme.layout_dark} else {theme.surface})
                };
                let fg=if row.item.disabled {theme.disabled_text}
                    else if highlighted {if dark {color(0xffffff)} else {theme.primary}}
                    else if row.item.danger {theme.error}
                    else if dark {theme.layout_dark_text} else {theme.text};
                let hover_bg=overrides.color_bg_hover.unwrap_or(
                    if dark {theme.layout_sider_hover} else {theme.info_bg});
                let focus_id=format!("ant-menu-{prefix}-{}",row.item.key);
                let expand_callback=on_open_change.clone();
                let select_callback=on_select.clone();
                BaseButton::new(focus_id)
                    .role(Role::MenuItem)
                    .accessibility_label(label.clone())
                    .selected(highlighted)
                    .disabled(row.item.disabled)
                    .track_focus(&row_handles[index])
                    .tab_stop(active==Some(index))
                    .w_full().h(px(item_height))
                    .px(px(if collapsed {6.} else {12.+row.depth as f32*16.}))
                    .flex().items_center().justify_start()
                    .gap(px(11.)).rounded(px(overrides.border_radius.unwrap_or(theme.border_radius)))
                    .bg(bg).text_color(overrides.color_text.unwrap_or(fg))
                    .text_size(px(overrides.font_size.unwrap_or(theme.font_size)))
                    .when(!row.item.disabled, |this| this.cursor_pointer()
                        .hover(|this|this.bg(hover_bg)))
                    .when_some(row.item.icon, |this, icon| {
                        this.child(AntIcon::new(icon).theme(theme).size(17.).color(fg))
                    })
                    .when(!collapsed, |this|this.child(div().min_w_0().flex_1().child(label)))
                    .when(is_submenu&&!collapsed, |this| {
                        this.child(AntIcon::new(if row.expanded {
                            AntIconName::DownOutlined
                        } else {
                            AntIconName::RightOutlined
                        }).theme(theme).size(12.).color(fg))
                    })
                    .when(is_submenu,|this|{
                        this.when_some(expand_callback,|this,callback|{
                            let new_open=!row.expanded;
                            this.on_click(move |_,window,cx|{
                                callback(open_key.clone(),new_open,window,cx);
                            })
                        })
                    })
                    .when(!is_submenu,|this|{
                        this.when_some(select_callback,|this,callback|{
                            this.on_click(move |_,window,cx|{
                                callback(key.clone(),window,cx);
                            })
                        })
                    })
            }))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn navigation_skips_disabled_and_wraps() {
        assert_eq!(next_menu_focus(&[true,false,true],0,MenuMove::Next),Some(2));
        assert_eq!(next_menu_focus(&[true,false,true],2,MenuMove::Next),Some(0));
        assert_eq!(next_menu_focus(&[true,false,true],0,MenuMove::Previous),Some(2));
        assert_eq!(next_menu_focus(&[false,false],0,MenuMove::Next),None);
        assert_eq!(next_menu_focus(&[],0,MenuMove::First),None);
        assert_eq!(next_menu_focus(&[false,true,false],0,MenuMove::First),Some(1));
        assert_eq!(next_menu_focus(&[false,true,false],0,MenuMove::Last),Some(1));
    }
    #[test]
    fn nested_visibility_and_controlled_expansion() {
        let items=vec![
            AntMenuItem::new("home","Home"),
            AntMenuItem::submenu("reports","Reports")
                .child(AntMenuItem::new("daily","Daily"))
                .child(AntMenuItem::submenu("advanced","Advanced")
                    .child(AntMenuItem::new("trends","Trends"))),
        ];
        let mut rows=Vec::new();
        flatten_visible(items.clone(),&BTreeSet::new(),Some("trends"),
            false,0,None,&mut rows);
        assert_eq!(rows.len(),2);
        assert!(rows[1].selected_descendant);
        let mut rows=Vec::new();
        flatten_visible(items.clone(),&BTreeSet::from(["reports".into()]),
            Some("daily"),false,0,None,&mut rows);
        assert_eq!(rows.iter().map(|r|r.item.key()).collect::<Vec<_>>(),
            vec!["home","reports","daily","advanced"]);
        assert_eq!(rows[2].parent.as_deref(),Some("reports"));
        let mut rows=Vec::new();
        flatten_visible(items.clone(),&BTreeSet::from(["reports".into(),"advanced".into()]),
            None,false,0,None,&mut rows);
        assert_eq!(rows.len(),5);
        let mut rows=Vec::new();
        flatten_visible(items,&BTreeSet::from(["reports".into()]),
            None,true,0,None,&mut rows);
        assert_eq!(rows.len(),2);
    }
    #[test]
    fn item_selection_respects_disabled_ancestors_and_submenu_headers() {
        let menu=AntMenu::new("nav")
            .selected_key("daily")
            .open_keys(["reports"])
            .item(AntMenuItem::new("home","Home").icon(AntIconName::HomeOutlined))
            .item(AntMenuItem::submenu("reports","Reports")
                .child(AntMenuItem::new("daily","Daily"))
                .child(AntMenuItem::new("locked","Locked").disabled(true)))
            .item(AntMenuItem::submenu("restricted","Restricted").disabled(true)
                .child(AntMenuItem::new("hidden","Hidden")));
        assert!(menu.can_select("home"));
        assert!(menu.can_select("daily"));
        assert!(!menu.can_select("reports"));
        assert!(!menu.can_select("locked"));
        assert!(!menu.can_select("hidden"));
        assert!(!menu.can_select("missing"));
        assert!(menu.can_expand("reports"));
        assert!(!menu.can_expand("restricted"));
        assert!(menu.open_keys.contains("reports"));
        assert_eq!(menu.selected.as_deref(),Some("daily"));
    }
    #[test]
    fn group_preserves_collapsed_and_size_defaults() {
        let menu=AntMenu::new("sidebar").dark(true).inline_collapsed(true)
            .item_height(-7.).selected_key("settings")
            .item(AntMenuItem::new("settings","Settings").danger(true));
        assert!(menu.dark);
        assert!(menu.inline_collapsed);
        assert_eq!(menu.item_height,24.);
        assert_eq!(menu.items[0].key(),"settings");
        assert!(menu.items[0].danger);
    }
}
