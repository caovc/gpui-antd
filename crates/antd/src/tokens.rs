//! Ant Design-inspired semantic color and geometry tokens.
//! Defaults align with the familiar Ant Design 5/6 light palette.

use gpui_kit::{Hsla, rgb};
use crate::design_tokens::AntComponentTokens;

/// GPUI paints in HSLA. Keeping a single conversion routine makes token
/// expressions usable with `.bg`, `.border_color` and `.text_color`.
pub fn color(hex: u32) -> Hsla {
    rgb(hex).into()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appearance {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug)]
pub struct AntTheme {
    pub appearance: Appearance,
    pub primary: Hsla,
    pub primary_hover: Hsla,
    pub primary_active: Hsla,
    pub text: Hsla,
    pub text_secondary: Hsla,
    pub surface: Hsla,
    pub surface_secondary: Hsla,
    pub border: Hsla,
    pub disabled_bg: Hsla,
    pub disabled_text: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub error: Hsla,
    pub info_bg: Hsla,
    pub success_bg: Hsla,
    pub warning_bg: Hsla,
    pub error_bg: Hsla,
    pub layout_body: Hsla,
    pub layout_dark: Hsla,
    pub layout_dark_text: Hsla,
    pub layout_sider_hover: Hsla,
    pub control_height: f32,
    pub control_height_sm: f32,
    pub control_height_lg: f32,
    pub border_radius: f32,
    pub border_radius_sm: f32,
    pub border_radius_lg: f32,
    pub font_size: f32,
    pub padding: f32,
    pub bg_elevated: Hsla,
    pub color_info: Hsla,
    pub color_link: Hsla,
    pub motion: bool,
    pub components: AntComponentTokens,
}

impl Default for AntTheme {
    fn default() -> Self { Self::light() }
}

impl AntTheme {
    pub fn light() -> Self {
        Self {
            appearance: Appearance::Light,
            primary: color(0x1677ff),
            primary_hover: color(0x4096ff),
            primary_active: color(0x0958d9),
            text: color(0x262626),
            text_secondary: color(0x595959),
            surface: color(0xffffff),
            surface_secondary: color(0xfafafa),
            border: color(0xd9d9d9),
            disabled_bg: color(0xf5f5f5),
            disabled_text: color(0xbfbfbf),
            success: color(0x52c41a),
            warning: color(0xfaad14),
            error: color(0xff4d4f),
            info_bg: color(0xe6f4ff),
            success_bg: color(0xf6ffed),
            warning_bg: color(0xfffbe6),
            error_bg: color(0xfff2f0),
            layout_body: color(0xf5f5f5),
            layout_dark: color(0x001529),
            layout_dark_text: color(0xffffff),
            layout_sider_hover: color(0x0c2b45),
            control_height_sm: 24.,
            control_height_lg: 40.,
            border_radius_sm: 4.,
            border_radius_lg: 8.,
            font_size: 14.,
            padding: 16.,
            bg_elevated: color(0xffffff),
            color_info: color(0x1677ff),
            color_link: color(0x1677ff),
            motion: true,
            components: AntComponentTokens::default(),
            control_height: 32.0,
            border_radius: 6.0,
        }
    }

    pub fn dark() -> Self {
        Self {
            appearance: Appearance::Dark,
            primary: color(0x1668dc),
            primary_hover: color(0x3c89e8),
            primary_active: color(0x1554ad),
            text: color(0xf3f3f3),
            text_secondary: color(0xbfbfbf),
            surface: color(0x141414),
            surface_secondary: color(0x1f1f1f),
            border: color(0x424242),
            disabled_bg: color(0x303030),
            disabled_text: color(0x737373),
            success: color(0x49aa19),
            warning: color(0xd89614),
            error: color(0xdc4446),
            info_bg: color(0x111d2c),
            success_bg: color(0x162312),
            warning_bg: color(0x2b2111),
            error_bg: color(0x2c1618),
            layout_body: color(0x000000),
            layout_dark: color(0x111a2c),
            layout_dark_text: color(0xf3f3f3),
            layout_sider_hover: color(0x1f314d),
            control_height_sm: 24.,
            control_height_lg: 40.,
            border_radius_sm: 4.,
            border_radius_lg: 8.,
            font_size: 14.,
            padding: 16.,
            bg_elevated: color(0x1f1f1f),
            color_info: color(0x1668dc),
            color_link: color(0x3c89e8),
            motion: true,
            components: AntComponentTokens::default(),
            control_height: 32.0,
            border_radius: 6.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn token_sizes_match_ant_design_defaults() {
        assert_eq!(AntTheme::light().control_height, 32.0);
        assert_eq!(AntTheme::light().border_radius, 6.0);
        assert_eq!(AntTheme::dark().appearance, Appearance::Dark);
    }
}
