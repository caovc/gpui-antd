//! Ant Design-inspired hover Tooltip using GPUI's native tooltip provider.
//!
//! This wrapper delegates popup lifecycle and the accessible Tooltip role to
//! GPUI Kit. It does NOT mount an absolute-positioned HTML/GPUI fake popup.
//! The first version supports pointer hover and theme-aware text; explicitly
//! verify keyboard focus and placement before claiming full Ant parity.

use gpui_kit::{
    AnyElement, App, ElementId, IntoElement, RenderOnce, SharedString,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};
use gpui_kit::component::tooltip::Tooltip as KitTooltip;
use gpui_kit::prelude::FluentBuilder as _;
use crate::AntTheme;

/// Descriptive tooltip attached to a GPUI element.
///
/// The child element is the trigger; GPUI Kit provides a native tooltip
/// overlay and Tooltip accessibility role. The title contains plain text.
/// Examples:
/// ```ignore
/// AntTooltip::new("save-help", "Save your changes")
///     .child(AntButton::new("save", "Save").primary())
/// ```
///
/// The tooltip is hover-driven in this alpha version; keyboard and all
/// Ant Design `placement` variants are not yet available in this wrapper.
#[derive(IntoElement)]
pub struct AntTooltip {
    id: ElementId,
    title: SharedString,
    theme: AntTheme,
    disabled: bool,
    children: Vec<AnyElement>,
}

impl AntTooltip {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id:id.into(),title:title.into(),theme:AntTheme::default(),
            disabled:false,children:Vec::new(),
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme=theme;self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled=disabled;self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());self
    }
    pub fn children(mut self, children: impl IntoIterator<Item=AnyElement>) -> Self {
        self.children.extend(children);self
    }
}

impl RenderOnce for AntTooltip {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme=self.theme;
        let title=self.title;
        div().id(self.id)
            .when(!self.disabled, |this| {
                this.tooltip(move |window, cx| {
                    KitTooltip::new(title.clone())
                        .bg(theme.text)
                        .text_color(theme.surface)
                        .border_color(theme.text_secondary)
                        .rounded(px(6.))
                        .build(window, cx)
                })
            })
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_builders_preserve_title_and_disabled_state() {
        let tooltip=AntTooltip::new("help", "Explain this action")
            .disabled(true)
            .theme(AntTheme::dark());
        assert_eq!(tooltip.title.as_ref(), "Explain this action");
        assert!(tooltip.disabled);
        assert_eq!(tooltip.children.len(), 0);
    }
}
