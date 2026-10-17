//! MIT-licensed official Ant Design SVG icons embedded for GPUI native and WASM.
//!
//! Source: ant-design/ant-design-icons, commit 7f2516ac91226d2b41f93b35cb5197c8d94f7189.
//! Asset keys are namespaced under "antd-icons/" to coexist with GPUI Kit.
//! Two-tone artwork has Ant Design default primary/secondary colors baked in.

use std::{borrow::Cow, time::Duration};
use gpui_kit::{
    Animation, AnimationExt as _, App, AssetSource, Hsla, IntoElement, RenderOnce, Result,
    SharedString, Styled as _, Transformation, Window, percentage, px, svg,
    prelude::FluentBuilder as _,
};
use crate::tokens::AntTheme;

/// Ant Design outline, filled and two-tone icon variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AntIconName {
    CheckOutlined,
    CloseOutlined,
    PlusOutlined,
    MinusOutlined,
    SearchOutlined,
    HomeOutlined,
    UserOutlined,
    SettingOutlined,
    BellOutlined,
    CalendarOutlined,
    ClockCircleOutlined,
    InfoCircleOutlined,
    QuestionCircleOutlined,
    CheckCircleOutlined,
    CloseCircleOutlined,
    ExclamationCircleOutlined,
    WarningOutlined,
    HeartOutlined,
    StarOutlined,
    EyeOutlined,
    EyeInvisibleOutlined,
    EditOutlined,
    DeleteOutlined,
    CopyOutlined,
    SaveOutlined,
    DownloadOutlined,
    UploadOutlined,
    MenuOutlined,
    FilterOutlined,
    SortAscendingOutlined,
    SortDescendingOutlined,
    ArrowLeftOutlined,
    ArrowRightOutlined,
    ArrowUpOutlined,
    ArrowDownOutlined,
    LoadingOutlined,
    LockOutlined,
    UnlockOutlined,
    MailOutlined,
    PhoneOutlined,
    FileOutlined,
    FolderOutlined,
    LinkOutlined,
    GithubOutlined,
    AppstoreOutlined,
    ShoppingCartOutlined,
    SmileOutlined,
    MoreOutlined,
    EllipsisOutlined,
    LeftOutlined,
    RightOutlined,
    UpOutlined,
    DownOutlined,
    CheckSquareOutlined,
    PlusCircleOutlined,
    MinusCircleOutlined,
    CaretDownOutlined,
    BulbOutlined,
    CheckCircleFilled,
    CloseCircleFilled,
    InfoCircleFilled,
    WarningFilled,
    HeartFilled,
    StarFilled,
    HomeFilled,
    BellFilled,
    QuestionCircleFilled,
    LikeFilled,
    SmileFilled,
    PlusCircleFilled,
    CheckCircleTwotone,
    CloseCircleTwotone,
    InfoCircleTwotone,
    WarningTwotone,
    HeartTwotone,
    StarTwotone,
    QuestionCircleTwotone,
    SmileTwotone,
}

