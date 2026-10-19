//! Ant Design hover Tooltip built on GPUI Kit **base** HoverCard.
//!
//! Kit's component tooltip has no placement API. Base `HoverCard` owns delayed
//! hover open/close, outside dismissal, and `Anchor` placement — so this
//! wrapper paints antd chrome (dark bg, light text, pad 6×8, radius 6) without
//! a fake absolute popup. Public constructor stays `AntTooltip::new(id, title)`.
//!
//! Placement mapping (antd side → Popup anchor of the bubble):
//! - Top    → `Anchor::BottomCenter` (bubble above trigger)
//! - Bottom → `Anchor::TopCenter`
//! - Left   → `Anchor::RightCenter`
//! - Right  → `Anchor::LeftCenter`

use std::time::Duration;

use gpui_kit::{
    Anchor, AnyElement, App, ElementId, Hsla, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    Styled as _, Window, div, px,
};
use gpui_kit::base::{HoverCard, TestSupportExt as _};
use gpui_kit::prelude::FluentBuilder as _;
use crate::AntTheme;
use crate::design_tokens::AntComponent;

/// Ant Design-style placement for the tooltip bubble.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// Map antd placement to the base Popup/HoverCard anchor.
pub fn tooltip_anchor(placement:TooltipPlacement)->Anchor{
    match placement {
        TooltipPlacement::Top => Anchor::BottomCenter,
        TooltipPlacement::Bottom => Anchor::TopCenter,
        TooltipPlacement::Left => Anchor::RightCenter,
        TooltipPlacement::Right => Anchor::LeftCenter,
    }
}

/// Pick readable tooltip text for a background using HSLA lightness.
///
/// Light backgrounds (l > 0.55) get `dark_text`; otherwise `light_text`.
pub fn tooltip_contrast_text(bg:Hsla,light_text:Hsla,dark_text:Hsla)->Hsla{
    if bg.l > 0.55 { dark_text } else { light_text }
}

/// Descriptive tooltip attached to a GPUI element via base HoverCard.
///
/// The child element is the trigger. Examples:
/// ```ignore
/// AntTooltip::new("save-help", "Save your changes")
///     .placement(TooltipPlacement::Top)
///     .child(AntButton::new("save", "Save").primary())
/// ```
#[derive(IntoElement)]
pub struct AntTooltip {
    id: ElementId,
    title: SharedString,
    theme: AntTheme,
    disabled: bool,
    placement: TooltipPlacement,
    /// Optional background override from `.color(Hsla)`.
    color: Option<Hsla>,
    children: Vec<AnyElement>,
}

impl AntTooltip {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id:id.into(),title:title.into(),theme:AntTheme::default(),
            disabled:false,placement:TooltipPlacement::Top,color:None,children:Vec::new(),
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme=theme;self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled=disabled;self
    }
    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement=placement;self
    }
    /// Override the tooltip background. Text color follows a simple luminance
    /// check on the HSLA `l` channel so light custom colors stay readable.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color=Some(color);self
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
        let color_override=self.color;
        let disabled=self.disabled;
        let children=self.children;
        let placement=self.placement;
        let comp=theme.component(AntComponent::Tooltip);
        let radius=comp.border_radius.unwrap_or(6.);
        let bg=color_override.unwrap_or_else(|| comp.color_bg.unwrap_or(theme.text));
        let default_fg=comp.color_text.unwrap_or(theme.surface);
        let fg=if color_override.is_some() {
            tooltip_contrast_text(bg, default_fg, theme.text)
        } else {
            default_fg
        };
        let border=comp.color_border.unwrap_or(theme.text_secondary);
        let font=comp.font_size.unwrap_or(14.);

        let id = self.id;
        if disabled {
            return div().id(id).children(children).into_any_element();
        }
        let body_id = ElementId::Name(format!("ant-tooltip-body-{id:?}").into());

        HoverCard::new(id)
            .anchor(tooltip_anchor(placement))
            // Faster than HoverCard defaults so tooltips feel like antd, not docs cards.
            .open_delay(Duration::from_millis(100))
            .close_delay(Duration::from_millis(100))
            .trigger(div().flex().children(children))
            .content(move |_, _, _| {
                div()
                    .id(body_id)
                    .role(Role::Tooltip)
                    .bg(bg)
                    .text_color(fg)
                    .border_1()
                    .border_color(border)
                    .rounded(px(radius))
                    .px(px(8.))
                    .py(px(6.))
                    .text_size(px(font))
                    .max_w(px(250.))
                    .child(div().id("ant-tooltip-text").test_support().child(title.clone()))
            })
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    #[test]
    fn tooltip_builders_preserve_title_disabled_and_placement() {
        let tooltip=AntTooltip::new("help", "Explain this action")
            .disabled(true)
            .placement(TooltipPlacement::Left)
            .theme(AntTheme::dark())
            .color(color(0xffffff));
        assert_eq!(tooltip.title.as_ref(), "Explain this action");
        assert!(tooltip.disabled);
        assert_eq!(tooltip.placement, TooltipPlacement::Left);
        assert!(tooltip.color.is_some());
        assert_eq!(tooltip.children.len(), 0);
    }

    #[test]
    fn placement_maps_to_popup_anchor() {
        assert_eq!(tooltip_anchor(TooltipPlacement::Top), Anchor::BottomCenter);
        assert_eq!(tooltip_anchor(TooltipPlacement::Bottom), Anchor::TopCenter);
        assert_eq!(tooltip_anchor(TooltipPlacement::Left), Anchor::RightCenter);
        assert_eq!(tooltip_anchor(TooltipPlacement::Right), Anchor::LeftCenter);
    }

    #[test]
    fn contrast_text_follows_background_lightness() {
        let light_bg=color(0xffffff);
        let dark_bg=color(0x000000);
        let light_text=color(0xfafafa);
        let dark_text=color(0x141414);
        assert_eq!(
            tooltip_contrast_text(light_bg, light_text, dark_text),
            dark_text
        );
        assert_eq!(
            tooltip_contrast_text(dark_bg, light_text, dark_text),
            light_text
        );
    }
}
