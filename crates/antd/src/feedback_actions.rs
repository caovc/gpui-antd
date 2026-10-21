//! Real GPUI Kit Root-hosted Ant Design Drawer, Message and Notification.
//!
//! Drawer owns a Kit Sheet: focus trap, Escape, backdrop and restoration are
//! handled by the registered Root instead of an absolute-positioned fake.
//! Message/Notification share Kit's in-app toast manager (stacking, dismiss,
//! hover pause). Their APIs are imperative like Ant Design's feedback APIs.
use crate::{AntButton, AntTheme};
use gpui_kit::component::{
    Placement, WindowExt as _,
    notification::{Notification as KitNotification, NotificationType},
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    Anchor, AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, px,
};
use std::rc::Rc;

/// The four supported sides of an Ant Design Drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerPlacement {
    Left,
    #[default]
    Right,
    Top,
    Bottom,
}
impl DrawerPlacement {
    fn kit(self) -> Placement {
        match self {
            Self::Left => Placement::Left,
            Self::Right => Placement::Right,
            Self::Top => Placement::Top,
            Self::Bottom => Placement::Bottom,
        }
    }
}

/// A native Sheet-triggering Drawer. The content is rebuilt for each render
/// via a closure, not retained as a once-rendered GPUI element.
///
/// The Kit Root must wrap the view (as in gpui_kit::open_window).
/// The Sheet owns Escape, mask click, close button and focus restoration.
/// Does not yet support Ant's nested push/afterOpenChange.
#[derive(IntoElement)]
pub struct AntDrawer {
    id: ElementId,
    trigger: SharedString,
    title: SharedString,
    body: SharedString,
    placement: DrawerPlacement,
    size: f32,
    mask: bool,
    mask_closable: bool,
    resizable: bool,
    disabled: bool,
    theme: AntTheme,
    content: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>>,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}
impl AntDrawer {
    pub fn new(id: impl Into<ElementId>, trigger: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            trigger: trigger.into(),
            title: "Drawer".into(),
            body: "".into(),
            placement: DrawerPlacement::Right,
            size: 378.,
            mask: true,
            mask_closable: true,
            resizable: false,
            disabled: false,
            theme: AntTheme::default(),
            content: None,
            on_close: None,
        }
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }
    pub fn body(mut self, body: impl Into<SharedString>) -> Self {
        self.body = body.into();
        self
    }
    pub fn placement(mut self, value: DrawerPlacement) -> Self {
        self.placement = value;
        self
    }
    pub fn size(mut self, value: f32) -> Self {
        self.size = if value.is_finite() {
            value.max(160.)
        } else {
            378.
        };
        self
    }
    pub fn mask(mut self, value: bool) -> Self {
        self.mask = value;
        self
    }
    pub fn mask_closable(mut self, value: bool) -> Self {
        self.mask_closable = value;
        self
    }
    pub fn resizable(mut self, value: bool) -> Self {
        self.resizable = value;
        self
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    /// Build arbitrary native GPUI content whenever Sheet re-renders.
    pub fn content(
        mut self,
        build: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.content = Some(Rc::new(build));
        self
    }
    pub fn on_close(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntDrawer {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let placement = self.placement.kit();
        let title = self.title;
        let body = self.body;
        let theme = self.theme;
        let size = self.size;
        let mask = self.mask;
        let mask_closable = self.mask_closable;
        let resizable = self.resizable;
        let content = self.content;
        let on_close = self.on_close;
        AntButton::new(self.id, self.trigger)
            .theme(theme)
            .disabled(self.disabled)
            .on_click(move |_, window, cx| {
                let title = title.clone();
                let body = body.clone();
                let content = content.clone();
                let on_close = on_close.clone();
                window.open_sheet_at(placement, cx, move |sheet, window, cx| {
                    let builder = content.clone();
                    let on_close = on_close.clone();
                    sheet
                        .title(title.clone())
                        .size(px(size))
                        .overlay(mask)
                        .overlay_closable(mask_closable)
                        .resizable(resizable)
                        .bg(theme.surface)
                        .text_color(theme.text)
                        .border_color(theme.border)
                        .on_close(move |_, window, cx| {
                            if let Some(callback) = &on_close {
                                callback(window, cx);
                            }
                        })
                        .child(
                            gpui_kit::div()
                                .flex()
                                .flex_col()
                                .gap(px(14.))
                                .child(body.clone())
                                .when_some(builder, |this, builder| {
                                    this.child((builder)(window, cx))
                                }),
                        )
                });
            })
    }
}

/// Feedback tint variants shared between in-app Message and Notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoticeTone {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}
impl NoticeTone {
    fn kit(self) -> NotificationType {
        match self {
            Self::Info => NotificationType::Info,
            Self::Success => NotificationType::Success,
            Self::Warning => NotificationType::Warning,
            Self::Error => NotificationType::Error,
        }
    }
}
struct MessageKey;
struct NotificationKey;

/// Imperative compact top-center message, using the GPUI Kit toast queue.
/// Auto-hide is Kit's standard duration; Ant Design's per-message duration,
/// promise/thenable handles, and loading variant are still future work.
pub struct AntMessage {
    text: SharedString,
    tone: NoticeTone,
    autohide: bool,
    key: Option<ElementId>,
    placement: Anchor,
}
impl AntMessage {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            tone: NoticeTone::Info,
            autohide: true,
            key: None,
            placement: Anchor::TopCenter,
        }
    }
    pub fn success(text: impl Into<SharedString>) -> Self {
        Self::new(text).tone(NoticeTone::Success)
    }
    pub fn error(text: impl Into<SharedString>) -> Self {
        Self::new(text).tone(NoticeTone::Error)
    }
    pub fn warning(text: impl Into<SharedString>) -> Self {
        Self::new(text).tone(NoticeTone::Warning)
    }
    pub fn tone(mut self, tone: NoticeTone) -> Self {
        self.tone = tone;
        self
    }
    pub fn persistent(mut self, persistent: bool) -> Self {
        self.autohide = !persistent;
        self
    }
    pub fn key(mut self, key: impl Into<ElementId>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn placement(mut self, placement: Anchor) -> Self {
        self.placement = placement;
        self
    }
    pub fn show(self, window: &mut Window, cx: &mut App) {
        let note = KitNotification::new()
            .message(self.text)
            .with_type(self.tone.kit())
            .placement(self.placement)
            .autohide(self.autohide);
        let note = match self.key {
            Some(key) => note.id1::<MessageKey>(key),
            None => note,
        };
        window.push_notification(note, cx);
    }
}

