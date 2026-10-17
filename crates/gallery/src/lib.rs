//! One set of real Rust/GPUI previews for desktop and browser WebAssembly.
extern crate gpui_kit as gpui;

use gpui_antd::{
    AntAlert, AntBadge, AntButton, AntCard, AntCheckbox, AntDivider,
    AntInput, AntProgress, AntRadio, AntRadioGroup, AntSwitch, AntTag, AntTheme, ControlSize, InputStatus, InputVariant, Tone,
};
use gpui_kit::{
    AppContext as _, Context, Entity, InteractiveElement as _, IntoElement, ParentElement as _, Render,
    StatefulInteractiveElement as _, Styled as _, Window, WindowOptions, div, px,
};
use gpui_kit::component::input::InputState;
use gpui_kit::assets::Assets;
use gpui_kit::base::StyledExt as _;
#[cfg(target_family = "wasm")]
use gpui_kit::App;

pub const COMPONENTS: &[&str] = &[
    "Button", "Input", "Checkbox", "Radio", "Switch", "Tag", "Badge",
    "Alert", "Card", "Progress", "Divider",
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
        Self {
            component, dark, button_clicks: 0, checked: true, radio_selected: Some(0),
            radio_button_selected: Some(1), switch_on: true,
            input, input_filled, input_borderless, input_decorated, input_large,
            input_error, input_password, input_disabled,
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t=self.theme();
        let demo=match self.component.as_str() {
            "Button"=>self.button_demo(t,cx),
            "Input"=>self.input_demo(t),
            "Checkbox"=>self.checkbox_demo(t,cx),
            "Radio"=>self.radio_demo(t,cx),
            "Switch"=>self.switch_demo(t,cx),
            "Tag"=>self.tag_demo(t),
            "Badge"=>self.badge_demo(t),
            "Alert"=>self.alert_demo(t),
            "Card"=>self.card_demo(t),
            "Progress"=>self.progress_demo(t),
            "Divider"=>self.divider_demo(t),
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
        .with_assets(Assets)
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

        let app=web_application().with_assets(Assets::new("https://gpui-kit.com/gallery/"));
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
