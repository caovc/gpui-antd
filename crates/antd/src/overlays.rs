//! Native Ant Design-inspired overlays on GPUI Kit base + Kit Dialog.
//!
//! Dropdown, Popover and Popconfirm use `gpui_kit::base::Popover` (focus,
//! Escape, outside dismiss, deferred placement) with Ant-styled triggers and
//! panels — the same pattern as `menu.rs` flyouts. Modal keeps Kit `Dialog`
//! for focus trap, mask and stacked z-order.
//!
//! Placement uses the same popup anchor math as Tooltip: `Anchor::Top*` sits
//! below the trigger, `Anchor::Bottom*` sits above it. Default is below, left-aligned.

use crate::design_tokens::AntComponent;
use crate::menu::{MenuMove, move_for_key, next_menu_focus};
use crate::{AntButton, AntIcon, AntIconName, AntTheme, color};
use gpui_kit::base::actions::Confirm;
use gpui_kit::base::{
    Button as BaseButton, Popover as BasePopover, StyledExt as _, TestSupportExt as _,
};
use gpui_kit::component::{
    WindowExt as _,
    dialog::{Dialog as KitDialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle},
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    Anchor, AnyElement, App, ElementId, FocusHandle, Focusable as _, Hsla, InteractiveElement as _,
    IntoElement, KeyDownEvent, ParentElement as _, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};
use std::rc::Rc;

type MenuSelect = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type OpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type Action = Rc<dyn Fn(&mut Window, &mut App) -> bool>;

/// Where a dropdown, popover or popconfirm opens relative to its trigger.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverlayPlacement {
    #[default]
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

/// Popup corner pinned to the opposite edge of the trigger. See `popup.rs`
/// `anchor_position`.
pub fn overlay_anchor(placement: OverlayPlacement) -> Anchor {
    match placement {
        OverlayPlacement::BottomLeft => Anchor::TopLeft,
        OverlayPlacement::BottomRight => Anchor::TopRight,
        OverlayPlacement::TopLeft => Anchor::BottomLeft,
        OverlayPlacement::TopRight => Anchor::BottomRight,
    }
}

/// Resolved chrome for overlay triggers and elevated panels.
#[derive(Clone, Copy, Debug, PartialEq)]
struct OverlayChrome {
    trigger_bg: Hsla,
    trigger_text: Hsla,
    trigger_border: Hsla,
    height: f32,
    radius: f32,
    panel_bg: Hsla,
    panel_text: Hsla,
    panel_border: Hsla,
    panel_radius: f32,
    header_bg: Hsla,
}

/// Map theme + optional component tokens into overlay chrome.
fn overlay_chrome(theme: AntTheme, kind: AntComponent, disabled: bool) -> OverlayChrome {
    let token = theme.component(kind);
    OverlayChrome {
        trigger_bg: if disabled {
            theme.disabled_bg
        } else {
            token.color_bg.unwrap_or(theme.surface)
        },
        trigger_text: if disabled {
            theme.disabled_text
        } else {
            token.color_text.unwrap_or(theme.text)
        },
        trigger_border: token.color_border.unwrap_or(theme.border),
        height: token
            .control_height
            .unwrap_or(theme.control_height)
            .max(24.),
        radius: token.border_radius.unwrap_or(theme.border_radius),
        panel_bg: theme.bg_elevated,
        panel_text: token.color_text.unwrap_or(theme.text),
        panel_border: token.color_border.unwrap_or(theme.border),
        panel_radius: token.border_radius.unwrap_or(theme.border_radius_lg),
        header_bg: token.color_header_bg.unwrap_or(theme.bg_elevated),
    }
}

fn transparent() -> Hsla {
    color(0x000000).opacity(0.)
}

