//! Additional Ant Design foundation widgets: Avatar, Statistic, Empty and Breadcrumb.
//! All render through the native GPUI element tree and honor AntTheme.

use crate::{AntIcon, AntIconName, AntLink, AntText, AntTheme, TextTone, color};
use gpui_kit::base::{
    Avatar as BaseAvatar, AvatarFallback as BaseAvatarFallback, AvatarImage as BaseAvatarImage,
    StyledExt as _,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ImageSource, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, div, px, relative,
};

/// Shape of Ant Design's user avatar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AvatarShape {
    #[default]
    Circle,
    Square,
}
/// Ant Design avatar size presets: 24 / 32 / 40 / custom logical pixels.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AvatarSize {
    Small,
    #[default]
    Middle,
    Large,
    Custom(f32),
}
impl AvatarSize {
    pub fn pixels(self) -> f32 {
        match self {
            Self::Small => 24.,
            Self::Middle => 32.,
            Self::Large => 40.,
            Self::Custom(value) => value.max(0.),
        }
    }
}

/// Initials fallback and optional image slot provided by GPUI Base Avatar.
#[derive(IntoElement)]
pub struct AntAvatar {
    label: SharedString,
    source: Option<ImageSource>,
    size: AvatarSize,
    shape: AvatarShape,
    background: Option<gpui_kit::Hsla>,
    theme: AntTheme,
}
impl AntAvatar {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            source: None,
            size: AvatarSize::Middle,
            shape: AvatarShape::Circle,
            background: None,
            theme: AntTheme::default(),
        }
    }
    pub fn src(mut self, source: impl Into<ImageSource>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }
    pub fn background(mut self, color: gpui_kit::Hsla) -> Self {
        self.background = Some(color);
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn initials(&self) -> String {
        initials(self.label.as_ref())
    }
}
fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}
impl RenderOnce for AntAvatar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let size = self.size.pixels();
        let radius = match self.shape {
            AvatarShape::Circle => size / 2.,
            AvatarShape::Square => 6.,
        };
        let content = self.initials();
        let bg = self.background.unwrap_or(self.theme.primary);
        BaseAvatar::new()
            .size(px(size))
            .flex_shrink_0()
            .rounded(px(radius))
            .overflow_hidden()
            .bg(bg)
            .text_color(color(0xffffff))
            .fallback(
                BaseAvatarFallback::new()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(radius))
                    .bg(bg)
                    .font_medium()
                    .text_size(px(size * 0.47))
                    .child(content),
            )
            .when_some(self.source, |this, src| {
                this.image(BaseAvatarImage::new(src).size_full().rounded(px(radius)))
            })
    }
}

/// A display-only numeric statistic, with optional prefix/suffix and precision.
#[derive(IntoElement)]
pub struct AntStatistic {
    title: SharedString,
    value: SharedString,
    prefix: Option<SharedString>,
    suffix: Option<SharedString>,
    tone: TextTone,
    theme: AntTheme,
}
impl AntStatistic {
    pub fn new(title: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            prefix: None,
            suffix: None,
            tone: TextTone::Default,
            theme: AntTheme::default(),
        }
    }
    pub fn number(title: impl Into<SharedString>, value: f64, precision: usize) -> Self {
        Self::new(title, format_number(value, precision.min(10)))
    }
    pub fn prefix(mut self, value: impl Into<SharedString>) -> Self {
        self.prefix = Some(value.into());
        self
    }
    pub fn suffix(mut self, value: impl Into<SharedString>) -> Self {
        self.suffix = Some(value.into());
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn tone(mut self, tone: TextTone) -> Self {
        self.tone = tone;
        self
    }
}
/// Group comma-separated thousands with Unicode-safe prefix/suffix handled by elements.
pub fn format_number(value: f64, precision: usize) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let formatted = format!("{:.*}", precision.min(10), value);
    let mut pair = formatted.splitn(2, '.');
    let integer = pair.next().unwrap_or("");
    let decimals = pair.next();
    let (sign, digits) = integer
        .strip_prefix('-')
        .map(|s| ("-", s))
        .unwrap_or(("", integer));
    let mut out = String::new();
    for (i, c) in digits.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    let integer: String = out.chars().rev().collect();
    match decimals {
        Some(decimals) => format!("{sign}{integer}.{decimals}"),
        None => format!("{sign}{integer}"),
    }
}
impl RenderOnce for AntStatistic {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        div()
            .flex()
            .flex_col()
            .gap(px(4.))
            .child(
                div()
                    .text_size(px(14.))
                    .text_color(t.text_secondary)
                    .child(self.title),
            )
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap(px(5.))
                    .text_size(px(24.))
                    .line_height(relative(1.33))
                    .font_semibold()
                    .text_color(self.tone.color(t))
                    .when_some(self.prefix, |this, text| {
                        this.child(div().text_size(px(18.)).child(text))
                    })
                    .child(self.value)
                    .when_some(self.suffix, |this, text| {
                        this.child(div().text_size(px(14.)).child(text))
                    }),
            )
    }
}

