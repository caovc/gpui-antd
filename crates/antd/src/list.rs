//! Ant Design-inspired native List with typed layout rows and optional actions.
//! All rows and extra content are genuine GPUI elements on desktop and WASM.

use crate::{AntEmpty, AntIcon, AntIconName, AntTheme};
use gpui_kit::base::StyledExt as _;
use gpui_kit::component::{Sizable as _, Size, spinner::Spinner};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _,
    Window, div, px,
};

/// Layout density of an Ant Design List item.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListSize {
    Small,
    #[default]
    Default,
    Large,
}
impl ListSize {
    pub fn padding(self) -> f32 {
        match self {
            Self::Small => 10.,
            Self::Default => 16.,
            Self::Large => 24.,
        }
    }
}

/// One list item: title/description, optional icon, extra slot and a right-side
/// action row. Custom action elements retain native GPUI focus and callbacks.
pub struct AntListItem {
    title: SharedString,
    description: Option<SharedString>,
    icon: Option<AntIconName>,
    actions: Vec<AnyElement>,
    extra: Option<AnyElement>,
    disabled: bool,
}
impl AntListItem {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
            actions: Vec::new(),
            extra: None,
            disabled: false,
        }
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn icon(mut self, icon: AntIconName) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn action(mut self, element: impl IntoElement) -> Self {
        self.actions.push(element.into_any_element());
        self
    }
    pub fn extra(mut self, element: impl IntoElement) -> Self {
        self.extra = Some(element.into_any_element());
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// List presentation. Items are application-owned Rust data, while the
/// List controls their consistent spacing, separators, empty state and
/// optional native loading indicator. This first milestone is non-virtual:
/// use Table/virtualized components for very large datasets.
#[derive(IntoElement)]
pub struct AntList {
    items: Vec<AntListItem>,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    theme: AntTheme,
    size: ListSize,
    bordered: bool,
    split: bool,
    loading: bool,
}
impl AntList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            header: None,
            footer: None,
            theme: AntTheme::default(),
            size: ListSize::Default,
            bordered: false,
            split: true,
            loading: false,
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn size(mut self, size: ListSize) -> Self {
        self.size = size;
        self
    }
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }
    pub fn split(mut self, split: bool) -> Self {
        self.split = split;
        self
    }
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
    pub fn item(mut self, item: AntListItem) -> Self {
        self.items.push(item);
        self
    }
    pub fn items(mut self, items: impl IntoIterator<Item = AntListItem>) -> Self {
        self.items.extend(items);
        self
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
impl Default for AntList {
    fn default() -> Self {
        Self::new()
    }
}
impl RenderOnce for AntList {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let pad = self.size.padding();
        let border = if self.bordered { t.border } else { t.surface };
        let length = self.items.len();
        div()
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .rounded(px(t.border_radius_lg))
            .when(self.bordered, |this| this.border_1())
            .border_color(border)
            .bg(t.surface)
            .when_some(self.header, |this, header| {
                this.child(
                    div()
                        .px(px(pad))
                        .py(px(12.))
                        .when(self.split, |this| this.border_b_1().border_color(t.border))
                        .child(header),
                )
            })
            .when(self.loading, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap(px(9.))
                        .p(px(20.))
                        .child(Spinner::new().with_size(Size::Small).color(t.primary))
                        .child(div().text_color(t.text_secondary).child("Loading...")),
                )
            })
            .when(length == 0 && !self.loading, |this| {
                this.child(AntEmpty::new().theme(t).compact(true))
            })
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let muted = if item.disabled {
                    t.disabled_text
                } else {
                    t.text
                };
                div()
                    .w_full()
                    .min_w_0()
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .px(px(pad))
                    .py(px(pad))
                    .when(self.split && index + 1 < length, |this| {
                        this.border_b_1().border_color(t.border)
                    })
                    .when(item.disabled, |this| this.opacity(0.7))
                    .when_some(item.icon, |this, icon| {
                        this.child(AntIcon::new(icon).size(28.).theme(t).color(muted))
                    })
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap(px(5.))
                            .child(div().font_medium().text_color(muted).child(item.title))
                            .when_some(item.description, |this, desc| {
                                this.child(
                                    div()
                                        .text_size(px(12.))
                                        .text_color(t.text_secondary)
                                        .child(desc),
                                )
                            }),
                    )
                    .when(!item.actions.is_empty(), |this| {
                        this.child(
                            div()
                                .flex()
                                .flex_wrap()
                                .items_center()
                                .gap(px(8.))
                                .children(item.actions),
                        )
                    })
                    .when_some(item.extra, |this, extra| this.child(extra))
            }))
            .when_some(self.footer, |this, footer| {
                this.child(
                    div()
                        .px(px(pad))
                        .py(px(12.))
                        .when(self.split, |this| this.border_t_1().border_color(t.border))
                        .child(footer),
                )
            })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn list_builders_and_density() {
        assert_eq!(ListSize::Small.padding(), 10.);
        assert_eq!(ListSize::Default.padding(), 16.);
        assert_eq!(ListSize::Large.padding(), 24.);
        let list = AntList::new()
            .bordered(true)
            .split(false)
            .size(ListSize::Large)
            .item(AntListItem::new("First"))
            .item(AntListItem::new("Second").description("Details"));
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
        assert!(list.bordered);
        assert!(!list.split);
    }
    #[test]
    fn empty_and_loading_are_independent() {
        assert!(AntList::new().is_empty());
        let list = AntList::new().loading(true);
        assert!(list.is_empty());
        assert!(list.loading);
    }
}
