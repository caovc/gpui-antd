//! Small, deterministic and validated parser for the docs Theme Designer.
//!
//! URL config is a transport envelope for a Rust/WASM preview. It is NOT
//! equivalent to React ConfigProvider and it is not persisted by GPUI.
use gpui_antd::{
    AntSeedTokens, AntTheme, AntThemeAlgorithm, AntComponent, AntComponentToken, color,
};

pub struct ThemeLabSpec {
    pub theme:AntTheme,
}
fn hex24(raw:&str)->Option<u32> {
    let s=raw.strip_prefix('#').unwrap_or(raw);
    if s.len()!=6 || !s.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    u32::from_str_radix(s,16).ok()
}
fn scalar(raw:&str,min:f32,max:f32)->Option<f32> {
    raw.parse::<f32>().ok().filter(|value|value.is_finite())
        .map(|value|value.clamp(min,max))
}
impl ThemeLabSpec {
    pub fn parse(component:&str,dark:bool)->Option<Self> {
        let input=component.strip_prefix("ThemeLab")?;
        if !input.is_empty() && !input.starts_with('|') {return None;}
        let mut seed=if dark {AntSeedTokens::dark()} else {AntSeedTokens::default()};
        let mut compact=false;
        let mut button=AntComponentToken::default();
        let mut card=AntComponentToken::default();
        let mut input_style=AntComponentToken::default();
        let mut menu=AntComponentToken::default();
        let mut layout=AntComponentToken::default();
        for raw in input.trim_start_matches('|').split('|').take(32) {
            let Some((key,value))=raw.split_once('=') else {continue};
            if value.len()>64 {continue;}
            match key {
                "colorPrimary"=>if let Some(hex)=hex24(value) {
                    seed.color_primary=hex;seed.color_info=hex;seed.color_link=hex;
                },
                "colorSuccess"=>if let Some(hex)=hex24(value){seed.color_success=hex},
                "colorWarning"=>if let Some(hex)=hex24(value){seed.color_warning=hex},
                "colorError"=>if let Some(hex)=hex24(value){seed.color_error=hex},
                "colorBgBase"=>if let Some(hex)=hex24(value){seed.color_bg_base=hex},
                "colorTextBase"=>if let Some(hex)=hex24(value){seed.color_text_base=hex},
                "radius"=>if let Some(value)=scalar(value,0.,32.){seed.border_radius=value},
                "height"=>if let Some(value)=scalar(value,20.,64.){seed.control_height=value},
                "fontSize"=>if let Some(value)=scalar(value,10.,28.){seed.font_size=value},
                "spacing"=>if let Some(value)=scalar(value,1.,16.){seed.spacing_unit=value},
                "compact"=>compact=value=="1",
                "motion"=>seed.motion=value!="0",
                "buttonPrimary"=>if let Some(hex)=hex24(value){
                    button.color_primary=Some(color(hex));
                },
                "buttonBg"=>if let Some(hex)=hex24(value){button.color_bg=Some(color(hex))},
                "buttonRadius"=>if let Some(v)=scalar(value,0.,32.){button.border_radius=Some(v)},
                "buttonHeight"=>if let Some(v)=scalar(value,20.,64.){button.control_height=Some(v)},
                "cardPadding"=>if let Some(v)=scalar(value,0.,64.){card.padding=Some(v)},
                "cardRadius"=>if let Some(v)=scalar(value,0.,32.){card.border_radius=Some(v)},
                "cardHeaderBg"=>if let Some(hex)=hex24(value){
                    card.color_header_bg=Some(color(hex));
                },
                "inputBg"=>if let Some(hex)=hex24(value){input_style.color_bg=Some(color(hex))},
                "inputRadius"=>if let Some(v)=scalar(value,0.,32.){
                    input_style.border_radius=Some(v)
                },
                "menuSelectedBg"=>if let Some(hex)=hex24(value){
                    menu.color_bg_selected=Some(color(hex));
                },
                "layoutBg"=>if let Some(hex)=hex24(value){
                    layout.color_bg=Some(color(hex));
                },
                _=>(),
            }
        }
        let algorithm=match (dark,compact) {
            (false,false)=>AntThemeAlgorithm::Default,
            (true,false)=>AntThemeAlgorithm::Dark,
            (false,true)=>AntThemeAlgorithm::Compact,
            (true,true)=>AntThemeAlgorithm::DarkCompact,
        };
        let theme=AntTheme::from_seed(seed,algorithm)
            .with_component(AntComponent::Button,button)
            .with_component(AntComponent::Card,card)
            .with_component(AntComponent::Input,input_style)
            .with_component(AntComponent::Menu,menu)
            .with_component(AntComponent::Layout,layout);
        Some(Self{theme})
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn designer_theme_uses_rust_seeds_and_scoped_overrides() {
        let theme=ThemeLabSpec::parse(
            "ThemeLab|colorPrimary=18b984|radius=11|height=40|fontSize=16|compact=1|buttonRadius=18|cardPadding=35|inputBg=fffcee|menuSelectedBg=114488",
            false).unwrap().theme;
        assert_eq!(theme.primary,color(0x18b984));
        assert_eq!(theme.border_radius,11.);
        assert_eq!(theme.control_height,36.);
        assert_eq!(theme.font_size,16.);
        assert_eq!(theme.component(AntComponent::Button).border_radius,Some(18.));
        assert_eq!(theme.component(AntComponent::Card).padding,Some(35.));
        assert_eq!(theme.component(AntComponent::Input).color_bg,Some(color(0xfffcee)));
        assert_eq!(theme.component(AntComponent::Menu).color_bg_selected,Some(color(0x114488)));
    }
    #[test]
    fn parse_rejects_injection_and_nonfinite_inputs() {
        assert!(ThemeLabSpec::parse("ThemeLab",false).is_some());
        assert!(ThemeLabSpec::parse("Other|colorPrimary=ff0000",false).is_none());
        let t=ThemeLabSpec::parse("ThemeLab|radius=nan|height=inf|buttonBg=z99|unknown=value",true)
            .unwrap().theme;
        assert_eq!(t.border_radius,6.);
        assert_eq!(t.control_height,32.);
        assert_eq!(t.component(AntComponent::Button).color_bg,None);
    }
}
