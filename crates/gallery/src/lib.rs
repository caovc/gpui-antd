//! One set of real Rust/GPUI previews for desktop and browser WebAssembly.
extern crate gpui_kit as gpui;
mod theme_lab;
use theme_lab::ThemeLabSpec;

use std::{borrow::Cow, collections::{BTreeMap, BTreeSet}, rc::Rc};

use gpui_antd::{
    AntAlert, AntBadge, AntButton, AntCard, AntCheckbox, AntDivider,
    AntInput, AntProgress, AntRadio, AntRadioGroup, AntSelect, AntDatePicker, AntForm, AntFormItem, FormSchema, FormRule, AntSwitch, AntTag, AntTheme, ControlSize, InputStatus, InputVariant, Tone,
    AntIcon, AntIconName, AntIconAssets, AntTitle, AntText, AntParagraph, AntLink, AntTypography, TextTone,
    AntSpace, AntFlex, AntRow, AntCol, SpaceSize, FlexAlign, FlexJustify,
    AntAvatar, AvatarShape, AvatarSize, AntStatistic, AntEmpty, AntBreadcrumb, AntBreadcrumbItem,
    AntTabs, AntTab, TabsVariant, AntTooltip,
    AntLayout, AntSider, AntMobileDrawer, AntHeader, AntContent, AntFooter, GridBreakpoint,
    AntDropdown, AntDropdownItem, AntPopover, AntModal, AntPopconfirm,
    AntMenu, AntMenuItem, AntTokenLayer, AntTokenValue, AntResolvedToken,
    AntPagination, AntSteps, AntStep, StepStatus, AntRate, AntCollapse, AntCollapseItem,
    AntDescriptions, AntDescriptionItem, AntResult, ResultStatus, AntSkeleton, AntSpin,
    AntInputNumber, AntSlider, AntSegmented, AntSegmentedOption, AntTimePicker, AntAutoComplete,
    AntDrawer, DrawerPlacement, AntMessage, AntNotification, NoticeTone,
    AntTree, AntTreeNode, AntTable, AntTableColumn, AntTableRow, TableSort,
    AntCalendar, AntFloatButton, AntFloatButtonGroup, AntFloatAction, FloatShape, FloatVariant,
    AntTimeline, AntTimelineItem, TimelineMode, TimelineTone,
    AntList, AntListItem, ListSize,
};
use gpui_kit::{
    AppContext as _, Context, Entity, InteractiveElement as _, IntoElement, ParentElement as _, Render,
    StatefulInteractiveElement as _, Styled as _, Window, WindowOptions, div, px,
    AssetSource, SharedString, Subscription,
};
use gpui_kit::component::input::{InputState, InputEvent};
use gpui_kit::component::slider::{SliderState, SliderEvent};
use gpui_kit::component::time_field::{TimeFieldState, HourCycle, TimeFieldEvent};
use gpui_kit::component::select::{SelectState, SearchableVec};
use gpui_kit::component::date_picker::DatePickerState;
use gpui_kit::component::calendar::{CalendarState, CalendarEvent, Date};
use gpui_kit::component::time_field::TimePrecision;
use gpui_kit::component::IndexPath;
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::WindowExt as _;
use gpui_kit::assets::Assets;
use gpui_kit::base::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
#[cfg(target_family = "wasm")]
use gpui_kit::App;

/// Retains GPUI Kit's default component icon source while also resolving
/// official Ant Design SVGs from embedded Rust assets. On WASM, GPUI Kit's
/// existing async icon loader is kept only for non-Ant paths.
#[derive(Default)]
struct GalleryAssets {
    #[cfg(target_family = "wasm")]
    fallback: Option<Assets>,
}
impl AssetSource for GalleryAssets {
    fn load(&self, path: &str) -> gpui_kit::Result<Option<Cow<'static, [u8]>>> {
        if path.starts_with("antd-icons/") {
            return AntIconAssets.load(path);
        }
        #[cfg(not(target_family = "wasm"))]
        { Assets.load(path) }
        #[cfg(target_family = "wasm")]
        { self.fallback.as_ref().expect("WASM icons source").load(path) }
    }
    fn list(&self, prefix: &str) -> gpui_kit::Result<Vec<SharedString>> {
        let mut icons = AntIconAssets.list(prefix)?;
        #[cfg(not(target_family = "wasm"))]
        icons.extend(Assets.list(prefix)?);
        #[cfg(target_family = "wasm")]
        icons.extend(self.fallback.as_ref().expect("WASM icons source").list(prefix)?);
        icons.sort();
        icons.dedup();
        Ok(icons)
    }
}
impl GalleryAssets {
    fn new() -> Self {
        Self {
            #[cfg(target_family = "wasm")]
            fallback: Some(Assets::new("https://gpui-kit.com/gallery/")),
        }
    }
}

pub const COMPONENTS: &[&str] = &[
    "Button", "Input", "Select", "DatePicker", "Form", "Checkbox", "Radio", "Switch", "Tag", "Badge",
    "Alert", "Card", "Progress", "Divider", "Pagination", "Steps", "Rate", "Collapse", "Descriptions", "Result", "Skeleton", "Spin", "Tabs", "Tooltip", "Icons", "Layout",
    "Dropdown", "Popover", "Modal", "Popconfirm", "Menu", "Typography", "Space", "Flex", "Grid", "Avatar", "Statistic", "Empty", "Breadcrumb", "Drawer", "Message", "Notification", "Tree", "Table", "Timeline", "List", "Calendar", "FloatButton", "InputNumber", "Slider", "Segmented", "TimePicker", "AutoComplete",
];

/// Options in the browser icon inspector are only display parameters.
/// State and icons stay in native Rust/GPUI; the docs DOM never paints them.
#[derive(Debug, Clone, PartialEq)]
struct IconPreview {
    name: String,
    size: f32,
    primary: Option<u32>,
    secondary: Option<u32>,
    spin: bool,
}

impl IconPreview {
    fn parse(raw: &str) -> Self {
        let mut parts = raw.split('|');
        let mut preview = Self {
            name: parts.next().unwrap_or_default().to_owned(),
            size: 64.,
            primary: None,
            secondary: None,
            spin: false,
        };
        for part in parts {
            let Some((key, value)) = part.split_once('=') else { continue; };
            match key {
                "size" => {
                    if let Ok(px) = value.parse::<f32>() {
                        if px.is_finite() {
                            preview.size = px.clamp(16., 128.);
                        }
                    }
                }
                "spin" => preview.spin = value == "1",
                "primary" | "secondary" if value.len() == 6 && value.is_ascii()
                    && value.bytes().all(|byte| byte.is_ascii_hexdigit()) => {
                    if let Ok(rgb) = u32::from_str_radix(value, 16) {
                        if key == "primary" {
                            preview.primary = Some(rgb);
                        } else {
                            preview.secondary = Some(rgb);
                        }
                    }
                }
                _ => {}
            }
        }
        preview
    }
}

pub struct Gallery {
    component: String,
    dark: bool,
    theme_override: Option<AntTheme>,
    button_clicks: usize,
    checked: bool,
    radio_selected: Option<usize>,
    radio_button_selected: Option<usize>,
    switch_on: bool,
    input: Entity<InputState>,
    input_filled: Entity<InputState>,
    input_borderless: Entity<InputState>,
    input_decorated: Entity<InputState>,
    input_large: Entity<InputState>,
    input_error: Entity<InputState>,
    input_password: Entity<InputState>,
    input_disabled: Entity<InputState>,
    select_basic: Entity<SelectState<Vec<&'static str>>>,
    select_search: Entity<SelectState<SearchableVec<&'static str>>>,
    select_disabled: Entity<SelectState<Vec<&'static str>>>,
    date_single: Entity<DatePickerState>,
    date_range: Entity<DatePickerState>,
    date_time: Entity<DatePickerState>,
    date_disabled: Entity<DatePickerState>,
    form_name: Entity<InputState>,
    form_email: Entity<InputState>,
    form_password: Entity<InputState>,
    form_confirm: Entity<InputState>,
    form_errors: BTreeMap<String, String>,
    form_attempted: bool,
    form_success: bool,
    paragraph_expanded: bool,
    tabs_index: usize,
    tabs_card_index: usize,
    layout_collapsed: bool,
    layout_mobile_open: bool,
    layout_selected: String,
    menu_selected: String,
    menu_open_keys: BTreeSet<String>,
    layout_menu_open_keys: BTreeSet<String>,
    dropdown_selected: String,
    popover_open: bool,
    modal_result: String,
    popconfirm_result: String,
    pagination_page: usize,
    steps_current: usize,
    rating: usize,
    collapse_keys: BTreeSet<String>,
    drawer_closed:usize,
    tree_selected:String,
    tree_expanded:BTreeSet<String>,
    tree_checked:BTreeSet<String>,
    table_sort_key:String,
    table_sort:TableSort,
    table_selected:BTreeSet<String>,
    table_page:usize,
    table_filters:BTreeMap<String,String>,
    timeline_reversed:bool,
    number_basic:Entity<InputState>,
    number_decimal:Entity<InputState>,
    number_disabled:Entity<InputState>,
    slider_single:Entity<SliderState>,
    slider_range:Entity<SliderState>,
    slider_vertical:Entity<SliderState>,
    slider_value:f32,
    time_basic:Entity<TimeFieldState>,
    time_seconds:Entity<TimeFieldState>,
    time_12:Entity<TimeFieldState>,
    time_disabled:Entity<TimeFieldState>,
    segmented_index:usize,
    autocomplete_input:Entity<InputState>,
    autocomplete_open:bool,
    autocomplete_active:usize,
    autocomplete_last:String,
    _input_subscriptions:Vec<Subscription>,
    calendar_single:Entity<CalendarState>,
    calendar_range:Entity<CalendarState>,
    calendar_restricted:Entity<CalendarState>,
    calendar_selected:String,
    calendar_range_selected:String,
    _calendar_subscriptions:Vec<Subscription>,
    float_group_open:bool,
    float_clicks:usize,
    float_last_action:String,
}

