//! Ant Design-inspired design-token system shared by desktop and WASM.
//!
//! Mirrors the official Seed -> Map -> Alias -> Component relationship,
//! not React's CSS-in-JS or a bit-for-bit reproduction of Ant's algorithms.
//! `AntTheme::from_seed` resolves native GPUI Hsla colors; per-component
//! overrides are typed, copyable and retain their own namespace.
//! Token scopes require explicit `.theme(theme)` until ConfigProvider ships.

use gpui_kit::Hsla;
use crate::tokens::{AntTheme, Appearance, color};

/// The four supported combinations of light/dark and regular/compact.
/// These are approximate GPUI-native derivations, not the React JS algorithm.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AntThemeAlgorithm {
    #[default] Default,
    Dark,
    Compact,
    DarkCompact,
}
impl AntThemeAlgorithm {
    pub const fn is_dark(self)->bool { matches!(self,Self::Dark|Self::DarkCompact) }
    pub const fn is_compact(self)->bool { matches!(self,Self::Compact|Self::DarkCompact) }
}

/// First-class Seed Tokens. Hex colors are RGB integers, not Hsla; this
/// keeps theme configs reproducible and easy to serialize to Rust/JSON.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AntSeedTokens {
    pub color_primary:u32,
    pub color_success:u32,
    pub color_warning:u32,
    pub color_error:u32,
    pub color_info:u32,
    pub color_link:u32,
    pub color_bg_base:u32,
    pub color_text_base:u32,
    pub control_height:f32,
    pub border_radius:f32,
    pub font_size:f32,
    pub spacing_unit:f32,
    pub motion:bool,
}
impl Default for AntSeedTokens {
    fn default()->Self {
        Self {
            color_primary:0x1677ff, color_success:0x52c41a,
            color_warning:0xfaad14, color_error:0xff4d4f,
            color_info:0x1677ff,color_link:0x1677ff,
            color_bg_base:0xffffff,color_text_base:0x262626,
            control_height:32.,border_radius:6.,font_size:14.,
            spacing_unit:4.,motion:true,
        }
    }
}
impl AntSeedTokens {
    pub fn dark()->Self {
        Self { color_primary:0x1668dc,color_success:0x49aa19,
            color_warning:0xd89614,color_error:0xdc4446,
            color_info:0x1668dc,color_link:0x3c89e8,
            color_bg_base:0x141414,color_text_base:0xf3f3f3,
            ..Self::default() }
    }
}

/// One explicit Component Token override namespace.
///
/// Fields left None inherit resolved global Map/Alias tokens. The fields
/// are shared shape values; consuming components only support the fields
/// listed in their own API/compatibility documentation.
#[derive(Clone,Copy,Debug,Default,PartialEq)]
pub struct AntComponentToken {
    pub color_primary:Option<Hsla>,
    pub color_bg:Option<Hsla>,
    pub color_text:Option<Hsla>,
    pub color_border:Option<Hsla>,
    pub color_bg_hover:Option<Hsla>,
    pub color_bg_selected:Option<Hsla>,
    pub color_header_bg:Option<Hsla>,
    pub border_radius:Option<f32>,
    pub control_height:Option<f32>,
    pub font_size:Option<f32>,
    pub padding:Option<f32>,
}
impl AntComponentToken {
    pub fn color_primary(mut self,value:Hsla)->Self{self.color_primary=Some(value);self}
    pub fn color_bg(mut self,value:Hsla)->Self{self.color_bg=Some(value);self}
    pub fn color_text(mut self,value:Hsla)->Self{self.color_text=Some(value);self}
    pub fn color_border(mut self,value:Hsla)->Self{self.color_border=Some(value);self}
    pub fn color_bg_hover(mut self,value:Hsla)->Self{self.color_bg_hover=Some(value);self}
    pub fn color_bg_selected(mut self,value:Hsla)->Self{self.color_bg_selected=Some(value);self}
    pub fn color_header_bg(mut self,value:Hsla)->Self{self.color_header_bg=Some(value);self}
    pub fn border_radius(mut self,value:f32)->Self{
        self.border_radius=Some(value.max(0.));self
    }
    pub fn control_height(mut self,value:f32)->Self{
        self.control_height=Some(value.max(0.));self
    }
    pub fn font_size(mut self,value:f32)->Self{
        self.font_size=Some(value.max(8.));self
    }
    pub fn padding(mut self,value:f32)->Self{
        self.padding=Some(value.max(0.));self
    }
}