/// Ant Design default outline control used as overlay triggers.
fn antd_outline_trigger(
    id: impl Into<ElementId>,
    label: SharedString,
    theme: AntTheme,
    chrome: OverlayChrome,
    disabled: bool,
    open: bool,
    caret: bool,
) -> BaseButton {
    let accent = theme.primary;
    let idle_border = chrome.trigger_border;
    let idle_text = chrome.trigger_text;
    let border = if open && !disabled {
        accent
    } else {
        idle_border
    };
    let text = if open && !disabled { accent } else { idle_text };
    BaseButton::new(id)
        .disabled(disabled)
        .h(px(chrome.height))
        .px(px(15.))
        .flex()
        .items_center()
        .justify_center()
        .gap(px(8.))
        .rounded(px(chrome.radius))
        .border_1()
        .border_color(border)
        .bg(chrome.trigger_bg)
        .text_color(text)
        .text_size(px(theme.font_size))
        .when(!disabled, |this| {
            this.cursor_pointer()
                .hover(|this| this.border_color(accent).text_color(accent))
        })
        .child(div().min_w_0().child(label))
        .when(caret, |this| {
            this.child(
                AntIcon::new(AntIconName::DownOutlined)
                    .theme(theme)
                    .size(12.)
                    .color(text),
            )
        })
}

/// Menu item with a stable key. Separators are non-interactive.
#[derive(Clone)]
pub struct AntDropdownItem {
    key: SharedString,
    label: SharedString,
    disabled: bool,
    checked: bool,
    danger: bool,
    separator: bool,
}
impl AntDropdownItem {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            disabled: false,
            checked: false,
            danger: false,
            separator: false,
        }
    }
    pub fn separator() -> Self {
        Self {
            key: "".into(),
            label: "".into(),
            disabled: true,
            checked: false,
            danger: false,
            separator: true,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn danger(mut self, danger: bool) -> Self {
        self.danger = danger;
        self
    }
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }
}