impl AntIconName {
    /// Icons packaged with the alpha release. More icons can be added by the
    /// pinned-source sync script rather than by copying Lucide lookalikes.
    pub const ALL: &'static [Self] = &[
        Self::CheckOutlined,
        Self::CloseOutlined,
        Self::PlusOutlined,
        Self::MinusOutlined,
        Self::SearchOutlined,
        Self::HomeOutlined,
        Self::UserOutlined,
        Self::SettingOutlined,
        Self::BellOutlined,
        Self::CalendarOutlined,
        Self::ClockCircleOutlined,
        Self::InfoCircleOutlined,
        Self::QuestionCircleOutlined,
        Self::CheckCircleOutlined,
        Self::CloseCircleOutlined,
        Self::ExclamationCircleOutlined,
        Self::WarningOutlined,
        Self::HeartOutlined,
        Self::StarOutlined,
        Self::EyeOutlined,
        Self::EyeInvisibleOutlined,
        Self::EditOutlined,
        Self::DeleteOutlined,
        Self::CopyOutlined,
        Self::SaveOutlined,
        Self::DownloadOutlined,
        Self::UploadOutlined,
        Self::MenuOutlined,
        Self::FilterOutlined,
        Self::SortAscendingOutlined,
        Self::SortDescendingOutlined,
        Self::ArrowLeftOutlined,
        Self::ArrowRightOutlined,
        Self::ArrowUpOutlined,
        Self::ArrowDownOutlined,
        Self::LoadingOutlined,
        Self::LockOutlined,
        Self::UnlockOutlined,
        Self::MailOutlined,
        Self::PhoneOutlined,
        Self::FileOutlined,
        Self::FolderOutlined,
        Self::LinkOutlined,
        Self::GithubOutlined,
        Self::AppstoreOutlined,
        Self::ShoppingCartOutlined,
        Self::SmileOutlined,
        Self::MoreOutlined,
        Self::EllipsisOutlined,
        Self::LeftOutlined,
        Self::RightOutlined,
        Self::UpOutlined,
        Self::DownOutlined,
        Self::CheckSquareOutlined,
        Self::PlusCircleOutlined,
        Self::MinusCircleOutlined,
        Self::CaretDownOutlined,
        Self::BulbOutlined,
        Self::CheckCircleFilled,
        Self::CloseCircleFilled,
        Self::InfoCircleFilled,
        Self::WarningFilled,
        Self::HeartFilled,
        Self::StarFilled,
        Self::HomeFilled,
        Self::BellFilled,
        Self::QuestionCircleFilled,
        Self::LikeFilled,
        Self::SmileFilled,
        Self::PlusCircleFilled,
        Self::CheckCircleTwotone,
        Self::CloseCircleTwotone,
        Self::InfoCircleTwotone,
        Self::WarningTwotone,
        Self::HeartTwotone,
        Self::StarTwotone,
        Self::QuestionCircleTwotone,
        Self::SmileTwotone,
    ];

    pub const fn path(self) -> &'static str {
        match self {
            Self::CheckOutlined => "antd-icons/outlined/check.svg",
            Self::CloseOutlined => "antd-icons/outlined/close.svg",
            Self::PlusOutlined => "antd-icons/outlined/plus.svg",
            Self::MinusOutlined => "antd-icons/outlined/minus.svg",
            Self::SearchOutlined => "antd-icons/outlined/search.svg",
            Self::HomeOutlined => "antd-icons/outlined/home.svg",
            Self::UserOutlined => "antd-icons/outlined/user.svg",
            Self::SettingOutlined => "antd-icons/outlined/setting.svg",
            Self::BellOutlined => "antd-icons/outlined/bell.svg",
            Self::CalendarOutlined => "antd-icons/outlined/calendar.svg",
            Self::ClockCircleOutlined => "antd-icons/outlined/clock-circle.svg",
            Self::InfoCircleOutlined => "antd-icons/outlined/info-circle.svg",
            Self::QuestionCircleOutlined => "antd-icons/outlined/question-circle.svg",
            Self::CheckCircleOutlined => "antd-icons/outlined/check-circle.svg",
            Self::CloseCircleOutlined => "antd-icons/outlined/close-circle.svg",
            Self::ExclamationCircleOutlined => "antd-icons/outlined/exclamation-circle.svg",
            Self::WarningOutlined => "antd-icons/outlined/warning.svg",
            Self::HeartOutlined => "antd-icons/outlined/heart.svg",
            Self::StarOutlined => "antd-icons/outlined/star.svg",
            Self::EyeOutlined => "antd-icons/outlined/eye.svg",
            Self::EyeInvisibleOutlined => "antd-icons/outlined/eye-invisible.svg",
            Self::EditOutlined => "antd-icons/outlined/edit.svg",
            Self::DeleteOutlined => "antd-icons/outlined/delete.svg",
            Self::CopyOutlined => "antd-icons/outlined/copy.svg",
            Self::SaveOutlined => "antd-icons/outlined/save.svg",
            Self::DownloadOutlined => "antd-icons/outlined/download.svg",
            Self::UploadOutlined => "antd-icons/outlined/upload.svg",
            Self::MenuOutlined => "antd-icons/outlined/menu.svg",
            Self::FilterOutlined => "antd-icons/outlined/filter.svg",
            Self::SortAscendingOutlined => "antd-icons/outlined/sort-ascending.svg",
            Self::SortDescendingOutlined => "antd-icons/outlined/sort-descending.svg",
            Self::ArrowLeftOutlined => "antd-icons/outlined/arrow-left.svg",
            Self::ArrowRightOutlined => "antd-icons/outlined/arrow-right.svg",
            Self::ArrowUpOutlined => "antd-icons/outlined/arrow-up.svg",
            Self::ArrowDownOutlined => "antd-icons/outlined/arrow-down.svg",
            Self::LoadingOutlined => "antd-icons/outlined/loading.svg",
            Self::LockOutlined => "antd-icons/outlined/lock.svg",
            Self::UnlockOutlined => "antd-icons/outlined/unlock.svg",
            Self::MailOutlined => "antd-icons/outlined/mail.svg",
            Self::PhoneOutlined => "antd-icons/outlined/phone.svg",
            Self::FileOutlined => "antd-icons/outlined/file.svg",
            Self::FolderOutlined => "antd-icons/outlined/folder.svg",
            Self::LinkOutlined => "antd-icons/outlined/link.svg",
            Self::GithubOutlined => "antd-icons/outlined/github.svg",
            Self::AppstoreOutlined => "antd-icons/outlined/appstore.svg",
            Self::ShoppingCartOutlined => "antd-icons/outlined/shopping-cart.svg",
            Self::SmileOutlined => "antd-icons/outlined/smile.svg",
            Self::MoreOutlined => "antd-icons/outlined/more.svg",
            Self::EllipsisOutlined => "antd-icons/outlined/ellipsis.svg",
            Self::LeftOutlined => "antd-icons/outlined/left.svg",
            Self::RightOutlined => "antd-icons/outlined/right.svg",
            Self::UpOutlined => "antd-icons/outlined/up.svg",
            Self::DownOutlined => "antd-icons/outlined/down.svg",
            Self::CheckSquareOutlined => "antd-icons/outlined/check-square.svg",
            Self::PlusCircleOutlined => "antd-icons/outlined/plus-circle.svg",
            Self::MinusCircleOutlined => "antd-icons/outlined/minus-circle.svg",
            Self::CaretDownOutlined => "antd-icons/outlined/caret-down.svg",
            Self::BulbOutlined => "antd-icons/outlined/bulb.svg",
            Self::CheckCircleFilled => "antd-icons/filled/check-circle.svg",
            Self::CloseCircleFilled => "antd-icons/filled/close-circle.svg",
            Self::InfoCircleFilled => "antd-icons/filled/info-circle.svg",
            Self::WarningFilled => "antd-icons/filled/warning.svg",
            Self::HeartFilled => "antd-icons/filled/heart.svg",
            Self::StarFilled => "antd-icons/filled/star.svg",
            Self::HomeFilled => "antd-icons/filled/home.svg",
            Self::BellFilled => "antd-icons/filled/bell.svg",
            Self::QuestionCircleFilled => "antd-icons/filled/question-circle.svg",
            Self::LikeFilled => "antd-icons/filled/like.svg",
            Self::SmileFilled => "antd-icons/filled/smile.svg",
            Self::PlusCircleFilled => "antd-icons/filled/plus-circle.svg",
            Self::CheckCircleTwotone => "antd-icons/twotone/check-circle.svg",
            Self::CloseCircleTwotone => "antd-icons/twotone/close-circle.svg",
            Self::InfoCircleTwotone => "antd-icons/twotone/info-circle.svg",
            Self::WarningTwotone => "antd-icons/twotone/warning.svg",
            Self::HeartTwotone => "antd-icons/twotone/heart.svg",
            Self::StarTwotone => "antd-icons/twotone/star.svg",
            Self::QuestionCircleTwotone => "antd-icons/twotone/question-circle.svg",
            Self::SmileTwotone => "antd-icons/twotone/smile.svg",
        }
    }

    pub const fn ant_name(self) -> &'static str {
        match self {
            Self::CheckOutlined => "CheckOutlined",
            Self::CloseOutlined => "CloseOutlined",
            Self::PlusOutlined => "PlusOutlined",
            Self::MinusOutlined => "MinusOutlined",
            Self::SearchOutlined => "SearchOutlined",
            Self::HomeOutlined => "HomeOutlined",
            Self::UserOutlined => "UserOutlined",
            Self::SettingOutlined => "SettingOutlined",
            Self::BellOutlined => "BellOutlined",
            Self::CalendarOutlined => "CalendarOutlined",
            Self::ClockCircleOutlined => "ClockCircleOutlined",
            Self::InfoCircleOutlined => "InfoCircleOutlined",
            Self::QuestionCircleOutlined => "QuestionCircleOutlined",
            Self::CheckCircleOutlined => "CheckCircleOutlined",
            Self::CloseCircleOutlined => "CloseCircleOutlined",
            Self::ExclamationCircleOutlined => "ExclamationCircleOutlined",
            Self::WarningOutlined => "WarningOutlined",
            Self::HeartOutlined => "HeartOutlined",
            Self::StarOutlined => "StarOutlined",
            Self::EyeOutlined => "EyeOutlined",
            Self::EyeInvisibleOutlined => "EyeInvisibleOutlined",
            Self::EditOutlined => "EditOutlined",
            Self::DeleteOutlined => "DeleteOutlined",
            Self::CopyOutlined => "CopyOutlined",
            Self::SaveOutlined => "SaveOutlined",
            Self::DownloadOutlined => "DownloadOutlined",
            Self::UploadOutlined => "UploadOutlined",
            Self::MenuOutlined => "MenuOutlined",
            Self::FilterOutlined => "FilterOutlined",
            Self::SortAscendingOutlined => "SortAscendingOutlined",
            Self::SortDescendingOutlined => "SortDescendingOutlined",
            Self::ArrowLeftOutlined => "ArrowLeftOutlined",
            Self::ArrowRightOutlined => "ArrowRightOutlined",
            Self::ArrowUpOutlined => "ArrowUpOutlined",
            Self::ArrowDownOutlined => "ArrowDownOutlined",
            Self::LoadingOutlined => "LoadingOutlined",
            Self::LockOutlined => "LockOutlined",
            Self::UnlockOutlined => "UnlockOutlined",
            Self::MailOutlined => "MailOutlined",
            Self::PhoneOutlined => "PhoneOutlined",
            Self::FileOutlined => "FileOutlined",
            Self::FolderOutlined => "FolderOutlined",
            Self::LinkOutlined => "LinkOutlined",
            Self::GithubOutlined => "GithubOutlined",
            Self::AppstoreOutlined => "AppstoreOutlined",
            Self::ShoppingCartOutlined => "ShoppingCartOutlined",
            Self::SmileOutlined => "SmileOutlined",
            Self::MoreOutlined => "MoreOutlined",
            Self::EllipsisOutlined => "EllipsisOutlined",
            Self::LeftOutlined => "LeftOutlined",
            Self::RightOutlined => "RightOutlined",
            Self::UpOutlined => "UpOutlined",
            Self::DownOutlined => "DownOutlined",
            Self::CheckSquareOutlined => "CheckSquareOutlined",
            Self::PlusCircleOutlined => "PlusCircleOutlined",
            Self::MinusCircleOutlined => "MinusCircleOutlined",
            Self::CaretDownOutlined => "CaretDownOutlined",
            Self::BulbOutlined => "BulbOutlined",
            Self::CheckCircleFilled => "CheckCircleFilled",
            Self::CloseCircleFilled => "CloseCircleFilled",
            Self::InfoCircleFilled => "InfoCircleFilled",
            Self::WarningFilled => "WarningFilled",
            Self::HeartFilled => "HeartFilled",
            Self::StarFilled => "StarFilled",
            Self::HomeFilled => "HomeFilled",
            Self::BellFilled => "BellFilled",
            Self::QuestionCircleFilled => "QuestionCircleFilled",
            Self::LikeFilled => "LikeFilled",
            Self::SmileFilled => "SmileFilled",
            Self::PlusCircleFilled => "PlusCircleFilled",
            Self::CheckCircleTwotone => "CheckCircleTwotone",
            Self::CloseCircleTwotone => "CloseCircleTwotone",
            Self::InfoCircleTwotone => "InfoCircleTwotone",
            Self::WarningTwotone => "WarningTwotone",
            Self::HeartTwotone => "HeartTwotone",
            Self::StarTwotone => "StarTwotone",
            Self::QuestionCircleTwotone => "QuestionCircleTwotone",
            Self::SmileTwotone => "SmileTwotone",
        }
    }

    pub const fn style(self) -> &'static str {
        match self {
            Self::CheckOutlined
            | Self::CloseOutlined
            | Self::PlusOutlined
            | Self::MinusOutlined
            | Self::SearchOutlined
            | Self::HomeOutlined
            | Self::UserOutlined
            | Self::SettingOutlined
            | Self::BellOutlined
            | Self::CalendarOutlined
            | Self::ClockCircleOutlined
            | Self::InfoCircleOutlined
            | Self::QuestionCircleOutlined
            | Self::CheckCircleOutlined
            | Self::CloseCircleOutlined
            | Self::ExclamationCircleOutlined
            | Self::WarningOutlined
            | Self::HeartOutlined
            | Self::StarOutlined
            | Self::EyeOutlined
            | Self::EyeInvisibleOutlined
            | Self::EditOutlined
            | Self::DeleteOutlined
            | Self::CopyOutlined
            | Self::SaveOutlined
            | Self::DownloadOutlined
            | Self::UploadOutlined
            | Self::MenuOutlined
            | Self::FilterOutlined
            | Self::SortAscendingOutlined
            | Self::SortDescendingOutlined
            | Self::ArrowLeftOutlined
            | Self::ArrowRightOutlined
            | Self::ArrowUpOutlined
            | Self::ArrowDownOutlined
            | Self::LoadingOutlined
            | Self::LockOutlined
            | Self::UnlockOutlined
            | Self::MailOutlined
            | Self::PhoneOutlined
            | Self::FileOutlined
            | Self::FolderOutlined
            | Self::LinkOutlined
            | Self::GithubOutlined
            | Self::AppstoreOutlined
            | Self::ShoppingCartOutlined
            | Self::SmileOutlined
            | Self::MoreOutlined
            | Self::EllipsisOutlined
            | Self::LeftOutlined
            | Self::RightOutlined
            | Self::UpOutlined
            | Self::DownOutlined
            | Self::CheckSquareOutlined
            | Self::PlusCircleOutlined
            | Self::MinusCircleOutlined
            | Self::CaretDownOutlined
            | Self::BulbOutlined => "outlined",
            Self::CheckCircleFilled
            | Self::CloseCircleFilled
            | Self::InfoCircleFilled
            | Self::WarningFilled
            | Self::HeartFilled
            | Self::StarFilled
            | Self::HomeFilled
            | Self::BellFilled
            | Self::QuestionCircleFilled
            | Self::LikeFilled
            | Self::SmileFilled
            | Self::PlusCircleFilled => "filled",
            Self::CheckCircleTwotone
            | Self::CloseCircleTwotone
            | Self::InfoCircleTwotone
            | Self::WarningTwotone
            | Self::HeartTwotone
            | Self::StarTwotone
            | Self::QuestionCircleTwotone
            | Self::SmileTwotone => "twotone",
        }
    }
}

