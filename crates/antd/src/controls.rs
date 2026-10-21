//! Behavior-backed Ant Design-inspired controls.

use crate::design_tokens::AntComponent;
use crate::tokens::{AntTheme, color};
use gpui_kit::base::StyledExt as _;
use gpui_kit::base::{
    Button as BaseButton, Checkbox as BaseCheckbox, CheckboxState, Switch as BaseSwitch,
};
use gpui_kit::component::{Sizable as _, Size, spinner::Spinner};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ControlSize {
    Small,
    #[default]
    Middle,
    Large,
}
impl ControlSize {
    pub fn height(self) -> f32 {
        match self {
            Self::Small => 24.,
            Self::Middle => 32.,
            Self::Large => 40.,
        }
    }
    pub fn horizontal_padding(self) -> f32 {
        match self {
            Self::Small => 8.,
            Self::Middle => 15.,
            Self::Large => 18.,
        }
    }
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
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Default,
            size: ControlSize::Middle,
            danger: false,
            ghost: false,
            disabled: false,
            loading: false,
            block: false,
            theme: AntTheme::default(),
            on_click: None,
        }
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn primary(self) -> Self {
        self.variant(ButtonVariant::Primary)
    }
    pub fn dashed(self) -> Self {
        self.variant(ButtonVariant::Dashed)
    }
    pub fn text(self) -> Self {
        self.variant(ButtonVariant::Text)
    }
    pub fn link(self) -> Self {
        self.variant(ButtonVariant::Link)
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
    pub fn danger(mut self, danger: bool) -> Self {
        self.danger = danger;
        self
    }
    pub fn ghost(mut self, ghost: bool) -> Self {
        self.ghost = ghost;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    pub fn block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for AntButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let component = t.component(AntComponent::Button);
        let accent = if self.danger {
            t.error
        } else {
            component.color_primary.unwrap_or(t.primary)
        };
        let disabled = self.disabled || self.loading;
        let is_primary = self.variant == ButtonVariant::Primary;
        let borderless = matches!(self.variant, ButtonVariant::Text | ButtonVariant::Link);
        let transparent = color(0x000000).opacity(0.);
        let bg = if disabled {
            t.disabled_bg
        } else if is_primary && !self.ghost {
            accent
        } else if self.ghost || borderless {
            transparent
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
        let border = if borderless {
            transparent
        } else if self.ghost || self.danger {
            accent
        } else {
            t.border
        };
        let hover_bg = if is_primary && !self.ghost {
            if self.danger {
                t.error
            } else {
                t.primary_hover
            }
        } else if self.variant == ButtonVariant::Text {
            t.surface_secondary
        } else {
            bg
        };
        let hover_fg = if is_primary && !self.ghost {
            color(0xffffff)
        } else if self.variant == ButtonVariant::Text {
            t.text
        } else {
            accent
        };
        let hover_border = if borderless { transparent } else { accent };
        let active_bg = if is_primary && !self.ghost {
            t.primary_active
        } else {
            hover_bg
        };

        let height = component.control_height.unwrap_or(match self.size {
            ControlSize::Small => t.control_height_sm,
            ControlSize::Middle => t.control_height,
            ControlSize::Large => t.control_height_lg,
        });
        let padding = component
            .padding
            .unwrap_or(self.size.horizontal_padding() * t.padding / 16.);
        BaseButton::new(self.id)
            .h(px(height))
            .px(px(padding))
            .gap(px(8.))
            .rounded(px(component.border_radius.unwrap_or(t.border_radius)))
            .border_1()
            .when(self.variant == ButtonVariant::Dashed, |this| {
                this.border_dashed()
            })
            .border_color(component.color_border.unwrap_or(border))
            .bg(component.color_bg.unwrap_or(bg))
            .text_color(component.color_text.unwrap_or(fg))
            .text_size(px(component.font_size.unwrap_or(
                if self.size == ControlSize::Large {
                    t.font_size + 2.
                } else {
                    t.font_size
                },
            )))
            .font_medium()
            .when(self.block, |this| this.w_full())
            .when(!disabled, |this| {
                this.cursor_pointer()
                    .hover(|this| {
                        this.bg(component.color_bg_hover.unwrap_or(hover_bg))
                            .text_color(hover_fg)
                            .border_color(hover_border)
                    })
                    .active(|this| this.bg(active_bg))
            })
            .disabled(disabled)
            .when(self.loading, |this| {
                this.child(Spinner::new().with_size(Size::Small).color(fg))
            })
            .child(self.label)
            .when_some(self.on_click, |this, handler| {
                this.on_click(move |event, window, cx| handler(event, window, cx))
            })
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
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            disabled: false,
            theme: AntTheme::default(),
            on_change: None,
        }
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntCheckbox {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let checked = self.checked;
        let comp = t.component(AntComponent::Checkbox);
        let mark_size = comp.control_height.unwrap_or(16.).clamp(12., 32.);
        let accent = comp.color_primary.unwrap_or(t.primary);
        BaseCheckbox::new(self.id)
            .checked(checked)
            .disabled(self.disabled)
            .flex()
            .items_center()
            .gap(px(8.))
            .text_size(px(comp.font_size.unwrap_or(t.font_size)))
            .text_color(if self.disabled {
                t.disabled_text
            } else {
                comp.color_text.unwrap_or(t.text)
            })
            .when(!self.disabled, |this| this.cursor_pointer())
            .child(
                div()
                    .size(px(mark_size))
                    .rounded(px(comp.border_radius.unwrap_or(4.)))
                    .border_1()
                    .border_color(comp.color_border.unwrap_or(if checked {
                        accent
                    } else {
                        t.border
                    }))
                    .bg(if checked {
                        accent
                    } else {
                        comp.color_bg.unwrap_or(t.surface)
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(if checked { "✓" } else { "" })
                    .text_color(color(0xffffff))
                    .text_size(px(12.)),
            )
            .child(self.label)
            .when_some(self.on_change, |this, handler| {
                this.on_change(move |next, _, window, cx| {
                    handler(next == CheckboxState::Checked, window, cx)
                })
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
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            theme: AntTheme::default(),
            on_change: None,
        }
    }
    pub fn checked(mut self, v: bool) -> Self {
        self.checked = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
    pub fn theme(mut self, t: AntTheme) -> Self {
        self.theme = t;
        self
    }
    pub fn on_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntSwitch {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let comp = t.component(AntComponent::Switch);
        let height = comp.control_height.unwrap_or(22.).clamp(14., 40.);
        let accent = comp.color_primary.unwrap_or(t.primary);
        BaseSwitch::new(self.id)
            .checked(self.checked)
            .disabled(self.disabled)
            .relative()
            .w(px(height * 2.))
            .h(px(height))
            .rounded_full()
            .bg(if self.disabled {
                t.disabled_bg
            } else if self.checked {
                accent
            } else {
                comp.color_bg.unwrap_or(color(0xbfbfbf))
            })
            .when(!self.disabled, |this| this.cursor_pointer())
            .child(
                div()
                    .absolute()
                    .top(px(2.))
                    .left(px(if self.checked { height + 2. } else { 2. }))
                    .size(px(height - 4.))
                    .rounded_full()
                    .bg(color(0xffffff)),
            )
            .when_some(self.on_change, |this, handler| {
                this.on_change(move |next, _, window, cx| handler(next, window, cx))
            })
    }
}
