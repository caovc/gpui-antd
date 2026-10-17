//! Native Ant Design-inspired Typography primitives.
//!
//! These are GPUI elements, not HTML text. Copy and link activation delegate
//! keyboard/focus semantics to GPUI Base.

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

/// Typography.Paragraph: readable line height and optional single-line ellipsis.
#[derive(IntoElement)]
pub struct AntParagraph {
    text: SharedString,
    theme: AntTheme,
    tone: TextTone,
    ellipsis: bool,
    width: Option<f32>,
}
impl AntParagraph {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into(), theme: AntTheme::default(),
            tone: TextTone::Default, ellipsis: false, width: None }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn tone(mut self, tone: TextTone) -> Self { self.tone = tone; self }
    /// Single line with clipping and a trailing ellipsis.
    pub fn ellipsis(mut self, ellipsis: bool) -> Self { self.ellipsis = ellipsis; self }
    pub fn width(mut self, px: f32) -> Self { self.width = Some(px.max(0.)); self }
}
impl RenderOnce for AntParagraph {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .text_size(px(14.))
            .line_height(relative(1.5715))
            .text_color(self.tone.color(self.theme))
            .when_some(self.width, |this, w| this.w(px(w)))
            .when(self.ellipsis, |this| this.min_w_0().truncate())
            .child(self.text)
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
    fn status_tones_map_to_theme_colors() {
        let t = AntTheme::light();
        assert_eq!(TextTone::Danger.color(t), t.error);
        assert_eq!(TextTone::Success.color(t), t.success);
        assert_eq!(TextTone::Secondary.color(t), t.text_secondary);
    }
}
