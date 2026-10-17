//! Ant Design Tree: parent-controlled expansion, selection, independent checks,
//! disabled nodes, and keyboard roving focus. All nodes paint in native GPUI.
//!
//! This first data-tree model is deliberately simple and non-virtualized. For
//! large file explorers use GPUI Kit's virtualized TreeState instead; lazy
//! loading, drag-drop and propagated/half-checked parents are parity backlog.
use std::{collections::BTreeSet,rc::Rc};
use gpui_kit::{
    App, ElementId, FocusHandle, InteractiveElement as _, IntoElement,
    KeyDownEvent, ParentElement as _, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};
use gpui_kit::base::{Button as BaseButton,StyledExt as _};
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntIcon,AntIconName,AntTheme};

type KeyChange=Rc<dyn Fn(SharedString,&mut Window,&mut App)>;
type ExpandChange=Rc<dyn Fn(SharedString,bool,&mut Window,&mut App)>;
type CheckChange=Rc<dyn Fn(SharedString,bool,&mut Window,&mut App)>;

/// Tree keys must be unique across roots and descendants.
#[derive(Clone,Debug)]
pub struct AntTreeNode {
    key:SharedString,
    title:SharedString,
    icon:Option<AntIconName>,
    disabled:bool,
    check_disabled:bool,
    children:Vec<AntTreeNode>,
}
impl AntTreeNode{
    pub fn new(key:impl Into<SharedString>,title:impl Into<SharedString>)->Self{
        Self{key:key.into(),title:title.into(),icon:None,disabled:false,
            check_disabled:false,children:Vec::new()}
    }
    pub fn icon(mut self,icon:AntIconName)->Self{self.icon=Some(icon);self}
    pub fn disabled(mut self,yes:bool)->Self{self.disabled=yes;self}
    pub fn check_disabled(mut self,yes:bool)->Self{self.check_disabled=yes;self}
    pub fn child(mut self,child:AntTreeNode)->Self{self.children.push(child);self}
    pub fn children(mut self,children:impl IntoIterator<Item=AntTreeNode>)->Self{
        self.children.extend(children);self
    }
}

#[derive(Clone,Debug)]
pub struct VisibleTreeNode {
    pub key:SharedString,
    pub title:SharedString,
    pub depth:usize,
    pub parent:Option<SharedString>,
    pub folder:bool,
    pub expanded:bool,
    pub disabled:bool,
    pub check_disabled:bool,
    pub icon:Option<AntIconName>,
}
/// The visible rows depend exclusively on application-controlled expanded keys.
pub fn visible_tree_nodes(
    roots:&[AntTreeNode],
    expanded:&BTreeSet<String>,
)->Vec<VisibleTreeNode>{
    fn push(items:&[AntTreeNode],open:&BTreeSet<String>,depth:usize,
        parent:Option<SharedString>,rows:&mut Vec<VisibleTreeNode>){
        for node in items {
            let folder=!node.children.is_empty();
            let is_open=folder&&open.contains(node.key.as_ref());
            rows.push(VisibleTreeNode{
                key:node.key.clone(),title:node.title.clone(),
                depth,parent:parent.clone(),folder,expanded:is_open,
                disabled:node.disabled,check_disabled:node.check_disabled,icon:node.icon,
            });
            if is_open && !node.disabled {
                push(&node.children,open,depth+1,Some(node.key.clone()),rows);
            }
        }
    }
    let mut out=Vec::new();
    push(roots,expanded,0,None,&mut out);
    out
}

/// Move keyboard focus only between visible, enabled nodes; don't change the
/// selected key unless Enter or pointer selection activates the node.
pub fn tree_focus_index(
    enabled:&[bool],current:usize,key:&str,
)->Option<usize>{
    let first=enabled.iter().position(|x|*x)?;
    let len=enabled.len();
    match key{
        "home"=>Some(first),
        "end"=>enabled.iter().rposition(|x|*x),
        "up"|"down"=>{
            for step in 1..=len {
                let ix=if key=="down"{
                    (current%len+step)%len
                } else {
                    (current%len+len-step%len)%len
                };
                if enabled[ix]{return Some(ix);}
            }
            None
        }
        _=>None,
    }
}

