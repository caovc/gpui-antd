use crate::{AntTheme, Appearance};
use gpui_kit::App;
use gpui_kit::component::{Theme, ThemeMode};

impl AntTheme {
    /// Apply Ant appearance and shared colors to the Kit application theme.
    ///
    /// Call after `gpui_kit::init`, from application initialization or an
    /// explicit theme-change update. Kit-backed controls (Tabs, editors,
    /// calendars and overlays) also read Kit's application theme in addition
    /// to the wrapper's `.theme(t)` presentation.
    ///
    /// This affects **every window in the App**. It is not a scoped provider;
    /// do not call it from a component's render method. Kit owns unmapped
    /// colors, fonts, spacing, radii, shadows and motion. Its supported update
    /// path reconciles solid/background colors and synchronizes Base.
    pub fn apply_to_kit(&self, cx: &mut App) {
        let mode = match self.appearance {
            Appearance::Light => ThemeMode::Light,
            Appearance::Dark => ThemeMode::Dark,
        };
        if Theme::global(cx).mode != mode {
            Theme::change(mode, None, cx);
        }
        Theme::update(cx, |kit| {
            // Ant supplies solid colors. Also replace each background token:
            // assigning an unchanged representative color alone would retain
            // a previous Kit gradient of that color.
            macro_rules! colors {
                ($($role:ident: $value:expr),* $(,)?) => { $(
                    let value = $value;
                    kit.colors.$role = value;
                    kit.tokens.$role = value.into();
                )* };
            }
            colors! {
                background: self.surface,
                foreground: self.text,
                primary: self.primary,
                primary_hover: self.primary_hover,
                primary_active: self.primary_active,
                // Existing bright foreground for strongly colored surfaces.
                primary_foreground: self.layout_dark_text,
                secondary: self.surface_secondary,
                secondary_foreground: self.text,
                secondary_hover: self.primary_bg,
                secondary_active: self.primary_bg,
                muted: self.surface_secondary,
                muted_foreground: self.text_secondary,
                accent: self.primary_bg,
                accent_foreground: self.primary,
                popover: self.bg_elevated,
                popover_foreground: self.text,
                border: self.border,
                input: self.border,
                ring: self.primary,
                selection: self.primary_bg,
                tab: self.surface_secondary,
                tab_bar: self.surface_secondary,
                tab_bar_segmented: self.surface_secondary,
                tab_active: self.surface,
                tab_foreground: self.text,
                tab_active_foreground: self.primary,
            }
        });
    }
}
