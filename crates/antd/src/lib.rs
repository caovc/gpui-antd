//! Native Ant Design-inspired components for GPUI.
//!
//! Visuals are styled from Ant Design semantic tokens. Buttons, checkboxes and
//! switches delegate focus, keyboard interaction and accessibility to GPUI Base.
//! The browser gallery runs these SAME Rust components through GPUI Web (WASM).
//!
//! This is an experimental, incremental implementation, not React Ant Design
//! compatibility nor a complete implementation of the Ant Design component API.

extern crate gpui_kit as gpui;

pub mod controls;
pub mod display;
pub mod tokens;

pub use controls::{AntButton, AntCheckbox, AntSwitch, ButtonVariant, ControlSize};
pub use display::{AntAlert, AntBadge, AntCard, AntDivider, AntProgress, AntTag, Tone};
pub use tokens::{AntTheme, Appearance, color};
