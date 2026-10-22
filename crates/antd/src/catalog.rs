//! Ant Design components that GPUI Kit does not provide as styled widgets.
//!
//! Drag and row windowing stay on Kit base primitives (`ResizablePanelGroup`,
//! `VirtualList`) because those own pointer capture and scroll measurement.
//! Everything the user sees — color, type, keyboard, and selection — is painted
//! here from `AntTheme`, not from Kit's component chrome. Text entry still uses
//! `InputState` so IME and caret behavior are not reimplemented.
//!
//! These are alpha APIs. OS file dialogs remain a gap.
//! Cascader, TreeSelect, ColorPicker, Mentions and Tour use base Popover
//! for placement, outside dismissal and Escape. BorderBeam moves a token-colored
//! segment; it is not a shader stroke.

use std::{collections::BTreeSet, rc::Rc, time::Duration};

use gpui_kit::base::{
    Button as BaseButton, Popover as BasePopover, h_resizable, resizable_panel, v_resizable,
    v_virtual_list,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    Anchor, Animation, AnimationExt as _, AnyElement, App, Context, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, KeyDownEvent, ParentElement as _, Render, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px, relative,
    size,
};
use qrcode::{Color as QrColor, EcLevel, QrCode};

use crate::overlays::{OverlayPlacement, overlay_anchor};
use crate::{AntIcon, AntIconName, AntText, AntTheme, AntTree, AntTreeNode, TextTone};

/// Menus open below the trigger, left-aligned. The named anchor is the popup
/// corner pinned to the opposite edge of the trigger.
fn dropdown_anchor() -> Anchor {
    overlay_anchor(OverlayPlacement::BottomLeft)
}

type Notify = Rc<dyn Fn(&mut Window, &mut App)>;
type KeyNotify = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;

#[derive(Clone, Debug)]
pub struct AntAnchorLink {
    key: SharedString,
    title: SharedString,
    disabled: bool,
}
impl AntAnchorLink {
    pub fn new(key: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }
}

pub fn next_anchor_index(enabled: &[bool], current: usize, key: &str) -> Option<usize> {
    let first = enabled.iter().position(|v| *v)?;
    match key {
        "home" => Some(first),
        "end" => enabled.iter().rposition(|v| *v),
        "up" | "down" => {
            let len = enabled.len();
            let start = if enabled.get(current) == Some(&true) {
                current
            } else {
                first
            };
            for step in 1..=len {
                let candidate = if key == "down" {
                    (start + step) % len
                } else {
                    (start + len - (step % len)) % len
                };
                if enabled[candidate] {
                    return Some(candidate);
                }
            }
            None
        }
        _ => None,
    }
}

#[derive(IntoElement)]
pub struct AntAnchor {
    id: ElementId,
    links: Vec<AntAnchorLink>,
    active: Option<SharedString>,
    theme: AntTheme,
    on_click: Option<KeyNotify>,
}
impl AntAnchor {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            links: Vec::new(),
            active: None,
            theme: AntTheme::default(),
            on_click: None,
        }
    }
    pub fn link(mut self, link: AntAnchorLink) -> Self {
        self.links.push(link);
        self
    }
    pub fn active(mut self, key: impl Into<SharedString>) -> Self {
        self.active = Some(key.into());
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_click(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntAnchor {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = self.theme;
        let active = self.active;
        let on_click = self.on_click;
        let enabled: Vec<bool> = self.links.iter().map(|link| !link.disabled).collect();
        let prefix = format!("{:?}", self.id);
        let handles: Vec<FocusHandle> = (0..self.links.len())
            .map(|index| {
                window
                    .use_keyed_state(format!("ant-anchor-{prefix}-{index}"), cx, |_, cx| {
                        cx.focus_handle()
                    })
                    .read(cx)
                    .clone()
            })
            .collect();
        let keyboard_handles = handles.clone();
        let keyboard_enabled = enabled;
        let keys: Vec<SharedString> = self.links.iter().map(|link| link.key.clone()).collect();
        let keyboard_click = on_click.clone();
        let on_keyboard = move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
            if event.keystroke.modifiers.modified() || event.keystroke.is_ime_in_progress() {
                return;
            }
            let Some(origin) = keyboard_handles
                .iter()
                .position(|handle| handle.is_focused(window))
            else {
                return;
            };
            if let Some(next) =
                next_anchor_index(&keyboard_enabled, origin, event.keystroke.key.as_str())
            {
                window.focus(&keyboard_handles[next], cx);
                if let Some(callback) = &keyboard_click {
                    callback(keys[next].clone(), window, cx);
                }
                window.prevent_default();
                cx.stop_propagation();
            }
        };
        div()
            .id(self.id)
            .role(Role::Navigation)
            .flex()
            .flex_col()
            .gap(px(2.))
            .border_l_1()
            .border_color(t.border)
            .on_key_down(on_keyboard)
            .children(
                self.links
                    .into_iter()
                    .enumerate()
                    .map(move |(index, link)| {
                        let chosen = active.as_deref() == Some(link.key.as_ref());
                        let key = link.key.clone();
                        let click = on_click.clone();
                        let handle = handles[index].clone();
                        BaseButton::new(ElementId::Name(format!("anchor-{key}").into()))
                            .disabled(link.disabled)
                            .track_focus(&handle)
                            .on_click(move |_, window, cx| {
                                if let Some(callback) = &click {
                                    callback(key.clone(), window, cx);
                                }
                            })
                            .child(
                                div()
                                    .px(px(12.))
                                    .py(px(6.))
                                    .border_l_2()
                                    .border_color(if chosen { t.primary } else { t.surface })
                                    .text_color(if link.disabled {
                                        t.disabled_text
                                    } else if chosen {
                                        t.primary
                                    } else {
                                        t.text
                                    })
                                    .child(link.title),
                            )
                    }),
            )
    }
}

pub fn watermark_count(width: f32, height: f32, gap: f32) -> usize {
    let gap = gap.max(24.);
    let cols = ((width / gap).floor() as usize).max(1);
    let rows = ((height / gap).floor() as usize).max(1);
    cols * rows
}

#[derive(IntoElement)]
pub struct AntWatermark {
    text: SharedString,
    theme: AntTheme,
    gap: f32,
    children: Vec<AnyElement>,
}
impl AntWatermark {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            theme: AntTheme::default(),
            gap: 96.,
            children: Vec::new(),
        }
    }
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(24.);
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}
impl RenderOnce for AntWatermark {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let cols = 4;
        let rows = 3;
        div()
            .relative()
            .w_full()
            .h(px(160.))
            .overflow_hidden()
            .rounded(px(t.border_radius))
            .bg(t.surface)
            .children(self.children)
            .children((0..rows).flat_map(|row| {
                let text = self.text.clone();
                (0..cols).map(move |col| {
                    div()
                        .absolute()
                        .left(px(col as f32 * self.gap))
                        .top(px(row as f32 * 48.))
                        .text_color(t.border)
                        .text_size(px(13.))
                        .child(text.clone())
                })
            }))
    }
}

