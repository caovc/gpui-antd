//! Native Ant Design-like Select trigger backed by GPUI Kit's searchable list.
//! The state/selection model and overlay keyboard handling stay in GPUI Kit.

use gpui_kit::{App, Entity, IntoElement, RenderOnce, SharedString, Styled as _, Window, px};
use gpui_kit::component::{
    Sizable, Size,
    select::{Select, SelectDelegate, SelectItem, SelectState},
};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntTheme, ControlSize};

/// A typed single-value Select. State is owned by the application:
///
/// ```ignore
/// let state = cx.new(|cx| SelectState::new(vec!["Rust", "Go"], None, window, cx));
/// AntSelect::new(&state).placeholder("Choose language").clearable(true)
/// ```
///
/// Set `SelectState::searchable(true)` during state creation for filtering.
/// Subscribe to `SelectEvent<D>::Confirm` to observe the selected value.
#[derive(IntoElement)]
pub struct AntSelect<D: SelectDelegate + 'static>
where
    <D::Item as SelectItem>::Value: PartialEq + Clone,
{
    state: Entity<SelectState<D>>,
    theme: AntTheme,
    size: ControlSize,
    disabled: bool,
    clearable: bool,
    appearance: bool,
    placeholder: Option<SharedString>,
    accessibility_label: Option<SharedString>,
    search_placeholder: Option<SharedString>,
    menu_width: Option<f32>,
}

impl<D> AntSelect<D>
where
    D: SelectDelegate + 'static,
    <D::Item as SelectItem>::Value: PartialEq + Clone,
{
    pub fn new(state: &Entity<SelectState<D>>) -> Self {
        Self {
            state: state.clone(), theme: AntTheme::default(),
            size: ControlSize::Middle, disabled: false,
            clearable: false, appearance: true,
            placeholder: None, accessibility_label: None,
            search_placeholder: None, menu_width: None,
        }
    }

    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn size(mut self, size: ControlSize) -> Self { self.size = size; self }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
    pub fn clearable(mut self, enabled: bool) -> Self { self.clearable = enabled; self }
    pub fn borderless(mut self, borderless: bool) -> Self { self.appearance = !borderless; self }
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = Some(text.into()); self
    }
    pub fn aria_label(mut self, text: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(text.into()); self
    }
    pub fn search_placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.search_placeholder = Some(text.into()); self
    }
    pub fn menu_width(mut self, width: f32) -> Self {
        self.menu_width = Some(width.max(0.)); self
    }
}

impl<D> RenderOnce for AntSelect<D>
where
    D: SelectDelegate + 'static,
    <D::Item as SelectItem>::Value: PartialEq + Clone,
{
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let size = match self.size {
            ControlSize::Small => Size::Small,
            ControlSize::Middle => Size::Medium,
            ControlSize::Large => Size::Large,
        };
        Select::new(&self.state)
            .with_size(size)
            .disabled(self.disabled)
            .cleanable(self.clearable)
            .appearance(self.appearance)
            .when_some(self.placeholder, |select, text| select.placeholder(text))
            .when_some(self.accessibility_label, |select, text| select.accessibility_label(text))
            .when_some(self.search_placeholder, |select, text| select.search_placeholder(text))
            .when_some(self.menu_width, |select, width| select.menu_width(px(width)))
            .rounded(px(self.theme.border_radius))
            .border_color(self.theme.border)
            .bg(if self.disabled { self.theme.disabled_bg } else { self.theme.surface })
    }
}