/// Centered empty state with a real Ant Design icon and an optional action.
#[derive(IntoElement)]
pub struct AntEmpty {
    description: SharedString,
    children: Vec<AnyElement>,
    icon: AntIconName,
    theme: AntTheme,
    compact: bool,
}
impl Default for AntEmpty {
    fn default() -> Self {
        Self::new()
    }
}
impl AntEmpty {
    pub fn new() -> Self {
        Self {
            description: "No data".into(),
            children: Vec::new(),
            icon: AntIconName::FileOutlined,
            theme: AntTheme::default(),
            compact: false,
        }
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = description.into();
        self
    }
    pub fn icon(mut self, icon: AntIconName) -> Self {
        self.icon = icon;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.children.push(element.into_any_element());
        self
    }
}
impl RenderOnce for AntEmpty {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .gap(px(12.))
            .py(px(if self.compact { 14. } else { 32. }))
            .child(
                AntIcon::new(self.icon)
                    .theme(t)
                    .color(t.disabled_text)
                    .size(if self.compact { 32. } else { 56. }),
            )
            .child(
                div()
                    .text_size(px(14.))
                    .text_color(t.text_secondary)
                    .child(self.description),
            )
            .children(self.children)
    }
}

/// Individual item in native breadcrumb navigation.
pub struct AntBreadcrumbItem {
    label: SharedString,
    href: Option<SharedString>,
    disabled: bool,
}
impl AntBreadcrumbItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            href: None,
            disabled: false,
        }
    }
    pub fn href(mut self, url: impl Into<SharedString>) -> Self {
        self.href = Some(url.into());
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Breadcrumb root: renders separators and keyboard-accessible GPUI Base links.
#[derive(IntoElement)]
pub struct AntBreadcrumb {
    items: Vec<AntBreadcrumbItem>,
    separator: SharedString,
    theme: AntTheme,
}
impl Default for AntBreadcrumb {
    fn default() -> Self {
        Self::new()
    }
}
impl AntBreadcrumb {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            separator: "/".into(),
            theme: AntTheme::default(),
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn separator(mut self, separator: impl Into<SharedString>) -> Self {
        self.separator = separator.into();
        self
    }
    pub fn child(mut self, item: AntBreadcrumbItem) -> Self {
        self.items.push(item);
        self
    }
}
impl RenderOnce for AntBreadcrumb {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let total = self.items.len();
        let t = self.theme;
        let separator = self.separator;
        let mut elements = Vec::with_capacity(total.saturating_mul(2));
        for (i, item) in self.items.into_iter().enumerate() {
            if i > 0 {
                elements.push(
                    div()
                        .text_size(px(12.))
                        .text_color(t.disabled_text)
                        .child(separator.clone())
                        .into_any_element(),
                );
            }
            let current = i + 1 == total;
            let value = match (current, item.href, item.disabled) {
                (false, Some(url), false) => AntLink::new(("breadcrumb", i), item.label, url)
                    .theme(t)
                    .into_any_element(),
                _ => AntText::new(item.label)
                    .tone(if current {
                        TextTone::Default
                    } else {
                        TextTone::Secondary
                    })
                    .theme(t)
                    .into_any_element(),
            };
            elements.push(value);
        }
        div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(8.))
            .children(elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unicode_name_initials() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("张 三"), "张三");
        assert_eq!(initials("Single"), "S");
    }
    #[test]
    fn statistic_formatting_preserves_precision() {
        assert_eq!(format_number(1234567.125, 2), "1,234,567.12");
        assert_eq!(format_number(-12345., 0), "-12,345");
        assert_eq!(format_number(0., 0), "0");
    }
    #[test]
    fn avatar_sizes_match_ant_tokens() {
        assert_eq!(AvatarSize::Small.pixels(), 24.);
        assert_eq!(AvatarSize::Middle.pixels(), 32.);
        assert_eq!(AvatarSize::Large.pixels(), 40.);
        assert_eq!(AvatarSize::Custom(-5.).pixels(), 0.);
    }
}