#[derive(IntoElement)]
pub struct AntBorderBeam {
    id: ElementId,
    theme: AntTheme,
    children: Vec<AnyElement>,
}
impl AntBorderBeam {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            theme: AntTheme::default(),
            children: Vec::new(),
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}
impl Default for AntBorderBeam {
    fn default() -> Self {
        Self::new("ant-border-beam")
    }
}
impl RenderOnce for AntBorderBeam {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let id = self.id;
        div()
            .relative()
            .w_full()
            .overflow_hidden()
            .p(px(2.))
            .rounded(px(t.border_radius + 2.))
            .bg(t.primary_border)
            .child(
                div()
                    .absolute()
                    .top(px(0.))
                    .left(px(0.))
                    .right(px(0.))
                    .h(px(2.))
                    .child(
                        div()
                            .h(px(2.))
                            .w(relative(0.18))
                            .bg(t.primary)
                            .with_animation(
                                id,
                                Animation::new(Duration::from_millis(1600)).repeat(),
                                |beam, delta| beam.ml(relative(delta * 0.82)),
                            ),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .p(px(12.))
                    .rounded(px(t.border_radius))
                    .bg(t.surface)
                    .children(self.children),
            )
    }
}

fn picker_trigger(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    theme: AntTheme,
) -> AnyElement {
    BaseButton::new(id)
        .h(px(32.))
        .min_w(px(180.))
        .px(px(10.))
        .flex()
        .items_center()
        .rounded(px(theme.border_radius))
        .border_1()
        .border_color(theme.border)
        .bg(theme.surface)
        .cursor_pointer()
        .child(AntText::new(label.into()).theme(theme))
        .into_any_element()
}

fn ant_handle(theme: AntTheme) -> gpui_kit::base::ResizeHandleRenderer {
    Rc::new(move |_ctx, _window, _cx| Some(div().size_full().bg(theme.border).into_any_element()))
}

/// Splitter drag is Kit's resizable group. The handle paint is ours.
#[derive(IntoElement)]
pub struct AntSplitter {
    id: ElementId,
    vertical: bool,
    theme: AntTheme,
    first: Vec<AnyElement>,
    second: Vec<AnyElement>,
}
impl AntSplitter {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            vertical: false,
            theme: AntTheme::default(),
            first: Vec::new(),
            second: Vec::new(),
        }
    }
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn first(mut self, child: impl IntoElement) -> Self {
        self.first.push(child.into_any_element());
        self
    }
    pub fn second(mut self, child: impl IntoElement) -> Self {
        self.second.push(child.into_any_element());
        self
    }
}
impl RenderOnce for AntSplitter {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let first = div()
            .p(px(8.))
            .size_full()
            .bg(theme.surface)
            .children(self.first);
        let second = div()
            .p(px(8.))
            .size_full()
            .bg(theme.surface_secondary)
            .children(self.second);
        let group = if self.vertical {
            v_resizable(self.id)
        } else {
            h_resizable(self.id)
        };
        group
            .size(px(if self.vertical { 280. } else { 180. }))
            .with_handle_appearance(ant_handle(theme))
            .child(resizable_panel().child(first))
            .child(resizable_panel().child(second))
    }
}

pub fn masonry_columns(count: usize, columns: usize) -> Vec<Vec<usize>> {
    let columns = columns.max(1);
    let mut out = vec![Vec::new(); columns];
    for index in 0..count {
        out[index % columns].push(index);
    }
    out
}

#[derive(Clone, Debug)]
pub struct AntMasonryItem {
    title: SharedString,
    lines: u8,
}
impl AntMasonryItem {
    pub fn new(title: impl Into<SharedString>, lines: u8) -> Self {
        Self {
            title: title.into(),
            lines: lines.clamp(1, 6),
        }
    }
}

#[derive(IntoElement)]
pub struct AntMasonry {
    columns: usize,
    items: Vec<AntMasonryItem>,
    theme: AntTheme,
}
impl AntMasonry {
    pub fn new() -> Self {
        Self {
            columns: 3,
            items: Vec::new(),
            theme: AntTheme::default(),
        }
    }
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }
    pub fn item(mut self, item: AntMasonryItem) -> Self {
        self.items.push(item);
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
}
impl Default for AntMasonry {
    fn default() -> Self {
        Self::new()
    }
}
impl RenderOnce for AntMasonry {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let layout = masonry_columns(self.items.len(), self.columns);
        div()
            .w_full()
            .flex()
            .gap(px(8.))
            .items_start()
            .children(layout.into_iter().map(|column| {
                div().flex_1().flex().flex_col().gap(px(8.)).children(
                    column
                        .into_iter()
                        .filter_map(|index| self.items.get(index))
                        .map(|item| {
                            div()
                                .p(px(10.))
                                .rounded(px(t.border_radius))
                                .bg(t.surface)
                                .border_1()
                                .border_color(t.border)
                                .child(AntText::new(item.title.clone()).theme(t))
                                .child(
                                    div()
                                        .h(px(item.lines as f32 * 14.))
                                        .mt(px(6.))
                                        .bg(t.surface_secondary)
                                        .rounded(px(4.)),
                                )
                        }),
                )
            }))
    }
}

pub fn carousel_index(len: usize, current: usize, delta: i32) -> usize {
    if len == 0 {
        return 0;
    }
    let len = len as i32;
    let next = current as i32 + delta;
    (((next % len) + len) % len) as usize
}

#[derive(Clone, Debug)]
pub struct AntCarouselSlide {
    title: SharedString,
    tone: TextTone,
}
impl AntCarouselSlide {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            tone: TextTone::Default,
        }
    }
    pub fn tone(mut self, tone: TextTone) -> Self {
        self.tone = tone;
        self
    }
}

