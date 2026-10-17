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
pub mod radio;
pub mod tokens;

pub use controls::{AntButton, AntCheckbox, AntSwitch, ButtonVariant, ControlSize};
pub use display::{AntAlert, AntBadge, AntCard, AntDivider, AntProgress, AntTag, Tone};
pub use input::{AntInput, InputStatus, InputVariant};
pub use radio::{AntRadio, AntRadioGroup};
pub use tokens::{AntTheme, Appearance, color};