impl Gallery {
    pub fn new(component: String, dark: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx)
            .placeholder("Type your message..."));
        let input_filled = cx.new(|cx| InputState::new(window, cx)
            .placeholder("Filled input"));
        let input_borderless = cx.new(|cx| InputState::new(window, cx)
            .placeholder("Borderless input"));
        let input_decorated = cx.new(|cx| InputState::new(window, cx)
            .placeholder("Username"));
        let input_large = cx.new(|cx| InputState::new(window, cx)
            .placeholder("Large input"));
        let input_error = cx.new(|cx| InputState::new(window, cx)
            .placeholder("Invalid email address"));
        let input_password = cx.new(|cx| InputState::new(window, cx)
            .masked(true).placeholder("Password"));
        let input_disabled = cx.new(|cx| InputState::new(window, cx)
            .placeholder("Disabled input").default_value("Cannot edit"));
        let select_basic = cx.new(|cx| SelectState::new(
            vec!["Apple", "Orange", "Banana", "Pineapple"],
            Some(IndexPath::default()), window, cx));
        let select_search = cx.new(|cx| SelectState::new(
            SearchableVec::new(vec!["Rust", "TypeScript", "Go", "Python", "Java"]),
            None, window, cx).searchable(true));
        let select_disabled = cx.new(|cx| SelectState::new(
            vec!["Enabled", "Disabled"],
            Some(IndexPath::default()), window, cx));
        let date_single = cx.new(|cx| DatePickerState::new(window, cx));
        let date_range = cx.new(|cx| DatePickerState::range(window, cx));
        let date_time = cx.new(|cx| DatePickerState::new(window, cx)
            .time_precision(TimePrecision::Minute));
        let date_disabled = cx.new(|cx| DatePickerState::new(window, cx));
        let form_name = cx.new(|cx| InputState::new(window, cx)
            .placeholder("Full name"));
        let form_email = cx.new(|cx| InputState::new(window, cx)
            .placeholder("name@example.com"));
        let form_password = cx.new(|cx| InputState::new(window, cx)
            .masked(true).placeholder("6 or more characters"));
        let form_confirm = cx.new(|cx| InputState::new(window, cx)
            .masked(true).placeholder("Repeat password"));
        let number_basic=cx.new(|cx|InputState::new(window,cx)
            .placeholder("Quantity").default_value("25").min(0.).max(100.).step(5.));
        let number_decimal=cx.new(|cx|InputState::new(window,cx)
            .placeholder("Amount").default_value("12.5").min(0.).max(200.).step(0.25));
        let number_disabled=cx.new(|cx|InputState::new(window,cx)
            .default_value("42").step(1.));
        let slider_single=cx.new(|_|SliderState::new().min(0.).max(100.)
            .step(5.).default_value(35.));
        let slider_range=cx.new(|_|SliderState::new().min(0.).max(100.)
            .step(1.).default_value(20. .. 80.));
        let slider_vertical=cx.new(|_|SliderState::new().min(0.).max(100.)
            .step(10.).default_value(60.));
        let time_basic=cx.new(|cx|TimeFieldState::new(window,cx));
        let time_seconds=cx.new(|cx|TimeFieldState::new(window,cx)
            .precision(TimePrecision::Second));
        let time_12=cx.new(|cx|TimeFieldState::new(window,cx)
            .hour_cycle(HourCycle::H12));
        let time_disabled=cx.new(|cx|TimeFieldState::new(window,cx));
        let autocomplete_input=cx.new(|cx|InputState::new(window,cx)
            .placeholder("Try typing Ru").default_value("Ru"));
        let mut _input_subscriptions=Vec::new();
        _input_subscriptions.push(cx.subscribe(&slider_single,
            |this,_,event:&SliderEvent,cx|{
                if let SliderEvent::Change(value)=event {
                    this.slider_value=value.start();
                    cx.notify();
                }
            }));
        _input_subscriptions.push(cx.subscribe(&autocomplete_input,
            |this,_,event:&InputEvent,cx|{
                match event{
                    InputEvent::Change|InputEvent::Focus=>{
                        this.autocomplete_open=true;
                        this.autocomplete_active=0;
                        cx.notify();
                    },
                    _=>{},
                }
            }));
        _input_subscriptions.push(cx.subscribe(&time_basic,
            |_,_,_:&TimeFieldEvent,cx|{cx.notify();}));
        let calendar_single=cx.new(|cx|CalendarState::new(window,cx));
        let calendar_range=cx.new(|cx|{
            let mut state=CalendarState::new(window,cx);
            state.apply_date(Date::Range(None,None));
            state
        });
        let calendar_restricted=cx.new(|cx|CalendarState::new(window,cx)
            .disabled_matcher(vec![0,6]));
        let mut _calendar_subscriptions=Vec::new();
        _calendar_subscriptions.push(cx.subscribe(&calendar_single,
            |this,_,event:&CalendarEvent,cx|{
                let CalendarEvent::Selected(date)=event;
                this.calendar_selected=date.to_string();
                cx.notify();
            }));
        _calendar_subscriptions.push(cx.subscribe(&calendar_range,
            |this,_,event:&CalendarEvent,cx|{
                let CalendarEvent::Selected(date)=event;
                this.calendar_range_selected=date.to_string();
                cx.notify();
            }));
        let theme_override=ThemeLabSpec::parse(&component,dark).map(|spec|spec.theme);
        Self {
            component, dark, theme_override, button_clicks: 0, checked: true, radio_selected: Some(0),
            radio_button_selected: Some(1), switch_on: true,
            input, input_filled, input_borderless, input_decorated, input_large,
            input_error, input_password, input_disabled,
            select_basic, select_search, select_disabled,
            date_single, date_range, date_time, date_disabled,
            form_name, form_email, form_password, form_confirm,
            form_errors: BTreeMap::new(), form_attempted: false, form_success: false,
            paragraph_expanded: false,
            tabs_index: 0, tabs_card_index: 1, layout_collapsed: false,
            layout_mobile_open: false,
            layout_selected: "dashboard".into(), menu_selected:"dashboard".into(),
            menu_open_keys:BTreeSet::new(),layout_menu_open_keys:BTreeSet::new(),
            dropdown_selected: "None".into(), popover_open: false,
            modal_result: "No action".into(),
            popconfirm_result: "Awaiting confirmation".into(),
            pagination_page:4, steps_current:1, rating:3,
            collapse_keys:BTreeSet::from(["first".to_owned()]),
            drawer_closed:0,tree_selected:"src/main.rs".to_owned(),
            tree_expanded:BTreeSet::from(["src".to_owned(),"src/lib".to_owned()]),
            tree_checked:BTreeSet::from(["src/main.rs".to_owned()]),
            table_sort_key:"score".to_owned(),table_sort:TableSort::Ascending,
            table_selected:BTreeSet::new(),table_page:1,
            table_filters:BTreeMap::new(),timeline_reversed:false,
            number_basic,number_decimal,number_disabled,
            slider_single,slider_range,slider_vertical,slider_value:35.,
            time_basic,time_seconds,time_12,time_disabled,
            segmented_index:1,autocomplete_input,autocomplete_open:true,
            autocomplete_active:0,autocomplete_last:"None".into(),_input_subscriptions,
            calendar_single,calendar_range,calendar_restricted,
            calendar_selected:"No date selected".to_owned(),
            calendar_range_selected:"No range selected".to_owned(),_calendar_subscriptions,
            float_group_open:false,float_clicks:0,float_last_action:"None".to_owned(),
        }
    }
    fn theme(&self) -> AntTheme {
        self.theme_override.unwrap_or_else(|| {
            if self.dark {AntTheme::dark()} else {AntTheme::light()}
        })
    }
    fn theme_lab_demo(&self, t: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let menu_change=cx.listener(|this,key:&SharedString,_,cx|{
            this.menu_selected=key.to_string();
            cx.notify();
        });
        let tap=cx.listener(|this,_:&gpui_kit::ClickEvent,_,cx|{
            this.button_clicks+=1;
            cx.notify();
        });
        let check_change=cx.listener(|this,value:&bool,_,cx|{
            this.checked=*value;cx.notify();
        });
        let switch_change=cx.listener(|this,value:&bool,_,cx|{
            this.switch_on=*value;cx.notify();
        });
        div().w_full().min_w_0().flex().flex_col().gap(px(t.padding))
            .child(AntTitle::new("Native Theme Lab").level(3).theme(t))
            .child(div().text_size(px(12.)).text_color(t.text_secondary)
                .child(format!("GPUI-derived tokens · controlHeight {}px · radius {}px · fontSize {}px",
                    t.control_height,t.border_radius,t.font_size)))
            .child(AntCard::new().theme(t).title("Buttons · component override")
                .child(div().flex().flex_wrap().items_center().gap(px(12.))
                    .child(AntButton::new("lab-primary","Primary").primary()
                        .theme(t).on_click(tap))
                    .child(AntButton::new("lab-default","Default").theme(t))
                    .child(AntButton::new("lab-danger","Danger").primary().danger(true).theme(t))
                    .child(AntButton::new("lab-disabled","Disabled").disabled(true).theme(t))
                    .child(AntButton::new("lab-small","Small").primary()
                        .size(ControlSize::Small).theme(t))
                    .child(AntButton::new("lab-large","Large").primary()
                        .size(ControlSize::Large).theme(t))))
            .child(div().flex().flex_wrap().gap(px(14.))
                .child(div().flex().flex_col().gap(px(10.))
                    .child(label("Input · background and radius overrides",t))
                    .child(AntInput::new(&self.input).theme(t).aria_label("Theme Lab input"))
                    .child(label("Feedback · semantic Seed colors",t))
                    .child(AntAlert::new("Saved successfully").tone(Tone::Green).theme(t))
                    .child(AntAlert::new("Please review warning").tone(Tone::Gold).theme(t))
                    .child(AntTag::new("Information").tone(Tone::Blue).theme(t)))
                .child(div().w(px(215.)).flex().flex_col()
                    .child(label("Menu · selected row override",t))
                    .child(AntMenu::new("lab-menu").theme(t).dark(self.dark)
                        .selected_key(self.menu_selected.clone())
                        .on_select(move |key,window,cx| menu_change(&key,window,cx))
                        .item(AntMenuItem::new("dashboard","Dashboard")
                            .icon(AntIconName::DashboardOutlined))
                        .item(AntMenuItem::new("analytics","Analytics")
                            .icon(AntIconName::BarChartOutlined))
                        .item(AntMenuItem::new("settings","Settings")
                            .icon(AntIconName::SettingOutlined)))))
            .child(label("Scoped data display and input tokens",t))
            .child(div().flex().flex_wrap().items_center().gap(px(16.))
                .child(AntCheckbox::new("lab-checkbox","Checkbox").theme(t)
                    .checked(self.checked)
                    .on_change(move |next,window,cx| check_change(&next,window,cx)))
                .child(AntSwitch::new("lab-switch").theme(t)
                    .checked(self.switch_on)
                    .on_change(move |next,window,cx| switch_change(&next,window,cx)))
                .child(AntBadge::count(12).theme(t))
                .child(AntTag::new("Tag").theme(t).tone(Tone::Blue))
                .child(AntTooltip::new("lab-tooltip","Token-aware tooltip").theme(t)
                    .child(AntButton::new("lab-hover","Hover for Tooltip").theme(t))))
            .child(AntProgress::new(72.).theme(t))
            .child(AntDivider::new().theme(t))
            .child(AntText::new("Typography component color and size overrides.")
                .theme(t).strong(true))
            .child(AntCard::new().theme(t).title("Card token override: padding and radius")
                .child(AntText::new("The body padding and header color come from Card tokens.")
                    .theme(t)))
            .child(AntCard::new().theme(t).title("Native Tree / Table / Timeline component tokens")
                .child(div().flex().flex_col().gap(px(16.))
                    .child(AntTree::new("lab-tree").theme(t)
                        .checkable(true).expanded_keys(["tree-root"])
                        .checked_keys(["tree-a"])
                        .child(AntTreeNode::new("tree-root","Components")
                            .child(AntTreeNode::new("tree-a","Button"))
                            .child(AntTreeNode::new("tree-b","Table"))))
                    .child(AntTable::new("lab-table").theme(t).pagination(false)
                        .column(AntTableColumn::new("name","Name").width(150.))
                        .column(AntTableColumn::new("status","Status").width(150.))
                        .row(AntTableRow::new("row").cells(["Build","Active"])))
                    .child(AntTimeline::new().theme(t)
                        .child(AntTimelineItem::new("Start").tone(TimelineTone::Success))
                        .child(AntTimelineItem::new("Ready").tone(TimelineTone::Processing)))))
            .child(AntCard::new().theme(t).title("Calendar and FloatButton component tokens")
                .child(div().flex().flex_wrap().gap(px(14.))
                    .child(AntCalendar::new(&self.calendar_single).theme(t)
                        .size(ControlSize::Small))
                    .child(div().relative().size(px(155.)).bg(t.surface_secondary)
                        .rounded(px(t.border_radius))
                        .child(AntFloatButton::new("lab-float","Token-aware FloatButton")
                            .theme(t).icon(AntIconName::PlusOutlined).position(14.,14.)))))
            .child(div().text_size(px(12.)).text_color(t.text_secondary)
                .child(format!("Primary clicked {} times · menu route {}",
                    self.button_clicks,self.menu_selected)))
            .into_any_element()
    }

    fn theme_tokens_demo(&self,t:AntTheme)->gpui_kit::AnyElement {
        let tokens=t.resolved_tokens();
        div().w_full().min_w_0().flex().flex_col().gap(px(16.))
            .child(AntTitle::new("Resolved Design Tokens").level(3).theme(t))
            .child(AntText::new(
                "Real Rust/GPUI Seed → Map → Alias values. Color tiles are painted by GPUI; component-scoped values apply only to the chosen component."
            ).theme(t).tone(TextTone::Secondary))
            .child(AntCard::new().theme(t).title("Map Token · derived values")
                .child(div().flex().flex_wrap().gap(px(8.))
                    .children(tokens.iter().copied()
                        .filter(|token|token.layer==AntTokenLayer::Map)
                        .map(|token|render_resolved_token(token,t)))))
            .child(AntCard::new().theme(t).title("Alias Token · semantic values")
                .child(div().flex().flex_wrap().gap(px(8.))
                    .children(tokens.iter().copied()
                        .filter(|token|token.layer==AntTokenLayer::Alias)
                        .map(|token|render_resolved_token(token,t)))))
            .into_any_element()
    }

    fn button_demo(&self, t: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let on_click = cx.listener(|this, _: &gpui_kit::ClickEvent, _, cx| {
            this.button_clicks += 1;
            cx.notify();
        });
        div().flex().flex_col().gap(px(20.))
            .child(label("Button types", t))
            .child(
                row()
                    .child(AntButton::new("primary", "Primary Button").primary().theme(t).on_click(on_click))
                    .child(AntButton::new("default", "Default Button").theme(t))
                    .child(AntButton::new("dashed", "Dashed Button").dashed().theme(t))
                    .child(AntButton::new("text", "Text Button").text().theme(t))
                    .child(AntButton::new("link", "Link Button").link().theme(t)),
            )
            .child(label("Sizes", t))
            .child(
                row()
                    .child(AntButton::new("large", "Large").primary().size(ControlSize::Large).theme(t))
                    .child(AntButton::new("middle", "Middle").primary().theme(t))
                    .child(AntButton::new("small", "Small").primary().size(ControlSize::Small).theme(t)),
            )
            .child(label("States", t))
            .child(
                row()
                    .child(AntButton::new("danger", "Danger").primary().danger(true).theme(t))
                    .child(AntButton::new("loading", "Loading").primary().loading(true).theme(t))
                    .child(AntButton::new("disabled", "Disabled").disabled(true).theme(t))
                    .child(AntButton::new("ghost", "Ghost").primary().ghost(true).theme(t)),
            )
            .child(div().text_color(t.text_secondary).text_size(px(13.)).child(format!("Primary clicked {} times", self.button_clicks)))
            .into_any_element()
    }
    fn checkbox_demo(&self, t:AntTheme, cx:&mut Context<Self>) -> gpui_kit::AnyElement {
        let on_change = cx.listener(|this, next:&bool, _, cx|{
            this.checked = *next;
            cx.notify();
        });
        div().flex().flex_col().gap(px(20.))
            .child(label("Basic usage",t))
            .child(row()
                .child(AntCheckbox::new("checkbox-controlled","Checkbox").checked(self.checked).theme(t)
                    .on_change(move |checked,window,cx| on_change(&checked,window,cx)))
                .child(AntCheckbox::new("checkbox-checked","Checked").checked(true).theme(t))
                .child(AntCheckbox::new("checkbox-empty","Unchecked").theme(t))
                .child(AntCheckbox::new("checkbox-disabled","Disabled").disabled(true).theme(t)))
            .child(div().text_size(px(13.)).text_color(t.text_secondary).child(format!("Controlled value: {}",self.checked)))
            .into_any_element()
    }
    fn radio_demo(&self, t: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let on_delivery = cx.listener(|this, index: &usize, _, cx| {
            this.radio_selected = Some(*index);
            cx.notify();
        });
        let on_buttons = cx.listener(|this, index: &usize, _, cx| {
            this.radio_button_selected = Some(*index);
            cx.notify();
        });
        div().flex().flex_col().gap(px(22.))
            .child(label("Basic RadioGroup: Arrow keys move selection; Tab enters the group", t))
            .child(
                AntRadioGroup::horizontal("delivery-group")
                    .label("Delivery method")
                    .theme(t)
                    .selected_index(self.radio_selected)
                    .child(AntRadio::new("delivery-standard", "Standard"))
                    .child(AntRadio::new("delivery-express", "Express"))
                    .child(AntRadio::new("delivery-unavailable", "Unavailable").disabled(true))
                    .on_change(move |index, window, cx| on_delivery(&index, window, cx))
            )
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Selected delivery option: {:?}", self.radio_selected)))
            .child(label("Radio button style", t))
            .child(
                AntRadioGroup::horizontal("layout-group")
                    .label("Layout")
                    .button_style(true)
                    .theme(t)
                    .selected_index(self.radio_button_selected)
                    .child(AntRadio::new("layout-left", "Left"))
                    .child(AntRadio::new("layout-center", "Center"))
                    .child(AntRadio::new("layout-right", "Right"))
                    .on_change(move |index, window, cx| on_buttons(&index, window, cx))
            )
            .child(label("Vertical group and disabled controls", t))
            .child(
                AntRadioGroup::vertical("pricing-group")
                    .label("Billing cycle")
                    .theme(t)
                    .selected_index(Some(1))
                    .disabled(true)
                    .child(AntRadio::new("pricing-monthly", "Monthly"))
                    .child(AntRadio::new("pricing-yearly", "Yearly"))
            )
            .into_any_element()
    }
    fn form_demo(&self, theme: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let submit = cx.listener(|this, _: &gpui_kit::ClickEvent, _, cx| {
            let values = BTreeMap::from([
                ("name".to_owned(), this.form_name.read(cx).value().to_string()),
                ("email".to_owned(), this.form_email.read(cx).value().to_string()),
                ("password".to_owned(), this.form_password.read(cx).value().to_string()),
                ("confirm".to_owned(), this.form_confirm.read(cx).value().to_string()),
            ]);
            let result = FormSchema::new()
                .field("name", "Full name", [FormRule::Required, FormRule::MinLength(2)])
                .field("email", "Email", [FormRule::Required, FormRule::Email])
                .field("password", "Password", [FormRule::Required, FormRule::MinLength(6)])
                .field("confirm", "Confirmation",
                    [FormRule::Required, FormRule::EqualsField("password".to_owned())])
                .validate(&values);
            this.form_success = result.is_valid();
            this.form_errors = result.errors;
            this.form_attempted = true;
            cx.notify();
        });
        let error = |key: &str| self.form_errors.get(key).cloned().map(Into::into);
        let status = |key: &str| if self.form_errors.contains_key(key) {
            InputStatus::Error
        } else {
            InputStatus::Normal
        };
        let message = if !self.form_attempted {
            "Submit this form to see validation feedback."
        } else if self.form_success {
            "All fields passed validation. Demo data was not submitted."
        } else {
            "Please review the highlighted fields."
        };
        div().flex().flex_col().w(px(400.)).gap(px(16.))
            .child(
                AntForm::new().theme(theme).gap(14.)
                    .child(
                        AntFormItem::new("Full name").theme(theme).required(true)
                            .error(error("name"))
                            .child(AntInput::new(&self.form_name).theme(theme)
                                .status(status("name")).aria_label("Full name")),
                    )
                    .child(
                        AntFormItem::new("Email").theme(theme).required(true)
                            .error(error("email"))
                            .child(AntInput::new(&self.form_email).theme(theme)
                                .status(status("email")).aria_label("Email address")),
                    )
                    .child(
                        AntFormItem::new("Password").theme(theme).required(true)
                            .error(error("password"))
                            .child(AntInput::new(&self.form_password).theme(theme)
                                .status(status("password")).password_toggle(true)
                                .aria_label("Password")),
                    )
                    .child(
                        AntFormItem::new("Confirm password").theme(theme).required(true)
                            .error(error("confirm"))
                            .child(AntInput::new(&self.form_confirm).theme(theme)
                                .status(status("confirm")).password_toggle(true)
                                .aria_label("Confirm password")),
                    )
                    .child(AntButton::new("form-submit", "Validate form")
                        .theme(theme).primary().block(true).on_click(submit)),
            )
            .child(div().text_size(px(12.)).text_color(if self.form_success {
                theme.success
            } else if self.form_attempted {
                theme.error
            } else {
                theme.text_secondary
            }).child(message))
            .into_any_element()
    }
    fn date_picker_demo(&self, theme: AntTheme) -> gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(15.))
            .child(label("Single date", theme))
            .child(div().w(px(275.)).child(
                AntDatePicker::new(&self.date_single)
                    .theme(theme).clearable(true)
                    .placeholder("Select a date")))
            .child(label("Date range · two-month calendar", theme))
            .child(div().w(px(320.)).child(
                AntDatePicker::new(&self.date_range)
                    .theme(theme).clearable(true)
                    .number_of_months(2).placeholder("Choose a range")))
            .child(label("Date and time · minute precision", theme))
            .child(div().w(px(290.)).child(
                AntDatePicker::new(&self.date_time)
                    .theme(theme).placeholder("Select date and time")))
            .child(label("Disabled", theme))
            .child(div().w(px(275.)).child(
                AntDatePicker::new(&self.date_disabled)
                    .theme(theme).disabled(true).placeholder("Unavailable")))
            .child(div().text_color(theme.text_secondary).text_size(px(12.))
                .child("The calendar, popup, selection and keyboard interactions run inside GPUI."))
            .into_any_element()
    }
    fn select_demo(&self, t: AntTheme) -> gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(18.))
            .child(label("Default Select", t))
            .child(div().w(px(270.)).child(
                AntSelect::new(&self.select_basic)
                    .theme(t)
                    .clearable(true)
                    .placeholder("Choose fruit")
                    .aria_label("Fruit selection")))
            .child(label("Searchable Select", t))
            .child(div().w(px(270.)).child(
                AntSelect::new(&self.select_search)
                    .theme(t)
                    .placeholder("Choose a language")
                    .search_placeholder("Type to filter…")
                    .aria_label("Programming language selection")
                    .menu_width(320.)))
            .child(label("Disabled", t))
            .child(div().w(px(270.)).child(
                AntSelect::new(&self.select_disabled)
                    .theme(t)
                    .disabled(true)
                    .aria_label("Disabled choice")))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child("GPUI Kit owns filtering, option focus, selection, Escape and popover placement."))
            .into_any_element()
    }
    fn switch_demo(&self, t:AntTheme, cx:&mut Context<Self>) -> gpui_kit::AnyElement {
        let on_change = cx.listener(|this,next:&bool,_,cx|{
            this.switch_on=*next;
            cx.notify();
        });
        div().flex().flex_col().gap(px(20.))
            .child(label("Switch control",t))
            .child(row()
                .child(AntSwitch::new("switch-controlled").checked(self.switch_on).theme(t)
                    .on_change(move |checked,window,cx| on_change(&checked,window,cx)))
                .child(AntSwitch::new("switch-off").theme(t))
                .child(AntSwitch::new("switch-disabled").disabled(true).checked(true).theme(t)))
            .child(div().text_size(px(13.)).text_color(t.text_secondary).child(format!("Controlled value: {}",self.switch_on)))
            .into_any_element()
    }
    fn input_demo(&self, t: AntTheme) -> gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(14.))
            .child(label("Outlined · Filled · Borderless", t))
            .child(row()
                .child(div().w(px(255.)).child(AntInput::new(&self.input)
                    .theme(t).clearable(true).aria_label("Outlined text input")))
                .child(div().w(px(255.)).child(AntInput::new(&self.input_filled)
                    .theme(t).variant(InputVariant::Filled).aria_label("Filled text input")))
                .child(div().w(px(255.)).child(AntInput::new(&self.input_borderless)
                    .theme(t).variant(InputVariant::Borderless).aria_label("Borderless text input"))))
            .child(label("Validation status", t))
            .child(div().w(px(330.)).child(AntInput::new(&self.input_error)
                .theme(t).status(InputStatus::Error).aria_label("Error input")))
            .child(label("Password and decorations", t))
            .child(row()
                .child(div().w(px(260.)).child(AntInput::new(&self.input_password)
                    .theme(t).password_toggle(true).aria_label("Password input")))
                .child(div().w(px(260.)).child(AntInput::new(&self.input_decorated)
                    .theme(t).prefix(div().text_color(t.primary).child("@"))
                    .suffix(div().text_color(t.text_secondary).child(".com")))))
            .child(label("Size and disabled state", t))
            .child(row()
                .child(div().w(px(230.)).child(AntInput::new(&self.input_large)
                    .theme(t).size(ControlSize::Large).aria_label("Large text input")))
                .child(div().w(px(230.)).child(AntInput::new(&self.input_disabled)
                    .theme(t).disabled(true).aria_label("Disabled text input"))))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child("GPUI Kit handles real keyboard, IME, caret, selection, copy and paste."))
            .into_any_element()
    }
    fn tag_demo(&self,t:AntTheme)->gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(20.))
            .child(label("Presets",t))
            .child(row()
                .child(AntTag::new("Default").theme(t))
                .child(AntTag::new("Processing").tone(Tone::Blue).theme(t))
                .child(AntTag::new("Success").tone(Tone::Green).theme(t))
                .child(AntTag::new("Warning").tone(Tone::Gold).theme(t))
                .child(AntTag::new("Error").tone(Tone::Red).theme(t))
                .child(AntTag::new("Magenta").tone(Tone::Purple).theme(t)))
            .into_any_element()
    }
    fn badge_demo(&self,t:AntTheme)->gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(20.))
            .child(label("Count and dot badges",t))
            .child(row().gap(px(32.))
                .child(AntBadge::count(5).theme(t))
                .child(AntBadge::count(29).theme(t))
                .child(AntBadge::count(123).theme(t))
                .child(AntBadge::count(0).dot().theme(t))
                .child(AntBadge::count(2).color(t.success).theme(t)))
            .into_any_element()
    }
    fn alert_demo(&self,t:AntTheme)->gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(12.)).w_full()
            .child(AntAlert::new("Info: Helpful context for this operation.").tone(Tone::Blue).theme(t))
            .child(AntAlert::new("Success: Changes saved successfully.").tone(Tone::Green).theme(t))
            .child(AntAlert::new("Warning: Please review your changes.").tone(Tone::Gold).theme(t))
            .child(AntAlert::new("Error: Something went wrong.").tone(Tone::Red).theme(t))
            .into_any_element()
    }
    fn card_demo(&self,t:AntTheme)->gpui_kit::AnyElement {
        row()
            .child(div().w(px(320.))
                .child(AntCard::new().title("Card title").theme(t)
                    .child(div().flex().flex_col().gap(px(8.))
                        .child("Card content")
                        .child("A native GPUI surface styled with Ant Design tokens."))))
            .child(div().w(px(240.)).child(AntCard::new().theme(t).child("A card without a header.")))
            .into_any_element()
    }
    fn progress_demo(&self,t:AntTheme)->gpui_kit::AnyElement {
        div().w(px(400.)).flex().flex_col().gap(px(18.))
            .child(AntProgress::new(30.).theme(t))
            .child(AntProgress::new(62.).tone(Tone::Green).theme(t))
            .child(AntProgress::new(86.).tone(Tone::Gold).theme(t))
            .child(AntProgress::new(100.).tone(Tone::Blue).theme(t))
            .into_any_element()
    }
    fn avatar_demo(&self, theme: AntTheme) -> gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(22.))
            .child(label("Circular avatar sizes", theme))
            .child(AntSpace::new().size(SpaceSize::Middle)
                .child(AntAvatar::new("Ada Lovelace").size(AvatarSize::Small).theme(theme))
                .child(AntAvatar::new("Grace Hopper").size(AvatarSize::Middle).theme(theme))
                .child(AntAvatar::new("Alan Turing").size(AvatarSize::Large).theme(theme))
                .child(AntAvatar::new("Alex Chen").size(AvatarSize::Custom(56.)).theme(theme)))
            .child(label("Square and custom color", theme))
            .child(AntSpace::new().size(SpaceSize::Middle)
                .child(AntAvatar::new("GPUI").shape(AvatarShape::Square).theme(theme)
                    .size(AvatarSize::Large))
                .child(AntAvatar::new("Design System").background(theme.error)
                    .shape(AvatarShape::Square).theme(theme)
                    .size(AvatarSize::Large))
                .child(AntAvatar::new("Rust").background(theme.success)
                    .size(AvatarSize::Custom(48.)).theme(theme)))
            .child(AntParagraph::new("Native GPUI Base fallback avatars. Pass src() to load an image.")
                .theme(theme).tone(TextTone::Secondary))
            .into_any_element()
    }

    fn statistic_demo(&self, theme: AntTheme) -> gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(22.))
            .child(label("Statistic summary with formatted numbers", theme))
            .child(AntSpace::new().size(SpaceSize::Large).wrap(true)
                .child(AntStatistic::number("Active Users", 112893., 0).theme(theme))
                .child(AntStatistic::number("Conversion Rate", 23.89, 2)
                    .theme(theme).suffix("%").tone(TextTone::Success))
                .child(AntStatistic::number("Revenue", 1057280.5, 2)
                    .theme(theme).prefix("$")))
            .child(label("Semantic colors and suffixes", theme))
            .child(AntSpace::new().size(SpaceSize::Large)
                .child(AntStatistic::new("Errors", "12")
                    .theme(theme).tone(TextTone::Danger))
                .child(AntStatistic::number("Latency", 124.8, 1)
                    .theme(theme).suffix("ms").tone(TextTone::Warning)))
            .into_any_element()
    }

    fn empty_demo(&self, theme: AntTheme) -> gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(16.))
            .child(label("Default empty state", theme))
            .child(AntEmpty::new().theme(theme)
                .child(AntButton::new("empty-action", "Create item")
                    .primary().theme(theme)))
            .child(label("Compact empty state", theme))
            .child(AntEmpty::new().theme(theme)
                .description("No search results")
                .icon(AntIconName::SearchOutlined)
                .compact(true))
            .into_any_element()
    }


    fn pagination_demo(&self, t:AntTheme, cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let changed=cx.listener(|this,page:&usize,_,cx|{
            this.pagination_page=*page;cx.notify();
        });
        div().w_full().flex().flex_col().gap(px(20.))
            .child(label("Controlled paging · click a page, the jump button or Prev/Next",t))
            .child(AntPagination::new("pagination-controlled",126)
                .page_size(10).current(self.pagination_page).show_total(true).theme(t)
                .on_change(move |page,_,window,cx|changed(&page,window,cx)))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Selected page: {}",self.pagination_page)))
            .child(label("Compact simple pager and disabled states",t))
            .child(AntPagination::new("pagination-simple",145)
                .current(4).simple(true).size(ControlSize::Small).theme(t))
            .child(AntPagination::new("pagination-disabled",70)
                .current(3).disabled(true).theme(t))
            .into_any_element()
    }

    fn steps_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let changed=cx.listener(|this,step:&usize,_,cx|{
            this.steps_current=*step;cx.notify();
        });
        div().flex().flex_col().gap(px(20.))
            .child(label("Clickable, parent-controlled workflow steps",t))
            .child(AntSteps::new("steps-basic").theme(t).current(self.steps_current)
                .on_change(move|index,window,cx|changed(&index,window,cx))
                .child(AntStep::new("Sign in").description("Create account"))
                .child(AntStep::new("Verify").description("Verify email"))
                .child(AntStep::new("Finish").description("Completed")))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Current step index: {}",self.steps_current)))
            .child(label("Vertical steps with disabled and error presentation",t))
            .child(AntSteps::new("steps-vertical").theme(t).current(2).vertical(true)
                .child(AntStep::new("Requested"))
                .child(AntStep::new("Approval").status(StepStatus::Error))
                .child(AntStep::new("Publish").disabled(true)))
            .into_any_element()
    }

    fn rate_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let changed=cx.listener(|this,value:&usize,_,cx|{
            this.rating=*value;cx.notify();
        });
        div().flex().flex_col().gap(px(20.))
            .child(label("Controlled star rating · click the selected star to clear",t))
            .child(AntRate::new("rating-input").value(self.rating).theme(t)
                .on_change(move|value,window,cx|changed(&value,window,cx)))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Rating: {} / 5",self.rating)))
            .child(label("Readonly disabled rating",t))
            .child(AntRate::new("rating-disabled").value(4).disabled(true).theme(t))
            .into_any_element()
    }

    fn collapse_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let changed=cx.listener(|this,keys:&BTreeSet<String>,_,cx|{
            this.collapse_keys=keys.clone();cx.notify();
        });
        div().flex().flex_col().gap(px(16.))
            .child(label("Controlled expansion · multiple panels can stay open",t))
            .child(AntCollapse::new("collapse-basic").theme(t)
                .open_keys(self.collapse_keys.iter().map(String::as_str))
                .on_change(move|keys,window,cx|changed(&keys,window,cx))
                .child(AntCollapseItem::new("first","What is GPUI Ant Design?")
                    .child(AntText::new("A single Rust component library running on desktop and in WASM.").theme(t)))
                .child(AntCollapseItem::new("second","How is expansion handled?")
                    .child(AntText::new("The application owns open panel keys and listens for changes.").theme(t)))
                .child(AntCollapseItem::new("locked","Disabled panel").disabled(true)
                    .child(AntText::new("Not reachable").theme(t))))
            .child(label("Borderless accordion",t))
            .child(AntCollapse::new("collapse-accordion").theme(t)
                .accordion(true).bordered(false).open_keys(["usage"])
                .child(AntCollapseItem::new("usage","Usage")
                    .child(AntText::new("One expanded panel at a time.").theme(t)))
                .child(AntCollapseItem::new("api","API")
                    .child(AntText::new("Controlled via open_keys.").theme(t))))
            .into_any_element()
    }

    fn descriptions_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().flex().flex_col().gap(px(20.))
            .child(label("Structured record details",t))
            .child(AntDescriptions::new().title("Project information").theme(t)
                .columns(3).bordered(true)
                .child(AntDescriptionItem::new("Project","gpui-antd"))
                .child(AntDescriptionItem::new("Language","Rust"))
                .child(AntDescriptionItem::new("Platform","Desktop / WASM"))
                .child(AntDescriptionItem::new("Theme","Light + Dark"))
                .child(AntDescriptionItem::new("License","MIT"))
                .child(AntDescriptionItem::new("Status","Early alpha")))
            .child(label("Unbordered description list",t))
            .child(AntDescriptions::new().columns(2).theme(t)
                .child(AntDescriptionItem::new("Owner","Application state"))
                .child(AntDescriptionItem::new("Renderer","Native GPUI")))
            .into_any_element()
    }

    fn list_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().w_full().flex().flex_col().gap(px(18.))
            .child(label("Bordered list · native icons and actions",t))
            .child(AntList::new().theme(t).bordered(true)
                .header(AntText::new("Team activity").theme(t).strong(true))
                .item(AntListItem::new("New deployment")
                    .icon(AntIconName::CloudUploadOutlined)
                    .description("Production build passed GPUI/WASM verification.")
                    .action(AntButton::new("list-view-1","View").theme(t).size(ControlSize::Small)))
                .item(AntListItem::new("Design tokens updated")
                    .icon(AntIconName::BgColorsOutlined)
                    .description("Semantic colors and component tokens were refreshed.")
                    .action(AntButton::new("list-view-2","Details").theme(t).size(ControlSize::Small)))
                .item(AntListItem::new("Archived workspace")
                    .icon(AntIconName::FolderOutlined).disabled(true)
                    .description("This entry is no longer active."))
                .footer(AntText::new("3 recent activities").theme(t).tone(TextTone::Secondary)))
            .child(label("Small list without borders",t))
            .child(AntList::new().theme(t).size(ListSize::Small).split(true)
                .item(AntListItem::new("Profile").description("Update your account"))
                .item(AntListItem::new("Security").description("Manage access")))
            .child(label("Loading and empty states",t))
            .child(div().flex().flex_wrap().gap(px(18.))
                .child(div().w(px(260.)).child(AntList::new().theme(t)
                    .bordered(true).loading(true)))
                .child(div().w(px(260.)).child(AntList::new().theme(t)
                    .bordered(true))))
            .into_any_element()
    }

    fn result_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().flex().flex_col().gap(px(18.))
            .child(label("Success state and native action",t))
            .child(AntResult::new(ResultStatus::Success).theme(t)
                .subtitle("Your changes have been saved.")
                .child(AntButton::new("result-back","Back to dashboard").primary().theme(t)))
            .child(label("Not found state",t))
            .child(AntResult::new(ResultStatus::NotFound).theme(t)
                .subtitle("The requested resource is unavailable."))
            .into_any_element()
    }

    fn skeleton_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().flex().flex_col().gap(px(20.))
            .child(label("Article loading placeholder",t))
            .child(AntSkeleton::new().theme(t).avatar(true).rows(4))
            .child(label("Text-only placeholder",t))
            .child(AntSkeleton::new().theme(t).rows(2))
            .into_any_element()
    }

    fn spin_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().flex().flex_col().gap(px(22.))
            .child(label("Native animated progress feedback",t))
            .child(AntSpin::new().theme(t).tip("Loading your data..."))
            .child(label("Inactive spinner preserves supplied content",t))
            .child(AntSpin::new().theme(t).spinning(false)
                .child(AntText::new("Data is ready.").theme(t)))
            .into_any_element()
    }


    fn input_number_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().flex().flex_col().gap(px(18.))
            .child(label("Native step buttons, Up/Down, min/max: 0–100 in steps of 5",t))
            .child(div().w(px(260.)).child(AntInputNumber::new(&self.number_basic)
                .theme(t).placeholder("Quantity").prefix(AntIcon::new(AntIconName::NumberOutlined))))
            .child(label("Decimal values and filled visual variant",t))
            .child(div().w(px(260.)).child(AntInputNumber::new(&self.number_decimal)
                .theme(t).variant(InputVariant::Filled).prefix(AntText::new("$").theme(t))))
            .child(label("Disabled numeric input",t))
            .child(div().w(px(260.)).child(AntInputNumber::new(&self.number_disabled)
                .theme(t).disabled(true)))
            .into_any_element()
    }

    fn slider_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().flex().flex_col().gap(px(20.))
            .child(label("Pointer/touch drag · Tab then arrows, Home/End, PageUp/PageDown",t))
            .child(div().w(px(520.)).child(AntSlider::new(&self.slider_single)
                .theme(t).show_value(true)))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Controlled value from native SliderEvent: {}",self.slider_value)))
            .child(label("Range slider with independently draggable handles",t))
            .child(div().w(px(520.)).child(AntSlider::new(&self.slider_range)
                .theme(t).show_value(true)))
            .child(label("Vertical and disabled",t))
            .child(div().flex().gap(px(35.))
                .child(AntSlider::new(&self.slider_vertical)
                    .theme(t).vertical(true).height(145.).show_value(true))
                .child(div().w(px(280.)).child(AntSlider::new(&self.slider_single)
                    .theme(t).disabled(true))))
            .into_any_element()
    }

    fn segmented_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let changed=cx.listener(|this,next:&usize,_,cx|{
            this.segmented_index=*next;cx.notify();
        });
        div().flex().flex_col().gap(px(18.))
            .child(label("Controlled view switch · Left/Right/Home/End keyboard navigation",t))
            .child(AntSegmented::new("segmented-views").theme(t)
                .selected_index(self.segmented_index)
                .on_change(move|next,window,cx|changed(&next,window,cx))
                .child(AntSegmentedOption::new("List").icon(AntIconName::BarsOutlined))
                .child(AntSegmentedOption::new("Grid").icon(AntIconName::AppstoreOutlined))
                .child(AntSegmentedOption::new("Kanban").icon(AntIconName::ProjectOutlined))
                .child(AntSegmentedOption::new("Locked").disabled(true)))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Selected view index: {}",self.segmented_index)))
            .child(label("Block layout and disabled state",t))
            .child(div().w(px(440.)).child(AntSegmented::new("segmented-block")
                .theme(t).block(true).selected_index(1)
                .child(AntSegmentedOption::new("Monthly"))
                .child(AntSegmentedOption::new("Quarterly"))
                .child(AntSegmentedOption::new("Yearly"))))
            .child(AntSegmented::new("segmented-disabled").theme(t).disabled(true)
                .child(AntSegmentedOption::new("Enabled"))
                .child(AntSegmentedOption::new("Disabled")))
            .into_any_element()
    }

    fn time_picker_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().flex().flex_col().gap(px(18.))
            .child(label("GPUI native segment editor: Up/Down change a segment; Tab advances",t))
            .child(row()
                .child(AntTimePicker::new(&self.time_basic).theme(t))
                .child(AntTimePicker::new(&self.time_seconds).theme(t)
                    .size(ControlSize::Large)))
            .child(label("12-hour cycle including AM/PM keyboard segment",t))
            .child(AntTimePicker::new(&self.time_12).theme(t))
            .child(label("Disabled and validation statuses",t))
            .child(row()
                .child(AntTimePicker::new(&self.time_basic).theme(t)
                    .status(InputStatus::Warning))
                .child(AntTimePicker::new(&self.time_disabled).theme(t).disabled(true)))
            .into_any_element()
    }

    fn autocomplete_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let on_selected=cx.listener(|this,selected:&SharedString,window,cx|{
            this.autocomplete_input.update(cx,|input,cx|{
                input.set_value(selected.clone(),window,cx);
            });
            this.autocomplete_last=selected.to_string();
            this.autocomplete_open=false;
            this.autocomplete_active=0;
            cx.notify();
        });
        let on_open=cx.listener(|this,open:&bool,_,cx|{
            this.autocomplete_open=*open;cx.notify();
        });
        let on_active=cx.listener(|this,index:&usize,_,cx|{
            this.autocomplete_active=*index;cx.notify();
        });
        let open_again=cx.listener(|this,_:&gpui_kit::ClickEvent,_,cx|{
            this.autocomplete_open=true;cx.notify();
        });
        div().flex().flex_col().gap(px(20.)).w_full()
            .child(label("Free-text input with filtered Rust/GPUI suggestions",t))
            .child(div().h(px(252.)).flex().flex_col().gap(px(8.))
                .child(AntAutoComplete::new("autocomplete-language",&self.autocomplete_input)
                    .theme(t).open(self.autocomplete_open).active_index(self.autocomplete_active)
                    .options(["Rust","Ruby","R","React","Python","Java","TypeScript","Go","Scala"])
                    .on_select(move|value,window,cx|on_selected(&value,window,cx))
                    .on_active_change(move|index,window,cx|on_active(&index,window,cx))
                    .on_open_change(move|open,window,cx|on_open(&open,window,cx)))
                .child(div().text_size(px(12.)).text_color(t.text_secondary)
                    .child("Type to filter; Up/Down choose a suggestion, Enter accepts, Esc closes."))
                .child(div().text_size(px(13.)).text_color(t.text_secondary)
                    .child(format!("Last selected: {}",self.autocomplete_last))))
            .child(AntButton::new("autocomplete-open","Show suggestions").theme(t)
                .on_click(open_again))
            .into_any_element()
    }


    fn drawer_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let on_closed=cx.listener(|this,_:&(),_,cx|{
            this.drawer_closed+=1;cx.notify();
        });
        div().flex().flex_col().gap(px(18.))
            .child(label("Real GPUI Kit Sheet · Escape, mask dismiss and focus restoration",t))
            .child(row()
                .child(AntDrawer::new("drawer-right","Open right Drawer")
                    .title("Edit profile").body("Your changes stay in a native GPUI Sheet.")
                    .theme(t).placement(DrawerPlacement::Right)
                    .on_close(move|window,cx|on_closed(&(),window,cx))
                    .content(move|_,_|div().flex().flex_col().gap(px(12.))
                        .child(AntText::new("Native panel content with Ant Design colors.").theme(t))
                        .child(AntTag::new("In progress").tone(Tone::Blue).theme(t))
                        .into_any_element()))
                .child(AntDrawer::new("drawer-left","Open left")
                    .theme(t).placement(DrawerPlacement::Left)
                    .title("Navigation drawer").body("Left-side navigation panel."))
                .child(AntDrawer::new("drawer-bottom","Open bottom")
                    .theme(t).placement(DrawerPlacement::Bottom).size(260.)
                    .title("Bottom sheet").body("Responsive action sheet.")))
            .child(label("Resizable Sheet and overlay options",t))
            .child(row()
                .child(AntDrawer::new("drawer-resize","Resizable").theme(t)
                    .resizable(true).size(400.)
                    .body("Drag the edge to resize this GPUI Sheet."))
                .child(AntDrawer::new("drawer-no-mask","Without mask").theme(t)
                    .mask(false).body("No modal backdrop.")))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Drawer closed {} times",self.drawer_closed)))
            .into_any_element()
    }

    fn message_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().flex().flex_col().gap(px(18.))
            .child(label("Global in-app toast queue · native GPU, top center",t))
            .child(row()
                .child(AntButton::new("message-success","Success").theme(t)
                    .on_click(|_,window,cx|{
                        AntMessage::success("Changes saved successfully").show(window,cx);
                    }))
                .child(AntButton::new("message-info","Info").theme(t)
                    .on_click(|_,window,cx|{
                        AntMessage::new("Your settings are synchronized").show(window,cx);
                    }))
                .child(AntButton::new("message-warning","Warning").theme(t)
                    .on_click(|_,window,cx|{
                        AntMessage::warning("Please review the selected values").show(window,cx);
                    }))
                .child(AntButton::new("message-error","Error").theme(t)
                    .on_click(|_,window,cx|{
                        AntMessage::error("Unable to save changes").show(window,cx);
                    })))
            .child(label("Replace a keyed message instead of creating duplicates",t))
            .child(row()
                .child(AntButton::new("message-loading-start","Start").theme(t)
                    .on_click(|_,window,cx|{
                        AntMessage::new("Operation running").key("demo-task")
                            .persistent(true).show(window,cx);
                    }))
                .child(AntButton::new("message-loading-done","Complete").theme(t)
                    .on_click(|_,window,cx|{
                        AntMessage::success("Operation complete").key("demo-task")
                            .show(window,cx);
                    })))
            .into_any_element()
    }

    fn notification_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().flex().flex_col().gap(px(18.))
            .child(label("Kit Root notification stack, top right by default",t))
            .child(row()
                .child(AntButton::new("notification-info","Information").theme(t)
                    .on_click(|_,window,cx|{
                        AntNotification::new("New version","A new native build is ready.")
                            .show(window,cx);
                    }))
                .child(AntButton::new("notification-success","Success").theme(t)
                    .on_click(|_,window,cx|{
                        AntNotification::new("Upload complete","All assets were validated.")
                            .tone(NoticeTone::Success).show(window,cx);
                    }))
                .child(AntButton::new("notification-error","Error").theme(t)
                    .on_click(|_,window,cx|{
                        AntNotification::new("Connection lost","Try reconnecting to the service.")
                            .tone(NoticeTone::Error).persistent(true).show(window,cx);
                    })))
            .child(label("Independent placement and update by key",t))
            .child(row()
                .child(AntButton::new("notification-bottom","Bottom left").theme(t)
                    .on_click(|_,window,cx|{
                        AntNotification::new("Alternative placement","Bottom left notifications")
                            .placement(gpui_kit::Anchor::BottomLeft).show(window,cx);
                    }))
                .child(AntButton::new("notification-persistent","Persistent").theme(t)
                    .on_click(|_,window,cx|{
                        AntNotification::new("Job status","Job 42 is still running.")
                            .key("job42").persistent(true).show(window,cx);
                    })))
            .into_any_element()
    }

    fn tree_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let selected=cx.listener(|this,key:&SharedString,_,cx|{
            this.tree_selected=key.to_string();cx.notify();
        });
        let expanded=cx.listener(|this,(key,opened):&(SharedString,bool),_,cx|{
            if *opened{this.tree_expanded.insert(key.to_string());}
            else{this.tree_expanded.remove(key.as_ref());}
            cx.notify();
        });
        let checked=cx.listener(|this,keys:&BTreeSet<String>,_,cx|{
            this.tree_checked=keys.clone();
            cx.notify();
        });
        let nodes=vec![
            AntTreeNode::new("src","src").icon(AntIconName::FolderOutlined)
                .child(AntTreeNode::new("src/lib","lib")
                    .child(AntTreeNode::new("src/lib/components.rs","components.rs"))
                    .child(AntTreeNode::new("src/lib/theme.rs","theme.rs")))
                .child(AntTreeNode::new("src/main.rs","main.rs"))
                .child(AntTreeNode::new("src/protected.rs","protected.rs").disabled(true)),
            AntTreeNode::new("docs","docs").icon(AntIconName::FolderOutlined)
                .child(AntTreeNode::new("docs/api.md","api.md"))
                .child(AntTreeNode::new("docs/guide.md","guide.md")),
            AntTreeNode::new("Cargo.toml","Cargo.toml"),
        ];
        let states=gpui_antd::tree_check_states(&nodes,&self.tree_checked,false);
        let half=states.values().filter(|status|**status==gpui_antd::TreeCheckState::Indeterminate).count();
        div().flex().flex_col().gap(px(12.))
            .child(label("Native GPUI tree · check parents to cascade; mixed branches show half-checks",t))
            .child(div().w(px(440.)).border_1().border_color(t.border)
                .rounded(px(8.)).child(AntTree::new("tree-example")
                    .theme(t).children(nodes).checkable(true)
                    .expanded_keys(self.tree_expanded.iter().map(String::as_str))
                    .checked_keys(self.tree_checked.iter().map(String::as_str))
                    .selected_key(self.tree_selected.clone())
                    .on_select(move|key,window,cx|selected(&key,window,cx))
                    .on_expand(move|key,value,window,cx|expanded(&(key,value),window,cx))
                    .on_checked_keys_change(move|keys,window,cx|checked(&keys,window,cx))))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Selected: {} · Checked: {} · Partially checked: {}",
                    self.tree_selected,self.tree_checked.len(),half)))
            .into_any_element()
    }

    fn table_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let sorted=cx.listener(|this,(key,order):&(SharedString,TableSort),_,cx|{
            this.table_sort_key=key.to_string();
            this.table_sort=*order;
            this.table_page=1;cx.notify();
        });
        let selection=cx.listener(|this,keys:&BTreeSet<String>,_,cx|{
            this.table_selected=keys.clone();cx.notify();
        });
        let page_change=cx.listener(|this,page:&usize,_,cx|{
            this.table_page=*page;cx.notify();
        });
        let filter_change=cx.listener(|this,(key,value):&(SharedString,Option<SharedString>),_,cx|{
            if let Some(value)=value{
                this.table_filters.insert(key.to_string(),value.to_string());
            }else{
                this.table_filters.remove(key.as_ref());
            }
            this.table_page=1;
            cx.notify();
        });
        let data=(1..=24).map(|i|{
            AntTableRow::new(format!("user-{i:02}"))
                .cells([format!("User {i:02}"),format!("{}",(i*7)%101),
                    if i%3==0{"Pending".to_owned()}else{"Active".to_owned()}])
        });
        div().w_full().flex().flex_col().gap(px(14.))
            .child(label("Native table · sort Score, filter Status, select all on the current page",t))
            .child(AntTable::new("table-demo").theme(t)
                .column(AntTableColumn::new("name","Name").sortable(true).width(180.))
                .column(AntTableColumn::new("score","Score").sortable(true).numeric(true).width(140.))
                .column(AntTableColumn::new("status","Status").width(220.)
                    .filters(["Active","Pending"]))
                .rows(data).selectable(true).striped(true)
                .selected_keys(self.table_selected.iter().map(String::as_str))
                .sort_by(self.table_sort_key.clone(),self.table_sort)
                .active_filters(self.table_filters.iter().map(|(key,value)|
                    (key.as_str(),value.as_str())))
                .page(self.table_page).page_size(5)
                .on_sort(move|key,order,window,cx|sorted(&(key,order),window,cx))
                .on_selection(move|keys,window,cx|selection(&keys,window,cx))
                .on_page_change(move|page,window,cx|page_change(&page,window,cx))
                .on_filter_change(move|key,value,window,cx|{
                    filter_change(&(key,value),window,cx)
                }))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Page {} · Sorted: {:?} · Filter: {} · Selected rows: {}",
                    self.table_page,self.table_sort,
                    self.table_filters.get("status").map(String::as_str).unwrap_or("All"),
                    self.table_selected.len())))
            .into_any_element()
    }


    fn timeline_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let reverse=cx.listener(|this,_:&gpui_kit::ClickEvent,_,cx|{
            this.timeline_reversed=!this.timeline_reversed;
            cx.notify();
        });
        div().w_full().flex().flex_col().gap(px(18.))
            .child(label("Chronological events · toggle native Rust order",t))
            .child(AntButton::new("timeline-reverse","Reverse timeline")
                .theme(t).on_click(reverse))
            .child(div().w(px(650.)).child(AntTimeline::new().theme(t)
                .mode(TimelineMode::Alternate).reverse(self.timeline_reversed)
                .child(AntTimelineItem::new("Build created")
                    .timestamp("09:10")
                    .description("Source fetched from the latest main branch.")
                    .tone(TimelineTone::Success))
                .child(AntTimelineItem::new("WASM compilation")
                    .timestamp("09:12")
                    .description("The same Rust component compiles to native and WASM.")
                    .icon(AntIconName::CodeOutlined)
                    .tone(TimelineTone::Processing))
                .child(AntTimelineItem::new("Visual regression")
                    .timestamp("09:14").description("Chromium validates rendered GPUI pixels.")
                    .tone(TimelineTone::Warning))
                .pending("Deploying to GitHub Pages")))
            .child(label("Left-aligned completed steps",t))
            .child(AntTimeline::new().theme(t)
                .child(AntTimelineItem::new("Start").tone(TimelineTone::Success))
                .child(AntTimelineItem::new("Validated").tone(TimelineTone::Success))
                .child(AntTimelineItem::new("Failed step").tone(TimelineTone::Error)))
            .into_any_element()
    }


    fn calendar_demo(&self,t:AntTheme)->gpui_kit::AnyElement{
        div().w_full().flex().flex_col().gap(px(18.))
            .child(label("Native month / year navigation and keyboard-selectable dates",t))
            .child(AntCalendar::new(&self.calendar_single).theme(t))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Selected single date: {}",self.calendar_selected)))
            .child(label("Range selection across two adjacent months",t))
            .child(AntCalendar::new(&self.calendar_range).theme(t)
                .size(ControlSize::Small).months(2))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Selected range: {}",self.calendar_range_selected)))
            .child(label("Business days only · Saturdays and Sundays disabled",t))
            .child(AntCalendar::new(&self.calendar_restricted)
                .theme(t).size(ControlSize::Small).bordered(false))
            .into_any_element()
    }

    fn float_button_demo(&self,t:AntTheme,cx:&mut Context<Self>)->gpui_kit::AnyElement{
        let clicked=cx.listener(|this,_:&gpui_kit::ClickEvent,_,cx|{
            this.float_clicks+=1;cx.notify();
        });
        let open=cx.listener(|this,open:&bool,_,cx|{
            this.float_group_open=*open;cx.notify();
        });
        let selected=cx.listener(|this,key:&SharedString,_,cx|{
            this.float_last_action=key.to_string();cx.notify();
        });
        div().flex().flex_col().gap(px(18.)).w_full()
            .child(label("Floating actions use native GPUI buttons and keyboard focus",t))
            .child(div().relative().w(px(520.)).h(px(180.))
                .border_1().border_color(t.border).rounded(px(t.border_radius))
                .bg(t.surface_secondary)
                .child(div().p(px(16.))
                    .child(AntText::new("Standalone primary action · 7 unread").theme(t)))
                .child(AntFloatButton::new("floating-primary","Add item")
                    .theme(t).variant(FloatVariant::Primary).shape(FloatShape::Circle)
                    .icon(AntIconName::PlusOutlined).badge(7).position(18.,18.)
                    .on_click(clicked)))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Standalone action clicked {} times",self.float_clicks)))
            .child(label("Controlled FloatButton.Group · expand and choose action",t))
            .child(div().relative().w(px(520.)).h(px(275.))
                .border_1().border_color(t.border).rounded(px(t.border_radius))
                .bg(t.surface_secondary)
                .child(div().p(px(16.)).text_color(t.text_secondary)
                    .child(format!("Last selected: {}",self.float_last_action)))
                .child(AntFloatButtonGroup::new("floating-group","Quick actions")
                    .theme(t).open(self.float_group_open)
                    .position(18.,18.)
                    .action(AntFloatAction::new("edit","Edit",AntIconName::EditOutlined))
                    .action(AntFloatAction::new("share","Share",AntIconName::ShareAltOutlined))
                    .action(AntFloatAction::new("delete","Delete",AntIconName::DeleteOutlined)
                        .danger(true).disabled(true))
                    .on_open_change(move|expanded,window,cx|open(&expanded,window,cx))
                    .on_select(move|key,window,cx|selected(&key,window,cx))))
            .into_any_element()
    }

    fn breadcrumb_demo(&self, theme: AntTheme) -> gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(20.))
            .child(label("Breadcrumb hierarchy", theme))
            .child(AntBreadcrumb::new().theme(theme)
                .child(AntBreadcrumbItem::new("Home").href("https://ant.design/"))
                .child(AntBreadcrumbItem::new("Components").href("https://ant.design/components/overview"))
                .child(AntBreadcrumbItem::new("Breadcrumb")))
            .child(label("Custom separator and disabled intermediate item", theme))
            .child(AntBreadcrumb::new().theme(theme).separator(">")
                .child(AntBreadcrumbItem::new("Home").href("https://ant.design/"))
                .child(AntBreadcrumbItem::new("Section").disabled(true))
                .child(AntBreadcrumbItem::new("Current page")))
            .into_any_element()
    }

    fn icon_single_demo(&self, t: AntTheme, raw: &str) -> gpui_kit::AnyElement {
        let preview = IconPreview::parse(raw);
        let selected = AntIconName::from_ant_name(&preview.name);
        div().w_full().flex().flex_col().items_center()
            .justify_center().gap(px(16.)).p(px(18.))
            .when_some(selected, |this, name| {
                let mut icon = AntIcon::new(name)
                    .size(preview.size).theme(t).spin(preview.spin);
                if name.style() == "twotone" {
                    if preview.primary.is_some() || preview.secondary.is_some() {
                        icon = icon.two_tone_colors(
                            preview.primary.map(gpui_antd::color).unwrap_or(t.primary),
                            preview.secondary.map(gpui_antd::color).unwrap_or(t.info_bg),
                        );
                    }
                } else if let Some(primary) = preview.primary {
                    icon = icon.color(gpui_antd::color(primary));
                }
                this.child(icon)
                    .child(div().text_color(t.text).text_size(px(18.))
                        .child(preview.name.clone()))
                    .child(div().text_color(t.text_secondary).text_size(px(12.))
                        .child(format!("Official SVG · {} · {}px{}", name.style(),
                            preview.size, if preview.spin { " · rotating" } else { "" })))
            })
            .when(selected.is_none(), |this| {
                this.child(div().text_color(t.error).child("Unknown Ant Design icon"))
            })
            .into_any_element()
    }

    fn icons_demo(&self, t: AntTheme) -> gpui_kit::AnyElement {
        let catalog = |style: &'static str| {
            // One horizontal native GPUI scroll strip per icon variant:
            // no deep wrapped rows masked by a fixed-height WASM iframe.
            div().w_full().min_w_0().flex().gap(px(12.))
                .overflow_x_scrollbar().id(format!("icon-strip-{style}")).pb(px(12.))
                .children(AntIconName::ALL.iter()
                    .filter(move |icon| icon.style() == style)
                    // Keep the WASM showcase bounded; the full 848-name
                    // catalog is searchable in the docs and each selected
                    // icon can be previewed in its own Rust GPUI view.
                    .take(24)
                    .map(move |name| {
                        // Long PascalCase enum names have no natural word-break
                        // opportunities in GPUI text. Show the official name
                        // as two complete lines (base + style), not ellipsis.
                        let full=name.ant_name();
                        let style_name=match name.style() {
                            "outlined"=>"Outlined",
                            "filled"=>"Filled",
                            _=>"TwoTone",
                        };
                        let base_name=full.strip_suffix(style_name).unwrap_or(full);
                        div().w(px(156.)).h(px(112.)).flex_none()
                            .flex().flex_col().items_center().justify_center().gap(px(8.))
                            .p(px(10.))
                            .rounded(px(8.)).border_1().border_color(t.border)
                            .child(AntIcon::new(*name).size(32.).theme(t))
                            .child(div().w_full().flex().flex_col().items_center().gap(px(2.))
                                .child(div().text_size(px(11.))
                                    .text_color(t.text).text_center().child(base_name))
                                .child(div().text_size(px(10.))
                                    .text_color(t.text_secondary).text_center().child(style_name)))
                    }))
        };
        div().w_full().min_w_0().flex().flex_col().gap(px(16.))
            .child(div().text_size(px(12.)).text_color(t.text_secondary)
                .child("Scroll each row sideways to explore 24 icon samples. The entire 848-icon catalog is searchable above."))
            .child(label("Outlined — official Ant Design SVGs", t))
            .child(catalog("outlined"))
            .child(label("Filled", t))
            .child(catalog("filled"))
            .child(label("Two Tone (theme-aware primary and secondary)", t))
            .child(catalog("twotone"))
            .child(label("Two Tone — theme palette and custom colors", t))
            .child(div().flex().flex_wrap().gap(px(28.))
                .child(div().flex().flex_col().items_center().gap(px(8.))
                    .child(AntIcon::new(AntIconName::CheckCircleTwoTone)
                        .size(48.).theme(t))
                    .child(div().text_size(px(11.)).text_color(t.text_secondary)
                        .child("Theme default")))
                .child(div().flex().flex_col().items_center().gap(px(8.))
                    .child(AntIcon::new(AntIconName::CheckCircleTwoTone)
                        .size(48.).theme(t).color(t.success))
                    .child(div().text_size(px(11.)).text_color(t.text_secondary)
                        .child("Primary tint")))
                .child(div().flex().flex_col().items_center().gap(px(8.))
                    .child(AntIcon::new(AntIconName::CheckCircleTwoTone)
                        .size(48.).theme(t).two_tone_colors(
                            gpui_antd::color(0x52c41a), gpui_antd::color(0xf6ffed)))
                    .child(div().text_size(px(11.)).text_color(t.text_secondary)
                        .child("Custom green")))
                .child(div().flex().flex_col().items_center().gap(px(8.))
                    .child(AntIcon::new(AntIconName::CheckCircleTwoTone)
                        .size(48.).theme(t).two_tone_colors(
                            gpui_antd::color(0xff4d4f), gpui_antd::color(0xfff2f0)))
                    .child(div().text_size(px(11.)).text_color(t.text_secondary)
                        .child("Custom red")))
                .child(div().flex().flex_col().items_center().gap(px(8.))
                    .child(AntIcon::new(AntIconName::CheckCircleTwoTone)
                        .size(48.).theme(t).spin(true))
                    .child(div().text_size(px(11.)).text_color(t.text_secondary)
                        .child("TwoTone spin"))))
            .child(label("Theme tint and animated loading", t))
            .child(
                AntSpace::new().size(SpaceSize::Middle)
                    .child(AntIcon::new(AntIconName::HeartFilled).size(28.).color(t.error))
                    .child(AntIcon::new(AntIconName::CheckOutlined).size(28.).color(t.success))
                    .child(AntIcon::new(AntIconName::InfoCircleOutlined).size(28.).color(t.primary))
                    .child(AntIcon::loading().size(26.).color(t.primary)),
            )
            .into_any_element()
    }

    // Internal visual-audit route: 24 real Rust/GPUI icons per deterministic
    // page, in a fixed 6x4 grid. The browser checks each glyph's painted pixels
    // and stores every page as a reviewable screenshot artifact.
    fn icon_audit_demo(&self, t: AntTheme, page: &str) -> gpui_kit::AnyElement {
        const PAGE_SIZE: usize = 24;
        let page = page.parse::<usize>().unwrap_or(0)
            .min((AntIconName::ALL.len() - 1) / PAGE_SIZE);
        let icons = AntIconName::ALL.iter().skip(page * PAGE_SIZE)
            .take(PAGE_SIZE).copied().collect::<Vec<_>>();
        div().w_full().flex().flex_col()
            .children(icons.chunks(6).map(|row| {
                div().flex().gap(px(0.))
                    .children(row.iter().map(|name| {
                        div().w(px(136.)).h(px(112.)).flex_none()
                            .flex().flex_col().items_center().pt(px(12.))
                            .child(AntIcon::new(*name).size(60.).theme(t))
                            .child(div().text_size(px(9.)).text_color(t.text_secondary)
                                .mt(px(8.)).child(name.ant_name()))
                    }))
            }))
            .into_any_element()
    }

    // Dedicated real-GPUI preview used for full-color custom-palette pixel tests.
    fn icon_palette_demo(&self, t: AntTheme) -> gpui_kit::AnyElement {
        div().w_full().flex().flex_col().items_center()
            .justify_center().gap(px(16.)).p(px(18.))
            .child(AntIcon::new(AntIconName::CheckCircleTwoTone)
                .size(64.).theme(t).two_tone_colors(
                    gpui_antd::color(0x52c41a), gpui_antd::color(0xf6ffed)))
            .child(div().text_color(t.text).text_size(px(18.))
                .child("CheckCircleTwoTone: custom green"))
            .into_any_element()
    }

    fn typography_demo(&self, t: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let on_expand = cx.listener(|this, expanded: &bool, _, cx| {
            this.paragraph_expanded = *expanded;
            cx.notify();
        });
        AntTypography::new().gap(15.)
            .child(AntTitle::new("Ant Design Typography").level(1).theme(t))
            .child(AntTitle::new("Heading level 2").level(2).theme(t))
            .child(AntTitle::new("Heading level 3").level(3).theme(t))
            .child(AntTitle::new("Heading level 4").level(4).theme(t))
            .child(AntTitle::new("Heading level 5").level(5).theme(t))
            .child(AntParagraph::new(
                "Typography offers readable visual hierarchy and consistent line height.                  The primitives below are real GPUI text elements with native callbacks."
            ).theme(t).width(680.))
            .child(AntSpace::new().wrap(true).size(SpaceSize::Middle)
                .child(AntText::new("Strong text").theme(t).strong(true))
                .child(AntText::new("Italic text").theme(t).italic(true))
                .child(AntText::new("Underline").theme(t).underline(true))
                .child(AntText::new("Deleted").theme(t).deleted(true))
                .child(AntText::new("Marked").theme(t).marked(true))
                .child(AntText::new("inline code").theme(t).code(true)))
            .child(AntSpace::new().wrap(true).size(SpaceSize::Middle)
                .child(AntText::new("Secondary").theme(t).tone(TextTone::Secondary))
                .child(AntText::new("Success").theme(t).tone(TextTone::Success))
                .child(AntText::new("Warning").theme(t).tone(TextTone::Warning))
                .child(AntText::new("Danger").theme(t).tone(TextTone::Danger))
                .child(AntText::new("Disabled").theme(t).disabled(true)))
            .child(AntText::new("Click the copy icon to write this text to your clipboard")
                .theme(t).copyable("typography-copy"))
            .child(AntLink::new("typography-link", "Open the Ant Design documentation", "https://ant.design/components/typography")
                .theme(t).underline(true))
            .child(AntParagraph::new(
                "A long single-line paragraph that is truncated to show how Ant Design ellipsis behaves when a container has a limited width."
            ).theme(t).ellipsis(true).width(360.))
            .child(label("Three-line truncation, with controlled expand / collapse", t))
            .child(AntParagraph::new(
                "The Ant Design Typography paragraph may contain more text than the card can show at once.                 Line clamping is performed by the GPUI text engine and works with the real                 browser WebAssembly backend too. Expand to reveal the full paragraph, then                 collapse back to three visual lines. This is not an HTML text stand-in."
            ).theme(t).width(440.).max_lines(3)
                .expanded(self.paragraph_expanded)
                .on_expand("typography-paragraph-toggle", move |state, window, cx| {
                    on_expand(&state, window, cx);
                }))
            .into_any_element()
    }

    fn space_demo(&self, t: AntTheme) -> gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(22.))
            .child(label("Space Small · 8px", t))
            .child(AntSpace::new().size(SpaceSize::Small)
                .child(AntButton::new("space-small-1", "Cancel").theme(t))
                .child(AntButton::new("space-small-2", "Save").theme(t).primary()))
            .child(label("Space Middle · 16px", t))
            .child(AntSpace::new().size(SpaceSize::Middle)
                .child(AntTag::new("Design").tone(Tone::Blue).theme(t))
                .child(AntTag::new("Components").tone(Tone::Green).theme(t))
                .child(AntTag::new("Rust").tone(Tone::Purple).theme(t)))
            .child(label("Space Large · 24px (vertical)", t))
            .child(AntSpace::new().size(SpaceSize::Large).vertical(true).align(FlexAlign::Start)
                .child(AntText::new("First item").theme(t))
                .child(AntText::new("Second item").theme(t))
                .child(AntText::new("Third item").theme(t)))
            .child(label("Wrap and custom size", t))
            .child(AntSpace::new().size(SpaceSize::Custom(9.)).wrap(true)
                .child(AntTag::new("Alpha").theme(t))
                .child(AntTag::new("Beta").tone(Tone::Blue).theme(t))
                .child(AntTag::new("Gamma").tone(Tone::Green).theme(t))
                .child(AntTag::new("Delta").tone(Tone::Gold).theme(t)))
            .into_any_element()
    }

    fn flex_demo(&self, t: AntTheme) -> gpui_kit::AnyElement {
        let chip = |name: &'static str| {
            div().rounded(px(6.)).bg(t.info_bg).border_1().border_color(t.primary)
                .text_color(t.primary).px(px(14.)).py(px(9.)).child(name)
        };
        div().flex().flex_col().gap(px(20.))
            .child(label("Horizontal / SpaceBetween", t))
            .child(AntFlex::new().gap(12.).fill(true)
                .justify(FlexJustify::SpaceBetween)
                .child(chip("Left")).child(chip("Center")).child(chip("Right")))
            .child(label("Centered", t))
            .child(AntFlex::new().gap(14.).fill(true)
                .justify(FlexJustify::Center)
                .child(chip("A")).child(chip("B")).child(chip("C")))
            .child(label("Vertical, aligned start", t))
            .child(AntFlex::new().vertical(true).gap(9.).align(FlexAlign::Start)
                .child(chip("Row one"))
                .child(chip("Row two"))
                .child(chip("Row three")))
            .child(label("Wrapping children", t))
            .child(AntFlex::new().wrap(true).gap(11.).fill(true)
                .child(chip("Tag one")).child(chip("Tag two")).child(chip("Tag three"))
                .child(chip("Tag four")))
            .into_any_element()
    }


    fn dropdown_demo(&self, t: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let on_select=cx.listener(|this, key: &SharedString, _, cx| {
            this.dropdown_selected=key.to_string();
            cx.notify();
        });
        div().w_full().flex().flex_col().gap(px(20.))
            .child(label("Native keyboard-accessible GPUI PopupMenu",t))
            .child(row()
                .child(AntDropdown::new("antd-dropdown-actions","Actions")
                    .theme(t).min_width(180.)
                    .item(AntDropdownItem::new("new","Create new"))
                    .item(AntDropdownItem::new("open","Open item"))
                    .item(AntDropdownItem::separator())
                    .item(AntDropdownItem::new("favorite","Favorite").checked(true))
                    .item(AntDropdownItem::new("disabled","Unavailable").disabled(true))
                    .on_select(move |key, window, cx| on_select(&key,window,cx)))
                .child(AntDropdown::new("antd-dropdown-more","More options")
                    .theme(t)
                    .item(AntDropdownItem::new("export","Export"))
                    .item(AntDropdownItem::new("share","Share")))
                .child(AntDropdown::new("antd-dropdown-disabled","Disabled")
                    .theme(t).disabled(true)
                    .item(AntDropdownItem::new("hidden","Hidden"))))
            .child(div().text_color(t.text_secondary).text_size(px(13.))
                .child(format!("Last selected action: {}",self.dropdown_selected)))
            .child(AntAlert::new("Try clicking Actions or using Enter, Arrow Up/Down and Escape inside its menu.")
                .tone(Tone::Blue).theme(t))
            .into_any_element()
    }

    fn popover_demo(&self, t: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let on_change=cx.listener(|this, open: &bool, _, cx| {
            this.popover_open=*open;
            cx.notify();
        });
        div().w_full().flex().flex_col().gap(px(20.))
            .child(label("Native anchored Popover with arbitrary Rust GPUI content",t))
            .child(row()
                .child(AntPopover::new("antd-popover-basic","Open popover")
                    .title("Quick actions").theme(t).width(290.).arrow(true)
                    .child(AntText::new("Choose what to do next.").theme(t))
                    .child(AntSpace::new().size(SpaceSize::Small)
                        .child(AntTag::new("Native GPUI").theme(t).tone(Tone::Green))
                        .child(AntIcon::new(AntIconName::CheckCircleFilled)
                            .theme(t).color(t.success))))
                .child(AntPopover::new("antd-popover-controlled","Controlled popover")
                    .title("Controlled state").theme(t).width(230.)
                    .open(self.popover_open)
                    .on_open_change(move |next,window,cx| on_change(&next,window,cx))
                    .child(AntText::new("The owning Rust Entity stores the open value.")
                        .theme(t)))
                .child(AntPopover::new("antd-popover-disabled","Disabled popover")
                    .theme(t).disabled(true).child("This cannot be opened")))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Controlled open state: {}",self.popover_open)))
            .child(AntAlert::new("Click outside or press Escape to dismiss. The popup is positioned and focused by GPUI Kit.")
                .tone(Tone::Blue).theme(t))
            .into_any_element()
    }

    fn modal_demo(&self, t: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let view=cx.entity().downgrade();
        let cancel_view=view.clone();
        div().w_full().flex().flex_col().gap(px(20.))
            .child(label("Native modal Dialog · focus trap, Escape, background overlay",t))
            .child(row()
                .child(AntModal::new("antd-modal-trigger","Open Modal","Edit workspace settings")
                    .theme(t).width(520.)
                    .description("Review the changes before saving.")
                    .body("A real GPUI dialog managed by the Kit Root. Press Escape or Cancel to dismiss.")
                    .on_ok(move |_,cx| {
                        let _=view.update(cx,|this,cx|{
                            this.modal_result="Confirmed".into();
                            cx.notify();
                        });
                        true
                    })
                    .on_cancel(move |_,cx| {
                        let _=cancel_view.update(cx,|this,cx|{
                            this.modal_result="Canceled".into();
                            cx.notify();
                        });
                        true
                    }))
                .child(AntModal::new("antd-modal-fixed-trigger","Non-dismissible mask",
                    "Persistent modal").theme(t)
                    .width(420.).mask_closable(false).keyboard(false)
                    .description("Backdrop and Escape are disabled.")
                    .body("Use the Cancel or OK button to close this modal.")))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Latest dialog result: {}",self.modal_result)))
            .child(AntAlert::new("Dialogs are layered above the GPUI gallery and use the framework's modal focus management.")
                .tone(Tone::Blue).theme(t))
            .into_any_element()
    }

    fn popconfirm_demo(&self, t: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let yes=cx.entity().downgrade();
        let no=cx.entity().downgrade();
        div().w_full().flex().flex_col().gap(px(20.))
            .child(label("Anchored confirmation popover (not a modal alert)",t))
            .child(row()
                .child(AntPopconfirm::new("antd-delete-confirm","Delete item",
                    "Delete this record?").theme(t)
                    .description("This operation cannot be undone.")
                    .on_confirm(move |_,cx| {
                        let _=yes.update(cx,|this,cx|{
                            this.popconfirm_result="Deleted".into();
                            cx.notify();
                        });
                        true
                    })
                    .on_cancel(move |_,cx| {
                        let _=no.update(cx,|this,cx|{
                            this.popconfirm_result="Canceled".into();
                            cx.notify();
                        });
                        true
                    }))
                .child(AntPopconfirm::new("antd-disabled-confirm","Disabled",
                    "Cannot activate").theme(t).disabled(true)))
            .child(div().text_size(px(13.)).text_color(t.text_secondary)
                .child(format!("Result: {}",self.popconfirm_result)))
            .child(AntAlert::new("Confirm and Cancel are native GPUI buttons; outside press and Escape dismiss the anchored popup.")
                .tone(Tone::Gold).theme(t))
            .into_any_element()
    }

    fn menu_demo(&self, theme: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let on_light=cx.listener(|this, key:&SharedString, _, cx|{
            this.menu_selected=key.to_string();
            cx.notify();
        });
        let on_dark=cx.listener(|this, key:&SharedString, _, cx|{
            this.menu_selected=key.to_string();
            cx.notify();
        });
        let on_collapsed=cx.listener(|this, key:&SharedString, _, cx|{
            this.menu_selected=key.to_string();
            cx.notify();
        });
        div().w_full().flex().flex_col().gap(px(20.))
            .child(label("GPUI Base-backed controlled navigation · click to select",theme))
            .child(div().flex().flex_wrap().gap(px(16.))
                .child(div().w(px(240.)).border_1().border_color(theme.border)
                    .rounded(px(7.))
                    .child(AntMenu::new("menu-light").theme(theme)
                        .selected_key(self.menu_selected.clone())
                        .on_select(move |key,window,cx| on_light(&key,window,cx))
                        .item(AntMenuItem::new("dashboard","Dashboard").icon(AntIconName::DashboardOutlined))
                        .item(AntMenuItem::new("analytics","Analytics").icon(AntIconName::BarChartOutlined))
                        .item(AntMenuItem::new("settings","Settings").icon(AntIconName::SettingOutlined))
                        .item(AntMenuItem::new("locked","Restricted").icon(AntIconName::LockOutlined).disabled(true))
                        .item(AntMenuItem::new("delete","Delete").icon(AntIconName::DeleteOutlined).danger(true))))
                .child(div().w(px(240.))
                    .child(AntMenu::new("menu-dark").theme(theme).dark(true)
                        .selected_key(self.menu_selected.clone())
                        .on_select(move |key,window,cx| on_dark(&key,window,cx))
                        .item(AntMenuItem::new("dashboard","Dashboard").icon(AntIconName::DashboardOutlined))
                        .item(AntMenuItem::new("analytics","Analytics").icon(AntIconName::BarChartOutlined))
                        .item(AntMenuItem::new("settings","Settings").icon(AntIconName::SettingOutlined))))
                .child(div().w(px(66.))
                    .child(AntMenu::new("menu-collapsed").theme(theme).dark(true)
                        .inline_collapsed(true)
                        .selected_key(self.menu_selected.clone())
                        .on_select(move |key,window,cx| on_collapsed(&key,window,cx))
                        .item(AntMenuItem::new("dashboard","Dashboard").icon(AntIconName::DashboardOutlined))
                        .item(AntMenuItem::new("analytics","Analytics").icon(AntIconName::BarChartOutlined))
                        .item(AntMenuItem::new("settings","Settings").icon(AntIconName::SettingOutlined)))))
            .child(div().text_size(px(13.)).text_color(theme.text_secondary)
                .child(format!("Selected menu key: {}",self.menu_selected)))
            .child(self.menu_tree_demo(theme,cx))
            .into_any_element()
    }

    /// Test-only route "MenuTree" and the embedded nested example on the
    /// public Menu page both use this same real Rust GPUI view.
    /// Collapsed icon-only navigation with controlled native anchored flyouts.
    /// A click outside or Escape dismisses the popover; clicking a leaf both
    /// selects the route and closes the flyout.
    fn menu_collapsed_demo(&self, theme: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let on_select=cx.listener(|this,key:&SharedString,_,cx|{
            this.menu_selected=key.to_string();cx.notify();
        });
        let on_open=cx.listener(|this,change:&(SharedString,bool),_,cx|{
            let (key,open)=change;
            if *open {
                this.menu_open_keys.insert(key.to_string());
            } else {
                this.menu_open_keys.remove(key.as_ref());
            }
            cx.notify();
        });
        div().w_full().min_h(px(470.)).flex().flex_col().gap(px(12.))
            .child(label("Collapsed icon menu · click or ArrowRight for a native GPUI flyout",theme))
            .child(div().flex().gap(px(24.))
                .child(div().w(px(76.)).flex_none().rounded(px(7.))
                    .bg(theme.layout_dark)
                    .child(AntMenu::new("menu-collapsed-flyouts").theme(theme).dark(true)
                        .inline_collapsed(true)
                        .selected_key(self.menu_selected.clone())
                        .open_keys(self.menu_open_keys.iter())
                        .on_open_change(move|key,open,window,cx|{
                            on_open(&(key,open),window,cx)
                        })
                        .on_select(move|key,window,cx|on_select(&key,window,cx))
                        .item(AntMenuItem::submenu("reports","Reports")
                            .icon(AntIconName::BarChartOutlined)
                            .child(AntMenuItem::new("daily","Daily report")
                                .icon(AntIconName::CalendarOutlined))
                            .child(AntMenuItem::submenu("advanced","Advanced reports")
                                .icon(AntIconName::AppstoreOutlined)
                                .child(AntMenuItem::new("trends","Trend analysis")
                                    .icon(AntIconName::LineChartOutlined))
                                .child(AntMenuItem::new("monthly","Monthly summary")
                                    .icon(AntIconName::FileTextOutlined))))
                        .item(AntMenuItem::new("dashboard","Dashboard")
                            .icon(AntIconName::DashboardOutlined))
                        .item(AntMenuItem::submenu("workspace","Workspace")
                            .icon(AntIconName::FolderOutlined)
                            .child(AntMenuItem::new("settings","Settings")
                                .icon(AntIconName::SettingOutlined))
                            .child(AntMenuItem::new("team","Team")
                                .icon(AntIconName::TeamOutlined)))
                        .item(AntMenuItem::submenu("disabled","Disabled group")
                            .icon(AntIconName::StopOutlined).disabled(true)
                            .child(AntMenuItem::new("hidden","Hidden child")))))
                .child(div().flex_1().flex().flex_col().gap(px(10.))
                    .child(AntText::new(format!("Selected: {}",self.menu_selected))
                        .theme(theme).strong(true))
                    .child(AntText::new(format!("Open: {}",
                        self.menu_open_keys.iter().cloned().collect::<Vec<_>>().join(", ")))
                        .theme(theme).tone(TextTone::Secondary))
                    .child(AntAlert::new("The popover is rendered in GPUI's native overlay layer. Escape and outside clicks dismiss it; disabled groups cannot open.")
                        .theme(theme).tone(Tone::Blue))))
            .into_any_element()
    }

    fn menu_tree_demo(&self, theme: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let on_select=cx.listener(|this, key: &SharedString, _, cx| {
            this.menu_selected=key.to_string();
            cx.notify();
        });
        let on_open=cx.listener(|this, change: &(SharedString,bool), _, cx| {
            let (key,open)=change;
            if *open {
                this.menu_open_keys.insert(key.to_string());
            } else {
                this.menu_open_keys.remove(key.as_ref());
            }
            cx.notify();
        });
        div().w_full().flex().flex_col().gap(px(12.))
            .child(label("Nested SubMenu · click headings, or use Arrow keys / Home / End", theme))
            .child(div().w(px(305.)).border_1().border_color(theme.border)
                .rounded(px(7.))
                .child(AntMenu::new("menu-nested").theme(theme)
                    .selected_key(self.menu_selected.clone())
                    .open_keys(self.menu_open_keys.iter())
                    .on_select(move |key,window,cx| on_select(&key,window,cx))
                    .on_open_change(move |key,open,window,cx| on_open(&(key,open),window,cx))
                    .item(AntMenuItem::submenu("reports","Reports")
                        .icon(AntIconName::BarChartOutlined)
                        .child(AntMenuItem::new("daily","Daily report")
                            .icon(AntIconName::CalendarOutlined))
                        .child(AntMenuItem::submenu("advanced","Advanced reports")
                            .icon(AntIconName::AppstoreOutlined)
                            .child(AntMenuItem::new("trends","Trend analysis")
                                .icon(AntIconName::LineChartOutlined))
                            .child(AntMenuItem::new("monthly","Monthly summary")
                                .icon(AntIconName::FileTextOutlined))))
                    .item(AntMenuItem::new("dashboard","Dashboard")
                        .icon(AntIconName::DashboardOutlined))
                    .item(AntMenuItem::submenu("workspace","Workspace")
                        .icon(AntIconName::FolderOutlined)
                        .child(AntMenuItem::new("settings","Settings")
                            .icon(AntIconName::SettingOutlined))
                        .child(AntMenuItem::new("team","Team")
                            .icon(AntIconName::TeamOutlined)))
                    .item(AntMenuItem::submenu("blocked","Disabled group")
                        .icon(AntIconName::StopOutlined).disabled(true)
                        .child(AntMenuItem::new("blocked-child","Hidden child")))))
            .child(div().text_size(px(12.)).text_color(theme.text_secondary)
                .child(format!("Selected: {} · Expanded: {}",
                    self.menu_selected,
                    self.menu_open_keys.iter().cloned().collect::<Vec<_>>().join(", "))))
            .child(AntAlert::new("Arrows move focus, Enter/Space activates a leaf or toggles a group. Right opens groups; Left/Escape close them.")
                .tone(Tone::Blue).theme(theme))
            .into_any_element()
    }

    fn layout_demo(&self, t: AntTheme, window: &Window, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let responsive=window.viewport_size().width < px(768.);
        let compact=if responsive {!self.layout_mobile_open} else {self.layout_collapsed};
        let on_collapse=cx.listener(|this, next: &bool, window, cx| {
            // Apply the next EFFECTIVE collapse state in its own mode.
            // Narrow expansion must not be immediately canceled by the
            // breakpoint, and should not alter the desktop collapse choice.
            if window.viewport_size().width < px(768.) {
                this.layout_mobile_open=!*next;
            } else {
                this.layout_collapsed=*next;
            }
            cx.notify();
        });
        let on_nav=cx.listener(|this, key: &SharedString, _, cx| {
            this.layout_selected=key.to_string();
            cx.notify();
        });
        let on_nav_open=cx.listener(|this, change: &(SharedString,bool), _, cx| {
            let (key,open)=change;
            if *open {
                this.layout_menu_open_keys.insert(key.to_string());
            } else {
                this.layout_menu_open_keys.remove(key.as_ref());
            }
            cx.notify();
        });
        let metric=|title: &'static str, value: f64, tone: TextTone| {
            div().p(px(12.)).border_1().border_color(t.border)
                .rounded(px(7.)).bg(t.surface)
                .child(AntStatistic::number(title, value, 0).theme(t).tone(tone))
        };
        div().w_full().flex().flex_col().gap(px(14.))
            .child(label("Responsive application shell · resize the window or collapse Sider",t))
            .child(div().w_full().border_1().border_color(t.border)
                .rounded(px(8.)).overflow_hidden()
                .child(AntLayout::new().horizontal(true).height(464.).theme(t)
                    .child(AntSider::new("demo-layout-sider")
                        .theme(t).dark(true).width(192.).collapsed_width(64.)
                        .collapsed(self.layout_collapsed)
                        .responsive_open(self.layout_mobile_open)
                        .breakpoint(GridBreakpoint::Md)
                        .collapsible(true)
                        .on_collapse(move |next, win, cx| on_collapse(&next, win, cx))
                        .child(div().h(px(60.)).flex().items_center()
                            .px(px(18.)).gap(px(9.))
                            .child(AntIcon::new(AntIconName::AppstoreOutlined)
                                .theme(t).size(21.).color(t.layout_dark_text))
                            .when(!compact, |this| this.child(div().font_semibold()
                                .text_color(t.layout_dark_text).child("GPUI Admin"))))
                        .child(AntMenu::new("layout-sidebar-menu")
                            .theme(t).dark(true).inline_collapsed(compact)
                            .selected_key(self.layout_selected.clone())
                            .open_keys(self.layout_menu_open_keys.iter())
                            .on_open_change(move |key,open,window,cx| on_nav_open(&(key,open),window,cx))
                            .on_select(move |key,window,cx| on_nav(&key,window,cx))
                            .item(AntMenuItem::new("dashboard","Dashboard")
                                .icon(AntIconName::DashboardOutlined))
                            .item(AntMenuItem::new("analytics","Analytics")
                                .icon(AntIconName::BarChartOutlined))
                            .item(AntMenuItem::new("documents","Documents")
                                .icon(AntIconName::FileTextOutlined))
                            .item(AntMenuItem::submenu("workspace","Workspace")
                                .icon(AntIconName::FolderOutlined)
                                .child(AntMenuItem::new("settings","Settings")
                                    .icon(AntIconName::SettingOutlined))
                                .child(AntMenuItem::new("team","Team")
                                    .icon(AntIconName::TeamOutlined)))))
                    .child(AntLayout::new().theme(t)
                        .child(AntHeader::new().theme(t).dark(false)
                            .height(56.).padding(20.)
                            .child(AntIcon::new(AntIconName::MenuOutlined)
                                .size(17.).theme(t))
                            .child(div().ml(px(12.)).font_semibold()
                                .text_color(t.text).child("Dashboard overview")))
                        .child(AntContent::new().theme(t).padding(18.)
                            .scrollable(true).scroll_id("layout-demo-main-scroll")
                            .child(div().w_full().flex().flex_col().gap(px(15.))
                                .child(AntTitle::new("Workspace overview").level(4).theme(t))
                                .child(AntRow::new().gutter(12.).vertical_gutter(10.)
                                    .child(AntCol::new(24).sm(12).md(8)
                                        .child(metric("Projects",18.,TextTone::Default)))
                                    .child(AntCol::new(24).sm(12).md(8)
                                        .child(metric("Completed",12.,TextTone::Success)))
                                    .child(AntCol::new(24).sm(12).md(8)
                                        .child(metric("In progress",6.,TextTone::Warning))))
                                .child(AntCard::new().theme(t).title("Recent activity")
                                    .child(div().flex().flex_col().gap(px(10.))
                                        .child(AntText::new("UI components compiled to WASM").theme(t))
                                        .child(AntProgress::new(75.).theme(t).show_info(true))))
                                .child(div().flex().flex_col().gap(px(9.))
                                    .children((1..=10).map(|index|
                                        div().text_size(px(12.))
                                            .text_color(t.text_secondary)
                                            .child(format!("Recent activity #{index}: GPUI native app shell update"))))))
                        .child(AntFooter::new().theme(t).padding(12.)
                            .child(AntText::new("GPUI Ant Design · native Rust layout")
                                .theme(t).tone(TextTone::Secondary)))))))
            .child(div().text_size(px(12.)).text_color(t.text_secondary)
                .child(format!("Sider: {} · {} viewport · content scrolls independently",
                    if compact {"collapsed"} else {"expanded"},
                    if responsive {"responsive"} else {"desktop"})))
            .into_any_element()
    }

    /// Narrow Layout route: GPUI Kit Sheet owns the modal navigation surface,
    /// backdrop dismissal, keyboard Escape, and focus restoration.
    fn layout_drawer_demo(
        &self,t:AntTheme,window:&Window,cx:&mut Context<Self>
    )->gpui_kit::AnyElement {
        let mobile=window.viewport_size().width < px(768.);
        let selected=self.layout_selected.clone();
        let selected_for_sheet=selected.clone();
        let on_select=Rc::new(cx.listener(|this,key:&SharedString,_,cx|{
            this.layout_selected=key.to_string();cx.notify();
        }));
        let desktop_select=on_select.clone();
        let drawer_select=on_select.clone();
        let drawer_open=cx.listener(|this,open:&bool,_,cx|{
            this.layout_mobile_open=*open;cx.notify();
        });
        let links=|| [
            AntMenuItem::new("dashboard","Dashboard").icon(AntIconName::DashboardOutlined),
            AntMenuItem::new("analytics","Analytics").icon(AntIconName::BarChartOutlined),
            AntMenuItem::new("documents","Documents").icon(AntIconName::FileTextOutlined),
            AntMenuItem::new("settings","Settings").icon(AntIconName::SettingOutlined),
        ];
        div().w_full().flex().flex_col().gap(px(14.))
            .child(label("Native left Sheet · resize below md to open Drawer",t))
            .child(div().w_full().border_1().border_color(t.border)
                .rounded(px(8.)).overflow_hidden()
                .child(AntLayout::new().horizontal(true).height(480.).theme(t)
                    .child(AntSider::new("drawer-layout-sider")
                        .theme(t).dark(true).width(192.).collapsed_width(64.)
                        .breakpoint(GridBreakpoint::Md).mobile_hidden(true)
                        .child(div().h(px(62.)).px(px(16.))
                            .flex().items_center()
                            .text_color(t.layout_dark_text).font_semibold()
                            .child("GPUI Admin"))
                        .child(AntMenu::new("drawer-desktop-nav").theme(t).dark(true)
                            .selected_key(selected.clone())
                            .items(links())
                            .on_select(move|key,w,cx|desktop_select(&key,w,cx))))
                    .child(AntLayout::new().theme(t)
                        .child(AntHeader::new().theme(t).dark(false)
                            .height(56.).padding(16.)
                            .child(AntMobileDrawer::new("drawer-layout-trigger")
                                .theme(t).title("Workspace navigation")
                                .width(280.).breakpoint(GridBreakpoint::Md)
                                .mask_closable(true)
                                .on_open_change(move|open,w,cx|{
                                    drawer_open(&open,w,cx);
                                })
                                .content(move|_,_|{
                                    let on_select=drawer_select.clone();
                                    AntMenu::new("drawer-sheet-navigation")
                                        .theme(t).selected_key(selected_for_sheet.clone())
                                        .items([
                                            AntMenuItem::new("dashboard","Dashboard")
                                                .icon(AntIconName::DashboardOutlined),
                                            AntMenuItem::new("analytics","Analytics")
                                                .icon(AntIconName::BarChartOutlined),
                                            AntMenuItem::new("documents","Documents")
                                                .icon(AntIconName::FileTextOutlined),
                                            AntMenuItem::new("settings","Settings")
                                                .icon(AntIconName::SettingOutlined),
                                        ])
                                        .on_select(move|key,window,cx|{
                                            on_select(&key,window,cx);
                                            window.close_sheet(cx);
                                        })
                                }))
                            .when(!mobile,|header|header.child(
                                AntIcon::new(AntIconName::MenuOutlined).theme(t).size(19.)
                            ))
                            .child(div().ml(px(12.)).font_semibold()
                                .text_color(t.text).child("Workspace")))
                        .child(AntContent::new().theme(t).scrollable(true)
                            .scroll_id("drawer-layout-content").padding(18.)
                            .child(div().w_full().flex().flex_col().gap(px(15.))
                                .child(AntTitle::new("Mobile dashboard").level(4).theme(t))
                                .child(AntCard::new().theme(t).title("Responsive navigation")
                                    .child(AntText::new(
                                        "On a narrow viewport the permanent sidebar gives way to an accessible left Drawer. Tap the header menu to open GPUI Kit Sheet; Escape and the mask close it."
                                    ).theme(t)))
                                .child(AntTag::new(if self.layout_mobile_open {
                                    "Drawer open"
                                } else {
                                    "Drawer closed"
                                }).theme(t).tone(Tone::Blue))
                                .child(AntText::new(format!("Active route: {}",selected))
                                    .theme(t).tone(TextTone::Secondary))))
                        .child(AntFooter::new().theme(t)
                            .child(AntText::new("GPUI native Sheet + Ant Design Layout")
                                .theme(t).tone(TextTone::Secondary))))))
            .child(AntText::new(if mobile {
                "Mobile viewport: permanent Sider hidden; open navigation in the header."
            } else {
                "Desktop viewport: regular Sider visible; Drawer trigger hidden."
            }).theme(t).tone(TextTone::Secondary))
            .into_any_element()
    }

    fn grid_demo(&self, t: AntTheme) -> gpui_kit::AnyElement {
        let cell = |label: &'static str| {
            div().w_full().h(px(42.))
                .flex().items_center().justify_center()
                .rounded(px(5.)).bg(t.info_bg)
                .border_1().border_color(t.primary)
                .text_color(t.primary).child(label)
        };
        div().flex().flex_col().gap(px(19.))
            .child(label("24-column responsive composition", t))
            .child(AntRow::new().gutter(12.)
                .child(AntCol::new(24).child(cell("col-24"))))
            .child(AntRow::new().gutter(12.)
                .child(AntCol::new(12).child(cell("col-12")))
                .child(AntCol::new(12).child(cell("col-12"))))
            .child(AntRow::new().gutter(12.)
                .child(AntCol::new(8).child(cell("col-8")))
                .child(AntCol::new(8).child(cell("col-8")))
                .child(AntCol::new(8).child(cell("col-8"))))
            .child(AntRow::new().gutter(12.)
                .child(AntCol::new(6).child(cell("col-6")))
                .child(AntCol::new(6).child(cell("col-6")))
                .child(AntCol::new(6).child(cell("col-6")))
                .child(AntCol::new(6).child(cell("col-6"))))
            .child(AntRow::new().gutter(12.)
                .child(AntCol::new(8).offset(8).child(cell("offset-8 col-8"))))
            .child(label("Responsive: xs=24 / sm=12 / md=8 / lg=6", t))
            .child(AntRow::new().gutter(12.).vertical_gutter(12.)
                .child(AntCol::new(24).sm(12).md(8).lg(6).child(cell("Item A")))
                .child(AntCol::new(24).sm(12).md(8).lg(6).child(cell("Item B")))
                .child(AntCol::new(24).sm(12).md(8).lg(6).child(cell("Item C")))
                .child(AntCol::new(24).sm(12).md(8).lg(6).child(cell("Item D"))))
            .into_any_element()
    }

    fn tabs_demo(&self, theme: AntTheme, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let change = cx.listener(|this, next: &usize, _, cx| {
            this.tabs_index = *next;
            cx.notify();
        });
        let change_card = cx.listener(|this, next: &usize, _, cx| {
            this.tabs_card_index = *next;
            cx.notify();
        });
        div().flex().flex_col().gap(px(16.))
            .child(label("Line tabs · controlled state and panel content", theme))
            .child(div().text_color(theme.text_secondary).text_size(px(12.))
                .child("Keyboard: Tab to the selected tab; Left/Right/Home/End move focus and select, Enter/Space activate. Disabled tabs are skipped."))
            .child(
                AntTabs::new("tabs-line")
                    .theme(theme)
                    .active_index(self.tabs_index)
                    .variant(TabsVariant::Line)
                    .on_change(move |index, window, cx| change(&index, window, cx))
                    .child(AntTab::new("Overview")
                        .icon(AntIconName::HomeOutlined)
                        .content(div().flex().flex_col().gap(px(10.))
                            .child("Overview panel")
                            .child(AntText::new("This panel is selected by Rust state.")
                                .theme(theme).tone(TextTone::Secondary))))
                    .child(AntTab::new("Settings")
                        .icon(AntIconName::SettingOutlined)
                        .content(div().flex().flex_col().gap(px(10.))
                            .child("Settings panel")
                            .child(AntSwitch::new("tabs-settings-toggle").checked(true).theme(theme))))
                    .child(AntTab::new("Disabled")
                        .disabled(true)
                        .content(div().child("Should never activate")))
            )
            .child(div().text_size(px(12.)).text_color(theme.text_secondary)
                .child(format!("Controlled active tab: {}", self.tabs_index)))
            .child(label("Card variant", theme))
            .child(
                AntTabs::new("tabs-card")
                    .theme(theme)
                    .size(ControlSize::Large)
                    .variant(TabsVariant::Card)
                    .active_index(self.tabs_card_index)
                    .on_change(move |index, window, cx| change_card(&index, window, cx))
                    .child(AntTab::new("First")
                        .content(AntText::new("First card content").theme(theme)))
                    .child(AntTab::new("Second")
                        .content(AntText::new("Second card content").theme(theme)))
                    .child(AntTab::new("Third")
                        .content(AntText::new("Third card content").theme(theme)))
            )
            .child(label("Pill and segmented appearances", theme))
            .child(
                AntTabs::new("tabs-pill").theme(theme)
                    .variant(TabsVariant::Pills).active_index(1)
                    .child(AntTab::new("Alpha"))
                    .child(AntTab::new("Beta"))
                    .child(AntTab::new("Gamma"))
            )
            .child(
                AntTabs::new("tabs-segmented").theme(theme)
                    .variant(TabsVariant::Segmented).active_index(0)
                    .child(AntTab::new("List"))
                    .child(AntTab::new("Grid"))
                    .child(AntTab::new("Columns"))
            )
            .into_any_element()
    }

    fn tooltip_demo(&self, theme: AntTheme) -> gpui_kit::AnyElement {
        div().flex().flex_col().gap(px(22.))
            .child(label("Hover a native GPUI control to reveal its tooltip", theme))
            .child(row()
                .child(AntTooltip::new("tooltip-save", "Save your changes")
                    .theme(theme)
                    .child(AntButton::new("tooltip-save-button", "Save")
                        .primary().theme(theme)))
                .child(AntTooltip::new("tooltip-info", "Additional information")
                    .theme(theme)
                    .child(AntButton::new("tooltip-info-button", "Info")
                        .theme(theme)))
                .child(AntTooltip::new("tooltip-delete", "Permanently delete this item")
                    .theme(theme)
                    .child(AntButton::new("tooltip-delete-button", "Delete")
                        .danger(true).theme(theme)))
            )
            .child(label("Tooltips can wrap icons, not only buttons", theme))
            .child(row()
                .child(AntTooltip::new("tooltip-icon-search", "Search items")
                    .theme(theme)
                    .child(div().p(px(10.))
                        .child(AntIcon::new(AntIconName::SearchOutlined)
                            .theme(theme).size(22.))))
                .child(AntTooltip::new("tooltip-icon-alert", "Important warning")
                    .theme(theme)
                    .child(div().p(px(10.))
                        .child(AntIcon::new(AntIconName::WarningOutlined)
                            .theme(theme).size(22.).color(theme.warning))))
            )
            .child(label("Disabled tooltip (hover deliberately suppressed)", theme))
            .child(AntTooltip::new("tooltip-disabled", "This should not appear")
                .theme(theme).disabled(true)
                .child(AntButton::new("tooltip-disabled-button", "No tooltip")
                    .theme(theme)))
            .child(div().text_size(px(12.)).text_color(theme.text_secondary)
                .child("Tooltip positioning and dismissal use GPUI's native overlay. Keyboard focus and all placements still need parity testing."))
            .into_any_element()
    }

    fn divider_demo(&self,t:AntTheme)->gpui_kit::AnyElement {
        div().flex().flex_col().w_full().gap(px(18.))
            .child("Text above")
            .child(AntDivider::new().theme(t))
            .child("Text below")
            .into_any_element()
    }
}