#[derive(IntoElement)]
pub struct AntCarousel {
    id: ElementId,
    slides: Vec<AntCarouselSlide>,
    index: usize,
    theme: AntTheme,
    on_change: Option<Rc<dyn Fn(usize, &mut Window, &mut App)>>,
}
impl AntCarousel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            slides: Vec::new(),
            index: 0,
            theme: AntTheme::default(),
            on_change: None,
        }
    }
    pub fn slide(mut self, slide: AntCarouselSlide) -> Self {
        self.slides.push(slide);
        self
    }
    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntCarousel {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let len = self.slides.len();
        let index = if len == 0 { 0 } else { self.index % len };
        let change = self.on_change.clone();
        let prev = change.clone();
        let id = self.id.clone();
        div()
            .id(id.clone())
            .w_full()
            .flex()
            .flex_col()
            .gap(px(8.))
            .child(
                div()
                    .h(px(96.))
                    .rounded(px(t.border_radius))
                    .bg(t.primary_bg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .when_some(self.slides.get(index).cloned(), |view, slide| {
                        view.child(AntText::new(slide.title).theme(t).tone(slide.tone))
                    }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        BaseButton::new(ElementId::Name(format!("{id:?}-prev").into()))
                            .disabled(len == 0)
                            .on_click(move |_, window, cx| {
                                if let Some(cb) = &prev {
                                    cb(carousel_index(len, index, -1), window, cx);
                                }
                            })
                            .child(
                                div()
                                    .px(px(10.))
                                    .py(px(4.))
                                    .rounded(px(t.border_radius))
                                    .border_1()
                                    .border_color(t.border)
                                    .child("Prev"),
                            ),
                    )
                    .children((0..len).map(|dot| {
                        let cb = change.clone();
                        BaseButton::new(ElementId::Name(format!("{id:?}-dot-{dot}").into()))
                            .on_click(move |_, window, cx| {
                                if let Some(cb) = &cb {
                                    cb(dot, window, cx);
                                }
                            })
                            .child(div().size(px(8.)).rounded(px(4.)).bg(if dot == index {
                                t.primary
                            } else {
                                t.border
                            }))
                    }))
                    .child({
                        let cb = change.clone();
                        BaseButton::new(ElementId::Name(format!("{id:?}-next").into()))
                            .disabled(len == 0)
                            .on_click(move |_, window, cx| {
                                if let Some(cb) = &cb {
                                    cb(carousel_index(len, index, 1), window, cx);
                                }
                            })
                            .child(
                                div()
                                    .px(px(10.))
                                    .py(px(4.))
                                    .rounded(px(t.border_radius))
                                    .border_1()
                                    .border_color(t.border)
                                    .child("Next"),
                            )
                    }),
            )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QrMatrix {
    pub width: usize,
    pub cells: Vec<bool>,
}
impl QrMatrix {
    pub fn dark(&self, x: usize, y: usize) -> bool {
        self.cells.get(y * self.width + x).copied().unwrap_or(false)
    }
}

pub fn qr_matrix(data: &str) -> Result<QrMatrix, String> {
    if data.is_empty() {
        return Err("empty".into());
    }
    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::L)
        .map_err(|err| err.to_string())?;
    let width = code.width();
    let mut cells = Vec::with_capacity(width * width);
    for y in 0..width {
        for x in 0..width {
            cells.push(code[(x, y)] == QrColor::Dark);
        }
    }
    Ok(QrMatrix { width, cells })
}

#[derive(IntoElement)]
pub struct AntQrCode {
    value: SharedString,
    size: f32,
    theme: AntTheme,
}
impl AntQrCode {
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            size: 128.,
            theme: AntTheme::default(),
        }
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(32.);
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
}
impl RenderOnce for AntQrCode {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        match qr_matrix(self.value.as_ref()) {
            Ok(matrix) => {
                let width = matrix.width;
                let cells = matrix.cells;
                let cell = self.size / width as f32;
                div()
                    .p(px(8.))
                    .bg(t.surface)
                    .rounded(px(t.border_radius))
                    .border_1()
                    .border_color(t.border)
                    .child(
                        div().relative().size(px(self.size)).bg(t.surface).children(
                            (0..width * width)
                                .filter(|&index| cells[index])
                                .map(|index| {
                                    let x = index % width;
                                    let y = index / width;
                                    div()
                                        .absolute()
                                        .left(px(x as f32 * cell))
                                        .top(px(y as f32 * cell))
                                        .size(px(cell.max(1.)))
                                        .bg(t.text)
                                }),
                        ),
                    )
            }
            Err(err) => div().child(AntText::new(err).theme(t).tone(TextTone::Danger)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AntTourStep {
    title: SharedString,
    description: SharedString,
}
impl AntTourStep {
    pub fn new(title: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
        }
    }
}

pub fn tour_index(len: usize, current: usize, next: bool) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let current = current.min(len - 1);
    if next {
        (current + 1 < len).then_some(current + 1)
    } else {
        current.checked_sub(1)
    }
}

#[derive(IntoElement)]
pub struct AntTour {
    id: ElementId,
    steps: Vec<AntTourStep>,
    index: usize,
    open: bool,
    theme: AntTheme,
    target: Option<AnyElement>,
    on_change: Option<Rc<dyn Fn(usize, &mut Window, &mut App)>>,
    on_close: Option<Notify>,
}
impl AntTour {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            steps: Vec::new(),
            index: 0,
            open: true,
            theme: AntTheme::default(),
            target: None,
            on_change: None,
            on_close: None,
        }
    }
    pub fn step(mut self, step: AntTourStep) -> Self {
        self.steps.push(step);
        self
    }
    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    /// Element the step card anchors to. Popover measures it; there is no spotlight mask.
    pub fn target(mut self, target: impl IntoElement) -> Self {
        self.target = Some(target.into_any_element());
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
    pub fn on_close(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntTour {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let len = self.steps.len();
        if len == 0 {
            return div().id(self.id).children(self.target).into_any_element();
        }
        let index = self.index.min(len - 1);
        let step = self.steps[index].clone();
        let change = self.on_change.clone();
        let close = self.on_close.clone();
        let id = self.id.clone();
        let card = div()
            .w(px(280.))
            .p(px(12.))
            .rounded(px(t.border_radius))
            .bg(t.surface)
            .border_1()
            .border_color(t.primary)
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(AntText::new(step.title).theme(t))
            .child(
                AntText::new(step.description)
                    .theme(t)
                    .tone(TextTone::Secondary),
            )
            .child(
                div()
                    .flex()
                    .gap(px(8.))
                    .mt(px(4.))
                    .child(step_button(
                        format!("{id:?}-back"),
                        "Back",
                        index == 0,
                        t,
                        {
                            let change = change.clone();
                            move |window, cx| {
                                if let Some(next) = tour_index(len, index, false) {
                                    if let Some(cb) = &change {
                                        cb(next, window, cx);
                                    }
                                }
                            }
                        },
                    ))
                    .child(step_button(
                        format!("{id:?}-next"),
                        if index + 1 == len { "Done" } else { "Next" },
                        false,
                        t,
                        {
                            let change = change.clone();
                            let close = close.clone();
                            move |window, cx| {
                                if let Some(next) = tour_index(len, index, true) {
                                    if let Some(cb) = &change {
                                        cb(next, window, cx);
                                    }
                                } else if let Some(cb) = &close {
                                    cb(window, cx);
                                }
                            }
                        },
                    )),
            );
        let Some(target) = self.target else {
            if !self.open {
                return div().id(id).into_any_element();
            }
            return div().id(id).relative().child(card).into_any_element();
        };
        let on_close = self.on_close.clone();
        BasePopover::new(self.id)
            .anchor(dropdown_anchor())
            .offset(px(8.))
            .open(self.open)
            .overlay_closable(true)
            .when_some(on_close, |popover, callback| {
                popover.on_open_change(move |open, window, cx| {
                    if !open {
                        callback(window, cx);
                    }
                })
            })
            .trigger_with(move |_, _, _| target)
            .content(move |_, _, _| card)
            .into_any_element()
    }
}

fn step_button(
    id: String,
    label: &'static str,
    disabled: bool,
    theme: AntTheme,
    f: impl Fn(&mut Window, &mut App) + 'static,
) -> gpui_kit::base::Button {
    BaseButton::new(ElementId::Name(id.into()))
        .disabled(disabled)
        .on_click(move |_, window, cx| f(window, cx))
        .child(
            div()
                .px(px(8.))
                .py(px(4.))
                .rounded(px(theme.border_radius))
                .bg(if disabled {
                    theme.surface_secondary
                } else {
                    theme.primary
                })
                .text_color(if disabled {
                    theme.disabled_text
                } else {
                    theme.surface
                })
                .child(label),
        )
}

#[derive(Clone, Debug)]
pub struct AntListyItem {
    pub key: SharedString,
    pub title: SharedString,
    pub group: SharedString,
    pub disabled: bool,
}
impl AntListyItem {
    pub fn new(
        key: impl Into<SharedString>,
        title: impl Into<SharedString>,
        group: impl Into<SharedString>,
    ) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            group: group.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListySlot {
    Header(SharedString),
    Item {
        key: SharedString,
        title: SharedString,
        disabled: bool,
    },
}
impl ListySlot {
    pub fn height(&self) -> f32 {
        match self {
            Self::Header(_) => 28.,
            Self::Item { .. } => 36.,
        }
    }
}

pub fn listy_slots(items: &[AntListyItem]) -> Vec<ListySlot> {
    let mut out = Vec::new();
    let mut last = None;
    for item in items {
        if last.as_deref() != Some(item.group.as_ref()) {
            out.push(ListySlot::Header(item.group.clone()));
            last = Some(item.group.clone());
        }
        out.push(ListySlot::Item {
            key: item.key.clone(),
            title: item.title.clone(),
            disabled: item.disabled,
        });
    }
    out
}

/// Windowed list. Kit `VirtualList` measures the viewport; row chrome is ours.
pub fn ant_listy<V: Render>(
    view: Entity<V>,
    id: impl Into<ElementId>,
    slots: Vec<ListySlot>,
    render: impl Fn(&mut V, &ListySlot, &mut Window, &mut Context<V>) -> AnyElement + 'static,
) -> gpui_kit::base::VirtualList {
    let heights = Rc::new(
        slots
            .iter()
            .map(|slot| size(px(320.), px(slot.height())))
            .collect(),
    );
    let slots = Rc::new(slots);
    v_virtual_list(view, id, heights, move |this, range, window, cx| {
        range
            .filter_map(|index| slots.get(index).map(|slot| render(this, slot, window, cx)))
            .collect()
    })
    .h(px(220.))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UploadStatus {
    Ready,
    Uploading,
    Done,
    Error,
}

#[derive(Clone, Debug)]
pub struct AntUploadFile {
    pub name: SharedString,
    pub status: UploadStatus,
}
impl AntUploadFile {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            status: UploadStatus::Ready,
        }
    }
    pub fn status(mut self, status: UploadStatus) -> Self {
        self.status = status;
        self
    }
}

pub fn upload_accepted(name: &str, accept: &str) -> bool {
    if accept.trim().is_empty() {
        return true;
    }
    let lower = name.to_ascii_lowercase();
    accept
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .any(|part| {
            let part = part.trim_start_matches('.').to_ascii_lowercase();
            lower.rsplit('.').next() == Some(part.as_str()) || lower.ends_with(&format!(".{part}"))
        })
}

pub fn upload_without(files: &[AntUploadFile], name: &str) -> Vec<AntUploadFile> {
    files
        .iter()
        .filter(|file| file.name.as_ref() != name)
        .cloned()
        .collect()
}

#[derive(IntoElement)]
pub struct AntUpload {
    id: ElementId,
    accept: SharedString,
    files: Vec<AntUploadFile>,
    theme: AntTheme,
    on_pick: Option<Notify>,
    on_remove: Option<KeyNotify>,
}
impl AntUpload {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            accept: SharedString::from(""),
            files: Vec::new(),
            theme: AntTheme::default(),
            on_pick: None,
            on_remove: None,
        }
    }
    pub fn accept(mut self, accept: impl Into<SharedString>) -> Self {
        self.accept = accept.into();
        self
    }
    pub fn file(mut self, file: AntUploadFile) -> Self {
        self.files.push(file);
        self
    }
    pub fn files(mut self, files: impl IntoIterator<Item = AntUploadFile>) -> Self {
        self.files.extend(files);
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    /// The parent opens the platform file dialog. This button only requests it.
    pub fn on_pick(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_pick = Some(Rc::new(f));
        self
    }
    pub fn on_remove(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_remove = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntUpload {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let on_pick = self.on_pick.clone();
        let id = self.id.clone();
        div()
            .id(id.clone())
            .w_full()
            .flex()
            .flex_col()
            .gap(px(8.))
            .child(
                BaseButton::new(ElementId::Name(format!("{id:?}-pick").into()))
                    .on_click(move |_, window, cx| {
                        if let Some(cb) = &on_pick {
                            cb(window, cx);
                        }
                    })
                    .child(
                        div()
                            .px(px(12.))
                            .h(px(32.))
                            .rounded(px(t.border_radius))
                            .border_1()
                            .border_color(t.primary)
                            .text_color(t.primary)
                            .flex()
                            .items_center()
                            .gap(px(6.))
                            .child(AntIcon::new(AntIconName::UploadOutlined).theme(t).size(14.))
                            .child("Choose file"),
                    ),
            )
            .children(self.files.into_iter().map(|file| {
                let on_remove = self.on_remove.clone();
                let name = file.name.clone();
                let label = match file.status {
                    UploadStatus::Ready => "Ready",
                    UploadStatus::Uploading => "Uploading",
                    UploadStatus::Done => "Done",
                    UploadStatus::Error => "Error",
                };
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(8.))
                    .py(px(6.))
                    .border_1()
                    .border_color(t.border)
                    .rounded(px(t.border_radius))
                    .child(AntText::new(format!("{} · {label}", file.name)).theme(t))
                    .child(
                        BaseButton::new(ElementId::Name(format!("upload-remove-{name}").into()))
                            .on_click(move |_, window, cx| {
                                if let Some(cb) = &on_remove {
                                    cb(name.clone(), window, cx);
                                }
                            })
                            .child(AntIcon::new(AntIconName::DeleteOutlined).theme(t).size(14.)),
                    )
            }))
    }
}