/// Render an actual official Ant Design SVG using GPUI's SVG painter.
///
/// Prefer `AntIcon::new(AntIconName::SearchOutlined)` in buttons and text.
/// Outline icons inherit the requested color through `currentColor` SVG fill.
#[derive(IntoElement)]
pub struct AntIcon {
    name: AntIconName,
    size: f32,
    color: Option<Hsla>,
    theme: AntTheme,
    spin: bool,
}

impl AntIcon {
    pub fn new(name: AntIconName) -> Self {
        Self { name, size: 16., color: None, theme: AntTheme::default(), spin: false }
    }
    pub fn loading() -> Self { Self::new(AntIconName::LoadingOutlined).spin(true) }
    pub fn size(mut self, size: f32) -> Self { self.size = size.max(0.); self }
    pub fn color(mut self, color: Hsla) -> Self { self.color = Some(color); self }
    pub fn theme(mut self, theme: AntTheme) -> Self { self.theme = theme; self }
    pub fn spin(mut self, spin: bool) -> Self { self.spin = spin; self }
    pub fn name(&self) -> AntIconName { self.name }
}
impl RenderOnce for AntIcon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        svg()
            .path(self.name.path())
            .size(px(self.size))
            .flex_shrink_0()
            .text_color(self.color.unwrap_or(self.theme.text))
            .when(self.spin, |this| {
                this.with_animation(
                    "ant-icon-spin",
                    Animation::new(Duration::from_millis(900)).repeat(),
                    |this, delta| this.transform(Transformation::rotate(percentage(delta))),
                )
            })
    }
}

