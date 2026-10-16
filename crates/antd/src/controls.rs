//! Behavior-backed Ant Design-inspired controls.

use std::rc::Rc;
use gpui_kit::{
    App, ClickEvent, ElementId, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window,
    div, px,
};
use gpui_kit::base::{Button as BaseButton, Checkbox as BaseCheckbox, CheckboxState, Switch as BaseSwitch};
use gpui_kit::prelude::FluentBuilder as _;
use crate::tokens::{AntTheme, color};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ControlSize {
    Small,
    #[default]
    Middle,
    Large,
}
impl ControlSize {
    pub fn height(self) -> f32 { match self { Self::Small => 24., Self::Middle => 32., Self::Large => 40. } }
    pub fn horizontal_padding(self) -> f32 { match self { Self::Small => 8., Self::Middle => 15., Self::Large => 18. } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Dashed,
    Text,
    Link,
}

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Ant Design button. GPUI Base handles Tab/Enter/Space, focus and a11y.
#[derive(IntoElement)]
pub struct AntButton {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    size: ControlSize,
    danger: bool,
    ghost: bool,
    disabled: bool,
    loading: bool,
    block: bool,
    theme: AntTheme,
    on_click: Option<ClickHandler>,
}
impl AntButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(), label: label.into(),
            variant: ButtonVariant::Default, size: ControlSize::Middle,
            danger: false, ghost: false, disabled: false, loading: false, block: false,
            theme: AntTheme::default(), on_click: None,
        }
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self { self.variant = variant; self }
    pub fn primary(self) -> Self { self.variant(ButtonVariant::Primary) }
    pub fn dashed(self) -> Self { self.variant(ButtonVariant::Dashed) }
    pub fn text(self) -> Self { self.variant(ButtonVariant::Text) }
    pub fn link(self) -> Self { self.variant(ButtonVariant::Link) }
    pub fn size(mut self, size: ControlSize) -> Self { self.size = size; self }
    pub fn danger(mut self, danger: bool) -> Self { self.danger = danger; self }
    pub fn ghost(mut self, ghost: bool) -> Self { self.ghost = ghost; self }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
    pub fn loading(mut self, loading: bool) -> Self { self.loading = loading; self }
    pub fn block(mut self, block: bool) -> Self { self.block = block; self }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler)); self
    }
}

impl RenderOnce for AntButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let accent = if self.danger { t.error } else { t.primary };
        let disabled = self.disabled || self.loading;
        let is_primary = self.variant == ButtonVariant::Primary;
        let borderless = matches!(self.variant, ButtonVariant::Text | ButtonVariant::Link);
        let bg = if disabled {
            t.disabled_bg
        } else if is_primary && !self.ghost {
            accent
        } else {
            t.surface
        };
        let fg = if disabled {
            t.disabled_text
        } else if is_primary && !self.ghost {
            color(0xffffff)
        } else if self.danger || matches!(self.variant, ButtonVariant::Link) || self.ghost {
            accent
        } else {
            t.text
        };
        let border = if borderless { bg } else if is_primary && self.ghost { accent } else { t.border };
        let hover_bg = if is_primary && !self.ghost { if self.danger { t.error } else { t.primary_hover } }
            else if self.variant == ButtonVariant::Text { t.surface_secondary } else { bg };
        let hover_fg = if is_primary && !self.ghost { color(0xffffff) } else { accent };
        let hover_border = if borderless { hover_bg } else { accent };
        let text = if self.loading { format!("◌ {}", self.label) } else { self.label.to_string() };
        BaseButton::new(self.id)
            .h(px(self.size.height()))
            .px(px(self.size.horizontal_padding()))
            .rounded(px(t.border_radius))
            .border_1()
            .border_color(border)
            .bg(bg)
            .text_color(fg)
            .text_size(px(if self.size == ControlSize::Large { 16. } else { 14. }))
            .font_medium()
            .when(self.block, |this| this.w_full())
            .when(!disabled, |this| {
                this.cursor_pointer()
                    .hover(|this| this.bg(hover_bg).text_color(hover_fg).border_color(hover_border))
                    .active(|this| this.bg(t.primary_active))
            })
            .disabled(disabled)
            .child(text)
            .when_some(self.on_click, |this, handler| this.on_click(move |event, window, cx| handler(event, window, cx)))
    }
}

/// Checked values are controlled: the owner updates state after on_change.
#[derive(IntoElement)]
pub struct AntCheckbox {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    theme: AntTheme,
    on_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
}
impl AntCheckbox {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self { id: id.into(), label: label.into(), checked: false, disabled: false, theme: AntTheme::default(), on_change: None }
    }
    pub fn checked(mut self, checked: bool) -> Self { self.checked=checked; self }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled=disabled; self }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme=theme; self }
    pub fn on_change(mut self, f: impl Fn(bool, &mut Window, &mut App)+'static) -> Self {
        self.on_change=Some(Rc::new(f)); self
    }
}
impl RenderOnce for AntCheckbox {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t=self.theme;
        let checked=self.checked;
        BaseCheckbox::new(self.id)
            .checked(checked)
            .disabled(self.disabled)
            .flex()
            .items_center()
            .gap(px(8.))
            .text_size(px(14.))
            .text_color(if self.disabled {t.disabled_text} else {t.text})
            .when(!self.disabled, |this| this.cursor_pointer())
            .child(
                div().size(px(16.)).rounded(px(4.)).border_1()
                    .border_color(if checked {t.primary} else {t.border})
                    .bg(if checked {t.primary} else {t.surface})
                    .flex().items_center().justify_center()
                    .child(if checked { "✓" } else { "" })
                    .text_color(color(0xffffff))
                    .text_size(px(12.))
            )
            .child(self.label)
            .when_some(self.on_change, |this, handler| {
                this.on_change(move |next, _, window, cx| handler(next == CheckboxState::Checked, window, cx))
            })
    }
}

/// Controlled switch with GPUI Base's keyboard and accessibility semantics.
#[derive(IntoElement)]
pub struct AntSwitch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    theme: AntTheme,
    on_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
}
impl AntSwitch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id:id.into(), checked:false, disabled:false, theme:AntTheme::default(), on_change:None }
    }
    pub fn checked(mut self, v: bool) -> Self {self.checked=v;self}
    pub fn disabled(mut self, v: bool) -> Self {self.disabled=v;self}
    pub fn theme(mut self, t: AntTheme) -> Self {self.theme=t;self}
    pub fn on_change(mut self, f: impl Fn(bool,&mut Window,&mut App)+'static) -> Self {
        self.on_change=Some(Rc::new(f));self
    }
}
impl RenderOnce for AntSwitch {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        BaseSwitch::new(self.id)
            .checked(self.checked)
            .disabled(self.disabled)
            .relative()
            .w(px(44.))
            .h(px(22.))
            .rounded_full()
            .bg(if self.disabled {t.disabled_bg} else if self.checked {t.primary} else {color(0xbfbfbf)})
            .when(!self.disabled, |this| this.cursor_pointer())
            .child(
                div().absolute().top(px(2.))
                    .left(px(if self.checked {24.} else {2.}))
                    .size(px(18.)).rounded_full()
                    .bg(color(0xffffff))
            )
            .when_some(self.on_change, |this, handler| {
                this.on_change(move |next, _, window, cx| handler(next, window, cx))
            })
    }
}