pub fn mention_query(text: &str, cursor: usize) -> Option<&str> {
    let cursor = cursor.min(text.len());
    let head = &text[..cursor];
    let start = head.rfind([' ', '\n', '@'])?;
    let token = if head.as_bytes().get(start) == Some(&b'@') {
        &head[start..]
    } else {
        return None;
    };
    if token[1..]
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.')
    {
        Some(&token[1..])
    } else {
        None
    }
}

pub fn apply_mention(text: &str, cursor: usize, value: &str) -> String {
    let cursor = cursor.min(text.len());
    let Some(query) = mention_query(text, cursor) else {
        return text.to_owned();
    };
    let start = cursor - query.len() - 1;
    format!("{}@{value} {}", &text[..start], &text[cursor..])
}

#[derive(Clone, Debug)]
pub struct AntMentionOption {
    value: SharedString,
    label: SharedString,
}
impl AntMentionOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

pub fn mention_matches<'a>(
    options: &'a [AntMentionOption],
    query: &str,
) -> Vec<&'a AntMentionOption> {
    let query = query.to_ascii_lowercase();
    options
        .iter()
        .filter(|option| {
            query.is_empty()
                || option.value.to_ascii_lowercase().contains(&query)
                || option.label.to_ascii_lowercase().contains(&query)
        })
        .collect()
}