/// Dropdown menu on Base Popover. Kit base owns Escape, outside dismiss and
/// deferred placement; the app owns selection keys and open state.
///
/// Trigger and menu panel use Ant tokens. Arrow keys move focus and skip
/// disabled rows; Enter activates the focused item.
#[derive(IntoElement)]
pub struct AntDropdown {
    id: ElementId,
    label: SharedString,
    entries: Vec<AntDropdownItem>,
    theme: AntTheme,
    disabled: bool,
    min_width: f32,
    open: Option<bool>,
    default_open: bool,
    placement: OverlayPlacement,
    on_select: Option<MenuSelect>,
    on_open_change: Option<OpenChange>,
}
impl AntDropdown {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            entries: Vec::new(),
            theme: AntTheme::default(),
            disabled: false,
            min_width: 160.,
            open: None,
            default_open: false,
            placement: OverlayPlacement::default(),
            on_select: None,
            on_open_change: None,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = width.max(96.);
        self
    }
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
    pub fn placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }
    /// Controlled open. Pair with `on_open_change`.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }
    pub fn item(mut self, entry: AntDropdownItem) -> Self {
        self.entries.push(entry);
        self
    }
    pub fn items(mut self, entries: impl IntoIterator<Item = AntDropdownItem>) -> Self {
        self.entries.extend(entries);
        self
    }
    pub fn on_select(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
    pub fn can_select(&self, key: &str) -> bool {
        self.entries
            .iter()
            .any(|e| !e.separator && !e.disabled && e.key.as_ref() == key)
    }
}
impl RenderOnce for AntDropdown {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let chrome = overlay_chrome(theme, AntComponent::Dropdown, self.disabled);
        let trigger_id = SharedString::from(format!("ant-dropdown-trigger:{:?}", self.id));
        if self.disabled {
            return antd_outline_trigger(trigger_id, self.label, theme, chrome, true, false, true)
                .into_any_element();
        }
        let focus_prefix = format!("ant-dropdown-focus:{:?}", self.id);
        let entries = self.entries;
        let on_select = self.on_select;
        let min_width = self.min_width;
        let label = self.label;
        BasePopover::new(self.id)
            .anchor(overlay_anchor(self.placement))
            .offset(px(4.))
            .default_open(self.default_open)
            .overlay_closable(true)
            .when_some(self.open, |this, open| this.open(open))
            .when_some(self.on_open_change, |this, callback| {
                this.on_open_change(move |open, window, cx| callback(*open, window, cx))
            })
            .trigger_with(move |is_open, _, _| {
                antd_outline_trigger(
                    trigger_id.clone(),
                    label.clone(),
                    theme,
                    chrome,
                    false,
                    is_open,
                    true,
                )
                .into_any_element()
            })
            .content(move |state, window, cx| {
                let dismiss = cx.weak_entity();
                let focus = state.focus_handle(cx);
                // Retain focus by the item's key, including across parent redraws.
                let handles: Vec<FocusHandle> = entries
                    .iter()
                    .map(|item| {
                        window
                            .use_keyed_state(format!("{focus_prefix}:{}", item.key), cx, |_, cx| {
                                cx.focus_handle()
                            })
                            .read(cx)
                            .clone()
                    })
                    .collect();
                let enabled: Vec<bool> = entries
                    .iter()
                    .map(|item| !item.separator && !item.disabled)
                    .collect();
                let first = enabled.iter().position(|enabled| *enabled);
                let row_handles = handles.clone();
                let confirm_handles = handles.clone();
                let confirm_keys: Vec<Option<SharedString>> = entries
                    .iter()
                    .map(|item| (!item.separator && !item.disabled).then(|| item.key.clone()))
                    .collect();
                let confirm_select = on_select.clone();
                let confirm_dismiss = dismiss.clone();
                div()
                    .id("ant-dropdown-menu")
                    .test_support()
                    .flex()
                    .flex_col()
                    .role(Role::Menu)
                    .track_focus(&focus)
                    // Popover binds Enter/Space to Confirm. Consume that action
                    // here so it activates a menu item before the host can toggle.
                    .on_action(move |_: &Confirm, window, cx| {
                        let focused = confirm_handles
                            .iter()
                            .position(|handle| handle.is_focused(window));
                        if let Some(key) = focused.or(first).and_then(|ix| confirm_keys[ix].clone())
                        {
                            if let Some(callback) = &confirm_select {
                                callback(key, window, cx);
                            }
                            let _ =
                                confirm_dismiss.update(cx, |state, cx| state.dismiss(window, cx));
                        }
                        cx.stop_propagation();
                    })
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.modifiers.modified()
                            || event.keystroke.is_ime_in_progress()
                        {
                            return;
                        }
                        let Some(command) = move_for_key(&event.keystroke.key, false) else {
                            return;
                        };
                        let origin = handles.iter().position(|handle| handle.is_focused(window));
                        let next = if let Some(origin) = origin {
                            next_menu_focus(&enabled, origin, command)
                        } else {
                            let edge = match command {
                                MenuMove::Previous | MenuMove::Last => MenuMove::Last,
                                _ => MenuMove::First,
                            };
                            next_menu_focus(&enabled, 0, edge)
                        };
                        if let Some(next) = next {
                            window.focus(&handles[next], cx);
                            window.prevent_default();
                            cx.stop_propagation();
                        }
                    })
                    .min_w(px(min_width))
                    .py(px(4.))
                    .bg(chrome.panel_bg)
                    .text_color(chrome.panel_text)
                    .border_1()
                    .border_color(chrome.panel_border)
                    .rounded(px(chrome.panel_radius))
                    .shadow_lg()
                    .children(entries.into_iter().enumerate().map(|(index, item)| {
                        if item.separator {
                            return div()
                                .mx(px(8.))
                                .my(px(4.))
                                .h(px(1.))
                                .bg(chrome.panel_border)
                                .into_any_element();
                        }
                        let key = item.key.clone();
                        let on_select = on_select.clone();
                        let dismiss = dismiss.clone();
                        let row_disabled = item.disabled;
                        let fg = if row_disabled {
                            theme.disabled_text
                        } else if item.danger {
                            theme.error
                        } else {
                            chrome.panel_text
                        };
                        BaseButton::new(format!("ant-dropdown-item:{}", item.key))
                            .role(Role::MenuItem)
                            .accessibility_label(item.label.clone())
                            .track_focus(&row_handles[index])
                            .tab_stop(first == Some(index))
                            .disabled(row_disabled)
                            .w_full()
                            .h(px(32.))
                            .px(px(12.))
                            .flex()
                            .items_center()
                            .gap(px(8.))
                            .rounded(px(theme.border_radius_sm))
                            .bg(transparent())
                            .text_color(fg)
                            .text_size(px(theme.font_size))
                            .focus_visible(|this| this.bg(theme.surface_secondary))
                            .when(!row_disabled, |this| {
                                this.cursor_pointer()
                                    .hover(|this| this.bg(theme.surface_secondary))
                            })
                            .when(item.checked, |this| {
                                this.child(
                                    AntIcon::new(AntIconName::CheckOutlined)
                                        .theme(theme)
                                        .size(12.)
                                        .color(fg),
                                )
                            })
                            .child(div().min_w_0().flex_1().child(item.label))
                            .when(!row_disabled, |this| {
                                this.on_click(move |_, window, cx| {
                                    if let Some(callback) = &on_select {
                                        callback(key.clone(), window, cx);
                                    }
                                    let _ = dismiss.update(cx, |state, cx| {
                                        state.dismiss(window, cx);
                                    });
                                })
                            })
                            .into_any_element()
                    }))
            })
            .into_any_element()
    }
}