/// Public, stable Ant Design component token namespaces. Not all component
/// overrides have connected styling yet; see the per-component parity map.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
#[repr(usize)]
pub enum AntComponent {
    Button,
    Input,
    Select,
    DatePicker,
    Form,
    Checkbox,
    Radio,
    Switch,
    Tag,
    Badge,
    Alert,
    Card,
    Progress,
    Divider,
    Tabs,
    Tooltip,
    Icons,
    Typography,
    Space,
    Flex,
    Grid,
    Layout,
    Menu,
    Dropdown,
    Popover,
    Modal,
    Popconfirm,
    Avatar,
    Statistic,
    Empty,
    Breadcrumb,
}
impl AntComponent {
    pub const ALL:[Self;31]=[Self::Button, Self::Input, Self::Select, Self::DatePicker, Self::Form, Self::Checkbox, Self::Radio, Self::Switch, Self::Tag, Self::Badge, Self::Alert, Self::Card, Self::Progress, Self::Divider, Self::Tabs, Self::Tooltip, Self::Icons, Self::Typography, Self::Space, Self::Flex, Self::Grid, Self::Layout, Self::Menu, Self::Dropdown, Self::Popover, Self::Modal, Self::Popconfirm, Self::Avatar, Self::Statistic, Self::Empty, Self::Breadcrumb];
    pub const fn name(self)->&'static str {
        match self {
            Self::Button => "Button",
            Self::Input => "Input",
            Self::Select => "Select",
            Self::DatePicker => "DatePicker",
            Self::Form => "Form",
            Self::Checkbox => "Checkbox",
            Self::Radio => "Radio",
            Self::Switch => "Switch",
            Self::Tag => "Tag",
            Self::Badge => "Badge",
            Self::Alert => "Alert",
            Self::Card => "Card",
            Self::Progress => "Progress",
            Self::Divider => "Divider",
            Self::Tabs => "Tabs",
            Self::Tooltip => "Tooltip",
            Self::Icons => "Icons",
            Self::Typography => "Typography",
            Self::Space => "Space",
            Self::Flex => "Flex",
            Self::Grid => "Grid",
            Self::Layout => "Layout",
            Self::Menu => "Menu",
            Self::Dropdown => "Dropdown",
            Self::Popover => "Popover",
            Self::Modal => "Modal",
            Self::Popconfirm => "Popconfirm",
            Self::Avatar => "Avatar",
            Self::Statistic => "Statistic",
            Self::Empty => "Empty",
            Self::Breadcrumb => "Breadcrumb",
        }
    }
    pub fn from_name(name:&str)->Option<Self> {
        match name {
            "Button" => Some(Self::Button),
            "Input" => Some(Self::Input),
            "Select" => Some(Self::Select),
            "DatePicker" => Some(Self::DatePicker),
            "Form" => Some(Self::Form),
            "Checkbox" => Some(Self::Checkbox),
            "Radio" => Some(Self::Radio),
            "Switch" => Some(Self::Switch),
            "Tag" => Some(Self::Tag),
            "Badge" => Some(Self::Badge),
            "Alert" => Some(Self::Alert),
            "Card" => Some(Self::Card),
            "Progress" => Some(Self::Progress),
            "Divider" => Some(Self::Divider),
            "Tabs" => Some(Self::Tabs),
            "Tooltip" => Some(Self::Tooltip),
            "Icons" => Some(Self::Icons),
            "Typography" => Some(Self::Typography),
            "Space" => Some(Self::Space),
            "Flex" => Some(Self::Flex),
            "Grid" => Some(Self::Grid),
            "Layout" => Some(Self::Layout),
            "Menu" => Some(Self::Menu),
            "Dropdown" => Some(Self::Dropdown),
            "Popover" => Some(Self::Popover),
            "Modal" => Some(Self::Modal),
            "Popconfirm" => Some(Self::Popconfirm),
            "Avatar" => Some(Self::Avatar),
            "Statistic" => Some(Self::Statistic),
            "Empty" => Some(Self::Empty),
            "Breadcrumb" => Some(Self::Breadcrumb),
            _=>None,
        }
    }
}

