//! Native Ant Design-inspired components for GPUI.
//!
//! Visuals are styled from semantic tokens. Buttons, checkboxes and switches
//! delegate focus, keyboard interaction and accessibility to GPUI Base.
//! The browser gallery runs these SAME Rust components through GPUI Web (WASM).
//!
//! Experimental: this is an incremental implementation, not React API parity.

extern crate gpui_kit as gpui;

pub mod app_layout;
pub mod auto_complete;
pub mod basics;
pub mod controls;
pub mod data_extra;
pub mod date_picker;
pub mod design_tokens;
pub mod display;
pub mod feedback_extra;
pub mod form;
mod icon_palette;
pub mod icons;
pub mod input;
pub mod input_number;
pub mod layout;
pub mod list;
pub mod menu;
pub mod navigation_extra;
pub mod overlays;
pub mod radio;
pub mod segmented;
pub mod select;
pub mod slider;
pub mod tabs;
pub mod time_picker;
pub mod tokens;
pub mod tooltip;
pub mod typography;

pub use app_layout::{AntContent, AntFooter, AntHeader, AntLayout, AntMobileDrawer, AntSider};
pub use basics::{
    AntAvatar, AntBreadcrumb, AntBreadcrumbItem, AntEmpty, AntStatistic, AvatarShape, AvatarSize,
};
pub use controls::{AntButton, AntCheckbox, AntSwitch, ButtonVariant, ControlSize};
pub use date_picker::AntDatePicker;
pub use design_tokens::{
    AntAliasTokenOverrides, AntAliasTokens, AntComponent, AntComponentToken, AntComponentTokens,
    AntMapTokenOverrides, AntMapTokens, AntResolvedToken, AntSeedTokens, AntThemeAlgorithm,
    AntTokenLayer, AntTokenValue,
};
pub use display::{AntAlert, AntBadge, AntCard, AntDivider, AntProgress, AntTag, Tone};
pub use form::{AntForm, AntFormItem, FormRule, FormSchema, FormValidation};
pub use icons::{AntIcon, AntIconAssets, AntIconName};
pub use input::{AntInput, InputStatus, InputVariant};
pub use layout::{
    AntCol, AntFlex, AntRow, AntSpace, FlexAlign, FlexJustify, GridBreakpoint, SpaceSize,
};
pub use overlays::{
    AntDropdown, AntDropdownItem, AntModal, AntPopconfirm, AntPopover, OverlayPlacement,
};
pub use radio::{AntRadio, AntRadioGroup};
pub use select::AntSelect;
pub use tabs::{AntTab, AntTabs, TabsVariant};
pub use tokens::{AntTheme, Appearance, color};
pub use tooltip::{AntTooltip, TooltipPlacement};
pub use typography::{AntLink, AntParagraph, AntText, AntTitle, AntTypography, TextTone};

pub use menu::{AntMenu, AntMenuItem, AntMenuMode};

pub use data_extra::{AntCollapse, AntCollapseItem, AntDescriptionItem, AntDescriptions, AntRate};
pub use feedback_extra::{AntResult, AntSkeleton, AntSpin, ResultStatus};
pub use navigation_extra::{AntPagination, AntStep, AntSteps, PageSlot, StepStatus};

pub use auto_complete::{AntAutoComplete, autocomplete_matches, next_autocomplete_index};
pub use input_number::AntInputNumber;
pub use segmented::{AntSegmented, AntSegmentedOption, next_segmented_index};
pub use slider::{AntSlider, slider_key_value};
pub use time_picker::AntTimePicker;

pub mod calendar;
pub mod catalog;
pub mod feedback_actions;
pub mod float_button;
pub mod table;
pub mod timeline;
pub mod tree;

pub use feedback_actions::{AntDrawer, AntMessage, AntNotification, DrawerPlacement, NoticeTone};
pub use table::{
    AntTable, AntTableColumn, AntTableRow, TableSort, filtered_table_indices, sorted_table_indices,
    table_view_indices, toggle_table_visible_selection,
};
pub use timeline::{AntTimeline, AntTimelineItem, TimelineMode, TimelineTone};
pub use tree::{
    AntTree, AntTreeNode, TreeCheckState, VisibleTreeNode, cascade_tree_checked_keys,
    tree_check_states, tree_focus_index, visible_tree_nodes,
};

pub use list::{AntList, AntListItem, ListSize};

pub use calendar::AntCalendar;
pub use catalog::{
    AntAffix, AntAnchor, AntAnchorLink, AntApp, AntBorderBeam, AntCarousel, AntCarouselSlide,
    AntCascader, AntCascaderOption, AntColorPicker, AntConfig, AntConfigProvider, AntDirection,
    AntImage, AntListyItem, AntMasonry, AntMasonryItem, AntMentionOption, AntMentions, AntQrCode,
    AntSplitter, AntTour, AntTourStep, AntTransfer, AntTransferItem, AntTreeSelect, AntUpload,
    AntUploadFile, AntWatermark, ListySlot, QrMatrix, UploadStatus, ant_listy, apply_mention,
    carousel_index, cascader_column, cascader_select, hsv_to_rgb, listy_slots, masonry_columns,
    mention_matches, mention_query, next_anchor_index, parse_hex, qr_matrix, tour_index,
    transfer_move, tree_select_title, upload_accepted, upload_without, watermark_count,
};
pub use float_button::{
    AntFloatAction, AntFloatButton, AntFloatButtonGroup, FloatShape, FloatVariant, float_offset,
};