/// Controlled or uncontrolled native Popover with themed text and content.
///
/// Base Popover owns placement, outside dismiss and focus lifecycle. The
/// trigger is an Ant outline control (not Kit Button). Base has no built-in
/// arrow; the `arrow` builder is retained for API stability and ignored.
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
    placement: OverlayPlacement,
    on_open_change: Option<OpenChange>,
}
impl AntPopover {
    pub fn new(id: impl Into<ElementId>, trigger_label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            trigger_label: trigger_label.into(),
            title: "".into(),
            children: Vec::new(),
            theme: AntTheme::default(),
            width: 260.,
            arrow: false,
            disabled: false,
            default_open: false,
            open: None,
            placement: OverlayPlacement::default(),
            on_open_change: None,
        }
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(120.);
        self
    }
    /// Retained for API stability. Base Popover has no arrow primitive.
    pub fn arrow(mut self, arrow: bool) -> Self {
        self.arrow = arrow;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
    /// Must be accompanied by `on_open_change` for a controlled popup.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }
    pub fn placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.children.push(element.into_any_element());
        self
    }
}
impl RenderOnce for AntPopover {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let chrome = overlay_chrome(theme, AntComponent::Popover, self.disabled);
        let trigger_id = SharedString::from(format!("ant-popover-trigger:{:?}", self.id));
        let _arrow = self.arrow; // base Popover has no arrow; field kept for API
        if self.disabled {
            return antd_outline_trigger(
                trigger_id,
                self.trigger_label,
                theme,
                chrome,
                true,
                false,
                false,
            )
            .into_any_element();
        }
        let title = self.title;
        let children = self.children;
        let width = self.width;
        let trigger_label = self.trigger_label;
        BasePopover::new(self.id)
            .anchor(overlay_anchor(self.placement))
            .offset(px(4.))
            .default_open(self.default_open)
            .overlay_closable(true)
            .when_some(self.open, |this, open| this.open(open))
            .when_some(self.on_open_change, |this, callback| {
                this.on_open_change(move |open, window, cx| callback(*open, window, cx))
            })
            .trigger_with(move |is_open, _, _| {
                antd_outline_trigger(
                    trigger_id.clone(),
                    trigger_label.clone(),
                    theme,
                    chrome,
                    false,
                    is_open,
                    false,
                )
                .into_any_element()
            })
            .content(move |_, _, _| {
                div()
                    .w(px(width))
                    .p(px(12.))
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .bg(chrome.panel_bg)
                    .text_color(chrome.panel_text)
                    .border_1()
                    .border_color(chrome.panel_border)
                    .rounded(px(chrome.panel_radius))
                    .shadow_lg()
                    .when(!title.is_empty(), |this| {
                        this.child(
                            div()
                                .font_semibold()
                                .text_color(chrome.panel_text)
                                .child(title),
                        )
                    })
                    .children(children)
            })
            .into_any_element()
    }
}

