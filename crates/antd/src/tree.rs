//! Ant Design Tree: parent-controlled expansion, selection, cascading or strict checks,
//! disabled nodes, and keyboard roving focus. All nodes paint in native GPUI.
//!
//! This first data-tree model is deliberately simple and non-virtualized. For
//! large file explorers use GPUI Kit's virtualized TreeState instead; lazy
//! loading and drag-drop remain parity backlog. Check state derives from keyed roots.
use crate::design_tokens::AntComponent;
use crate::{AntIcon, AntIconName, AntTheme};
use gpui_kit::base::{Button as BaseButton, StyledExt as _};
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, ElementId, FocusHandle, InteractiveElement as _, IntoElement, KeyDownEvent,
    ParentElement as _, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    Styled as _, Window, div, px,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

type KeyChange = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type ExpandChange = Rc<dyn Fn(SharedString, bool, &mut Window, &mut App)>;
type CheckChange = Rc<dyn Fn(SharedString, bool, &mut Window, &mut App)>;

/// Tree keys must be unique across roots and descendants.
#[derive(Clone, Debug)]
pub struct AntTreeNode {
    key: SharedString,
    title: SharedString,
    icon: Option<AntIconName>,
    disabled: bool,
    check_disabled: bool,
    children: Vec<AntTreeNode>,
}
impl AntTreeNode {
    pub fn new(key: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            icon: None,
            disabled: false,
            check_disabled: false,
            children: Vec::new(),
        }
    }
    pub fn icon(mut self, icon: AntIconName) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn disabled(mut self, yes: bool) -> Self {
        self.disabled = yes;
        self
    }
    pub fn check_disabled(mut self, yes: bool) -> Self {
        self.check_disabled = yes;
        self
    }
    pub fn child(mut self, child: AntTreeNode) -> Self {
        self.children.push(child);
        self
    }
    pub fn children(mut self, children: impl IntoIterator<Item = AntTreeNode>) -> Self {
        self.children.extend(children);
        self
    }
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }
    pub fn title(&self) -> &SharedString {
        &self.title
    }
    pub fn child_nodes(&self) -> &[AntTreeNode] {
        &self.children
    }
}

#[derive(Clone, Debug)]
pub struct VisibleTreeNode {
    pub key: SharedString,
    pub title: SharedString,
    pub depth: usize,
    pub parent: Option<SharedString>,
    pub folder: bool,
    pub expanded: bool,
    pub disabled: bool,
    pub check_disabled: bool,
    pub icon: Option<AntIconName>,
}
/// The visible rows depend exclusively on application-controlled expanded keys.
pub fn visible_tree_nodes(
    roots: &[AntTreeNode],
    expanded: &BTreeSet<String>,
) -> Vec<VisibleTreeNode> {
    fn push(
        items: &[AntTreeNode],
        open: &BTreeSet<String>,
        depth: usize,
        parent: Option<SharedString>,
        rows: &mut Vec<VisibleTreeNode>,
    ) {
        for node in items {
            let folder = !node.children.is_empty();
            let is_open = folder && open.contains(node.key.as_ref());
            rows.push(VisibleTreeNode {
                key: node.key.clone(),
                title: node.title.clone(),
                depth,
                parent: parent.clone(),
                folder,
                expanded: is_open,
                disabled: node.disabled,
                check_disabled: node.check_disabled,
                icon: node.icon,
            });
            if is_open && !node.disabled {
                push(
                    &node.children,
                    open,
                    depth + 1,
                    Some(node.key.clone()),
                    rows,
                );
            }
        }
    }
    let mut out = Vec::new();
    push(roots, expanded, 0, None, &mut out);
    out
}

/// Move keyboard focus only between visible, enabled nodes; don't change the
/// selected key unless Enter or pointer selection activates the node.
pub fn tree_focus_index(enabled: &[bool], current: usize, key: &str) -> Option<usize> {
    let first = enabled.iter().position(|x| *x)?;
    let len = enabled.len();
    match key {
        "home" => Some(first),
        "end" => enabled.iter().rposition(|x| *x),
        "up" | "down" => {
            for step in 1..=len {
                let ix = if key == "down" {
                    (current % len + step) % len
                } else {
                    (current % len + len - step % len) % len
                };
                if enabled[ix] {
                    return Some(ix);
                }
            }
            None
        }
        _ => None,
    }
}

