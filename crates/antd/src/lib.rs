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
mod icon_palette;
pub mod icons;
pub mod radio;
pub mod select;
pub mod date_picker;
pub mod form;
pub mod tokens;
pub mod design_tokens;
pub mod typography;
pub mod layout;
pub mod basics;
pub mod tabs;
pub mod tooltip;
pub mod app_layout;
pub mod overlays;
pub mod menu;
pub mod navigation_extra;
pub mod data_extra;
pub mod list;
pub mod feedback_extra;
pub mod input_number;
pub mod slider;
pub mod segmented;
pub mod time_picker;
pub mod auto_complete;

pub use controls::{AntButton, AntCheckbox, AntSwitch, ButtonVariant, ControlSize};
pub use display::{AntAlert, AntBadge, AntCard, AntDivider, AntProgress, AntTag, Tone};
pub use input::{AntInput, InputStatus, InputVariant};
pub use icons::{AntIcon, AntIconAssets, AntIconName};
pub use radio::{AntRadio, AntRadioGroup};
pub use select::AntSelect;
pub use date_picker::AntDatePicker;
pub use form::{AntForm, AntFormItem, FormRule, FormSchema, FormValidation};
pub use tokens::{AntTheme, Appearance, color};
pub use design_tokens::{AntSeedTokens, AntMapTokens, AntAliasTokens,
    AntMapTokenOverrides, AntAliasTokenOverrides,
    AntThemeAlgorithm, AntComponent, AntComponentToken, AntComponentTokens,
    AntTokenLayer, AntTokenValue, AntResolvedToken};
pub use typography::{AntTitle, AntText, AntParagraph, AntLink, AntTypography, TextTone};
pub use layout::{AntSpace, AntFlex, AntRow, AntCol, SpaceSize, FlexAlign, FlexJustify, GridBreakpoint};
pub use basics::{AntAvatar, AvatarShape, AvatarSize, AntStatistic, AntEmpty, AntBreadcrumb, AntBreadcrumbItem};
pub use tabs::{AntTab, AntTabs, TabsVariant};
pub use tooltip::AntTooltip;
pub use app_layout::{AntLayout, AntHeader, AntSider, AntMobileDrawer, AntContent, AntFooter};
pub use overlays::{AntDropdown, AntDropdownItem, AntPopover, AntModal, AntPopconfirm};

pub use menu::{AntMenu, AntMenuItem};

pub use navigation_extra::{AntPagination, PageSlot, AntSteps, AntStep, StepStatus};
pub use data_extra::{AntRate, AntCollapse, AntCollapseItem, AntDescriptions, AntDescriptionItem};
pub use feedback_extra::{AntResult, ResultStatus, AntSkeleton, AntSpin};

pub use input_number::AntInputNumber;
pub use slider::{AntSlider, slider_key_value};
pub use segmented::{AntSegmented, AntSegmentedOption, next_segmented_index};
pub use time_picker::AntTimePicker;
pub use auto_complete::{AntAutoComplete, autocomplete_matches, next_autocomplete_index};

pub mod feedback_actions;
pub mod tree;
pub mod table;
pub mod timeline;
pub mod calendar;
pub mod float_button;

pub use feedback_actions::{AntDrawer, DrawerPlacement, AntMessage, AntNotification, NoticeTone};
pub use tree::{AntTree, AntTreeNode, VisibleTreeNode, TreeCheckState, visible_tree_nodes,
    tree_focus_index, tree_check_states, cascade_tree_checked_keys};
pub use table::{AntTable, AntTableColumn, AntTableRow, TableSort, sorted_table_indices,
    filtered_table_indices, table_view_indices, toggle_table_visible_selection};
pub use timeline::{AntTimeline, AntTimelineItem, TimelineMode, TimelineTone};

pub use list::{AntList, AntListItem, ListSize};

pub use calendar::AntCalendar;
pub use float_button::{AntFloatButton, AntFloatButtonGroup, AntFloatAction, FloatShape, FloatVariant, float_offset};