#[derive(IntoElement)]
pub struct AntMentions {
    id: ElementId,
    text: SharedString,
    cursor: usize,
    options: Vec<AntMentionOption>,
    theme: AntTheme,
    trigger: Option<AnyElement>,
    focus_handle: Option<FocusHandle>,
    on_select: Option<KeyNotify>,
}
impl AntMentions {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            text: SharedString::from(""),
            cursor: 0,
            options: Vec::new(),
            theme: AntTheme::default(),
            trigger: None,
            focus_handle: None,
            on_select: None,
        }
    }
    pub fn text(mut self, text: impl Into<SharedString>, cursor: usize) -> Self {
        self.text = text.into();
        self.cursor = cursor;
        self
    }
    pub fn option(mut self, option: AntMentionOption) -> Self {
        self.options.push(option);
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    /// Field the suggestion list anchors to. Without it, matches stay inline.
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }
    /// Keep keyboard input in the editor while suggestions are open.
    /// Pass the field's retained focus handle alongside `.trigger(...)`.
    pub fn track_focus(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
        self
    }
    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntMentions {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = self.theme;
        let query = mention_query(self.text.as_ref(), self.cursor).map(str::to_owned);
        let matches = query
            .as_deref()
            .map(|query| mention_matches(&self.options, query))
            .unwrap_or_default();
        let list = div()
            .min_w(px(180.))
            .border_1()
            .border_color(t.border)
            .rounded(px(t.border_radius))
            .bg(t.surface)
            .children(matches.into_iter().map(|option| {
                let on_select = self.on_select.clone();
                let value = option.value.clone();
                BaseButton::new(ElementId::Name(format!("mention-{value}").into()))
                    .on_click(move |_, window, cx| {
                        if let Some(cb) = &on_select {
                            cb(value.clone(), window, cx);
                        }
                    })
                    .child(
                        div()
                            .px(px(10.))
                            .py(px(6.))
                            .w_full()
                            .child(AntText::new(option.label.clone()).theme(t)),
                    )
            }));
        let anchor_id = SharedString::from(format!("mentions-anchor-{:?}", self.id));
        let dismissed = window.use_keyed_state(
            SharedString::from(format!("mentions-dismiss-{:?}", self.id)),
            cx,
            |_, _| (None::<String>, false),
        );
        // This is transient popup state, keyed by component identity. A changed
        // query starts a fresh suggestion session, including returning to an
        // earlier query after clearing the field.
        dismissed.update(cx, |state, _| {
            if state.0 != query {
                *state = (query.clone(), false);
            }
        });
        let suppressed = dismissed.read(cx).1;
        let Some(trigger) = self.trigger else {
            return div()
                .id(self.id)
                .w_full()
                .flex()
                .flex_col()
                .gap(px(4.))
                .when(query.is_some(), |panel| panel.child(list))
                .into_any_element();
        };
        // The field stays outside the trigger. Popover mouse-down toggles and
        // stops propagation, which would steal caret clicks from InputState.
        // Open is derived from the query, so a dismiss has to be remembered or
        // the next frame forces the popup open again.
        let dismiss_from_editor = dismissed.clone();
        let visible = query.is_some() && !suppressed;
        div()
            .w_full()
            .flex()
            .flex_col()
            .on_key_down(move |event, _, cx| {
                if visible && event.keystroke.key == "escape" {
                    dismiss_from_editor.update(cx, |state, cx| {
                        state.1 = true;
                        cx.notify();
                    });
                    cx.stop_propagation();
                }
            })
            .child(trigger)
            .child(
                BasePopover::new(self.id)
                    .anchor(dropdown_anchor())
                    .offset(px(4.))
                    .open(visible)
                    .when_some(self.focus_handle, |popover, handle| {
                        popover.track_focus(&handle)
                    })
                    .overlay_closable(true)
                    .on_open_change(move |open, _, cx| {
                        if !*open {
                            dismissed.update(cx, |state, cx| {
                                state.1 = true;
                                cx.notify();
                            });
                        }
                    })
                    .trigger_with(move |_, _, _| {
                        div().id(anchor_id).w_full().h(px(0.)).into_any_element()
                    })
                    .content(move |_, _, _| list),
            )
            .into_any_element()
    }
}

pub fn parse_hex(input: &str) -> Option<u32> {
    let hex = input.trim().trim_start_matches('#');
    if hex.len() != 6 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return None;
    }
    u32::from_str_radix(hex, 16).ok()
}

pub fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> u32 {
    let hue = hue.rem_euclid(360.);
    let saturation = saturation.clamp(0., 1.);
    let value = value.clamp(0., 1.);
    let c = value * saturation;
    let x = c * (1. - ((hue / 60.) % 2. - 1.).abs());
    let m = value - c;
    let (r, g, b) = match hue {
        h if h < 60. => (c, x, 0.),
        h if h < 120. => (x, c, 0.),
        h if h < 180. => (0., c, x),
        h if h < 240. => (0., x, c),
        h if h < 300. => (x, 0., c),
        _ => (c, 0., x),
    };
    let channel = |n: f32| ((n + m) * 255.).round() as u32;
    (channel(r) << 16) | (channel(g) << 8) | channel(b)
}