/// Copyable per-component overrides. Each Ant Design component has a
/// separate slot; changing Card.padding does not affect Button.padding.
#[derive(Clone,Copy,Debug)]
pub struct AntComponentTokens {
    entries:[AntComponentToken;31],
}
impl Default for AntComponentTokens {
    fn default()->Self{
        Self {entries:[AntComponentToken::default();31]}
    }
}
impl AntComponentTokens {
    pub fn get(&self,name:AntComponent)->AntComponentToken {
        self.entries[name as usize]
    }
    pub fn set(&mut self,name:AntComponent,token:AntComponentToken) {
        self.entries[name as usize]=token;
    }
    pub fn with(mut self,name:AntComponent,token:AntComponentToken)->Self {
        self.set(name,token);self
    }
}

/// Derived Map Token snapshot (non-exhaustive, extend alongside actual
/// component adoption). Aliases and component properties derive from these.
#[derive(Clone,Copy,Debug)]
pub struct AntMapTokens {
    pub color_primary:Hsla,
    pub color_primary_hover:Hsla,
    pub color_primary_active:Hsla,
    pub color_bg_container:Hsla,
    pub color_bg_elevated:Hsla,
    pub color_border:Hsla,
    pub border_radius_sm:f32,
    pub border_radius_lg:f32,
    pub control_height_sm:f32,
    pub control_height_lg:f32,
    pub font_size:f32,
    pub motion:bool,
}
/// Derived Alias Tokens consumed across multiple components.
#[derive(Clone,Copy,Debug)]
pub struct AntAliasTokens {
    pub color_text:Hsla,
    pub color_text_secondary:Hsla,
    pub color_link:Hsla,
    pub color_success:Hsla,
    pub color_warning:Hsla,
    pub color_error:Hsla,
    pub padding:f32,
    pub font_size:f32,
    pub border_radius:f32,
}

