//! Verify application theme projection and live Kit-backed widget state.
extern crate gpui_kit as gpui;

use gpui_antd::{
    AntInput, AntSeedTokens, AntTab, AntTabs, AntTheme, AntThemeAlgorithm, Appearance,
};
use gpui_kit::base::{Theme as BaseTheme, ThemeAppearance};
use gpui_kit::component::{Theme, ThemeMode, input::InputState};
use gpui_kit::test::TestWindowExt as _;
use gpui_kit::{
    AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, Styled as _,
    TestAppContext, Window, WindowOptions, div, linear_color_stop, linear_gradient, px,
};

#[gpui_kit::test]
fn appearance_colors_backgrounds_and_base_stay_in_sync(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    cx.update(|cx| {
        let custom = AntTheme::from_seed(
            AntSeedTokens {
                color_primary: 0x9a44b2,
                ..AntSeedTokens::dark()
            },
            AntThemeAlgorithm::Dark,
        );
        for ant in [
            AntTheme::light(),
            AntTheme::dark(),
            custom,
            AntTheme::light(),
        ] {
            ant.apply_to_kit(cx);
            let kit = Theme::global(cx);
            assert_eq!(
                kit.mode,
                if ant.appearance == Appearance::Dark {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                }
            );
            assert_eq!(kit.background, ant.surface);
            assert_eq!(kit.foreground, ant.text);
            assert_eq!(kit.primary, ant.primary);
            assert_eq!(kit.primary_hover, ant.primary_hover);
            assert_eq!(kit.primary_foreground, ant.layout_dark_text);
            assert_eq!(kit.tab_foreground, ant.text);
            assert_eq!(kit.tab_active_foreground, ant.primary);
            assert_eq!(kit.tokens.tab_bar.color, ant.surface_secondary);
            assert_eq!(kit.tokens.tab_bar.background, ant.surface_secondary.into());
            assert_eq!(kit.tokens.tab_active.color, ant.surface);
            assert_eq!(kit.tokens.primary.background, ant.primary.into());
            let base = BaseTheme::global(cx);
            assert_eq!(
                base.appearance,
                if ant.appearance == Appearance::Dark {
                    ThemeAppearance::Dark
                } else {
                    ThemeAppearance::Light
                }
            );
            assert_eq!(base.tokens.colors.background, ant.surface);
            assert_eq!(base.tokens.colors.foreground, ant.text);
            assert_eq!(base.tokens.colors.primary, ant.primary);
            assert_eq!(base.tokens.colors.ring, ant.primary);
        }
    });
}

#[gpui_kit::test]
fn reapplying_ant_colors_replaces_a_kit_gradient_with_the_requested_solid(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    cx.update(|cx| {
        let ant = AntTheme::light();
        ant.apply_to_kit(cx);
        let sidebar = linear_gradient(
            135.,
            linear_color_stop(ant.surface, 0.),
            linear_color_stop(ant.primary, 1.),
        );
        Theme::update(cx, |kit| {
            kit.tokens.primary.background = linear_gradient(
                135.,
                linear_color_stop(ant.primary, 0.),
                linear_color_stop(ant.surface, 1.),
            );
            kit.tokens.tab_bar.background = linear_gradient(
                135.,
                linear_color_stop(ant.surface_secondary, 0.),
                linear_color_stop(ant.primary, 1.),
            );
            kit.tokens.sidebar.background = sidebar;
            kit.radius = px(17.);
            kit.font_family = "Application font".into();
            kit.danger = ant.success;
        });
        ant.apply_to_kit(cx);
        let kit = Theme::global(cx);
        assert_eq!(kit.tokens.primary.background, ant.primary.into());
        assert_eq!(kit.tokens.tab_bar.background, ant.surface_secondary.into());
        assert_eq!(kit.tokens.sidebar.background, sidebar);
        assert_eq!(kit.radius, px(17.));
        assert_eq!(kit.font_family.as_ref(), "Application font");
        assert_eq!(kit.danger, ant.success);
    });
}

struct ThemedEditor {
    input: Entity<InputState>,
    selected: usize,
}
impl Render for ThemedEditor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.weak_entity();
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .child(AntInput::new(&self.input))
            .child(
                AntTabs::new("theme-tabs")
                    .active_index(self.selected)
                    .child(AntTab::new("First"))
                    .child(AntTab::new("Second"))
                    .on_change(move |index, _, cx| {
                        owner
                            .update(cx, |view, cx| {
                                view.selected = index;
                                cx.notify();
                            })
                            .unwrap();
                    }),
            )
    }
}

#[gpui_kit::test]
fn live_theme_changes_keep_editor_value_focus_and_tab_activation(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let (handle, view) = cx.update(|cx| {
        AntTheme::light().apply_to_kit(cx);
        gpui_kit::open_window(WindowOptions::default(), cx, |window, cx| {
            cx.new(|cx| ThemedEditor {
                input: cx.new(|cx| InputState::new(window, cx)),
                selected: 0,
            })
        })
        .unwrap()
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let editor = view.read(cx).input.clone();
        let id = ("input", editor.entity_id());
        window.click(id, cx);
        window.input("kept", cx);
        let focus = window.focused(cx);
        let mut expected = String::from("kept");
        for ant in [AntTheme::dark(), AntTheme::light()] {
            ant.apply_to_kit(cx);
            window.render_frame(cx);
            assert_eq!(window.focused(cx), focus);
            assert_eq!(window.find(id).value(), Some(expected.as_str()));
            window.input("!", cx);
            expected.push('!');
        }
        assert_eq!(editor.read(cx).value().as_ref(), "kept!!");
        window.press("tab", cx);
        window.press("right", cx);
        assert_eq!(view.read(cx).selected, 1);
        assert_eq!(window.find(1usize).selected(), Some(true));
    })
    .unwrap();
}