#[derive(IntoElement)]
pub struct AntColorPicker {
    id: ElementId,
    color: u32,
    open: bool,
    theme: AntTheme,
    on_change: Option<Rc<dyn Fn(u32, &mut Window, &mut App)>>,
    on_open: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
}
impl AntColorPicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            color: 0x1677ff,
            open: false,
            theme: AntTheme::default(),
            on_change: None,
            on_open: None,
        }
    }
    pub fn color(mut self, color: u32) -> Self {
        self.color = color & 0xffffff;
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, f: impl Fn(u32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
    pub fn on_open_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntColorPicker {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let presets = [0x1677ff, 0x52c41a, 0xfaad14, 0xff4d4f, 0x722ed1, 0x13c2c2];
        let on_change = self.on_change.clone();
        let on_open = self.on_open.clone();
        let color = self.color;
        let close_on_pick = on_open.clone();
        BasePopover::new(self.id.clone())
            .anchor(dropdown_anchor())
            .offset(px(4.))
            .open(self.open)
            .overlay_closable(true)
            .when_some(on_open, |popover, callback| {
                popover.on_open_change(move |open, window, cx| callback(*open, window, cx))
            })
            .trigger_with({
                let id = self.id.clone();
                move |_, _, _| {
                    div()
                        .id(SharedString::from(format!("color-trigger-{id:?}")))
                        .size(px(32.))
                        .rounded(px(t.border_radius))
                        .bg(crate::color(color))
                        .border_1()
                        .border_color(t.border)
                        .into_any_element()
                }
            })
            .content(move |_, _, _| {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .p(px(8.))
                    .border_1()
                    .border_color(t.border)
                    .rounded(px(t.border_radius))
                    .bg(t.surface)
                    .child(
                        div()
                            .flex()
                            .gap(px(6.))
                            .children(presets.into_iter().map(|preset| {
                                let cb = on_change.clone();
                                let close = close_on_pick.clone();
                                BaseButton::new(ElementId::Name(
                                    format!("color-{preset:06x}").into(),
                                ))
                                .on_click(move |_, window, cx| {
                                    if let Some(cb) = &cb {
                                        cb(preset, window, cx);
                                    }
                                    if let Some(close) = &close {
                                        close(false, window, cx);
                                    }
                                })
                                .child(
                                    div()
                                        .size(px(22.))
                                        .rounded(px(4.))
                                        .bg(crate::color(preset))
                                        .border_1()
                                        .border_color(if preset == color {
                                            t.primary
                                        } else {
                                            t.border
                                        }),
                                )
                            })),
                    )
                    .child(
                        AntText::new(format!("#{color:06x}"))
                            .theme(t)
                            .tone(TextTone::Secondary),
                    )
            })
    }
}

#[derive(Clone, Debug)]
pub struct AntCascaderOption {
    value: SharedString,
    label: SharedString,
    disabled: bool,
    children: Vec<AntCascaderOption>,
}
impl AntCascaderOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
            children: Vec::new(),
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn child(mut self, child: AntCascaderOption) -> Self {
        self.children.push(child);
        self
    }
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

pub fn cascader_column<'a>(
    options: &'a [AntCascaderOption],
    path: &[String],
    depth: usize,
) -> &'a [AntCascaderOption] {
    let mut column = options;
    for key in path.iter().take(depth) {
        match column.iter().find(|option| option.value.as_ref() == key) {
            Some(option) => column = &option.children,
            None => return &[],
        }
    }
    column
}

pub fn cascader_labels(options: &[AntCascaderOption], path: &[String]) -> Vec<String> {
    let mut labels = Vec::new();
    let mut column = options;
    for key in path {
        let Some(option) = column.iter().find(|option| option.value.as_ref() == key) else {
            break;
        };
        labels.push(option.label.to_string());
        column = &option.children;
    }
    labels
}

pub fn cascader_select(path: &[String], depth: usize, value: &str, leaf: bool) -> Vec<String> {
    let mut next: Vec<String> = path.iter().take(depth).cloned().collect();
    next.push(value.to_owned());
    if !leaf { next } else { next }
}

