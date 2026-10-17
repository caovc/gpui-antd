//! One set of real Rust/GPUI previews for desktop and browser WebAssembly.
extern crate gpui_kit as gpui;

use std::{borrow::Cow, collections::BTreeMap};

use gpui_antd::{
    AntAlert, AntBadge, AntButton, AntCard, AntCheckbox, AntDivider,
    AntInput, AntProgress, AntRadio, AntRadioGroup, AntSelect, AntDatePicker, AntForm, AntFormItem, FormSchema, FormRule, AntSwitch, AntTag, AntTheme, ControlSize, InputStatus, InputVariant, Tone,
    AntIcon, AntIconName, AntIconAssets, AntTitle, AntText, AntParagraph, AntLink, AntTypography, TextTone,
    AntSpace, AntFlex, AntRow, AntCol, SpaceSize, FlexAlign, FlexJustify,
    AntAvatar, AvatarShape, AvatarSize, AntStatistic, AntEmpty, AntBreadcrumb, AntBreadcrumbItem,
    AntTabs, AntTab, TabsVariant, AntTooltip,
    AntLayout, AntSider, AntHeader, AntContent, AntFooter, GridBreakpoint,
};
use gpui_kit::{
    AppContext as _, Context, Entity, InteractiveElement as _, IntoElement, ParentElement as _, Render,
    StatefulInteractiveElement as _, Styled as _, Window, WindowOptions, div, px,
    AssetSource, SharedString,
};
use gpui_kit::component::input::InputState;
use gpui_kit::component::select::{SelectState, SearchableVec};
use gpui_kit::component::date_picker::DatePickerState;
use gpui_kit::component::time_field::TimePrecision;
use gpui_kit::component::IndexPath;
use gpui_kit::component::scroll::ScrollableElement as _;
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
    "Alert", "Card", "Progress", "Divider", "Tabs", "Tooltip", "Icons", "Layout", "Typography", "Space", "Flex", "Grid", "Avatar", "Statistic", "Empty", "Breadcrumb",
];

pub struct Gallery {
    component: String,
    dark: bool,
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
        Self {
            component, dark, button_clicks: 0, checked: true, radio_selected: Some(0),
            radio_button_selected: Some(1), switch_on: true,
            input, input_filled, input_borderless, input_decorated, input_large,
            input_error, input_password, input_disabled,
            select_basic, select_search, select_disabled,
            date_single, date_range, date_time, date_disabled,
            form_name, form_email, form_password, form_confirm,
            form_errors: BTreeMap::new(), form_attempted: false, form_success: false,
            paragraph_expanded: false,
            tabs_index: 0, tabs_card_index: 1, layout_collapsed: false,
        }
    }
    fn theme(&self) -> AntTheme {
        if self.dark {AntTheme::dark()} else {AntTheme::light()}
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

    fn icon_single_demo(&self, t: AntTheme, name: &str) -> gpui_kit::AnyElement {
        let selected=AntIconName::from_ant_name(name);
        div().w_full().flex().flex_col().items_center()
            .justify_center().gap(px(16.)).p(px(18.))
            .when_some(selected, |this, icon| {
                this.child(AntIcon::new(icon).size(64.).theme(t))
                    .child(div().text_color(t.text).text_size(px(18.)).child(name.to_owned()))
                    .child(div().text_color(t.text_secondary).text_size(px(12.))
                        .child(format!("Official SVG · {}",icon.style())))
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
                        div().w(px(156.)).h(px(104.)).flex_none()
                            .flex().flex_col().items_center().justify_center().gap(px(11.))
                            .p(px(10.))
                            .rounded(px(8.)).border_1().border_color(t.border)
                            .child(AntIcon::new(*name).size(32.).theme(t))
                            .child(div().w_full().text_size(px(11.))
                                .text_color(t.text_secondary)
                                .text_center().child(name.ant_name()))
                    }))
        };
        div().w_full().min_w_0().flex().flex_col().gap(px(16.))
            .child(div().text_size(px(12.)).text_color(t.text_secondary)
                .child("Scroll each row sideways to explore 24 icon samples. The entire 848-icon catalog is searchable above."))
            .child(label("Outlined — official Ant Design SVGs", t))
            .child(catalog("outlined"))
            .child(label("Filled", t))
            .child(catalog("filled"))
            .child(label("Two Tone (primary blue and light fill)", t))
            .child(catalog("twotone"))
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

    fn layout_demo(&self, t: AntTheme, window: &Window, cx: &mut Context<Self>) -> gpui_kit::AnyElement {
        let compact=self.layout_collapsed || window.viewport_size().width < px(768.);
        let on_collapse=cx.listener(|this, next: &bool, _, cx| {
            this.layout_collapsed=*next;
            cx.notify();
        });
        let nav=|icon, title: &'static str| {
            div().w_full().h(px(42.)).px(px(17.))
                .flex().items_center().gap(px(12.))
                .text_color(t.layout_dark_text)
                .child(AntIcon::new(icon).theme(t).size(17.).color(t.layout_dark_text))
                .when(!compact, |this| this.child(title))
        };
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
                        .collapsed(self.layout_collapsed).breakpoint(GridBreakpoint::Md)
                        .collapsible(true)
                        .on_collapse(move |next, win, cx| on_collapse(&next, win, cx))
                        .child(div().h(px(60.)).flex().items_center()
                            .px(px(18.)).gap(px(9.))
                            .child(AntIcon::new(AntIconName::AppstoreOutlined)
                                .theme(t).size(21.).color(t.layout_dark_text))
                            .when(!compact, |this| this.child(div().font_semibold()
                                .text_color(t.layout_dark_text).child("GPUI Admin"))))
                        .child(nav(AntIconName::DashboardOutlined, "Dashboard"))
                        .child(nav(AntIconName::BarChartOutlined, "Analytics"))
                        .child(nav(AntIconName::FileTextOutlined, "Documents"))
                        .child(nav(AntIconName::SettingOutlined, "Settings")))
                    .child(AntLayout::new().theme(t)
                        .child(AntHeader::new().theme(t).dark(false)
                            .height(56.).padding(20.)
                            .child(AntIcon::new(AntIconName::MenuOutlined)
                                .size(17.).theme(t))
                            .child(div().ml(px(12.)).font_semibold()
                                .text_color(t.text).child("Dashboard overview")))
                        .child(AntContent::new().theme(t).padding(18.)
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
                                        .child(AntProgress::new(75.).theme(t).show_info(true)))))
                        .child(AntFooter::new().theme(t).padding(12.)
                            .child(AntText::new("GPUI Ant Design · native Rust layout")
                                .theme(t).tone(TextTone::Secondary)))))))
            .child(div().text_size(px(12.)).text_color(t.text_secondary)
                .child(format!("Sider: {} · auto-collapse below md (768px)", if compact { "collapsed" } else { "expanded" })))
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

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t=self.theme();
        let demo=match self.component.as_str() {
            "Button"=>self.button_demo(t,cx),
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
            name if name.starts_with("Icon:") =>
                self.icon_single_demo(t, name.trim_start_matches("Icon:")),
            "Typography"=>self.typography_demo(t,cx),
            "Space"=>self.space_demo(t),
            "Flex"=>self.flex_demo(t),
            "Grid"=>self.grid_demo(t),
            "Layout"=>self.layout_demo(t,window,cx),
            "Avatar"=>self.avatar_demo(t),
            "Statistic"=>self.statistic_demo(t),
            "Empty"=>self.empty_demo(t),
            "Breadcrumb"=>self.breadcrumb_demo(t),
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
