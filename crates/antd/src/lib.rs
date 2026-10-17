//! Native Ant Design-inspired components for GPUI.
//!
//! Visuals are styled from semantic tokens. Buttons, checkboxes and switches
//! delegate focus, keyboard interaction and accessibility to GPUI Base.
//! The browser gallery runs these SAME Rust components through GPUI Web (WASM).
//!
//! Experimental: this is an incremental implementation, not React API parity.

extern crate gpui_kit as gpui;

pub mod controls;
pub mod display;
pub mod input;
pub mod icons;
pub mod radio;
pub mod select;
pub mod date_picker;
pub mod form;
pub mod tokens;
pub mod typography;
pub mod layout;
pub mod basics;

pub use controls::{AntButton, AntCheckbox, AntSwitch, ButtonVariant, ControlSize};
pub use display::{AntAlert, AntBadge, AntCard, AntDivider, AntProgress, AntTag, Tone};
pub use input::{AntInput, InputStatus, InputVariant};
pub use icons::{AntIcon, AntIconAssets, AntIconName};
pub use radio::{AntRadio, AntRadioGroup};
pub use select::AntSelect;
pub use date_picker::AntDatePicker;
pub use form::{AntForm, AntFormItem, FormRule, FormSchema, FormValidation};
pub use tokens::{AntTheme, Appearance, color};
pub use typography::{AntTitle, AntText, AntParagraph, AntLink, AntTypography, TextTone};
pub use layout::{AntSpace, AntFlex, AntRow, AntCol, SpaceSize, FlexAlign, FlexJustify};
pub use basics::{AntAvatar, AvatarShape, AvatarSize, AntStatistic, AntEmpty, AntBreadcrumb, AntBreadcrumbItem};
