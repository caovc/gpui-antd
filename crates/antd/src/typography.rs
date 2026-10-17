//! Native Ant Design-inspired Typography primitives.
//!
//! These are GPUI elements, not HTML text. Copy and link activation delegate
//! keyboard/focus semantics to GPUI Base.

use std::rc::Rc;
use gpui_kit::{
    App, ClipboardItem, ElementId, FontWeight, Hsla, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px, relative,
};
use gpui_kit::base::{Button as BaseButton, Link as BaseLink, StyledExt as _};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntIcon, AntIconName, AntTheme};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextTone {
    #[default]
    Default,
    Secondary,
    Success,
    Warning,
    Danger,
}
impl TextTone {
    pub fn color(self, t: AntTheme) -> Hsla {
        match self {
            Self::Default => t.text,
            Self::Secondary => t.text_secondary,
            Self::Success => t.success,
            Self::Warning => t.warning,
            Self::Danger => t.error,
        }
    }
}

/// Typography.Title levels 1-5, matching Ant Design font sizes 38/30/24/20/16px.
#[derive(IntoElement)]
pub struct AntTitle {
    text: SharedString,
    level: u8,
    theme: AntTheme,
    tone: TextTone,
}
impl AntTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into(), level: 1, theme: AntTheme::default(), tone: TextTone::Default }
    }
    pub fn level(mut self, level: u8) -> Self { self.level = level.clamp(1, 5); self }
    pub fn theme(mut self, t: AntTheme) -> Self { self.theme = t; self }
    pub fn tone(mut self, t: TextTone) -> Self { self.tone = t; self }
    pub fn font_size(&self) -> f32 {
        match self.level { 1 => 38., 2 => 30., 3 => 24., 4 => 20., _ => 16. }
    }
}
impl RenderOnce for AntTitle {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let font_size = self.font_size();
        div()
            .w_full()
            .font_weight(FontWeight::SEMIBOLD)
            .text_size(px(font_size))
            .line_height(relative(1.23))
            .text_color(self.tone.color(self.theme))
            .child(self.text)
    }
}

/// Typography.Text with emphasis, strong, code, mark, strike and clipboard.
#[derive(IntoElement)]
pub struct AntText {
    text: SharedString,
    tone: TextTone,
    theme: AntTheme,
    strong: bool,
    italic: bool,
    underline: bool,
    deleted: bool,
    code: bool,
    marked: bool,
    disabled: bool,
    copy_id: Option<ElementId>,
}
impl AntText {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(), tone: TextTone::Default, theme: AntTheme::default(),
            strong: false, italic: false, underline: false, deleted: false,
            code: false, marked: false, disabled: false, copy_id: None,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn tone(mut self, tone: TextTone) -> Self { self.tone = tone; self }
    pub fn strong(mut self, strong: bool) -> Self { self.strong = strong; self }
    pub fn italic(mut self, italic: bool) -> Self { self.italic = italic; self }
    pub fn underline(mut self, underline: bool) -> Self { self.underline = underline; self }
    pub fn deleted(mut self, deleted: bool) -> Self { self.deleted = deleted; self }
    pub fn code(mut self, code: bool) -> Self { self.code = code; self }
    pub fn marked(mut self, marked: bool) -> Self { self.marked = marked; self }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
    /// Keyboard-accessible copy button; `id` must be unique in its GPUI view.
    pub fn copyable(mut self, id: impl Into<ElementId>) -> Self {
        self.copy_id = Some(id.into()); self
    }
}
impl RenderOnce for AntText {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let value = self.text.clone();
        let foreground = if self.disabled { theme.disabled_text } else { self.tone.color(theme) };
        let body = div()
            .text_size(px(14.))
            .line_height(relative(1.5715))
            .text_color(foreground)
            .when(self.strong, |this| this.font_semibold())
            .when(self.italic, |this| this.italic())
            .when(self.underline, |this| this.underline())
            .when(self.deleted, |this| this.line_through())
            .when(self.code, |this| this.bg(theme.surface_secondary)
                .border_1().border_color(theme.border)
                .rounded(px(4.)).px(px(5.)).py(px(1.)))
            .when(self.marked, |this| this.bg(theme.warning_bg))
            .child(self.text);
        div().flex().flex_wrap().items_center().gap(px(7.))
            .child(body)
            .when_some(self.copy_id, |this, id| {
                this.child(
                    BaseButton::new(id)
                        .cursor_pointer()
                        .text_color(theme.primary)
                        .child(AntIcon::new(AntIconName::CopyOutlined)
                            .theme(theme).size(14.).color(theme.primary))
                        .on_click(move |_, _, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(value.to_string()));
                        }),
                )
            })
    }
}