/// Visual checkbox state. Indeterminate is derived from enabled descendants;
/// callers own checked keys and do not need to store a second half-check set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeCheckState {
    Unchecked,
    Checked,
    Indeterminate,
}

/// Evaluate one node from the controlled checked-key set. A disabled node or
/// checkbox-disabled node blocks participation in its ancestor's check state.
/// An explicitly checked disabled node still paints as checked.
fn node_check_state(node: &AntTreeNode, checked: &BTreeSet<String>) -> Option<TreeCheckState> {
    if node.disabled || node.check_disabled {
        return None;
    }
    let mut eligible = 0usize;
    let mut completed = 0usize;
    let mut touched = 0usize;
    for child in &node.children {
        if let Some(state) = node_check_state(child, checked) {
            eligible += 1;
            match state {
                TreeCheckState::Checked => {
                    completed += 1;
                    touched += 1;
                }
                TreeCheckState::Indeterminate => {
                    touched += 1;
                }
                TreeCheckState::Unchecked => {}
            }
        }
    }
    if eligible == 0 {
        Some(if checked.contains(node.key.as_ref()) {
            TreeCheckState::Checked
        } else {
            TreeCheckState::Unchecked
        })
    } else if completed == eligible {
        Some(TreeCheckState::Checked)
    } else if touched > 0 {
        Some(TreeCheckState::Indeterminate)
    } else {
        Some(TreeCheckState::Unchecked)
    }
}

/// Derive display state for all nodes, including collapsed descendants.
///
/// When strictly checking, each key is independent and there is no
/// indeterminate state. Otherwise, a parent is full only when all its
/// eligible child branches are full. A disabled subtree does not contribute.
pub fn tree_check_states(
    roots: &[AntTreeNode],
    checked: &BTreeSet<String>,
    strict: bool,
) -> BTreeMap<String, TreeCheckState> {
    fn collect(
        nodes: &[AntTreeNode],
        checked: &BTreeSet<String>,
        strict: bool,
        out: &mut BTreeMap<String, TreeCheckState>,
    ) {
        for node in nodes {
            let state = if strict || node.disabled || node.check_disabled {
                if checked.contains(node.key.as_ref()) {
                    TreeCheckState::Checked
                } else {
                    TreeCheckState::Unchecked
                }
            } else {
                node_check_state(node, checked).unwrap_or(TreeCheckState::Unchecked)
            };
            out.insert(node.key.to_string(), state);
            collect(&node.children, checked, strict, out);
        }
    }
    let mut out = BTreeMap::new();
    collect(roots, checked, strict, &mut out);
    out
}

/// Pure state transition for Ant Design's non-strict parent-child checking.
///
/// Checking a branch applies to every enabled and checkable descendant,
/// including those hidden under collapsed nodes. Ancestors are then
/// recalculated bottom-up. Disabled subtrees retain their checked values and
/// do not contribute to the parent's full/partial calculation.
/// Invalid and disabled target keys are a no-op.
pub fn cascade_tree_checked_keys(
    roots: &[AntTreeNode],
    checked: &BTreeSet<String>,
    target: &str,
    next: bool,
) -> BTreeSet<String> {
    fn apply_branch(node: &AntTreeNode, keys: &mut BTreeSet<String>, check: bool) {
        if node.disabled {
            return;
        }
        if !node.check_disabled {
            if check {
                keys.insert(node.key.to_string());
            } else {
                keys.remove(node.key.as_ref());
            }
        }
        for child in &node.children {
            apply_branch(child, keys, check);
        }
    }
    fn locate(
        nodes: &[AntTreeNode],
        target: &str,
        check: bool,
        keys: &mut BTreeSet<String>,
    ) -> bool {
        for node in nodes {
            if node.key.as_ref() == target {
                if node.disabled || node.check_disabled {
                    return false;
                }
                apply_branch(node, keys, check);
                return true;
            }
            if !node.disabled && locate(&node.children, target, check, keys) {
                return true;
            }
        }
        false
    }
    fn normalize(node: &AntTreeNode, keys: &mut BTreeSet<String>) {
        if node.disabled {
            return;
        }
        for child in &node.children {
            normalize(child, keys);
        }
        if node.check_disabled {
            return;
        }
        let states: Vec<_> = node
            .children
            .iter()
            .filter_map(|child| node_check_state(child, keys))
            .collect();
        if !states.is_empty() {
            if states.iter().all(|state| *state == TreeCheckState::Checked) {
                keys.insert(node.key.to_string());
            } else {
                keys.remove(node.key.as_ref());
            }
        }
    }
    let mut out = checked.clone();
    if !locate(roots, target, next, &mut out) {
        return out;
    }
    for node in roots {
        normalize(node, &mut out);
    }
    out
}