fn row()->gpui_kit::Div {
    div().flex().flex_wrap().items_center().gap(px(12.))
}
fn label(text:&'static str,t:AntTheme)->gpui_kit::Div {
    div().text_size(px(13.)).font_medium().text_color(t.text_secondary).child(text)
}

fn render_resolved_token(token:AntResolvedToken,t:AntTheme)->gpui_kit::AnyElement {
    let value=match token.value {
        AntTokenValue::Color(value)=>div().size(px(26.)).flex_none()
            .rounded(px(5.)).border_1().border_color(t.border).bg(value)
            .into_any_element(),
        AntTokenValue::Number(value)=>div().text_size(px(12.))
            .text_color(t.text_secondary).child(format!("{value:.1} px"))
            .into_any_element(),
        AntTokenValue::Boolean(value)=>div().text_size(px(12.))
            .text_color(t.text_secondary).child(if value {"on"} else {"off"})
            .into_any_element(),
    };
    div().flex().items_center().justify_between().gap(px(8.))
        .flex_1().min_w(px(174.)).h(px(56.))
        .rounded(px(6.)).border_1().border_color(t.border)
        .bg(t.surface_secondary).px(px(11.))
        .child(div().min_w_0().text_color(t.text).text_size(px(12.))
            .child(token.name))
        .child(value)
        .into_any_element()
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t=self.theme();
        let demo=match self.component.as_str() {
            "Button"=>self.button_demo(t,cx),
            name if name.starts_with("ThemeLab") &&
                name.split('|').any(|part|part=="scene=tokens") => self.theme_tokens_demo(t),
            name if name.starts_with("ThemeLab") => self.theme_lab_demo(t,cx),
            "Input"=>self.input_demo(t),
            "Select"=>self.select_demo(t),
            "DatePicker"=>self.date_picker_demo(t),
            "Form"=>self.form_demo(t,cx),
            "Checkbox"=>self.checkbox_demo(t,cx),
            "Radio"=>self.radio_demo(t,cx),
            "Switch"=>self.switch_demo(t,cx),
            "Tag"=>self.tag_demo(t),
            "Badge"=>self.badge_demo(t),
            "Alert"=>self.alert_demo(t),
            "Card"=>self.card_demo(t),
            "Progress"=>self.progress_demo(t),
            "Divider"=>self.divider_demo(t),
            "Tabs"=>self.tabs_demo(t,cx),
            "Tooltip"=>self.tooltip_demo(t),
            "Icons"=>self.icons_demo(t),
            // Internal icon-palette screenshot route; not a separate public component.
            "IconPalette"=>self.icon_palette_demo(t),
            name if name.starts_with("Icon:") =>
                self.icon_single_demo(t, name.trim_start_matches("Icon:")),
            name if name.starts_with("IconAudit:") =>
                self.icon_audit_demo(t, name.trim_start_matches("IconAudit:")),
            "Typography"=>self.typography_demo(t,cx),
            "Space"=>self.space_demo(t),
            "Flex"=>self.flex_demo(t),
            "Grid"=>self.grid_demo(t),
            "Layout"=>self.layout_demo(t,window,cx),
            "LayoutDrawer"=>self.layout_drawer_demo(t,window,cx),
            "Menu"=>self.menu_demo(t,cx),
            "MenuTree"=>self.menu_tree_demo(t,cx),
            "MenuCollapsed"=>self.menu_collapsed_demo(t,cx),
            "Dropdown"=>self.dropdown_demo(t,cx),
            "Popover"=>self.popover_demo(t,cx),
            "Modal"=>self.modal_demo(t,cx),
            "Popconfirm"=>self.popconfirm_demo(t,cx),
            "Avatar"=>self.avatar_demo(t),
            "Statistic"=>self.statistic_demo(t),
            "Empty"=>self.empty_demo(t),
            "Breadcrumb"=>self.breadcrumb_demo(t),
            "Pagination"=>self.pagination_demo(t,cx),
            "Steps"=>self.steps_demo(t,cx),
            "Rate"=>self.rate_demo(t,cx),
            "Collapse"=>self.collapse_demo(t,cx),
            "Descriptions"=>self.descriptions_demo(t),
            "Result"=>self.result_demo(t),
            "Skeleton"=>self.skeleton_demo(t),
            "Spin"=>self.spin_demo(t),
            "InputNumber"=>self.input_number_demo(t),
            "Slider"=>self.slider_demo(t),
            "Segmented"=>self.segmented_demo(t,cx),
            "TimePicker"=>self.time_picker_demo(t),
            "AutoComplete"=>self.autocomplete_demo(t,cx),
            "Drawer"=>self.drawer_demo(t,cx),
            "Message"=>self.message_demo(t),
            "Notification"=>self.notification_demo(t),
            "Tree"=>self.tree_demo(t,cx),
            "Table"=>self.table_demo(t,cx),
            "Timeline"=>self.timeline_demo(t,cx),
            "List"=>self.list_demo(t),
            "Calendar"=>self.calendar_demo(t),
            "FloatButton"=>self.float_button_demo(t,cx),
            _=>self.button_demo(t,cx)
        };
        div().id("gallery-root")
            .size_full().overflow_y_scroll()
            .bg(t.surface)
            .text_color(t.text)
            .font_family("IBM Plex Sans")
            .p(px(24.))
            .child(demo)
    }
}