/// Typography.Paragraph: readable line height, line-clamping and controlled expansion.
type ExpandHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct AntParagraph {
    text: SharedString,
    theme: AntTheme,
    tone: TextTone,
    ellipsis: bool,
    width: Option<f32>,
    max_lines: Option<usize>,
    expanded: bool,
    expand_id: Option<ElementId>,
    on_expand: Option<ExpandHandler>,
}
impl AntParagraph {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into(), theme: AntTheme::default(),
            tone: TextTone::Default, ellipsis: false, width: None,
            max_lines: None, expanded: false, expand_id: None, on_expand: None }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn tone(mut self, tone: TextTone) -> Self { self.tone = tone; self }
    /// Truncate the visible paragraph to one line.
    pub fn ellipsis(mut self, ellipsis: bool) -> Self { self.ellipsis = ellipsis; self }
    /// GPUI's native text system truncates at the given number of visual lines.
    pub fn max_lines(mut self, lines: usize) -> Self {
        self.max_lines = Some(lines.max(1)); self
    }
    pub fn width(mut self, width: f32) -> Self { self.width = Some(width.max(0.)); self }
    /// The parent is the source of truth for expansion state.
    pub fn expanded(mut self, expanded: bool) -> Self { self.expanded = expanded; self }
    /// Attach a keyboard-accessible Expand / Collapse button.
    /// The callback receives the requested expanded state; store it in the
    /// owning GPUI Entity and call cx.notify() to update the paragraph.
    pub fn on_expand(mut self, id: impl Into<ElementId>,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self
    {
        self.expand_id = Some(id.into());
        self.on_expand = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for AntParagraph {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme=self.theme;
        let expanded=self.expanded;
        let line_limit=if expanded { None } else { self.max_lines };
        let text=div()
            .w_full()
            .text_size(px(14.))
            .line_height(relative(1.5715))
            .text_color(self.tone.color(theme))
            .when_some(self.width, |this, w| this.w(px(w)))
            .when(self.ellipsis && line_limit.is_none() && !expanded,
                |this| this.min_w_0().truncate())
            .when_some(line_limit, |this, count| this.min_w_0().line_clamp(count))
            .child(self.text);
        div().w_full().flex().flex_col().items_start().gap(px(6.))
            .child(text)
            .when_some(self.expand_id.zip(self.on_expand), |this, (id, handler)| {
                this.child(
                    BaseButton::new(id)
                        .cursor_pointer()
                        .text_size(px(12.))
                        .text_color(theme.primary)
                        .child(if expanded { "Collapse" } else { "Expand" })
                        .on_click(move |_, window, cx| handler(!expanded, window, cx)),
                )
            })
    }
}

/// Typography.Link built on GPUI Base's keyboard-focusable Link primitive.
///
/// External absolute HTTP(S) links open via the operating system.
#[derive(IntoElement)]
pub struct AntLink {
    id: ElementId,
    text: SharedString,
    href: SharedString,
    theme: AntTheme,
    disabled: bool,
    underline: bool,
}
impl AntLink {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>, href: impl Into<SharedString>) -> Self {
        Self { id: id.into(), text: text.into(), href: href.into(),
            theme: AntTheme::default(), disabled: false, underline: false }
    }
    pub fn theme(mut self, t: AntTheme) -> Self { self.theme = t; self }
    pub fn disabled(mut self, v: bool) -> Self { self.disabled = v; self }
    pub fn underline(mut self, v: bool) -> Self { self.underline = v; self }
}
impl RenderOnce for AntLink {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        BaseLink::new(self.id)
            .href(self.href)
            .open_with(|href, _, _, cx| cx.open_url(href))
            .disabled(self.disabled)
            .text_size(px(14.))
            .text_color(if self.disabled { t.disabled_text } else { t.primary })
            .when(self.underline, |this| this.underline())
            .when(!self.disabled, |this| this.cursor_pointer()
                .hover(|this| this.text_color(t.primary_hover)))
            .child(self.text)
    }
}

/// A light typography layout container for titles, paragraphs and inline text.
#[derive(IntoElement)]
pub struct AntTypography {
    children: Vec<gpui_kit::AnyElement>,
    gap: f32,
}
impl Default for AntTypography { fn default() -> Self { Self::new() } }
impl AntTypography {
    pub fn new() -> Self { Self { children: Vec::new(), gap: 14. } }
    pub fn gap(mut self, gap: f32) -> Self { self.gap = gap.max(0.); self }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}
impl RenderOnce for AntTypography {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().flex().flex_col().gap(px(self.gap)).children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn titles_match_ant_design_typography_scale() {
        let expected = [38., 30., 24., 20., 16.];
        for (i, size) in expected.iter().enumerate() {
            assert_eq!(AntTitle::new("Heading").level((i+1) as u8).font_size(), *size);
        }
        assert_eq!(AntTitle::new("Large").level(0).font_size(), 38.);
        assert_eq!(AntTitle::new("Small").level(9).font_size(), 16.);
    }
    #[test]
    fn paragraph_limits_and_controlled_expansion() {
        let paragraph=AntParagraph::new("Long text").max_lines(0)
            .expanded(true).width(320.);
        assert_eq!(paragraph.max_lines, Some(1));
        assert!(paragraph.expanded);
        assert_eq!(paragraph.width, Some(320.));
        let paragraph=AntParagraph::new("Other text").max_lines(3);
        assert_eq!(paragraph.max_lines, Some(3));
        assert!(!paragraph.expanded);
    }
    #[test]
    fn status_tones_map_to_theme_colors() {
        let t = AntTheme::light();
        assert_eq!(TextTone::Danger.color(t), t.error);
        assert_eq!(TextTone::Success.color(t), t.success);
        assert_eq!(TextTone::Secondary.color(t), t.text_secondary);
    }
}