/// Embedded source for every curated Ant Design SVG. No network or
/// font installation is required in desktop apps or WASM.
/// Register `AntIconAssets` or a composite source alongside GPUI Kit assets.
#[derive(Debug, Clone, Copy, Default)]
pub struct AntIconAssets;
impl AssetSource for AntIconAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let svg: Option<&'static [u8]> = match path {
            "antd-icons/outlined/check.svg" => Some(include_bytes!("../assets/antd-icons/outlined/check.svg").as_slice()),
            "antd-icons/outlined/close.svg" => Some(include_bytes!("../assets/antd-icons/outlined/close.svg").as_slice()),
            "antd-icons/outlined/plus.svg" => Some(include_bytes!("../assets/antd-icons/outlined/plus.svg").as_slice()),
            "antd-icons/outlined/minus.svg" => Some(include_bytes!("../assets/antd-icons/outlined/minus.svg").as_slice()),
            "antd-icons/outlined/search.svg" => Some(include_bytes!("../assets/antd-icons/outlined/search.svg").as_slice()),
            "antd-icons/outlined/home.svg" => Some(include_bytes!("../assets/antd-icons/outlined/home.svg").as_slice()),
            "antd-icons/outlined/user.svg" => Some(include_bytes!("../assets/antd-icons/outlined/user.svg").as_slice()),
            "antd-icons/outlined/setting.svg" => Some(include_bytes!("../assets/antd-icons/outlined/setting.svg").as_slice()),
            "antd-icons/outlined/bell.svg" => Some(include_bytes!("../assets/antd-icons/outlined/bell.svg").as_slice()),
            "antd-icons/outlined/calendar.svg" => Some(include_bytes!("../assets/antd-icons/outlined/calendar.svg").as_slice()),
            "antd-icons/outlined/clock-circle.svg" => Some(include_bytes!("../assets/antd-icons/outlined/clock-circle.svg").as_slice()),
            "antd-icons/outlined/info-circle.svg" => Some(include_bytes!("../assets/antd-icons/outlined/info-circle.svg").as_slice()),
            "antd-icons/outlined/question-circle.svg" => Some(include_bytes!("../assets/antd-icons/outlined/question-circle.svg").as_slice()),
            "antd-icons/outlined/check-circle.svg" => Some(include_bytes!("../assets/antd-icons/outlined/check-circle.svg").as_slice()),
            "antd-icons/outlined/close-circle.svg" => Some(include_bytes!("../assets/antd-icons/outlined/close-circle.svg").as_slice()),
            "antd-icons/outlined/exclamation-circle.svg" => Some(include_bytes!("../assets/antd-icons/outlined/exclamation-circle.svg").as_slice()),
            "antd-icons/outlined/warning.svg" => Some(include_bytes!("../assets/antd-icons/outlined/warning.svg").as_slice()),
            "antd-icons/outlined/heart.svg" => Some(include_bytes!("../assets/antd-icons/outlined/heart.svg").as_slice()),
            "antd-icons/outlined/star.svg" => Some(include_bytes!("../assets/antd-icons/outlined/star.svg").as_slice()),
            "antd-icons/outlined/eye.svg" => Some(include_bytes!("../assets/antd-icons/outlined/eye.svg").as_slice()),
            "antd-icons/outlined/eye-invisible.svg" => Some(include_bytes!("../assets/antd-icons/outlined/eye-invisible.svg").as_slice()),
            "antd-icons/outlined/edit.svg" => Some(include_bytes!("../assets/antd-icons/outlined/edit.svg").as_slice()),
            "antd-icons/outlined/delete.svg" => Some(include_bytes!("../assets/antd-icons/outlined/delete.svg").as_slice()),
            "antd-icons/outlined/copy.svg" => Some(include_bytes!("../assets/antd-icons/outlined/copy.svg").as_slice()),
            "antd-icons/outlined/save.svg" => Some(include_bytes!("../assets/antd-icons/outlined/save.svg").as_slice()),
            "antd-icons/outlined/download.svg" => Some(include_bytes!("../assets/antd-icons/outlined/download.svg").as_slice()),
            "antd-icons/outlined/upload.svg" => Some(include_bytes!("../assets/antd-icons/outlined/upload.svg").as_slice()),
            "antd-icons/outlined/menu.svg" => Some(include_bytes!("../assets/antd-icons/outlined/menu.svg").as_slice()),
            "antd-icons/outlined/filter.svg" => Some(include_bytes!("../assets/antd-icons/outlined/filter.svg").as_slice()),
            "antd-icons/outlined/sort-ascending.svg" => Some(include_bytes!("../assets/antd-icons/outlined/sort-ascending.svg").as_slice()),
            "antd-icons/outlined/sort-descending.svg" => Some(include_bytes!("../assets/antd-icons/outlined/sort-descending.svg").as_slice()),
            "antd-icons/outlined/arrow-left.svg" => Some(include_bytes!("../assets/antd-icons/outlined/arrow-left.svg").as_slice()),
            "antd-icons/outlined/arrow-right.svg" => Some(include_bytes!("../assets/antd-icons/outlined/arrow-right.svg").as_slice()),
            "antd-icons/outlined/arrow-up.svg" => Some(include_bytes!("../assets/antd-icons/outlined/arrow-up.svg").as_slice()),
            "antd-icons/outlined/arrow-down.svg" => Some(include_bytes!("../assets/antd-icons/outlined/arrow-down.svg").as_slice()),
            "antd-icons/outlined/loading.svg" => Some(include_bytes!("../assets/antd-icons/outlined/loading.svg").as_slice()),
            "antd-icons/outlined/lock.svg" => Some(include_bytes!("../assets/antd-icons/outlined/lock.svg").as_slice()),
            "antd-icons/outlined/unlock.svg" => Some(include_bytes!("../assets/antd-icons/outlined/unlock.svg").as_slice()),
            "antd-icons/outlined/mail.svg" => Some(include_bytes!("../assets/antd-icons/outlined/mail.svg").as_slice()),
            "antd-icons/outlined/phone.svg" => Some(include_bytes!("../assets/antd-icons/outlined/phone.svg").as_slice()),
            "antd-icons/outlined/file.svg" => Some(include_bytes!("../assets/antd-icons/outlined/file.svg").as_slice()),
            "antd-icons/outlined/folder.svg" => Some(include_bytes!("../assets/antd-icons/outlined/folder.svg").as_slice()),
            "antd-icons/outlined/link.svg" => Some(include_bytes!("../assets/antd-icons/outlined/link.svg").as_slice()),
            "antd-icons/outlined/github.svg" => Some(include_bytes!("../assets/antd-icons/outlined/github.svg").as_slice()),
            "antd-icons/outlined/appstore.svg" => Some(include_bytes!("../assets/antd-icons/outlined/appstore.svg").as_slice()),
            "antd-icons/outlined/shopping-cart.svg" => Some(include_bytes!("../assets/antd-icons/outlined/shopping-cart.svg").as_slice()),
            "antd-icons/outlined/smile.svg" => Some(include_bytes!("../assets/antd-icons/outlined/smile.svg").as_slice()),
            "antd-icons/outlined/more.svg" => Some(include_bytes!("../assets/antd-icons/outlined/more.svg").as_slice()),
            "antd-icons/outlined/ellipsis.svg" => Some(include_bytes!("../assets/antd-icons/outlined/ellipsis.svg").as_slice()),
            "antd-icons/outlined/left.svg" => Some(include_bytes!("../assets/antd-icons/outlined/left.svg").as_slice()),
            "antd-icons/outlined/right.svg" => Some(include_bytes!("../assets/antd-icons/outlined/right.svg").as_slice()),
            "antd-icons/outlined/up.svg" => Some(include_bytes!("../assets/antd-icons/outlined/up.svg").as_slice()),
            "antd-icons/outlined/down.svg" => Some(include_bytes!("../assets/antd-icons/outlined/down.svg").as_slice()),
            "antd-icons/outlined/check-square.svg" => Some(include_bytes!("../assets/antd-icons/outlined/check-square.svg").as_slice()),
            "antd-icons/outlined/plus-circle.svg" => Some(include_bytes!("../assets/antd-icons/outlined/plus-circle.svg").as_slice()),
            "antd-icons/outlined/minus-circle.svg" => Some(include_bytes!("../assets/antd-icons/outlined/minus-circle.svg").as_slice()),
            "antd-icons/outlined/caret-down.svg" => Some(include_bytes!("../assets/antd-icons/outlined/caret-down.svg").as_slice()),
            "antd-icons/outlined/bulb.svg" => Some(include_bytes!("../assets/antd-icons/outlined/bulb.svg").as_slice()),
            "antd-icons/filled/check-circle.svg" => Some(include_bytes!("../assets/antd-icons/filled/check-circle.svg").as_slice()),
            "antd-icons/filled/close-circle.svg" => Some(include_bytes!("../assets/antd-icons/filled/close-circle.svg").as_slice()),
            "antd-icons/filled/info-circle.svg" => Some(include_bytes!("../assets/antd-icons/filled/info-circle.svg").as_slice()),
            "antd-icons/filled/warning.svg" => Some(include_bytes!("../assets/antd-icons/filled/warning.svg").as_slice()),
            "antd-icons/filled/heart.svg" => Some(include_bytes!("../assets/antd-icons/filled/heart.svg").as_slice()),
            "antd-icons/filled/star.svg" => Some(include_bytes!("../assets/antd-icons/filled/star.svg").as_slice()),
            "antd-icons/filled/home.svg" => Some(include_bytes!("../assets/antd-icons/filled/home.svg").as_slice()),
            "antd-icons/filled/bell.svg" => Some(include_bytes!("../assets/antd-icons/filled/bell.svg").as_slice()),
            "antd-icons/filled/question-circle.svg" => Some(include_bytes!("../assets/antd-icons/filled/question-circle.svg").as_slice()),
            "antd-icons/filled/like.svg" => Some(include_bytes!("../assets/antd-icons/filled/like.svg").as_slice()),
            "antd-icons/filled/smile.svg" => Some(include_bytes!("../assets/antd-icons/filled/smile.svg").as_slice()),
            "antd-icons/filled/plus-circle.svg" => Some(include_bytes!("../assets/antd-icons/filled/plus-circle.svg").as_slice()),
            "antd-icons/twotone/check-circle.svg" => Some(include_bytes!("../assets/antd-icons/twotone/check-circle.svg").as_slice()),
            "antd-icons/twotone/close-circle.svg" => Some(include_bytes!("../assets/antd-icons/twotone/close-circle.svg").as_slice()),
            "antd-icons/twotone/info-circle.svg" => Some(include_bytes!("../assets/antd-icons/twotone/info-circle.svg").as_slice()),
            "antd-icons/twotone/warning.svg" => Some(include_bytes!("../assets/antd-icons/twotone/warning.svg").as_slice()),
            "antd-icons/twotone/heart.svg" => Some(include_bytes!("../assets/antd-icons/twotone/heart.svg").as_slice()),
            "antd-icons/twotone/star.svg" => Some(include_bytes!("../assets/antd-icons/twotone/star.svg").as_slice()),
            "antd-icons/twotone/question-circle.svg" => Some(include_bytes!("../assets/antd-icons/twotone/question-circle.svg").as_slice()),
            "antd-icons/twotone/smile.svg" => Some(include_bytes!("../assets/antd-icons/twotone/smile.svg").as_slice()),
            _ => None,
        };
        Ok(svg.map(Cow::Borrowed))
    }

    fn list(&self, prefix: &str) -> Result<Vec<SharedString>> {
        Ok(AntIconName::ALL.iter()
            .map(|name| name.path())
            .filter(|path| path.starts_with(prefix))
            .map(SharedString::from)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_curated_names_load_a_real_svg() {
        let source = AntIconAssets;
        assert_eq!(AntIconName::ALL.len(), 78);
        for name in AntIconName::ALL {
            let bytes = source.load(name.path()).unwrap().expect(name.path());
            assert!(bytes.windows(4).any(|w| w == b"<svg"), "{}", name.path());
            assert_eq!(name.path().starts_with("antd-icons/"), true);
        }
        assert_eq!(source.list("antd-icons/").unwrap().len(), AntIconName::ALL.len());
        assert!(source.load("icons/unknown.svg").unwrap().is_none());
        assert_eq!(source.list("unknown").unwrap().len(), 0);
    }

    #[test]
    fn filled_twotone_and_outlined_present() {
        assert!(AntIconName::CheckOutlined.path().contains("/outlined/"));
        assert!(AntIconName::CheckCircleFilled.path().contains("/filled/"));
        assert!(AntIconName::CheckCircleTwotone.path().contains("/twotone/"));
        assert_eq!(AntIcon::loading().name(), AntIconName::LoadingOutlined);
    }
}
