//! Small, deterministic and validated parser for the docs Theme Designer.
//!
//! URL config is a transport envelope for a Rust/WASM preview. It is NOT
//! equivalent to React ConfigProvider and it is not persisted by GPUI.
use gpui_antd::{
    AntSeedTokens, AntTheme, AntThemeAlgorithm, AntComponent, AntComponentToken, AntComponentTokens, AntMapTokenOverrides,
    AntAliasTokenOverrides, color,
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
        let mut component_tokens=AntComponentTokens::default();
        let mut map_overrides=AntMapTokenOverrides::default();
        let mut alias_overrides=AntAliasTokenOverrides::default();
        for raw in input.trim_start_matches('|').split('|').take(512) {
            let Some((key,value))=raw.split_once('=') else {continue};
            if value.len()>64 {continue;}
            match key {
                "colorPrimary"=>if let Some(hex)=hex24(value) {
                    seed.color_primary=hex;seed.color_info=hex;seed.color_link=hex;
                },
                "colorInfo"=>if let Some(hex)=hex24(value){seed.color_info=hex},
                "colorLink"=>if let Some(hex)=hex24(value){seed.color_link=hex},
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
                _=>{
                    // One namespace per documented component. The registry
                    // stores all 31; runtime consumers are rolled out
                    // incrementally and documented on the parity page.
                    if let Some((name,field))=key.split_once('.') {
                        if name=="Map" {
                            match field {
                                "colorPrimaryHover"=>if let Some(v)=hex24(value){
                                    map_overrides.color_primary_hover=Some(color(v))
                                },
                                "colorPrimaryActive"=>if let Some(v)=hex24(value){
                                    map_overrides.color_primary_active=Some(color(v))
                                },
                                "colorPrimaryBg"=>if let Some(v)=hex24(value){
                                    map_overrides.color_primary_bg=Some(color(v))
                                },
                                "colorPrimaryBorder"=>if let Some(v)=hex24(value){
                                    map_overrides.color_primary_border=Some(color(v))
                                },
                                "colorBgElevated"=>if let Some(v)=hex24(value){
                                    map_overrides.color_bg_elevated=Some(color(v))
                                },
                                "colorBgLayout"=>if let Some(v)=hex24(value){
                                    map_overrides.color_bg_layout=Some(color(v))
                                },
                                "controlHeightSM"=>if let Some(v)=scalar(value,18.,64.){
                                    map_overrides.control_height_sm=Some(v)
                                },
                                "controlHeightLG"=>if let Some(v)=scalar(value,20.,72.){
                                    map_overrides.control_height_lg=Some(v)
                                },
                                _=>(),
                            }
                            continue;
                        }
                        if name=="Alias" {
                            match field {
                                "colorLink"=>if let Some(v)=hex24(value){
                                    alias_overrides.color_link=Some(color(v))
                                },
                                "colorTextDisabled"=>if let Some(v)=hex24(value){
                                    alias_overrides.color_text_disabled=Some(color(v))
                                },
                                "colorSuccess"=>if let Some(v)=hex24(value){
                                    alias_overrides.color_success=Some(color(v))
                                },
                                "colorWarning"=>if let Some(v)=hex24(value){
                                    alias_overrides.color_warning=Some(color(v))
                                },
                                "colorError"=>if let Some(v)=hex24(value){
                                    alias_overrides.color_error=Some(color(v))
                                },
                                _=>(),
                            }
                            continue;
                        }
                        if let Some(component)=AntComponent::from_name(name) {
                            let mut tokens=component_tokens.get(component);
                            match field {
                                "colorPrimary"=>if let Some(v)=hex24(value){
                                    tokens.color_primary=Some(color(v))
                                },
                                "colorBg"=>if let Some(v)=hex24(value){
                                    tokens.color_bg=Some(color(v))
                                },
                                "colorText"=>if let Some(v)=hex24(value){
                                    tokens.color_text=Some(color(v))
                                },
                                "colorBorder"=>if let Some(v)=hex24(value){
                                    tokens.color_border=Some(color(v))
                                },
                                "colorBgHover"=>if let Some(v)=hex24(value){
                                    tokens.color_bg_hover=Some(color(v))
                                },
                                "colorBgSelected"=>if let Some(v)=hex24(value){
                                    tokens.color_bg_selected=Some(color(v))
                                },
                                "colorHeaderBg"=>if let Some(v)=hex24(value){
                                    tokens.color_header_bg=Some(color(v))
                                },
                                "borderRadius"=>if let Some(v)=scalar(value,0.,32.){
                                    tokens.border_radius=Some(v)
                                },
                                "controlHeight"=>if let Some(v)=scalar(value,20.,64.){
                                    tokens.control_height=Some(v)
                                },
                                "fontSize"=>if let Some(v)=scalar(value,8.,32.){
                                    tokens.font_size=Some(v)
                                },
                                "padding"=>if let Some(v)=scalar(value,0.,64.){
                                    tokens.padding=Some(v)
                                },
                                _=>(),
                            }
                            component_tokens.set(component,tokens);
                        }
                    }
                },
            }
        }
        let algorithm=match (dark,compact) {
            (false,false)=>AntThemeAlgorithm::Default,
            (true,false)=>AntThemeAlgorithm::Dark,
            (false,true)=>AntThemeAlgorithm::Compact,
            (true,true)=>AntThemeAlgorithm::DarkCompact,
        };
        let mut theme=AntTheme::from_seed(seed,algorithm)
            .with_map_overrides(map_overrides)
            .with_alias_overrides(alias_overrides)
            .with_component(AntComponent::Button,button)
            .with_component(AntComponent::Card,card)
            .with_component(AntComponent::Input,input_style)
            .with_component(AntComponent::Menu,menu)
            .with_component(AntComponent::Layout,layout);
        for component in AntComponent::ALL {
            let patch=component_tokens.get(component);
            let current=theme.component(component);
            // URL-delivered per-component overrides win over legacy
            // design-lab demo flags, without cross-component bleeding.
            theme=theme.with_component(component,AntComponentToken {
                color_primary:patch.color_primary.or(current.color_primary),
                color_bg:patch.color_bg.or(current.color_bg),
                color_text:patch.color_text.or(current.color_text),
                color_border:patch.color_border.or(current.color_border),
                color_bg_hover:patch.color_bg_hover.or(current.color_bg_hover),
                color_bg_selected:patch.color_bg_selected.or(current.color_bg_selected),
                color_header_bg:patch.color_header_bg.or(current.color_header_bg),
                border_radius:patch.border_radius.or(current.border_radius),
                control_height:patch.control_height.or(current.control_height),
                font_size:patch.font_size.or(current.font_size),
                padding:patch.padding.or(current.padding),
            });
        }
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
    fn generic_component_namespace_supports_every_catalog_entry() {
        let t=ThemeLabSpec::parse(
            "ThemeLab|Button.colorPrimary=ff7711|Card.padding=33|Menu.colorBgSelected=007755|Badge.colorBg=abcdef",
            false).unwrap().theme;
        assert_eq!(t.component(AntComponent::Button).color_primary,Some(color(0xff7711)));
        assert_eq!(t.component(AntComponent::Card).padding,Some(33.));
        assert_eq!(t.component(AntComponent::Menu).color_bg_selected,Some(color(0x007755)));
        assert_eq!(t.component(AntComponent::Badge).color_bg,Some(color(0xabcdef)));
        assert_eq!(t.component(AntComponent::Alert).color_bg,None);
    }
    #[test]
    fn independent_info_link_seeds_and_large_component_catalog_survive() {
        let values=std::iter::repeat_n("Unknown.colorBg=eeeeee",155)
            .collect::<Vec<_>>().join("|");
        let route=format!("ThemeLab|colorPrimary=1677ff|colorInfo=114488|colorLink=beef12|{values}|Tooltip.borderRadius=13");
        let theme=ThemeLabSpec::parse(&route,false).unwrap().theme;
        assert_eq!(theme.color_info,color(0x114488));
        assert_eq!(theme.color_link,color(0xbeef12));
        assert_eq!(theme.component(AntComponent::Tooltip).border_radius,Some(13.));
    }
    #[test]
    fn map_and_alias_overrides_apply_after_seed_and_before_components() {
        let theme=ThemeLabSpec::parse(
            "ThemeLab|colorPrimary=1677ff|colorLink=112233|Map.colorPrimaryBg=f0faee|Map.controlHeightSM=23|Alias.colorLink=aabbcc|Alias.colorTextDisabled=777777|Button.colorPrimary=0055aa",
            false).unwrap().theme;
        assert_eq!(theme.primary,color(0x1677ff));
        assert_eq!(theme.map_tokens().color_primary_bg,color(0xf0faee));
        assert_eq!(theme.map_tokens().control_height_sm,23.);
        assert_eq!(theme.alias_tokens().color_link,color(0xaabbcc));
        assert_eq!(theme.alias_tokens().color_text_disabled,color(0x777777));
        assert_eq!(theme.component(AntComponent::Button).color_primary,Some(color(0x0055aa)));
        assert_eq!(theme.component(AntComponent::Card).color_primary,None);
        let invalid=ThemeLabSpec::parse(
            "ThemeLab|Map.colorPrimaryBg=zzzzzz|Map.controlHeightSM=nan|Alias.colorLink=bad",
            false).unwrap().theme;
        assert_eq!(invalid.alias_tokens().color_link,color(0x1677ff));
        assert_eq!(invalid.control_height_sm,24.);
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