/// Modal dialog. Kit Dialog owns Escape, focus trap, stacked layers and mask.
///
/// `on_ok` and `on_cancel` return true to dismiss, false to stay open.
/// Body is plain text in this alpha API.
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
    pub fn new(
        id: impl Into<ElementId>,
        trigger_label: impl Into<SharedString>,
        title: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            trigger_label: trigger_label.into(),
            title: title.into(),
            description: "".into(),
            body: "".into(),
            theme: AntTheme::default(),
            width: 520.,
            keyboard: true,
            mask_closable: true,
            on_ok: None,
            on_cancel: None,
        }
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = description.into();
        self
    }
    pub fn body(mut self, body: impl Into<SharedString>) -> Self {
        self.body = body.into();
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(260.);
        self
    }
    pub fn keyboard(mut self, keyboard: bool) -> Self {
        self.keyboard = keyboard;
        self
    }
    pub fn mask_closable(mut self, closable: bool) -> Self {
        self.mask_closable = closable;
        self
    }
    pub fn on_ok(mut self, handler: impl Fn(&mut Window, &mut App) -> bool + 'static) -> Self {
        self.on_ok = Some(Rc::new(handler));
        self
    }
    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) -> bool + 'static) -> Self {
        self.on_cancel = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for AntModal {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let chrome = overlay_chrome(theme, AntComponent::Modal, false);
        let title = self.title;
        let description = self.description;
        let body = self.body;
        let ok = self.on_ok;
        let cancel = self.on_cancel;
        KitDialog::new(cx)
            .trigger(AntButton::new(self.id, self.trigger_label).theme(theme))
            .width(px(self.width))
            .keyboard(self.keyboard)
            .overlay_closable(self.mask_closable)
            .close_button(true)
            .content(move |content, _, _| {
                let ok = ok.clone();
                let cancel = cancel.clone();
                content
                    .bg(chrome.panel_bg)
                    .text_color(chrome.panel_text)
                    .rounded(px(chrome.panel_radius))
                    .child(
                        DialogHeader::new()
                            .bg(chrome.header_bg)
                            .text_color(chrome.panel_text)
                            .px(px(24.))
                            .pt(px(20.))
                            .pb(px(8.))
                            .child(
                                DialogTitle::new()
                                    .text_color(chrome.panel_text)
                                    .child(title.clone()),
                            )
                            .when(!description.is_empty(), |this| {
                                this.child(
                                    DialogDescription::new()
                                        .text_color(theme.text_secondary)
                                        .child(description.clone()),
                                )
                            }),
                    )
                    .child(
                        div()
                            .px(px(24.))
                            .py(px(12.))
                            .text_color(chrome.panel_text)
                            .child(body.clone()),
                    )
                    .child(
                        DialogFooter::new()
                            .px(px(24.))
                            .pb(px(20.))
                            .pt(px(12.))
                            .bg(chrome.panel_bg)
                            .child(
                                AntButton::new("ant-modal-cancel", "Cancel")
                                    .theme(theme)
                                    .on_click(move |_, window, cx| {
                                        if cancel.as_ref().is_none_or(|cb| cb(window, cx)) {
                                            window.close_dialog(cx);
                                        }
                                    }),
                            )
                            .child(
                                AntButton::new("ant-modal-ok", "OK")
                                    .theme(theme)
                                    .primary()
                                    .on_click(move |_, window, cx| {
                                        if ok.as_ref().is_none_or(|cb| cb(window, cx)) {
                                            window.close_dialog(cx);
                                        }
                                    }),
                            ),
                    )
            })
    }
}