/// Click a label to select; click the chevron or Right/Left to expand/collapse.
/// Checks may be strict (independent) or cascading (derived half-check states).
#[derive(IntoElement)]
pub struct AntTree {
    id: ElementId,
    nodes: Vec<AntTreeNode>,
    expanded_keys: BTreeSet<String>,
    checked_keys: BTreeSet<String>,
    selected_key: Option<SharedString>,
    checkable: bool,
    check_strictly: bool,
    show_icon: bool,
    block_node: bool,
    disabled: bool,
    max_height: f32,
    theme: AntTheme,
    on_select: Option<KeyChange>,
    on_expand: Option<ExpandChange>,
    on_check: Option<CheckChange>,
    on_checked_keys_change: Option<Rc<dyn Fn(BTreeSet<String>, &mut Window, &mut App)>>,
}
impl AntTree {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            nodes: Vec::new(),
            expanded_keys: BTreeSet::new(),
            checked_keys: BTreeSet::new(),
            selected_key: None,
            checkable: false,
            check_strictly: false,
            show_icon: true,
            block_node: true,
            disabled: false,
            max_height: 320.,
            theme: AntTheme::default(),
            on_select: None,
            on_expand: None,
            on_check: None,
            on_checked_keys_change: None,
        }
    }
    pub fn child(mut self, node: AntTreeNode) -> Self {
        self.nodes.push(node);
        self
    }
    pub fn children(mut self, nodes: impl IntoIterator<Item = AntTreeNode>) -> Self {
        self.nodes.extend(nodes);
        self
    }
    pub fn expanded_keys<I, S>(mut self, keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.expanded_keys = keys.into_iter().map(|k| k.as_ref().to_owned()).collect();
        self
    }
    pub fn checked_keys<I, S>(mut self, keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.checked_keys = keys.into_iter().map(|k| k.as_ref().to_owned()).collect();
        self
    }
    pub fn selected_key(mut self, key: impl Into<SharedString>) -> Self {
        self.selected_key = Some(key.into());
        self
    }
    pub fn checkable(mut self, yes: bool) -> Self {
        self.checkable = yes;
        self
    }
    /// Independent checkboxes (Ant Design checkStrictly) rather than
    /// default cascading/indeterminate branch state.
    pub fn check_strictly(mut self, strict: bool) -> Self {
        self.check_strictly = strict;
        self
    }
    pub fn show_icon(mut self, yes: bool) -> Self {
        self.show_icon = yes;
        self
    }
    pub fn block_node(mut self, yes: bool) -> Self {
        self.block_node = yes;
        self
    }
    pub fn disabled(mut self, yes: bool) -> Self {
        self.disabled = yes;
        self
    }
    pub fn max_height(mut self, value: f32) -> Self {
        self.max_height = if value.is_finite() {
            value.max(56.)
        } else {
            320.
        };
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(f));
        self
    }
    pub fn on_expand(
        mut self,
        f: impl Fn(SharedString, bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_expand = Some(Rc::new(f));
        self
    }
    /// Legacy single-key request. In cascading mode, prefer
    /// on_checked_keys_change to update the full resulting key set.
    pub fn on_check(
        mut self,
        f: impl Fn(SharedString, bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_check = Some(Rc::new(f));
        self
    }
    /// Receives the full requested checked-key set after native activation.
    /// Parent Entity applies the result and calls cx.notify().
    pub fn on_checked_keys_change(
        mut self,
        f: impl Fn(BTreeSet<String>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_keys_change = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntTree {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let overrides = theme.component(AntComponent::Tree);
        let bg = overrides.color_bg.unwrap_or(theme.surface);
        let selected_bg = overrides.color_bg_selected.unwrap_or(theme.info_bg);
        let hover_bg = overrides.color_bg_hover.unwrap_or(theme.info_bg);
        let fg_base = overrides.color_text.unwrap_or(theme.text);
        let primary = overrides.color_primary.unwrap_or(theme.primary);
        let radius = overrides.border_radius.unwrap_or(theme.border_radius);
        let height = overrides.control_height.unwrap_or(32.).max(24.);
        let font_size = overrides.font_size.unwrap_or(theme.font_size);
        let check_states = tree_check_states(&self.nodes, &self.checked_keys, self.check_strictly);
        let tree_roots = Rc::new(self.nodes.clone());
        let strict = self.check_strictly;
        let nodes = visible_tree_nodes(&self.nodes, &self.expanded_keys);
        let enabled: Vec<bool> = nodes
            .iter()
            .map(|node| !node.disabled && !self.disabled)
            .collect();
        let selected = self.selected_key.clone();
        let tab_stop = selected
            .as_ref()
            .and_then(|key| {
                nodes
                    .iter()
                    .position(|row| row.key == *key && !row.disabled)
            })
            .or_else(|| enabled.iter().position(|value| *value));
        let prefix = format!("{:?}", self.id);
        let focus_handles: Vec<FocusHandle> = nodes
            .iter()
            .map(|node| {
                window
                    .use_keyed_state(format!("ant-tree:{prefix}:{}", node.key), cx, |_, cx| {
                        cx.focus_handle()
                    })
                    .read(cx)
                    .clone()
            })
            .collect();
        let focus_keyboard = focus_handles.clone();
        let enabled_keyboard = enabled.clone();
        let keys: Vec<SharedString> = nodes.iter().map(|n| n.key.clone()).collect();
        let parents: Vec<Option<SharedString>> = nodes.iter().map(|n| n.parent.clone()).collect();
        let expanded: Vec<bool> = nodes.iter().map(|n| n.expanded).collect();
        let folders: Vec<bool> = nodes.iter().map(|n| n.folder).collect();
        let on_expand_keyboard = self.on_expand.clone();
        let on_keyboard = move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
            if event.keystroke.modifiers.modified() || event.keystroke.is_ime_in_progress() {
                return;
            }
            let Some(origin) = focus_keyboard.iter().position(|f| f.is_focused(window)) else {
                return;
            };
            let key = event.keystroke.key.as_str();
            if let Some(next) = tree_focus_index(&enabled_keyboard, origin, key) {
                window.focus(&focus_keyboard[next], cx);
                window.prevent_default();
                cx.stop_propagation();
                return;
            }
            if key == "right" && folders[origin] {
                if !expanded[origin] {
                    if let Some(f) = &on_expand_keyboard {
                        f(keys[origin].clone(), true, window, cx);
                    }
                } else if let Some(ix) = ((origin + 1)..keys.len()).find(|ix| {
                    parents[*ix].as_ref() == Some(&keys[origin]) && enabled_keyboard[*ix]
                }) {
                    window.focus(&focus_keyboard[ix], cx);
                }
                window.prevent_default();
                cx.stop_propagation();
            } else if key == "left" {
                if folders[origin] && expanded[origin] {
                    if let Some(f) = &on_expand_keyboard {
                        f(keys[origin].clone(), false, window, cx);
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                } else if let Some(parent) = &parents[origin] {
                    if let Some(ix) = keys.iter().position(|key| key == parent) {
                        window.focus(&focus_keyboard[ix], cx);
                        window.prevent_default();
                        cx.stop_propagation();
                    }
                }
            }
        };
        let disabled = self.disabled;
        let checkable = self.checkable;
        let show_icon = self.show_icon;
        let block = self.block_node;
        let checked = self.checked_keys;
        let on_expand = self.on_expand;
        let on_select = self.on_select;
        let on_check = self.on_check;
        let on_checked_keys_change = self.on_checked_keys_change;
        div()
            .id(self.id)
            .role(Role::Tree)
            .w_full()
            .max_h(px(self.max_height))
            .overflow_y_scrollbar()
            .flex()
            .flex_col()
            .gap(px(2.))
            .p(px(4.))
            .on_key_down(on_keyboard)
            .children(nodes.into_iter().enumerate().map(move |(index, node)| {
                let is_selected = selected.as_ref() == Some(&node.key);
                let check_state = check_states
                    .get(node.key.as_ref())
                    .copied()
                    .unwrap_or(TreeCheckState::Unchecked);
                let is_checked = check_state == TreeCheckState::Checked;
                let row_disabled = disabled || node.disabled;
                let key = node.key.clone();
                let toggle_key = key.clone();
                let select_key = key.clone();
                let check_key = key.clone();
                let expand_listener = on_expand.clone();
                let select_listener = on_select.clone();
                let check_listener = on_check.clone();
                let checked_set_listener = on_checked_keys_change.clone();
                let source_roots = tree_roots.clone();
                let previous_checks = checked.clone();
                let title = node.title.clone();
                let requested_expansion = !node.expanded;
                let fg = if row_disabled {
                    theme.disabled_text
                } else if is_selected {
                    primary
                } else {
                    fg_base
                };
                let arrow = BaseButton::new(format!("ant-tree-expand:{prefix}:{key}"))
                    .tab_stop(false)
                    .disabled(row_disabled || !node.folder)
                    .accessibility_label(format!(
                        "{} {}",
                        if node.expanded { "Collapse" } else { "Expand" },
                        title
                    ))
                    .size(px(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        AntIcon::new(if node.expanded {
                            AntIconName::DownOutlined
                        } else {
                            AntIconName::RightOutlined
                        })
                        .theme(theme)
                        .color(fg)
                        .size(12.),
                    )
                    .when_some(expand_listener, |this, listener| {
                        this.on_click(move |_, window, cx| {
                            listener(toggle_key.clone(), requested_expansion, window, cx);
                        })
                    });
                let label = BaseButton::new(format!("ant-tree:{prefix}:{key}"))
                    .role(Role::TreeItem)
                    .selected(is_selected)
                    .disabled(row_disabled)
                    .accessibility_label(title.clone())
                    .track_focus(&focus_handles[index])
                    .tab_stop(tab_stop == Some(index))
                    .when(block, |this| this.flex_1())
                    .h(px(height))
                    .px(px(8.))
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .rounded(px(radius))
                    .bg(if is_selected { selected_bg } else { bg })
                    .text_color(fg)
                    .text_size(px(font_size))
                    .when(!row_disabled, |this| {
                        this.cursor_pointer().hover(|this| this.bg(hover_bg))
                    })
                    .when(show_icon, |this| {
                        this.child(
                            AntIcon::new(node.icon.unwrap_or(if node.folder {
                                if node.expanded {
                                    AntIconName::FolderOpenOutlined
                                } else {
                                    AntIconName::FolderOutlined
                                }
                            } else {
                                AntIconName::FileOutlined
                            }))
                            .theme(theme)
                            .color(fg)
                            .size(16.),
                        )
                    })
                    .child(title)
                    .when_some(select_listener, |this, listener| {
                        this.on_click(move |_, window, cx| {
                            listener(select_key.clone(), window, cx);
                        })
                    });
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .gap(px(3.))
                    .pl(px(node.depth as f32 * 20.))
                    .when(node.folder, |this| this.child(arrow))
                    .when(!node.folder, |this| this.child(div().w(px(28.))))
                    .when(checkable, |this| {
                        let label_text = format!(
                            "{} {}",
                            match check_state {
                                TreeCheckState::Checked => "Uncheck",
                                TreeCheckState::Indeterminate => "Partially checked",
                                TreeCheckState::Unchecked => "Check",
                            },
                            node.title
                        );
                        let check = BaseButton::new(format!("ant-tree-check:{prefix}:{key}"))
                            .tab_stop(true)
                            .disabled(row_disabled || node.check_disabled)
                            .accessibility_label(label_text)
                            .size(px(24.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                AntIcon::new(match check_state {
                                    TreeCheckState::Checked => AntIconName::CheckSquareFilled,
                                    TreeCheckState::Indeterminate => AntIconName::MinusSquareFilled,
                                    TreeCheckState::Unchecked => AntIconName::BorderOutlined,
                                })
                                .theme(theme)
                                .color(if check_state != TreeCheckState::Unchecked {
                                    primary
                                } else {
                                    theme.text_secondary
                                })
                                .size(17.),
                            )
                            .when(
                                check_listener.is_some() || checked_set_listener.is_some(),
                                |this| {
                                    this.on_click(move |_, window, cx| {
                                        let requested = !is_checked;
                                        if let Some(listener) = &checked_set_listener {
                                            let next = if strict {
                                                let mut next = previous_checks.clone();
                                                if requested {
                                                    next.insert(check_key.to_string());
                                                } else {
                                                    next.remove(check_key.as_ref());
                                                }
                                                next
                                            } else {
                                                cascade_tree_checked_keys(
                                                    &source_roots,
                                                    &previous_checks,
                                                    check_key.as_ref(),
                                                    requested,
                                                )
                                            };
                                            listener(next, window, cx);
                                        }
                                        if let Some(listener) = &check_listener {
                                            listener(check_key.clone(), requested, window, cx);
                                        }
                                    })
                                },
                            );
                        this.child(check)
                    })
                    .child(label)
            }))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Vec<AntTreeNode> {
        vec![
            AntTreeNode::new("src", "src")
                .child(AntTreeNode::new("a", "a.rs"))
                .child(AntTreeNode::new("disabled", "no.rs").disabled(true)),
            AntTreeNode::new("readme", "README"),
        ]
    }
    #[test]
    fn visible_rows_respect_open_keys() {
        let closed = visible_tree_nodes(&sample(), &BTreeSet::new());
        assert_eq!(
            closed
                .iter()
                .map(|row| row.key.as_ref())
                .collect::<Vec<_>>(),
            vec!["src", "readme"]
        );
        let expanded = visible_tree_nodes(&sample(), &BTreeSet::from(["src".into()]));
        assert_eq!(expanded.len(), 4);
        assert_eq!(expanded[1].depth, 1);
        assert_eq!(expanded[1].parent.as_deref(), Some("src"));
        assert!(expanded[2].disabled);
    }
    #[test]
    fn tree_keyboard_skips_disabled() {
        assert_eq!(tree_focus_index(&[true, false, true], 0, "down"), Some(2));
        assert_eq!(tree_focus_index(&[true, false, true], 2, "down"), Some(0));
        assert_eq!(tree_focus_index(&[false, true, true], 2, "home"), Some(1));
        assert_eq!(tree_focus_index(&[], 0, "up"), None);
        assert_eq!(tree_focus_index(&[true], 0, "escape"), None);
    }

    #[test]
    fn cascaded_checks_propagate_and_show_partial_parents() {
        let roots = sample();
        let empty = BTreeSet::new();
        let half = cascade_tree_checked_keys(&roots, &empty, "a", true);
        // The only other child is disabled, so the parent is fully
        // selected as soon as every eligible child is selected.
        assert_eq!(half, BTreeSet::from(["a".to_owned(), "src".to_owned()]));
        let states = tree_check_states(&roots, &half, false);
        assert_eq!(states["a"], TreeCheckState::Checked);
        assert_eq!(states["src"], TreeCheckState::Checked);
        // Disabled "disabled" subtree contributes no votes.
        assert_eq!(states["disabled"], TreeCheckState::Unchecked);
        let after = cascade_tree_checked_keys(&roots, &half, "src", false);
        assert!(after.is_empty());
    }
    #[test]
    fn deep_cascade_indeterminate_and_disabled_preserved() {
        let roots = vec![
            AntTreeNode::new("root", "Root")
                .child(
                    AntTreeNode::new("branch", "Branch")
                        .child(AntTreeNode::new("one", "One"))
                        .child(AntTreeNode::new("two", "Two")),
                )
                .child(AntTreeNode::new("locked", "Locked").disabled(true)),
        ];
        let pre = BTreeSet::from(["locked".to_owned()]);
        let one = cascade_tree_checked_keys(&roots, &pre, "one", true);
        assert_eq!(
            tree_check_states(&roots, &one, false)["root"],
            TreeCheckState::Indeterminate
        );
        assert_eq!(
            tree_check_states(&roots, &one, false)["branch"],
            TreeCheckState::Indeterminate
        );
        assert!(one.contains("locked"));
        let two = cascade_tree_checked_keys(&roots, &one, "two", true);
        assert!(two.contains("branch") && two.contains("root") && two.contains("locked"));
        let cleared = cascade_tree_checked_keys(&roots, &two, "branch", false);
        assert!(!cleared.contains("root"));
        assert_eq!(cleared, BTreeSet::from(["locked".to_owned()]));
        assert_eq!(
            cascade_tree_checked_keys(&roots, &pre, "locked", false),
            pre
        );
        assert_eq!(
            cascade_tree_checked_keys(&roots, &pre, "missing", true),
            pre
        );
    }
    #[test]
    fn strict_check_state_does_not_derive_parents() {
        let keys = BTreeSet::from(["a".to_owned()]);
        let states = tree_check_states(&sample(), &keys, true);
        assert_eq!(states["src"], TreeCheckState::Unchecked);
        assert_eq!(states["a"], TreeCheckState::Checked);
    }
}