fn mix_rgb(a:u32,b:u32,blend:f32)->u32 {
    let amt=blend.clamp(0.,1.);
    let channel=|shift:u32| {
        let x=((a>>shift)&255) as f32;
        let y=((b>>shift)&255) as f32;
        ((x*(1.-amt)+y*amt).round() as u32).min(255)
    };
    (channel(16)<<16)|(channel(8)<<8)|channel(0)
}
impl AntTheme {
    /// Resolve Seed -> Map/Alias values for the selected native algorithm.
    /// The default primary is #1677ff and regular/compact heights are 32/28.
    pub fn from_seed(seed:AntSeedTokens,algorithm:AntThemeAlgorithm)->Self {
        let dark=algorithm.is_dark();
        let mut theme=if dark {Self::dark()} else {Self::light()};
        theme.appearance=if dark {Appearance::Dark} else {Appearance::Light};
        let primary=seed.color_primary;
        let surface=seed.color_bg_base;
        let text=seed.color_text_base;
        theme.primary=color(primary);
        theme.primary_hover=color(mix_rgb(primary,if dark {0xffffff} else {0xffffff},
            if dark {0.26} else {0.19}));
        theme.primary_active=color(mix_rgb(primary,0x000000,
            if dark {0.10} else {0.17}));
        theme.success=color(seed.color_success);
        theme.warning=color(seed.color_warning);
        theme.error=color(seed.color_error);
        theme.color_info=color(seed.color_info);
        theme.color_link=color(seed.color_link);
        theme.surface=color(surface);
        theme.surface_secondary=color(mix_rgb(surface,text,if dark {0.075} else {0.025}));
        theme.bg_elevated=color(mix_rgb(surface,if dark {0xffffff}else{0xffffff},
            if dark {0.06} else {0.}));
        theme.text=color(text);
        theme.text_secondary=color(mix_rgb(text,surface,if dark {0.28} else {0.27}));
        theme.border=color(mix_rgb(surface,text,if dark {0.20} else {0.17}));
        theme.disabled_bg=color(mix_rgb(surface,text,if dark {0.11} else {0.045}));
        theme.disabled_text=color(mix_rgb(text,surface,0.54));
        theme.info_bg=color(mix_rgb(seed.color_info,surface,if dark {0.87} else {0.88}));
        theme.success_bg=color(mix_rgb(seed.color_success,surface,0.92));
        theme.warning_bg=color(mix_rgb(seed.color_warning,surface,0.91));
        theme.error_bg=color(mix_rgb(seed.color_error,surface,0.92));
        theme.border_radius=seed.border_radius.clamp(0.,32.);
        theme.border_radius_sm=(theme.border_radius-2.).max(0.);
        theme.border_radius_lg=theme.border_radius+2.;
        theme.font_size=seed.font_size.clamp(10.,28.);
        theme.padding=seed.spacing_unit.clamp(1.,16.)*4.;
        theme.control_height=(seed.control_height-if algorithm.is_compact(){4.}else{0.}).clamp(20.,64.);
        theme.control_height_sm=(theme.control_height-8.).max(18.);
        theme.control_height_lg=theme.control_height+8.;
        theme.motion=seed.motion;
        theme
    }
    pub fn map_tokens(self)->AntMapTokens{
        AntMapTokens {
            color_primary:self.primary,
            color_primary_hover:self.primary_hover,
            color_primary_active:self.primary_active,
            color_bg_container:self.surface,
            color_bg_elevated:self.bg_elevated,
            color_border:self.border,
            border_radius_sm:self.border_radius_sm,
            border_radius_lg:self.border_radius_lg,
            control_height_sm:self.control_height_sm,
            control_height_lg:self.control_height_lg,
            font_size:self.font_size,
            motion:self.motion,
        }
    }
    pub fn alias_tokens(self)->AntAliasTokens {
        AntAliasTokens {
            color_text:self.text,color_text_secondary:self.text_secondary,
            color_link:self.color_link,color_success:self.success,
            color_warning:self.warning,color_error:self.error,
            padding:self.padding,font_size:self.font_size,
            border_radius:self.border_radius,
        }
    }
    pub fn component(self,name:AntComponent)->AntComponentToken {
        self.components.get(name)
    }
    pub fn with_component(mut self,name:AntComponent,token:AntComponentToken)->Self {
        self.components.set(name,token);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_include_each_documented_component_without_collisions() {
        let keys:std::collections::HashSet<&str>=AntComponent::ALL.iter()
            .map(|kind|kind.name()).collect();
        assert_eq!(keys.len(),31);
        for kind in AntComponent::ALL {
            assert_eq!(AntComponent::from_name(kind.name()),Some(kind));
        }
        assert_eq!(AntComponent::from_name("ReactOnly"),None);
    }
    #[test]
    fn all_theme_algorithms_resolve_seed_values() {
        let mut seed=AntSeedTokens::default();
        seed.color_primary=0x15aa77;
        seed.border_radius=12.;
        let a=AntTheme::from_seed(seed,AntThemeAlgorithm::Default);
        assert_eq!(a.primary,color(0x15aa77));
        assert_eq!(a.border_radius,12.);
        assert_eq!(a.border_radius_sm,10.);
        assert_eq!(a.border_radius_lg,14.);
        assert_eq!(a.control_height,32.);
        assert_eq!(a.appearance,Appearance::Light);
        let compact=AntTheme::from_seed(seed,AntThemeAlgorithm::Compact);
        assert_eq!(compact.control_height,28.);
        let dark=AntTheme::from_seed(AntSeedTokens::dark(),AntThemeAlgorithm::DarkCompact);
        assert_eq!(dark.appearance,Appearance::Dark);
        assert_eq!(dark.control_height,28.);
        assert_eq!(dark.map_tokens().control_height_sm,20.);
    }
    #[test]
    fn component_overrides_are_scoped() {
        let theme=AntTheme::light().with_component(
            AntComponent::Card,AntComponentToken::default().padding(32.).border_radius(16.)
        );
        assert_eq!(theme.component(AntComponent::Card).padding,Some(32.));
        assert_eq!(theme.component(AntComponent::Card).border_radius,Some(16.));
        assert_eq!(theme.component(AntComponent::Button).padding,None);
    }
    #[test]
    fn invalid_scalar_values_are_clamped() {
        let seed=AntSeedTokens{border_radius:-45.,control_height:4.,
            font_size:500.,spacing_unit:-6.,..AntSeedTokens::default()};
        let t=AntTheme::from_seed(seed,AntThemeAlgorithm::Compact);
        assert_eq!(t.border_radius,0.);
        assert_eq!(t.control_height,20.);
        assert_eq!(t.font_size,28.);
        assert_eq!(t.padding,4.);
    }
}