/// Anchored Popconfirm on Base Popover with local Confirm/Cancel actions.
#[derive(IntoElement)]
pub struct AntPopconfirm {
    id: ElementId,
    trigger_label: SharedString,
    title: SharedString,
    description: SharedString,
    theme: AntTheme,
    width: f32,
    disabled: bool,
    placement: OverlayPlacement,
    on_confirm: Option<Action>,
    on_cancel: Option<Action>,
}
impl AntPopconfirm {
    pub fn new(
        id: impl Into<ElementId>,
        trigger_label: impl Into<SharedString>,
        title: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            trigger_label: trigger_label.into(),
            title: title.into(),
            description: "".into(),
            theme: AntTheme::default(),
            width: 280.,
            disabled: false,
            placement: OverlayPlacement::default(),
            on_confirm: None,
            on_cancel: None,
        }
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = description.into();
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(160.);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }
    /// Return true to close; false to keep the confirmation popover open.
    pub fn on_confirm(mut self, handler: impl Fn(&mut Window, &mut App) -> bool + 'static) -> Self {
        self.on_confirm = Some(Rc::new(handler));
        self
    }
    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) -> bool + 'static) -> Self {
        self.on_cancel = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for AntPopconfirm {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let chrome = overlay_chrome(theme, AntComponent::Popconfirm, self.disabled);
        let trigger_id = SharedString::from(format!("ant-popconfirm-trigger:{:?}", self.id));
        if self.disabled {
            return antd_outline_trigger(
                trigger_id,
                self.trigger_label,
                theme,
                chrome,
                true,
                false,
                false,
            )
            .into_any_element();
        }
        let title = self.title;
        let desc = self.description;
        let width = self.width;
        let confirm = self.on_confirm;
        let cancel = self.on_cancel;
        let trigger_label = self.trigger_label;
        BasePopover::new(self.id)
            .anchor(overlay_anchor(self.placement))
            .offset(px(4.))
            .overlay_closable(true)
            .trigger_with(move |is_open, _, _| {
                antd_outline_trigger(
                    trigger_id.clone(),
                    trigger_label.clone(),
                    theme,
                    chrome,
                    false,
                    is_open,
                    false,
                )
                .into_any_element()
            })
            .content(move |_, _, cx| {
                let close_cancel = cx.weak_entity();
                let close_confirm = cx.weak_entity();
                let cancel = cancel.clone();
                let confirm = confirm.clone();
                div()
                    .w(px(width))
                    .p(px(12.))
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .bg(chrome.panel_bg)
                    .text_color(chrome.panel_text)
                    .border_1()
                    .border_color(chrome.panel_border)
                    .rounded(px(chrome.panel_radius))
                    .shadow_lg()
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .gap(px(8.))
                            .child(
                                AntIcon::new(AntIconName::ExclamationCircleFilled)
                                    .theme(theme)
                                    .size(18.)
                                    .color(theme.warning),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(7.))
                                    .child(
                                        div()
                                            .font_semibold()
                                            .text_color(chrome.panel_text)
                                            .child(title.clone()),
                                    )
                                    .when(!desc.is_empty(), |this| {
                                        this.child(
                                            div()
                                                .text_size(px(13.))
                                                .text_color(theme.text_secondary)
                                                .child(desc.clone()),
                                        )
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .gap(px(8.))
                            .child(
                                AntButton::new("ant-popconfirm-no", "Cancel")
                                    .theme(theme)
                                    .on_click(move |_, window, cx| {
                                        if cancel.as_ref().is_none_or(|cb| cb(window, cx)) {
                                            let _ = close_cancel.update(cx, |state, cx| {
                                                state.dismiss(window, cx);
                                            });
                                        }
                                    }),
                            )
                            .child(
                                AntButton::new("ant-popconfirm-yes", "Confirm")
                                    .theme(theme)
                                    .primary()
                                    .size(crate::ControlSize::Small)
                                    .on_click(move |_, window, cx| {
                                        if confirm.as_ref().is_none_or(|cb| cb(window, cx)) {
                                            let _ = close_confirm.update(cx, |state, cx| {
                                                state.dismiss(window, cx);
                                            });
                                        }
                                    }),
                            ),
                    )
            })
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design_tokens::AntComponentToken;

    #[test]
    fn dropdown_rejects_disabled_and_separator_selections() {
        let menu = AntDropdown::new("menu", "More")
            .item(AntDropdownItem::new("open", "Open"))
            .item(AntDropdownItem::new("delete", "Delete").disabled(true))
            .item(AntDropdownItem::separator())
            .item(AntDropdownItem::new("marked", "Marked").checked(true))
            .item(AntDropdownItem::new("wipe", "Wipe").danger(true));
        assert!(menu.can_select("open"));
        assert!(menu.can_select("marked"));
        assert!(menu.can_select("wipe"));
        assert!(!menu.can_select("delete"));
        assert!(!menu.can_select(""));
        assert!(!menu.can_select("missing"));
        assert!(
            menu.entries
                .iter()
                .any(|e| e.danger && e.key.as_ref() == "wipe")
        );
    }

    #[test]
    fn overlay_placement_uses_opposite_popup_corner() {
        // Top* anchors sit below the trigger; Bottom* anchors sit above it.
        assert_eq!(
            overlay_anchor(OverlayPlacement::BottomLeft),
            Anchor::TopLeft
        );
        assert_eq!(
            overlay_anchor(OverlayPlacement::TopLeft),
            Anchor::BottomLeft
        );
        assert_eq!(
            overlay_anchor(OverlayPlacement::BottomRight),
            Anchor::TopRight
        );
        assert_eq!(
            overlay_anchor(OverlayPlacement::TopRight),
            Anchor::BottomRight
        );
    }

    #[test]
    fn overlay_size_bounds_and_controlled_open() {
        let popover = AntPopover::new("popover", "Show").width(-1.).open(true);
        assert_eq!(popover.width, 120.);
        assert_eq!(popover.open, Some(true));
        let modal = AntModal::new("modal", "Launch", "Title").width(100.);
        assert_eq!(modal.width, 260.);
        let confirmation = AntPopconfirm::new("confirm", "Delete", "Really delete?")
            .width(20.)
            .disabled(true);
        assert_eq!(confirmation.width, 160.);
        assert!(confirmation.disabled);
        let menu = AntDropdown::new("d", "More").open(true).default_open(false);
        assert_eq!(menu.open, Some(true));
        assert!(!menu.default_open);
    }

    #[test]
    fn overlay_chrome_defaults_match_ant_tokens() {
        let light = AntTheme::light();
        let chrome = overlay_chrome(light, AntComponent::Dropdown, false);
        assert_eq!(chrome.height, 32.);
        assert_eq!(chrome.radius, light.border_radius);
        assert_eq!(chrome.panel_radius, light.border_radius_lg);
        assert_eq!(chrome.trigger_bg, light.surface);
        assert_eq!(chrome.trigger_text, light.text);
        assert_eq!(chrome.trigger_border, light.border);
        assert_eq!(chrome.panel_bg, light.bg_elevated);

        let disabled = overlay_chrome(light, AntComponent::Popover, true);
        assert_eq!(disabled.trigger_bg, light.disabled_bg);
        assert_eq!(disabled.trigger_text, light.disabled_text);
    }

    #[test]
    fn overlay_chrome_honors_component_overrides() {
        let theme = AntTheme::light().with_component(
            AntComponent::Modal,
            AntComponentToken::default()
                .control_height(40.)
                .border_radius(10.)
                .color_header_bg(AntTheme::light().surface_secondary),
        );
        let chrome = overlay_chrome(theme, AntComponent::Modal, false);
        assert_eq!(chrome.height, 40.);
        assert_eq!(chrome.radius, 10.);
        assert_eq!(chrome.panel_radius, 10.);
        assert_eq!(chrome.header_bg, AntTheme::light().surface_secondary);
    }
}