#[derive(IntoElement)]
pub struct AntCascader {
    id: ElementId,
    options: Vec<AntCascaderOption>,
    path: Vec<String>,
    open: bool,
    placeholder: SharedString,
    theme: AntTheme,
    on_change: Option<Rc<dyn Fn(Vec<String>, bool, &mut Window, &mut App)>>,
    on_open: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
}
impl AntCascader {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            options: Vec::new(),
            path: Vec::new(),
            open: false,
            placeholder: "Select".into(),
            theme: AntTheme::default(),
            on_change: None,
            on_open: None,
        }
    }
    pub fn option(mut self, option: AntCascaderOption) -> Self {
        self.options.push(option);
        self
    }
    pub fn path(mut self, path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.path = path.into_iter().map(Into::into).collect();
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(
        mut self,
        f: impl Fn(Vec<String>, bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
    pub fn on_open_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntCascader {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let label = if self.path.is_empty() {
            self.placeholder.clone()
        } else {
            let labels = cascader_labels(&self.options, &self.path);
            if labels.is_empty() {
                self.placeholder.clone()
            } else {
                SharedString::from(labels.join(" / "))
            }
        };
        let on_open = self.on_open.clone();
        let id = self.id.clone();
        let options = self.options;
        let path = self.path;
        let on_change = self.on_change;
        BasePopover::new(self.id)
            .anchor(dropdown_anchor())
            .offset(px(4.))
            .open(self.open)
            .overlay_closable(true)
            .when_some(on_open.clone(), |popover, callback| {
                popover.on_open_change(move |open, window, cx| callback(*open, window, cx))
            })
            .trigger_with(move |_, _, _| {
                picker_trigger(format!("cascader-trigger-{id:?}"), label, t)
            })
            .content(move |_, _, _| {
                let depths = path.len() + 1;
                div()
                    .flex()
                    .border_1()
                    .border_color(t.border)
                    .rounded(px(t.border_radius))
                    .bg(t.surface)
                    .children((0..depths).filter_map(|depth| {
                        let column = cascader_column(&options, &path, depth);
                        if column.is_empty() {
                            return None;
                        }
                        let path = path.clone();
                        let on_change = on_change.clone();
                        let on_open = on_open.clone();
                        Some(
                            div()
                                .w(px(140.))
                                .flex()
                                .flex_col()
                                .border_r_1()
                                .border_color(t.border)
                                .children(column.iter().map(|option| {
                                    let value = option.value.to_string();
                                    let leaf = option.children.is_empty();
                                    let chosen = path.get(depth).map(String::as_str)
                                        == Some(option.value.as_ref());
                                    let cb = on_change.clone();
                                    let close = on_open.clone();
                                    let path = path.clone();
                                    BaseButton::new(ElementId::Name(
                                        format!("cascader-{depth}-{value}").into(),
                                    ))
                                    .disabled(option.disabled)
                                    .on_click(move |_, window, cx| {
                                        let next = cascader_select(&path, depth, &value, leaf);
                                        if let Some(cb) = &cb {
                                            cb(next, leaf, window, cx);
                                        }
                                        if leaf {
                                            if let Some(close) = &close {
                                                close(false, window, cx);
                                            }
                                        }
                                    })
                                    .child(
                                        div()
                                            .px(px(10.))
                                            .py(px(6.))
                                            .w_full()
                                            .bg(if chosen { t.primary_bg } else { t.surface })
                                            .text_color(if option.disabled {
                                                t.disabled_text
                                            } else {
                                                t.text
                                            })
                                            .child(option.label.clone()),
                                    )
                                })),
                        )
                    }))
            })
    }
}

pub fn transfer_move(
    source: &BTreeSet<String>,
    target: &BTreeSet<String>,
    selected: &BTreeSet<String>,
    to_target: bool,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut source = source.clone();
    let mut target = target.clone();
    for key in selected {
        if to_target && source.remove(key) {
            target.insert(key.clone());
        }
        if !to_target && target.remove(key) {
            source.insert(key.clone());
        }
    }
    (source, target)
}

#[derive(Clone, Debug)]
pub struct AntTransferItem {
    key: SharedString,
    title: SharedString,
    disabled: bool,
}
impl AntTransferItem {
    pub fn new(key: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }
}

#[derive(IntoElement)]
pub struct AntTransfer {
    id: ElementId,
    items: Vec<AntTransferItem>,
    target: BTreeSet<String>,
    selected: BTreeSet<String>,
    theme: AntTheme,
    on_change: Option<Rc<dyn Fn(BTreeSet<String>, &mut Window, &mut App)>>,
    on_select: Option<Rc<dyn Fn(String, bool, &mut Window, &mut App)>>,
}
impl AntTransfer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            target: BTreeSet::new(),
            selected: BTreeSet::new(),
            theme: AntTheme::default(),
            on_change: None,
            on_select: None,
        }
    }
    pub fn item(mut self, item: AntTransferItem) -> Self {
        self.items.push(item);
        self
    }
    pub fn target_keys(mut self, keys: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.target = keys.into_iter().map(Into::into).collect();
        self
    }
    pub fn selected_keys(mut self, keys: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.selected = keys.into_iter().map(Into::into).collect();
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(
        mut self,
        f: impl Fn(BTreeSet<String>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
    pub fn on_select(mut self, f: impl Fn(String, bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntTransfer {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let source: BTreeSet<String> = self
            .items
            .iter()
            .map(|item| item.key.to_string())
            .filter(|key| !self.target.contains(key))
            .collect();
        let target = self.target.clone();
        let selected = self.selected.clone();
        let on_change = self.on_change.clone();
        let source_right = source.clone();
        let target_right = target.clone();
        let selected_right = selected.clone();
        let source_left = source.clone();
        let target_left = target.clone();
        let selected_left = selected.clone();
        let on_change_left = on_change.clone();
        div()
            .id(self.id.clone())
            .flex()
            .gap(px(8.))
            .items_start()
            .child(transfer_list(
                "source",
                &self.items,
                &source,
                &self.selected,
                t,
                self.on_select.clone(),
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.))
                    .pt(px(28.))
                    .child(transfer_move_button(
                        format!("{:?}-right", self.id),
                        ">",
                        t,
                        {
                            let on_change = on_change.clone();
                            move |window, cx| {
                                let (_source, next) = transfer_move(
                                    &source_right,
                                    &target_right,
                                    &selected_right,
                                    true,
                                );
                                if let Some(cb) = &on_change {
                                    cb(next, window, cx);
                                }
                            }
                        },
                    ))
                    .child(transfer_move_button(
                        format!("{:?}-left", self.id),
                        "<",
                        t,
                        {
                            move |window, cx| {
                                let (_source, next) = transfer_move(
                                    &source_left,
                                    &target_left,
                                    &selected_left,
                                    false,
                                );
                                if let Some(cb) = &on_change_left {
                                    cb(next, window, cx);
                                }
                            }
                        },
                    )),
            )
            .child(transfer_list(
                "target",
                &self.items,
                &self.target,
                &self.selected,
                t,
                self.on_select,
            ))
    }
}

fn transfer_list(
    side: &str,
    items: &[AntTransferItem],
    keys: &BTreeSet<String>,
    selected: &BTreeSet<String>,
    theme: AntTheme,
    on_select: Option<Rc<dyn Fn(String, bool, &mut Window, &mut App)>>,
) -> gpui_kit::Div {
    div()
        .w(px(160.))
        .border_1()
        .border_color(theme.border)
        .rounded(px(theme.border_radius))
        .children(
            items
                .iter()
                .filter(|item| keys.contains(item.key.as_ref()))
                .map(|item| {
                    let key = item.key.to_string();
                    let checked = selected.contains(&key);
                    let cb = on_select.clone();
                    BaseButton::new(ElementId::Name(format!("transfer-{side}-{key}").into()))
                        .disabled(item.disabled)
                        .on_click(move |_, window, cx| {
                            if let Some(cb) = &cb {
                                cb(key.clone(), !checked, window, cx);
                            }
                        })
                        .child(
                            div()
                                .px(px(8.))
                                .py(px(6.))
                                .w_full()
                                .bg(if checked {
                                    theme.primary_bg
                                } else {
                                    theme.surface
                                })
                                .text_color(if item.disabled {
                                    theme.disabled_text
                                } else {
                                    theme.text
                                })
                                .child(item.title.clone()),
                        )
                }),
        )
}

fn transfer_move_button(
    id: String,
    label: &'static str,
    theme: AntTheme,
    f: impl Fn(&mut Window, &mut App) + 'static,
) -> gpui_kit::base::Button {
    BaseButton::new(ElementId::Name(id.into()))
        .on_click(move |_, window, cx| f(window, cx))
        .child(
            div()
                .size(px(28.))
                .rounded(px(theme.border_radius))
                .border_1()
                .border_color(theme.border)
                .flex()
                .items_center()
                .justify_center()
                .child(label),
        )
}

pub fn tree_select_title(nodes: &[AntTreeNode], key: &str) -> Option<SharedString> {
    fn walk(nodes: &[AntTreeNode], key: &str) -> Option<SharedString> {
        for node in nodes {
            if node.key() == key {
                return Some(node.title().clone());
            }
            if let Some(found) = walk(node.child_nodes(), key) {
                return Some(found);
            }
        }
        None
    }
    walk(nodes, key)
}

#[derive(IntoElement)]
pub struct AntTreeSelect {
    id: ElementId,
    nodes: Vec<AntTreeNode>,
    open: bool,
    selected: Option<SharedString>,
    theme: AntTheme,
    on_open: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    on_select: Option<KeyNotify>,
}
impl AntTreeSelect {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            nodes: Vec::new(),
            open: false,
            selected: None,
            theme: AntTheme::default(),
            on_open: None,
            on_select: None,
        }
    }
    pub fn node(mut self, node: AntTreeNode) -> Self {
        self.nodes.push(node);
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn selected(mut self, key: impl Into<SharedString>) -> Self {
        self.selected = Some(key.into());
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_open(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open = Some(Rc::new(f));
        self
    }
    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(f));
        self
    }
}
impl RenderOnce for AntTreeSelect {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let label = self
            .selected
            .as_ref()
            .and_then(|key| tree_select_title(&self.nodes, key))
            .unwrap_or_else(|| SharedString::from("Select"));
        let on_open = self.on_open.clone();
        let on_select = self.on_select.clone();
        let id = self.id.clone();
        BasePopover::new(self.id)
            .anchor(dropdown_anchor())
            .offset(px(4.))
            .open(self.open)
            .overlay_closable(true)
            .when_some(on_open.clone(), |popover, callback| {
                popover.on_open_change(move |open, window, cx| callback(*open, window, cx))
            })
            .trigger_with({
                let id = id.clone();
                move |_, _, _| picker_trigger(format!("tree-select-trigger-{id:?}"), label, t)
            })
            .content(move |_, _, _| {
                div()
                    .w(px(280.))
                    .border_1()
                    .border_color(t.border)
                    .rounded(px(t.border_radius))
                    .bg(t.surface)
                    .p(px(4.))
                    .child(
                        AntTree::new(ElementId::Name(format!("{id:?}-tree").into()))
                            .theme(t)
                            .children(self.nodes)
                            .selected_key(self.selected.clone().unwrap_or_default())
                            .on_select(move |key, window, cx| {
                                if let Some(cb) = &on_select {
                                    cb(key, window, cx);
                                }
                                if let Some(close) = &on_open {
                                    close(false, window, cx);
                                }
                            }),
                    )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_navigation_skips_disabled_and_wraps() {
        let enabled = [true, false, true];
        assert_eq!(next_anchor_index(&enabled, 0, "down"), Some(2));
        assert_eq!(next_anchor_index(&enabled, 2, "down"), Some(0));
        assert_eq!(next_anchor_index(&enabled, 2, "home"), Some(0));
    }

    #[test]
    fn carousel_wraps_both_directions() {
        assert_eq!(carousel_index(3, 0, -1), 2);
        assert_eq!(carousel_index(3, 2, 1), 0);
        assert_eq!(carousel_index(0, 4, 1), 0);
    }

    #[test]
    fn masonry_round_robin_keeps_every_index() {
        let columns = masonry_columns(5, 2);
        assert_eq!(columns, vec![vec![0, 2, 4], vec![1, 3]]);
    }

    #[test]
    fn qr_matrix_has_finder_corners() {
        let matrix = qr_matrix("gpui-antd").unwrap();
        assert!(matrix.width >= 21);
        assert_eq!(matrix.cells.len(), matrix.width * matrix.width);
        assert!(matrix.dark(0, 0));
        assert!(qr_matrix("").is_err());
    }

    #[test]
    fn tour_stops_at_ends() {
        assert_eq!(tour_index(3, 0, false), None);
        assert_eq!(tour_index(3, 2, true), None);
        assert_eq!(tour_index(3, 1, true), Some(2));
    }

    #[test]
    fn mentions_replace_the_active_token() {
        assert_eq!(mention_query("hi @al", 6), Some("al"));
        assert_eq!(mention_query("hi ", 3), None);
        assert_eq!(apply_mention("hi @al", 6, "alice"), "hi @alice ");
    }

    #[test]
    fn hex_and_hsv_are_stable() {
        assert_eq!(parse_hex("#1677FF"), Some(0x1677ff));
        assert_eq!(parse_hex("gg0000"), None);
        assert_eq!(hsv_to_rgb(0., 1., 1.), 0xff0000);
        assert_eq!(hsv_to_rgb(120., 1., 1.), 0x00ff00);
    }

    #[test]
    fn cascader_column_follows_the_path() {
        let options = vec![
            AntCascaderOption::new("zj", "Zhejiang")
                .child(AntCascaderOption::new("hz", "Hangzhou")),
        ];
        assert_eq!(
            cascader_column(&options, &["zj".into()], 1)[0].value(),
            "hz"
        );
        assert!(cascader_column(&options, &["missing".into()], 1).is_empty());
        assert_eq!(
            cascader_labels(&options, &["zj".into(), "hz".into()]),
            vec!["Zhejiang".to_owned(), "Hangzhou".to_owned()]
        );
        assert!(cascader_labels(&options, &["missing".into()]).is_empty());
        assert_eq!(
            cascader_select(&["zj".into()], 0, "js", false),
            vec!["js".to_owned()]
        );
    }

    #[test]
    fn transfer_moves_only_selected_keys() {
        let source = BTreeSet::from(["a".into(), "b".into()]);
        let target = BTreeSet::new();
        let selected = BTreeSet::from(["a".into()]);
        let (source, target) = transfer_move(&source, &target, &selected, true);
        assert!(!source.contains("a"));
        assert!(target.contains("a"));
        assert!(source.contains("b"));
    }

    #[test]
    fn upload_filter_and_remove() {
        assert!(upload_accepted("photo.PNG", ".png,.jpg"));
        assert!(!upload_accepted("notes.txt", ".png"));
        assert!(upload_accepted("any", ""));
        let files = vec![AntUploadFile::new("a.png"), AntUploadFile::new("b.png")];
        assert_eq!(upload_without(&files, "a.png").len(), 1);
    }

    #[test]
    fn listy_inserts_a_header_when_the_group_changes() {
        let items = vec![
            AntListyItem::new("1", "One", "A"),
            AntListyItem::new("2", "Two", "A"),
            AntListyItem::new("3", "Three", "B"),
        ];
        let slots = listy_slots(&items);
        assert_eq!(slots.len(), 5);
        assert!(matches!(slots[0], ListySlot::Header(_)));
        assert!(matches!(slots[3], ListySlot::Header(_)));
    }

    #[test]
    fn watermark_count_never_returns_zero() {
        assert_eq!(watermark_count(10., 10., 4.), 1);
        assert!(watermark_count(200., 100., 50.) >= 4);
    }
}