/// Click a label to select; click the chevron or Right/Left to expand/collapse.
/// Optional checkboxes are strictly controlled (no parent-child cascade yet).
#[derive(IntoElement)]
pub struct AntTree {
    id:ElementId,
    nodes:Vec<AntTreeNode>,
    expanded_keys:BTreeSet<String>,
    checked_keys:BTreeSet<String>,
    selected_key:Option<SharedString>,
    checkable:bool,
    show_icon:bool,
    block_node:bool,
    disabled:bool,
    max_height:f32,
    theme:AntTheme,
    on_select:Option<KeyChange>,
    on_expand:Option<ExpandChange>,
    on_check:Option<CheckChange>,
}
impl AntTree{
    pub fn new(id:impl Into<ElementId>)->Self{
        Self{id:id.into(),nodes:Vec::new(),expanded_keys:BTreeSet::new(),
            checked_keys:BTreeSet::new(),selected_key:None,checkable:false,
            show_icon:true,block_node:true,disabled:false,max_height:320.,
            theme:AntTheme::default(),on_select:None,on_expand:None,on_check:None}
    }
    pub fn child(mut self,node:AntTreeNode)->Self{self.nodes.push(node);self}
    pub fn children(mut self,nodes:impl IntoIterator<Item=AntTreeNode>)->Self{
        self.nodes.extend(nodes);self
    }
    pub fn expanded_keys<I,S>(mut self,keys:I)->Self where I:IntoIterator<Item=S>,S:AsRef<str>{
        self.expanded_keys=keys.into_iter().map(|k|k.as_ref().to_owned()).collect();self
    }
    pub fn checked_keys<I,S>(mut self,keys:I)->Self where I:IntoIterator<Item=S>,S:AsRef<str>{
        self.checked_keys=keys.into_iter().map(|k|k.as_ref().to_owned()).collect();self
    }
    pub fn selected_key(mut self,key:impl Into<SharedString>)->Self{
        self.selected_key=Some(key.into());self
    }
    pub fn checkable(mut self,yes:bool)->Self{self.checkable=yes;self}
    pub fn show_icon(mut self,yes:bool)->Self{self.show_icon=yes;self}
    pub fn block_node(mut self,yes:bool)->Self{self.block_node=yes;self}
    pub fn disabled(mut self,yes:bool)->Self{self.disabled=yes;self}
    pub fn max_height(mut self,value:f32)->Self{
        self.max_height=if value.is_finite(){value.max(56.)}else{320.};self
    }
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
    pub fn on_select(mut self,f:impl Fn(SharedString,&mut Window,&mut App)+'static)->Self{
        self.on_select=Some(Rc::new(f));self
    }
    pub fn on_expand(mut self,f:impl Fn(SharedString,bool,&mut Window,&mut App)+'static)->Self{
        self.on_expand=Some(Rc::new(f));self
    }
    pub fn on_check(mut self,f:impl Fn(SharedString,bool,&mut Window,&mut App)+'static)->Self{
        self.on_check=Some(Rc::new(f));self
    }
}
impl RenderOnce for AntTree{
    fn render(self,window:&mut Window,cx:&mut App)->impl IntoElement{
        let theme=self.theme;
        let nodes=visible_tree_nodes(&self.nodes,&self.expanded_keys);
        let enabled:Vec<bool>=nodes.iter().map(|node|!node.disabled&&!self.disabled).collect();
        let selected=self.selected_key.clone();
        let tab_stop=selected.as_ref().and_then(|key|{
            nodes.iter().position(|row|row.key==*key&&!row.disabled)
        }).or_else(||enabled.iter().position(|value|*value));
        let prefix=format!("{:?}",self.id);
        let focus_handles:Vec<FocusHandle>=nodes.iter().map(|node|{
            window.use_keyed_state(format!("ant-tree:{prefix}:{}",node.key),cx,
                |_,cx|cx.focus_handle()).read(cx).clone()
        }).collect();
        let focus_keyboard=focus_handles.clone();
        let enabled_keyboard=enabled.clone();
        let keys:Vec<SharedString>=nodes.iter().map(|n|n.key.clone()).collect();
        let parents:Vec<Option<SharedString>>=nodes.iter().map(|n|n.parent.clone()).collect();
        let expanded:Vec<bool>=nodes.iter().map(|n|n.expanded).collect();
        let folders:Vec<bool>=nodes.iter().map(|n|n.folder).collect();
        let on_expand_keyboard=self.on_expand.clone();
        let on_keyboard=move |event:&KeyDownEvent,window:&mut Window,cx:&mut App|{
            if event.keystroke.modifiers.modified()||event.keystroke.is_ime_in_progress(){return;}
            let Some(origin)=focus_keyboard.iter().position(|f|f.is_focused(window))else{return;};
            let key=event.keystroke.key.as_str();
            if let Some(next)=tree_focus_index(&enabled_keyboard,origin,key){
                window.focus(&focus_keyboard[next],cx);
                window.prevent_default();cx.stop_propagation();
                return;
            }
            if key=="right"&&folders[origin] {
                if !expanded[origin]{
                    if let Some(f)=&on_expand_keyboard{f(keys[origin].clone(),true,window,cx);}
                }else if let Some(ix)=((origin+1)..keys.len()).find(|ix|
                    parents[*ix].as_ref()==Some(&keys[origin]) && enabled_keyboard[*ix]){
                    window.focus(&focus_keyboard[ix],cx);
                }
                window.prevent_default();cx.stop_propagation();
            }else if key=="left"{
                if folders[origin]&&expanded[origin]{
                    if let Some(f)=&on_expand_keyboard{f(keys[origin].clone(),false,window,cx);}
                    window.prevent_default();cx.stop_propagation();
                }else if let Some(parent)=&parents[origin]{
                    if let Some(ix)=keys.iter().position(|key|key==parent){
                        window.focus(&focus_keyboard[ix],cx);
                        window.prevent_default();cx.stop_propagation();
                    }
                }
            }
        };
        let disabled=self.disabled;
        let checkable=self.checkable;
        let show_icon=self.show_icon;
        let block=self.block_node;
        let checked=self.checked_keys;
        let on_expand=self.on_expand;
        let on_select=self.on_select;
        let on_check=self.on_check;
        div().id(self.id).role(Role::Tree)
            .w_full().max_h(px(self.max_height)).overflow_y_scrollbar()
            .flex().flex_col().gap(px(2.)).p(px(4.))
            .on_key_down(on_keyboard)
            .children(nodes.into_iter().enumerate().map(move|(index,node)|{
                let is_selected=selected.as_ref()==Some(&node.key);
                let is_checked=checked.contains(node.key.as_ref());
                let row_disabled=disabled||node.disabled;
                let key=node.key.clone();
                let toggle_key=key.clone();
                let select_key=key.clone();
                let check_key=key.clone();
                let expand_listener=on_expand.clone();
                let select_listener=on_select.clone();
                let check_listener=on_check.clone();
                let title=node.title.clone();
                let requested_expansion=!node.expanded;
                let fg=if row_disabled{theme.disabled_text}
                    else if is_selected{theme.primary}else{theme.text};
                let arrow=BaseButton::new(format!("ant-tree-expand:{prefix}:{key}"))
                    .tab_stop(false).disabled(row_disabled||!node.folder)
                    .accessibility_label(format!("{} {}",if node.expanded{"Collapse"}else{"Expand"},title))
                    .size(px(28.)).flex().items_center().justify_center()
                    .child(AntIcon::new(if node.expanded{AntIconName::DownOutlined}
                        else{AntIconName::RightOutlined}).theme(theme).color(fg).size(12.))
                    .when_some(expand_listener,|this,listener|{
                        this.on_click(move|_,window,cx|{
                            listener(toggle_key.clone(),requested_expansion,window,cx);
                        })
                    });
                let label=BaseButton::new(format!("ant-tree:{prefix}:{key}"))
                    .role(Role::TreeItem)
                    .selected(is_selected).disabled(row_disabled)
                    .accessibility_label(title.clone())
                    .track_focus(&focus_handles[index])
                    .tab_stop(tab_stop==Some(index))
                    .when(block,|this|this.flex_1())
                    .h(px(32.)).px(px(8.)).flex().items_center().gap(px(8.))
                    .rounded(px(theme.border_radius))
                    .bg(if is_selected{theme.info_bg}else{theme.surface})
                    .text_color(fg).text_size(px(14.))
                    .when(!row_disabled,|this|this.cursor_pointer()
                        .hover(|this|this.bg(theme.info_bg)))
                    .when(show_icon,|this|this.child(
                        AntIcon::new(node.icon.unwrap_or(if node.folder{
                            if node.expanded{AntIconName::FolderOpenOutlined}
                            else{AntIconName::FolderOutlined}
                        }else{AntIconName::FileOutlined}))
                        .theme(theme).color(fg).size(16.)))
                    .child(title)
                    .when_some(select_listener,|this,listener|{
                        this.on_click(move|_,window,cx|{
                            listener(select_key.clone(),window,cx);
                        })
                    });
                div().w_full().flex().items_center().gap(px(3.))
                    .pl(px(node.depth as f32*20.))
                    .when(node.folder,|this|this.child(arrow))
                    .when(!node.folder,|this|this.child(div().w(px(28.))))
                    .when(checkable,|this|{
                        let label_text=format!("{} {}",if is_checked{"Uncheck"}else{"Check"},node.title);
                        let check=BaseButton::new(format!("ant-tree-check:{prefix}:{key}"))
                            .tab_stop(true).disabled(row_disabled||node.check_disabled)
                            .accessibility_label(label_text)
                            .size(px(24.)).flex().items_center().justify_center()
                            .child(AntIcon::new(if is_checked{AntIconName::CheckSquareFilled}
                                else{AntIconName::BorderOutlined})
                                .theme(theme).color(if is_checked{theme.primary}else{theme.text_secondary}).size(17.))
                            .when_some(check_listener,|this,listener|{
                                this.on_click(move|_,window,cx|{
                                    listener(check_key.clone(),!is_checked,window,cx);
                                })
                            });
                        this.child(check)
                    })
                    .child(label)
            }))
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    fn sample()->Vec<AntTreeNode>{
        vec![AntTreeNode::new("src","src")
            .child(AntTreeNode::new("a","a.rs"))
            .child(AntTreeNode::new("disabled","no.rs").disabled(true)),
            AntTreeNode::new("readme","README")]
    }
    #[test]
    fn visible_rows_respect_open_keys(){
        let closed=visible_tree_nodes(&sample(),&BTreeSet::new());
        assert_eq!(closed.iter().map(|row|row.key.as_ref()).collect::<Vec<_>>(),vec!["src","readme"]);
        let expanded=visible_tree_nodes(&sample(),&BTreeSet::from(["src".into()]));
        assert_eq!(expanded.len(),4);
        assert_eq!(expanded[1].depth,1);
        assert_eq!(expanded[1].parent.as_deref(),Some("src"));
        assert!(expanded[2].disabled);
    }
    #[test]
    fn tree_keyboard_skips_disabled(){
        assert_eq!(tree_focus_index(&[true,false,true],0,"down"),Some(2));
        assert_eq!(tree_focus_index(&[true,false,true],2,"down"),Some(0));
        assert_eq!(tree_focus_index(&[false,true,true],2,"home"),Some(1));
        assert_eq!(tree_focus_index(&[],0,"up"),None);
        assert_eq!(tree_focus_index(&[true],0,"escape"),None);
    }
}