#[cfg(not(target_family="wasm"))]
pub fn run_desktop() {
    gpui_kit::application()
         .with_assets(GalleryAssets::new())
        .run(|cx| {
            gpui_kit::init(cx);
            gpui_kit::open_window(WindowOptions::default(),cx,|window,cx|{
                cx.new(|cx| Gallery::new("Button".to_owned(),false,window,cx))
            }).expect("Failed to open gallery window");
            cx.activate(true);
        });
}

#[cfg(target_family="wasm")]
mod browser {
    use super::*;
    use gpui_kit::{Application, ApplicationHandle};
    use gpui_kit::web::{CanvasFontFallback, WebBackendPreference, WebPlatform};
    use std::{borrow::Cow,cell::RefCell,rc::Rc,sync::Arc};
    use wasm_bindgen::prelude::*;

    thread_local! {
        static APPLICATION: RefCell<Option<ApplicationHandle>> = const {RefCell::new(None)};
    }

    fn web_application() -> Application {
        let platform=Rc::new(WebPlatform::new_with_backend_and_font_fallback(
            false,WebBackendPreference::Auto,CanvasFontFallback::EmojiAndCjk));
        let client=Arc::new(platform.fetch_http_client());
        Application::with_platform(platform).with_http_client(client)
    }

    /// Main entry point invoked by the docs page after loading WASM bindings.
    #[wasm_bindgen]
    pub fn run(component:Option<String>,dark:Option<bool>)->Result<(),JsValue> {
        console_error_panic_hook::set_once();
        let _=console_log::init_with_level(log::Level::Info);
        gpui_kit::platform::web_init();

        let app=web_application().with_assets(GalleryAssets::new());
        let component=component.unwrap_or_else(||"Button".to_owned());
        let dark=dark.unwrap_or(false);
        APPLICATION.with(|app_handle|{
            *app_handle.borrow_mut()=Some(app.run_embedded(move |cx: &mut App|{
                gpui_kit::init(cx);
                cx.text_system().add_fonts(vec![Cow::Borrowed(
                    include_bytes!("../fonts/IBMPlexSans-Regular.ttf").as_slice()
                )]).expect("Unable to load WASM gallery font");

                gpui_kit::open_window(WindowOptions::default(),cx,move |window,cx|{
                    cx.new(|cx| Gallery::new(component,dark,window,cx))
                }).expect("Unable to open WASM gallery window");
                cx.activate(true);
            }));
        });
        Ok(())
    }
}

#[cfg(test)]
mod icon_preview_tests {
    use super::*;

    #[test]
    fn display_parameter_parser_clamps_and_rejects_bad_colors() {
        let p = IconPreview::parse(
            "CheckCircleTwoTone|size=96|spin=1|primary=52c41a|secondary=f6ffed");
        assert_eq!(p.name, "CheckCircleTwoTone");
        assert_eq!(p.size, 96.);
        assert!(p.spin);
        assert_eq!(p.primary, Some(0x52c41a));
        assert_eq!(p.secondary, Some(0xf6ffed));

        let bad = IconPreview::parse(
            "CheckCircleTwoTone|size=nan|spin=x|primary=ZZZZZZ|secondary=1234567|unknown=x");
        assert_eq!(bad.size, 64.);
        assert!(!bad.spin);
        assert_eq!(bad.primary, None);
        assert_eq!(bad.secondary, None);

        let large = IconPreview::parse("SearchOutlined|size=900000|primary=ff4d4f");
        assert_eq!(large.size, 128.);
        assert_eq!(large.primary, Some(0xff4d4f));
        assert_eq!(IconPreview::parse("SearchOutlined|size=-20").size, 16.);
    }
}