/// Rich top-right toast with heading, body, severity, persistent and keyed
/// replace behavior. It inherits Kit's stack, dismiss, and accessible UI.
pub struct AntNotification {
    title: SharedString,
    description: SharedString,
    tone: NoticeTone,
    autohide: bool,
    key: Option<ElementId>,
    placement: Anchor,
}
impl AntNotification {
    pub fn new(title: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            tone: NoticeTone::Info,
            autohide: true,
            key: None,
            placement: Anchor::TopRight,
        }
    }
    pub fn tone(mut self, tone: NoticeTone) -> Self {
        self.tone = tone;
        self
    }
    pub fn persistent(mut self, yes: bool) -> Self {
        self.autohide = !yes;
        self
    }
    pub fn key(mut self, key: impl Into<ElementId>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn placement(mut self, placement: Anchor) -> Self {
        self.placement = placement;
        self
    }
    pub fn show(self, window: &mut Window, cx: &mut App) {
        let note = KitNotification::new()
            .title(self.title)
            .message(self.description)
            .with_type(self.tone.kit())
            .placement(self.placement)
            .autohide(self.autohide);
        let note = match self.key {
            Some(key) => note.id1::<NotificationKey>(key),
            None => note,
        };
        window.push_notification(note, cx);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn drawer_sizes_reject_invalid_dimensions() {
        assert_eq!(AntDrawer::new("drawer", "Open").size(f32::NAN).size, 378.);
        assert_eq!(AntDrawer::new("drawer", "Open").size(-4.).size, 160.);
        assert_eq!(
            AntDrawer::new("drawer", "Open")
                .placement(DrawerPlacement::Bottom)
                .placement,
            DrawerPlacement::Bottom
        );
    }
    #[test]
    fn feedback_defaults_match_intent() {
        let m = AntMessage::success("saved").key("save");
        assert_eq!(m.tone, NoticeTone::Success);
        assert!(m.autohide);
        assert_eq!(m.placement, Anchor::TopCenter);
        let n = AntNotification::new("Error", "Not connected")
            .persistent(true)
            .tone(NoticeTone::Error);
        assert!(!n.autohide);
        assert_eq!(n.placement, Anchor::TopRight);
    }
}
