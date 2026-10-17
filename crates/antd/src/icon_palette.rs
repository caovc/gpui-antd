//! Runtime palettes for official Ant Design TwoTone SVGs.
//!
//! GPUI `svg()` paints a single alpha mask. TwoTone uses `img()` and a
//! distinct embedded resource key per palette so both fills survive.
//! The original generated SVG bytes are never modified on disk.

use gpui_kit::Hsla;

/// Stable CSS #RRGGBBAA for a GPUI HSLA token (including alpha).
fn rgba_hex(color: Hsla) -> u32 {
    let rgba = color.to_rgb();
    let byte = |v: f32| (v.clamp(0., 1.) * 255.).round() as u32;
    (byte(rgba.r) << 24) | (byte(rgba.g) << 16) | (byte(rgba.b) << 8) | byte(rgba.a)
}

pub(crate) fn palette_asset_key(path: &str, primary: Hsla, secondary: Hsla) -> String {
    let primary = rgba_hex(primary);
    let secondary = rgba_hex(secondary);
    if primary == 0x1677ffff && secondary == 0xe6f4ffff {
        path.to_owned()
    } else {
        format!("{path}?primary={primary:08x}&secondary={secondary:08x}")
    }
}

/// Parse exactly two eight-digit colors from a namespaced TwoTone asset key.
///
/// Other keys are unchanged. Invalid queries fail closed, so arbitrary text
/// cannot be interpolated into SVG markup.
pub(crate) fn parse_palette_asset_key(path: &str) -> Option<(&str, Option<(&str, &str)>)> {
    let Some((path, parameters)) = path.split_once('?') else {
        return Some((path, None));
    };
    if !path.starts_with("antd-icons/twotone/") {
        return None;
    }
    let (primary, secondary) = parameters
        .strip_prefix("primary=")?
        .split_once("&secondary=")?;
    let valid = |value: &str| value.len() == 8 && value.bytes().all(|v| v.is_ascii_hexdigit());
    if !valid(primary) || !valid(secondary) {
        return None;
    }
    Some((path, Some((primary, secondary))))
}

/// Swap only pinned Ant Design palette literals, retaining paths and viewBox.
pub(crate) fn recolor_svg(svg: &[u8], primary: &str, secondary: &str) -> Vec<u8> {
    let svg = std::str::from_utf8(svg).expect("Official Ant Design SVG is UTF-8");
    svg.replace("#1677ff", &format!("#{primary}"))
        .replace("#e6f4ff", &format!("#{secondary}"))
        .into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AntIconAssets, AntIconName, tokens::{AntTheme, color}};
    use gpui_kit::AssetSource as _;

    #[test]
    fn stable_light_dark_custom_and_alpha_keys() {
        let path = AntIconName::CheckCircleTwoTone.path();
        let light = AntTheme::light();
        let dark = AntTheme::dark();
        assert_eq!(palette_asset_key(path, light.primary, light.info_bg), path);
        assert_eq!(
            palette_asset_key(path, dark.primary, dark.info_bg),
            "antd-icons/twotone/check-circle.svg?primary=1668dcff&secondary=111d2cff"
        );
        assert_eq!(
            palette_asset_key(path, color(0x52c41a), color(0xf6ffed)),
            "antd-icons/twotone/check-circle.svg?primary=52c41aff&secondary=f6ffedff"
        );
        assert_eq!(
            palette_asset_key(path, color(0xff0000).alpha(0.5), color(0xffffff)),
            "antd-icons/twotone/check-circle.svg?primary=ff000080&secondary=ffffffff"
        );
    }

    #[test]
    fn malformed_virtual_palette_keys_are_rejected() {
        for invalid in [
            "antd-icons/twotone/check-circle.svg?primary=green&secondary=red",
            "antd-icons/twotone/check-circle.svg?primary=12345678",
            "antd-icons/twotone/check-circle.svg?primary=12345678&secondary=abcdefgh",
            "antd-icons/twotone/check-circle.svg?primary=12345678&secondary=12345678&x=1",
            "antd-icons/outlined/check-circle.svg?primary=12345678&secondary=12345678",
        ] {
            assert!(parse_palette_asset_key(invalid).is_none(), "{invalid}");
        }
    }

    #[test]
    fn all_150_two_tone_assets_load_with_dark_and_custom_palettes() {
        let source = AntIconAssets;
        let dark = AntTheme::dark();
        let custom_primary = color(0x52c41a);
        let custom_secondary = color(0xf6ffed);
        let mut two_tone_count = 0;

        for name in AntIconName::ALL.iter().copied().filter(|name| name.style() == "twotone") {
            two_tone_count += 1;
            for key in [
                name.path().to_owned(),
                palette_asset_key(name.path(), dark.primary, dark.info_bg),
                palette_asset_key(name.path(), custom_primary, custom_secondary),
            ] {
                let bytes = source.load(&key).unwrap().expect(&key);
                let svg = std::str::from_utf8(&bytes).unwrap();
                assert!(svg.contains(r#"viewBox="0 0 1024 1024""#), "{key}");
                assert!(svg.contains(r#"<svg width="128" height="128""#), "{key}");
                assert!(svg.contains("<path "), "{key}");
            }
        }

        assert_eq!(two_tone_count, 150);
        assert_eq!(source.list("antd-icons/").unwrap().len(), 848);
        for name in [
            AntIconName::CarryOutTwoTone,
            AntIconName::CheckCircleTwoTone,
            AntIconName::CheckSquareTwoTone,
            AntIconName::CiCircleTwoTone,
            AntIconName::CiTwoTone,
        ] {
            let key = palette_asset_key(name.path(), dark.primary, dark.info_bg);
            let bytes = source.load(&key).unwrap().unwrap();
            let svg = std::str::from_utf8(&bytes).unwrap();
            assert!(svg.contains(r##"fill="#1668dcff""##), "{key}");
            assert!(svg.contains(r##"fill="#111d2cff""##), "{key}");
            assert!(!svg.contains(r##"fill="#e6f4ff""##), "{key}");
        }
        assert!(source.load(
            "antd-icons/twotone/check-circle.svg?primary=green&secondary=red"
        ).unwrap().is_none());
        assert!(source.load(
            "antd-icons/twotone/missing.svg?primary=1668dcff&secondary=111d2cff"
        ).unwrap().is_none());
    }
}
