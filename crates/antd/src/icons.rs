//! MIT-licensed official Ant Design SVG icons embedded for GPUI native and WASM.
//!
//! Source: ant-design/ant-design-icons, commit 7f2516ac91226d2b41f93b35cb5197c8d94f7189.
//! Asset keys are namespaced under "antd-icons/" to coexist with GPUI Kit.
//! Bundled two-tone artwork remains unmodified; runtime palettes are optional.

use crate::icon_palette::{
    TwoToneLayer, layer_asset_key, normalize_two_tone_svg, palette_asset_key,
    parse_layer_asset_key, parse_palette_asset_key, recolor_svg, svg_layer_mask,
};
use crate::tokens::AntTheme;
use gpui_kit::component::{Icon as KitIcon, Sizable as _, Size};
use gpui_kit::{
    Animation, AnimationExt as _, App, AssetSource, Hsla, IntoElement, ParentElement as _,
    RenderOnce, Result, SharedString, Styled as _, Transformation, Window, div, img, percentage,
    prelude::FluentBuilder as _, px, svg,
};
use std::{borrow::Cow, time::Duration};

/// Every official Ant Design SVG icon, generated from a pinned upstream revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AntIconName {
    AccountBookOutlined,
    AimOutlined,
    AlertOutlined,
    AlibabaOutlined,
    AlignCenterOutlined,
    AlignLeftOutlined,
    AlignRightOutlined,
    AlipayCircleOutlined,
    AlipayOutlined,
    AliwangwangOutlined,
    AliyunOutlined,
    AmazonOutlined,
    AndroidOutlined,
    AntCloudOutlined,
    AntDesignOutlined,
    ApartmentOutlined,
    ApiOutlined,
    AppleOutlined,
    AppstoreAddOutlined,
    AppstoreOutlined,
    AreaChartOutlined,
    ArrowDownOutlined,
    ArrowLeftOutlined,
    ArrowRightOutlined,
    ArrowUpOutlined,
    ArrowsAltOutlined,
    AudioMutedOutlined,
    AudioOutlined,
    AuditOutlined,
    BackwardOutlined,
    BaiduOutlined,
    BankOutlined,
    BarChartOutlined,
    BarcodeOutlined,
    BarsOutlined,
    BehanceSquareOutlined,
    BehanceOutlined,
    BellOutlined,
    BgColorsOutlined,
    BilibiliOutlined,
    BlockOutlined,
    BoldOutlined,
    BookOutlined,
    BorderBottomOutlined,
    BorderHorizontalOutlined,
    BorderInnerOutlined,
    BorderLeftOutlined,
    BorderOuterOutlined,
    BorderRightOutlined,
    BorderTopOutlined,
    BorderVerticleOutlined,
    BorderOutlined,
    BorderlessTableOutlined,
    BoxPlotOutlined,
    BranchesOutlined,
    BugOutlined,
    BuildOutlined,
    BulbOutlined,
    CalculatorOutlined,
    CalendarOutlined,
    CameraOutlined,
    CarOutlined,
    CaretDownOutlined,
    CaretLeftOutlined,
    CaretRightOutlined,
    CaretUpOutlined,
    CarryOutOutlined,
    CheckCircleOutlined,
    CheckSquareOutlined,
    CheckOutlined,
    ChromeOutlined,
    CiCircleOutlined,
    CiOutlined,
    ClearOutlined,
    ClockCircleOutlined,
    CloseCircleOutlined,
    CloseSquareOutlined,
    CloseOutlined,
    CloudDownloadOutlined,
    CloudServerOutlined,
    CloudSyncOutlined,
    CloudUploadOutlined,
    CloudOutlined,
    ClusterOutlined,
    CodeSandboxOutlined,
    CodeOutlined,
    CodepenCircleOutlined,
    CodepenOutlined,
    CoffeeOutlined,
    ColumnHeightOutlined,
    ColumnWidthOutlined,
    CommentOutlined,
    CompassOutlined,
    CompressOutlined,
    ConsoleSqlOutlined,
    ContactsOutlined,
    ContainerOutlined,
    ControlOutlined,
    CopyOutlined,
    CopyrightCircleOutlined,
    CopyrightOutlined,
    CreditCardOutlined,
    CrownOutlined,
    CustomerServiceOutlined,
    DashOutlined,
    DashboardOutlined,
    DatabaseOutlined,
    DeleteColumnOutlined,
    DeleteRowOutlined,
    DeleteOutlined,
    DeliveredProcedureOutlined,
    DeploymentUnitOutlined,
    DesktopOutlined,
    DiffOutlined,
    DingdingOutlined,
    DingtalkOutlined,
    DisconnectOutlined,
    DiscordOutlined,
    DislikeOutlined,
    DockerOutlined,
    DollarCircleOutlined,
    DollarOutlined,
    DotChartOutlined,
    DotNetOutlined,
    DoubleLeftOutlined,
    DoubleRightOutlined,
    DownCircleOutlined,
    DownSquareOutlined,
    DownOutlined,
    DownloadOutlined,
    DragOutlined,
    DribbbleSquareOutlined,
    DribbbleOutlined,
    DropboxOutlined,
    EditOutlined,
    EllipsisOutlined,
    EnterOutlined,
    EnvironmentOutlined,
    EuroCircleOutlined,
    EuroOutlined,
    ExceptionOutlined,
    ExclamationCircleOutlined,
    ExclamationOutlined,
    ExpandAltOutlined,
    ExpandOutlined,
    ExperimentOutlined,
    ExportOutlined,
    EyeInvisibleOutlined,
    EyeOutlined,
    FacebookOutlined,
    FallOutlined,
    FastBackwardOutlined,
    FastForwardOutlined,
    FieldBinaryOutlined,
    FieldNumberOutlined,
    FieldStringOutlined,
    FieldTimeOutlined,
    FileAddOutlined,
    FileDoneOutlined,
    FileExcelOutlined,
    FileExclamationOutlined,
    FileGifOutlined,
    FileImageOutlined,
    FileJpgOutlined,
    FileMarkdownOutlined,
    FilePdfOutlined,
    FilePptOutlined,
    FileProtectOutlined,
    FileSearchOutlined,
    FileSyncOutlined,
    FileTextOutlined,
    FileUnknownOutlined,
    FileWordOutlined,
    FileZipOutlined,
    FileOutlined,
    FilterOutlined,
    FireOutlined,
    FlagOutlined,
    FolderAddOutlined,
    FolderOpenOutlined,
    FolderViewOutlined,
    FolderOutlined,
    FontColorsOutlined,
    FontSizeOutlined,
    ForkOutlined,
    FormOutlined,
    FormatPainterOutlined,
    ForwardOutlined,
    FrownOutlined,
    FullscreenExitOutlined,
    FullscreenOutlined,
    FunctionOutlined,
    FundProjectionScreenOutlined,
    FundViewOutlined,
    FundOutlined,
    FunnelPlotOutlined,
    GatewayOutlined,
    GifOutlined,
    GiftOutlined,
    GithubOutlined,
    GitlabOutlined,
    GlobalOutlined,
    GoldOutlined,
    GooglePlusOutlined,
    GoogleOutlined,
    GroupOutlined,
    HarmonyOSOutlined,
    HddOutlined,
    HeartOutlined,
    HeatMapOutlined,
    HighlightOutlined,
    HistoryOutlined,
    HolderOutlined,
    HomeOutlined,
    HourglassOutlined,
    Html5Outlined,
    IdcardOutlined,
    IeOutlined,
    ImportOutlined,
    InboxOutlined,
    InfoCircleOutlined,
    InfoOutlined,
    InsertRowAboveOutlined,
    InsertRowBelowOutlined,
    InsertRowLeftOutlined,
    InsertRowRightOutlined,
    InstagramOutlined,
    InsuranceOutlined,
    InteractionOutlined,
    IssuesCloseOutlined,
    ItalicOutlined,
    JavaScriptOutlined,
    JavaOutlined,
    KeyOutlined,
    KubernetesOutlined,
    LaptopOutlined,
    LayoutOutlined,
    LeftCircleOutlined,
    LeftSquareOutlined,
    LeftOutlined,
    LikeOutlined,
    LineChartOutlined,
    LineHeightOutlined,
    LineOutlined,
    LinkOutlined,
    LinkedinOutlined,
    LinuxOutlined,
    Loading3QuartersOutlined,
    LoadingOutlined,
    LockOutlined,
    LoginOutlined,
    LogoutOutlined,
    MacCommandOutlined,
    MailOutlined,
    ManOutlined,
    MedicineBoxOutlined,
    MediumWorkmarkOutlined,
    MediumOutlined,
    MehOutlined,
    MenuFoldOutlined,
    MenuUnfoldOutlined,
    MenuOutlined,
    MergeCellsOutlined,
    MergeOutlined,
    MessageOutlined,
    MinusCircleOutlined,
    MinusSquareOutlined,
    MinusOutlined,
    MobileOutlined,
    MoneyCollectOutlined,
    MonitorOutlined,
    MoonOutlined,
    MoreOutlined,
    MutedOutlined,
    NodeCollapseOutlined,
    NodeExpandOutlined,
    NodeIndexOutlined,
    NotificationOutlined,
    NumberOutlined,
    OneToOneOutlined,
    OpenAIOutlined,
    OrderedListOutlined,
    PaperClipOutlined,
    PartitionOutlined,
    PauseCircleOutlined,
    PauseOutlined,
    PayCircleOutlined,
    PercentageOutlined,
    PhoneOutlined,
    PicCenterOutlined,
    PicLeftOutlined,
    PicRightOutlined,
    PictureOutlined,
    PieChartOutlined,
    PinterestOutlined,
    PlayCircleOutlined,
    PlaySquareOutlined,
    PlusCircleOutlined,
    PlusSquareOutlined,
    PlusOutlined,
    PoundCircleOutlined,
    PoundOutlined,
    PoweroffOutlined,
    PrinterOutlined,
    ProductOutlined,
    ProfileOutlined,
    ProjectOutlined,
    PropertySafetyOutlined,
    PullRequestOutlined,
    PushpinOutlined,
    PythonOutlined,
    QqOutlined,
    QrcodeOutlined,
    QuestionCircleOutlined,
    QuestionOutlined,
    RadarChartOutlined,
    RadiusBottomleftOutlined,
    RadiusBottomrightOutlined,
    RadiusSettingOutlined,
    RadiusUpleftOutlined,
    RadiusUprightOutlined,
    ReadOutlined,
    ReconciliationOutlined,
    RedEnvelopeOutlined,
    RedditOutlined,
    RedoOutlined,
    ReloadOutlined,
    RestOutlined,
    RetweetOutlined,
    RightCircleOutlined,
    RightSquareOutlined,
    RightOutlined,
    RiseOutlined,
    RobotOutlined,
    RocketOutlined,
    RollbackOutlined,
    RotateLeftOutlined,
    RotateRightOutlined,
    RubyOutlined,
    SafetyCertificateOutlined,
    SafetyOutlined,
    SaveOutlined,
    ScanOutlined,
    ScheduleOutlined,
    ScissorOutlined,
    SearchOutlined,
    SecurityScanOutlined,
    SelectOutlined,
    SendOutlined,
    SettingOutlined,
    ShakeOutlined,
    ShareAltOutlined,
    ShopOutlined,
    ShoppingCartOutlined,
    ShoppingOutlined,
    ShrinkOutlined,
    SignatureOutlined,
    SisternodeOutlined,
    SketchOutlined,
    SkinOutlined,
    SkypeOutlined,
    SlackSquareOutlined,
    SlackOutlined,
    SlidersOutlined,
    SmallDashOutlined,
    SmileOutlined,
    SnippetsOutlined,
    SolutionOutlined,
    SortAscendingOutlined,
    SortDescendingOutlined,
    SoundOutlined,
    SplitCellsOutlined,
    SpotifyOutlined,
    StarOutlined,
    StepBackwardOutlined,
    StepForwardOutlined,
    StockOutlined,
    StopOutlined,
    StrikethroughOutlined,
    SubnodeOutlined,
    SunOutlined,
    SwapLeftOutlined,
    SwapRightOutlined,
    SwapOutlined,
    SwitcherOutlined,
    SyncOutlined,
    TableOutlined,
    TabletOutlined,
    TagOutlined,
    TagsOutlined,
    TaobaoCircleOutlined,
    TaobaoOutlined,
    TeamOutlined,
    ThunderboltOutlined,
    TikTokOutlined,
    ToTopOutlined,
    ToolOutlined,
    TrademarkCircleOutlined,
    TrademarkOutlined,
    TransactionOutlined,
    TranslationOutlined,
    TrophyOutlined,
    TruckOutlined,
    TwitchOutlined,
    TwitterOutlined,
    UnderlineOutlined,
    UndoOutlined,
    UngroupOutlined,
    UnlockOutlined,
    UnorderedListOutlined,
    UpCircleOutlined,
    UpSquareOutlined,
    UpOutlined,
    UploadOutlined,
    UsbOutlined,
    UserAddOutlined,
    UserDeleteOutlined,
    UserSwitchOutlined,
    UserOutlined,
    UsergroupAddOutlined,
    UsergroupDeleteOutlined,
    VerifiedOutlined,
    VerticalAlignBottomOutlined,
    VerticalAlignMiddleOutlined,
    VerticalAlignTopOutlined,
    VerticalLeftOutlined,
    VerticalRightOutlined,
    VideoCameraAddOutlined,
    VideoCameraOutlined,
    WalletOutlined,
    WarningOutlined,
    WechatWorkOutlined,
    WechatOutlined,
    WeiboCircleOutlined,
    WeiboSquareOutlined,
    WeiboOutlined,
    WhatsAppOutlined,
    WifiOutlined,
    WindowsOutlined,
    WomanOutlined,
    XOutlined,
    YahooOutlined,
    YoutubeOutlined,
    YuqueOutlined,
    ZhihuOutlined,
    ZoomInOutlined,
    ZoomOutOutlined,
    AccountBookFilled,
    AlertFilled,
    AlipayCircleFilled,
    AlipaySquareFilled,
    AliwangwangFilled,
    AmazonCircleFilled,
    AmazonSquareFilled,
    AndroidFilled,
    AnthropicFilled,
    ApiFilled,
    AppleFilled,
    AppstoreFilled,
    AudioFilled,
    BackwardFilled,
    BankFilled,
    BehanceCircleFilled,
    BehanceSquareFilled,
    BellFilled,
    BilibiliFilled,
    BookFilled,
    BoxPlotFilled,
    BugFilled,
    BuildFilled,
    BulbFilled,
    CalculatorFilled,
    CalendarFilled,
    CameraFilled,
    CarFilled,
    CaretDownFilled,
    CaretLeftFilled,
    CaretRightFilled,
    CaretUpFilled,
    CarryOutFilled,
    CheckCircleFilled,
    CheckSquareFilled,
    ChromeFilled,
    CiCircleFilled,
    ClaudeFilled,
    ClockCircleFilled,
    CloseCircleFilled,
    CloseSquareFilled,
    CloudFilled,
    CodeSandboxCircleFilled,
    CodeSandboxSquareFilled,
    CodeFilled,
    CodepenCircleFilled,
    CodepenSquareFilled,
    CompassFilled,
    ContactsFilled,
    ContainerFilled,
    ControlFilled,
    CopyFilled,
    CopyrightCircleFilled,
    CreditCardFilled,
    CrownFilled,
    CustomerServiceFilled,
    DashboardFilled,
    DatabaseFilled,
    DeepSeekFilled,
    DeleteFilled,
    DiffFilled,
    DingtalkCircleFilled,
    DingtalkSquareFilled,
    DiscordFilled,
    DislikeFilled,
    DollarCircleFilled,
    DownCircleFilled,
    DownSquareFilled,
    DribbbleCircleFilled,
    DribbbleSquareFilled,
    DropboxCircleFilled,
    DropboxSquareFilled,
    EditFilled,
    ElevenLabsFilled,
    EnvironmentFilled,
    EuroCircleFilled,
    ExclamationCircleFilled,
    ExperimentFilled,
    EyeInvisibleFilled,
    EyeFilled,
    FacebookFilled,
    FastBackwardFilled,
    FastForwardFilled,
    FileAddFilled,
    FileExcelFilled,
    FileExclamationFilled,
    FileImageFilled,
    FileMarkdownFilled,
    FilePdfFilled,
    FilePptFilled,
    FileTextFilled,
    FileUnknownFilled,
    FileWordFilled,
    FileZipFilled,
    FileFilled,
    FilterFilled,
    FireFilled,
    FlagFilled,
    FolderAddFilled,
    FolderOpenFilled,
    FolderFilled,
    FormatPainterFilled,
    ForwardFilled,
    FrownFilled,
    FundFilled,
    FunnelPlotFilled,
    GeminiFilled,
    GiftFilled,
    GithubFilled,
    GitlabFilled,
    GoldFilled,
    GoldenFilled,
    GoogleCircleFilled,
    GooglePlusCircleFilled,
    GooglePlusSquareFilled,
    GoogleSquareFilled,
    HddFilled,
    HeartFilled,
    HighlightFilled,
    HomeFilled,
    HourglassFilled,
    Html5Filled,
    HuggingFaceFilled,
    IdcardFilled,
    IeCircleFilled,
    IeSquareFilled,
    InfoCircleFilled,
    InstagramFilled,
    InsuranceFilled,
    InteractionFilled,
    LayoutFilled,
    LeftCircleFilled,
    LeftSquareFilled,
    LikeFilled,
    LinkedinFilled,
    LockFilled,
    MacCommandFilled,
    MailFilled,
    MastodonFilled,
    MedicineBoxFilled,
    MediumCircleFilled,
    MediumSquareFilled,
    MehFilled,
    MergeFilled,
    MessageFilled,
    MetaFilled,
    MinusCircleFilled,
    MinusSquareFilled,
    MistralFilled,
    MobileFilled,
    MoneyCollectFilled,
    MoonFilled,
    MutedFilled,
    NetflixFilled,
    NotificationFilled,
    OllamaFilled,
    OpenAIFilled,
    PauseCircleFilled,
    PayCircleFilled,
    PerplexityFilled,
    PhoneFilled,
    PictureFilled,
    PieChartFilled,
    PinterestFilled,
    PlayCircleFilled,
    PlaySquareFilled,
    PlusCircleFilled,
    PlusSquareFilled,
    PoundCircleFilled,
    PrinterFilled,
    ProductFilled,
    ProfileFilled,
    ProjectFilled,
    PropertySafetyFilled,
    PushpinFilled,
    QqCircleFilled,
    QqSquareFilled,
    QuestionCircleFilled,
    QwenFilled,
    ReadFilled,
    ReconciliationFilled,
    RedEnvelopeFilled,
    RedditCircleFilled,
    RedditSquareFilled,
    ReplicateFilled,
    RestFilled,
    RightCircleFilled,
    RightSquareFilled,
    RobotFilled,
    RocketFilled,
    SafetyCertificateFilled,
    SaveFilled,
    ScheduleFilled,
    SecurityScanFilled,
    SettingFilled,
    ShopFilled,
    ShoppingFilled,
    SignalFilled,
    SignatureFilled,
    SketchCircleFilled,
    SketchSquareFilled,
    SkinFilled,
    SkypeFilled,
    SlackCircleFilled,
    SlackSquareFilled,
    SlidersFilled,
    SmileFilled,
    SnapchatFilled,
    SnippetsFilled,
    SoundFilled,
    SpotifyFilled,
    StarFilled,
    StepBackwardFilled,
    StepForwardFilled,
    StopFilled,
    SunFilled,
    SwitcherFilled,
    TabletFilled,
    TagFilled,
    TagsFilled,
    TaobaoCircleFilled,
    TaobaoSquareFilled,
    TelegramFilled,
    ThreadsFilled,
    ThunderboltFilled,
    TikTokFilled,
    ToolFilled,
    TrademarkCircleFilled,
    TrophyFilled,
    TruckFilled,
    TwitchFilled,
    TwitterCircleFilled,
    TwitterSquareFilled,
    UnlockFilled,
    UpCircleFilled,
    UpSquareFilled,
    UsbFilled,
    VideoCameraFilled,
    WalletFilled,
    WarningFilled,
    WechatWorkFilled,
    WechatFilled,
    WeiboCircleFilled,
    WeiboSquareFilled,
    WindowsFilled,
    XFilled,
    YahooFilled,
    YoutubeFilled,
    YuqueFilled,
    ZhihuCircleFilled,
    ZhihuSquareFilled,
    AccountBookTwoTone,
    AlertTwoTone,
    ApiTwoTone,
    AppstoreTwoTone,
    AudioTwoTone,
    BankTwoTone,
    BellTwoTone,
    BookTwoTone,
    BoxPlotTwoTone,
    BugTwoTone,
    BuildTwoTone,
    BulbTwoTone,
    CalculatorTwoTone,
    CalendarTwoTone,
    CameraTwoTone,
    CarTwoTone,
    CarryOutTwoTone,
    CheckCircleTwoTone,
    CheckSquareTwoTone,
    CiCircleTwoTone,
    CiTwoTone,
    ClockCircleTwoTone,
    CloseCircleTwoTone,
    CloseSquareTwoTone,
    CloudTwoTone,
    CodeTwoTone,
    CompassTwoTone,
    ContactsTwoTone,
    ContainerTwoTone,
    ControlTwoTone,
    CopyTwoTone,
    CopyrightCircleTwoTone,
    CopyrightTwoTone,
    CreditCardTwoTone,
    CrownTwoTone,
    CustomerServiceTwoTone,
    DashboardTwoTone,
    DatabaseTwoTone,
    DeleteTwoTone,
    DiffTwoTone,
    DislikeTwoTone,
    DollarCircleTwoTone,
    DollarTwoTone,
    DownCircleTwoTone,
    DownSquareTwoTone,
    EditTwoTone,
    EnvironmentTwoTone,
    EuroCircleTwoTone,
    EuroTwoTone,
    ExclamationCircleTwoTone,
    ExperimentTwoTone,
    EyeInvisibleTwoTone,
    EyeTwoTone,
    FileAddTwoTone,
    FileExcelTwoTone,
    FileExclamationTwoTone,
    FileImageTwoTone,
    FileMarkdownTwoTone,
    FilePdfTwoTone,
    FilePptTwoTone,
    FileTextTwoTone,
    FileUnknownTwoTone,
    FileWordTwoTone,
    FileZipTwoTone,
    FileTwoTone,
    FilterTwoTone,
    FireTwoTone,
    FlagTwoTone,
    FolderAddTwoTone,
    FolderOpenTwoTone,
    FolderTwoTone,
    FrownTwoTone,
    FundTwoTone,
    FunnelPlotTwoTone,
    GiftTwoTone,
    GoldTwoTone,
    HddTwoTone,
    HeartTwoTone,
    HighlightTwoTone,
    HomeTwoTone,
    HourglassTwoTone,
    Html5TwoTone,
    IdcardTwoTone,
    InfoCircleTwoTone,
    InsuranceTwoTone,
    InteractionTwoTone,
    LayoutTwoTone,
    LeftCircleTwoTone,
    LeftSquareTwoTone,
    LikeTwoTone,
    LockTwoTone,
    MailTwoTone,
    MedicineBoxTwoTone,
    MehTwoTone,
    MessageTwoTone,
    MinusCircleTwoTone,
    MinusSquareTwoTone,
    MobileTwoTone,
    MoneyCollectTwoTone,
    NotificationTwoTone,
    PauseCircleTwoTone,
    PhoneTwoTone,
    PictureTwoTone,
    PieChartTwoTone,
    PlayCircleTwoTone,
    PlaySquareTwoTone,
    PlusCircleTwoTone,
    PlusSquareTwoTone,
    PoundCircleTwoTone,
    PrinterTwoTone,
    ProfileTwoTone,
    ProjectTwoTone,
    PropertySafetyTwoTone,
    PushpinTwoTone,
    QuestionCircleTwoTone,
    ReconciliationTwoTone,
    RedEnvelopeTwoTone,
    RestTwoTone,
    RightCircleTwoTone,
    RightSquareTwoTone,
    RocketTwoTone,
    SafetyCertificateTwoTone,
    SaveTwoTone,
    ScheduleTwoTone,
    SecurityScanTwoTone,
    SettingTwoTone,
    ShopTwoTone,
    ShoppingTwoTone,
    SkinTwoTone,
    SlidersTwoTone,
    SmileTwoTone,
    SnippetsTwoTone,
    SoundTwoTone,
    StarTwoTone,
    StopTwoTone,
    SwitcherTwoTone,
    TabletTwoTone,
    TagTwoTone,
    TagsTwoTone,
    ThunderboltTwoTone,
    ToolTwoTone,
    TrademarkCircleTwoTone,
    TrophyTwoTone,
    UnlockTwoTone,
    UpCircleTwoTone,
    UpSquareTwoTone,
    UsbTwoTone,
    VideoCameraTwoTone,
    WalletTwoTone,
    WarningTwoTone,
}

impl AntIconName {
    pub const ALL: &'static [Self] = &[
        Self::AccountBookOutlined,
        Self::AimOutlined,
        Self::AlertOutlined,
        Self::AlibabaOutlined,
        Self::AlignCenterOutlined,
        Self::AlignLeftOutlined,
        Self::AlignRightOutlined,
        Self::AlipayCircleOutlined,
        Self::AlipayOutlined,
        Self::AliwangwangOutlined,
        Self::AliyunOutlined,
        Self::AmazonOutlined,
        Self::AndroidOutlined,
        Self::AntCloudOutlined,
        Self::AntDesignOutlined,
        Self::ApartmentOutlined,
        Self::ApiOutlined,
        Self::AppleOutlined,
        Self::AppstoreAddOutlined,
        Self::AppstoreOutlined,
        Self::AreaChartOutlined,
        Self::ArrowDownOutlined,
        Self::ArrowLeftOutlined,
        Self::ArrowRightOutlined,
        Self::ArrowUpOutlined,
        Self::ArrowsAltOutlined,
        Self::AudioMutedOutlined,
        Self::AudioOutlined,
        Self::AuditOutlined,
        Self::BackwardOutlined,
        Self::BaiduOutlined,
        Self::BankOutlined,
        Self::BarChartOutlined,
        Self::BarcodeOutlined,
        Self::BarsOutlined,
        Self::BehanceSquareOutlined,
        Self::BehanceOutlined,
        Self::BellOutlined,
        Self::BgColorsOutlined,
        Self::BilibiliOutlined,
        Self::BlockOutlined,
        Self::BoldOutlined,
        Self::BookOutlined,
        Self::BorderBottomOutlined,
        Self::BorderHorizontalOutlined,
        Self::BorderInnerOutlined,
        Self::BorderLeftOutlined,
        Self::BorderOuterOutlined,
        Self::BorderRightOutlined,
        Self::BorderTopOutlined,
        Self::BorderVerticleOutlined,
        Self::BorderOutlined,
        Self::BorderlessTableOutlined,
        Self::BoxPlotOutlined,
        Self::BranchesOutlined,
        Self::BugOutlined,
        Self::BuildOutlined,
        Self::BulbOutlined,
        Self::CalculatorOutlined,
        Self::CalendarOutlined,
        Self::CameraOutlined,
        Self::CarOutlined,
        Self::CaretDownOutlined,
        Self::CaretLeftOutlined,
        Self::CaretRightOutlined,
        Self::CaretUpOutlined,
        Self::CarryOutOutlined,
        Self::CheckCircleOutlined,
        Self::CheckSquareOutlined,
        Self::CheckOutlined,
        Self::ChromeOutlined,
        Self::CiCircleOutlined,
        Self::CiOutlined,
        Self::ClearOutlined,
        Self::ClockCircleOutlined,
        Self::CloseCircleOutlined,
        Self::CloseSquareOutlined,
        Self::CloseOutlined,
        Self::CloudDownloadOutlined,
        Self::CloudServerOutlined,
        Self::CloudSyncOutlined,
        Self::CloudUploadOutlined,
        Self::CloudOutlined,
        Self::ClusterOutlined,
        Self::CodeSandboxOutlined,
        Self::CodeOutlined,
        Self::CodepenCircleOutlined,
        Self::CodepenOutlined,
        Self::CoffeeOutlined,
        Self::ColumnHeightOutlined,
        Self::ColumnWidthOutlined,
        Self::CommentOutlined,
        Self::CompassOutlined,
        Self::CompressOutlined,
        Self::ConsoleSqlOutlined,
        Self::ContactsOutlined,
        Self::ContainerOutlined,
        Self::ControlOutlined,
        Self::CopyOutlined,
        Self::CopyrightCircleOutlined,
        Self::CopyrightOutlined,
        Self::CreditCardOutlined,
        Self::CrownOutlined,
        Self::CustomerServiceOutlined,
        Self::DashOutlined,
        Self::DashboardOutlined,
        Self::DatabaseOutlined,
        Self::DeleteColumnOutlined,
        Self::DeleteRowOutlined,
        Self::DeleteOutlined,
        Self::DeliveredProcedureOutlined,
        Self::DeploymentUnitOutlined,
        Self::DesktopOutlined,
        Self::DiffOutlined,
        Self::DingdingOutlined,
        Self::DingtalkOutlined,
        Self::DisconnectOutlined,
        Self::DiscordOutlined,
        Self::DislikeOutlined,
        Self::DockerOutlined,
        Self::DollarCircleOutlined,
        Self::DollarOutlined,
        Self::DotChartOutlined,
        Self::DotNetOutlined,
        Self::DoubleLeftOutlined,
        Self::DoubleRightOutlined,
        Self::DownCircleOutlined,
        Self::DownSquareOutlined,
        Self::DownOutlined,
        Self::DownloadOutlined,
        Self::DragOutlined,
        Self::DribbbleSquareOutlined,
        Self::DribbbleOutlined,
        Self::DropboxOutlined,
        Self::EditOutlined,
        Self::EllipsisOutlined,
        Self::EnterOutlined,
        Self::EnvironmentOutlined,
        Self::EuroCircleOutlined,
        Self::EuroOutlined,
        Self::ExceptionOutlined,
        Self::ExclamationCircleOutlined,
        Self::ExclamationOutlined,
        Self::ExpandAltOutlined,
        Self::ExpandOutlined,
        Self::ExperimentOutlined,
        Self::ExportOutlined,
        Self::EyeInvisibleOutlined,
        Self::EyeOutlined,
        Self::FacebookOutlined,
        Self::FallOutlined,
        Self::FastBackwardOutlined,
        Self::FastForwardOutlined,
        Self::FieldBinaryOutlined,
        Self::FieldNumberOutlined,
        Self::FieldStringOutlined,
        Self::FieldTimeOutlined,
        Self::FileAddOutlined,
        Self::FileDoneOutlined,
        Self::FileExcelOutlined,
        Self::FileExclamationOutlined,
        Self::FileGifOutlined,
        Self::FileImageOutlined,
        Self::FileJpgOutlined,
        Self::FileMarkdownOutlined,
        Self::FilePdfOutlined,
        Self::FilePptOutlined,
        Self::FileProtectOutlined,
        Self::FileSearchOutlined,
        Self::FileSyncOutlined,
        Self::FileTextOutlined,
        Self::FileUnknownOutlined,
        Self::FileWordOutlined,
        Self::FileZipOutlined,
        Self::FileOutlined,
        Self::FilterOutlined,
        Self::FireOutlined,
        Self::FlagOutlined,
        Self::FolderAddOutlined,
        Self::FolderOpenOutlined,
        Self::FolderViewOutlined,
        Self::FolderOutlined,
        Self::FontColorsOutlined,
        Self::FontSizeOutlined,
        Self::ForkOutlined,
        Self::FormOutlined,
        Self::FormatPainterOutlined,
        Self::ForwardOutlined,
        Self::FrownOutlined,
        Self::FullscreenExitOutlined,
        Self::FullscreenOutlined,
        Self::FunctionOutlined,
        Self::FundProjectionScreenOutlined,
        Self::FundViewOutlined,
        Self::FundOutlined,
        Self::FunnelPlotOutlined,
        Self::GatewayOutlined,
        Self::GifOutlined,
        Self::GiftOutlined,
        Self::GithubOutlined,
        Self::GitlabOutlined,
        Self::GlobalOutlined,
        Self::GoldOutlined,
        Self::GooglePlusOutlined,
        Self::GoogleOutlined,
        Self::GroupOutlined,
        Self::HarmonyOSOutlined,
        Self::HddOutlined,
        Self::HeartOutlined,
        Self::HeatMapOutlined,
        Self::HighlightOutlined,
        Self::HistoryOutlined,
        Self::HolderOutlined,
        Self::HomeOutlined,
        Self::HourglassOutlined,
        Self::Html5Outlined,
        Self::IdcardOutlined,
        Self::IeOutlined,
        Self::ImportOutlined,
        Self::InboxOutlined,
        Self::InfoCircleOutlined,
        Self::InfoOutlined,
        Self::InsertRowAboveOutlined,
        Self::InsertRowBelowOutlined,
        Self::InsertRowLeftOutlined,
        Self::InsertRowRightOutlined,
        Self::InstagramOutlined,
        Self::InsuranceOutlined,
        Self::InteractionOutlined,
        Self::IssuesCloseOutlined,
        Self::ItalicOutlined,
        Self::JavaScriptOutlined,
        Self::JavaOutlined,
        Self::KeyOutlined,
        Self::KubernetesOutlined,
        Self::LaptopOutlined,
        Self::LayoutOutlined,
        Self::LeftCircleOutlined,
        Self::LeftSquareOutlined,
        Self::LeftOutlined,
        Self::LikeOutlined,
        Self::LineChartOutlined,
        Self::LineHeightOutlined,
        Self::LineOutlined,
        Self::LinkOutlined,
        Self::LinkedinOutlined,
        Self::LinuxOutlined,
        Self::Loading3QuartersOutlined,
        Self::LoadingOutlined,
        Self::LockOutlined,
        Self::LoginOutlined,
        Self::LogoutOutlined,
        Self::MacCommandOutlined,
        Self::MailOutlined,
        Self::ManOutlined,
        Self::MedicineBoxOutlined,
        Self::MediumWorkmarkOutlined,
        Self::MediumOutlined,
        Self::MehOutlined,
        Self::MenuFoldOutlined,
        Self::MenuUnfoldOutlined,
        Self::MenuOutlined,
        Self::MergeCellsOutlined,
        Self::MergeOutlined,
        Self::MessageOutlined,
        Self::MinusCircleOutlined,
        Self::MinusSquareOutlined,
        Self::MinusOutlined,
        Self::MobileOutlined,
        Self::MoneyCollectOutlined,
        Self::MonitorOutlined,
        Self::MoonOutlined,
        Self::MoreOutlined,
        Self::MutedOutlined,
        Self::NodeCollapseOutlined,
        Self::NodeExpandOutlined,
        Self::NodeIndexOutlined,
        Self::NotificationOutlined,
        Self::NumberOutlined,
        Self::OneToOneOutlined,
        Self::OpenAIOutlined,
        Self::OrderedListOutlined,
        Self::PaperClipOutlined,
        Self::PartitionOutlined,
        Self::PauseCircleOutlined,
        Self::PauseOutlined,
        Self::PayCircleOutlined,
        Self::PercentageOutlined,
        Self::PhoneOutlined,
        Self::PicCenterOutlined,
        Self::PicLeftOutlined,
        Self::PicRightOutlined,
        Self::PictureOutlined,
        Self::PieChartOutlined,
        Self::PinterestOutlined,
        Self::PlayCircleOutlined,
        Self::PlaySquareOutlined,
        Self::PlusCircleOutlined,
        Self::PlusSquareOutlined,
        Self::PlusOutlined,
        Self::PoundCircleOutlined,
        Self::PoundOutlined,
        Self::PoweroffOutlined,
        Self::PrinterOutlined,
        Self::ProductOutlined,
        Self::ProfileOutlined,
        Self::ProjectOutlined,
        Self::PropertySafetyOutlined,
        Self::PullRequestOutlined,
        Self::PushpinOutlined,
        Self::PythonOutlined,
        Self::QqOutlined,
        Self::QrcodeOutlined,
        Self::QuestionCircleOutlined,
        Self::QuestionOutlined,
        Self::RadarChartOutlined,
        Self::RadiusBottomleftOutlined,
        Self::RadiusBottomrightOutlined,
        Self::RadiusSettingOutlined,
        Self::RadiusUpleftOutlined,
        Self::RadiusUprightOutlined,
        Self::ReadOutlined,
        Self::ReconciliationOutlined,
        Self::RedEnvelopeOutlined,
        Self::RedditOutlined,
        Self::RedoOutlined,
        Self::ReloadOutlined,
        Self::RestOutlined,
        Self::RetweetOutlined,
        Self::RightCircleOutlined,
        Self::RightSquareOutlined,
        Self::RightOutlined,
        Self::RiseOutlined,
        Self::RobotOutlined,
        Self::RocketOutlined,
        Self::RollbackOutlined,
        Self::RotateLeftOutlined,
        Self::RotateRightOutlined,
        Self::RubyOutlined,
        Self::SafetyCertificateOutlined,
        Self::SafetyOutlined,
        Self::SaveOutlined,
        Self::ScanOutlined,
        Self::ScheduleOutlined,
        Self::ScissorOutlined,
        Self::SearchOutlined,
        Self::SecurityScanOutlined,
        Self::SelectOutlined,
        Self::SendOutlined,
        Self::SettingOutlined,
        Self::ShakeOutlined,
        Self::ShareAltOutlined,
        Self::ShopOutlined,
        Self::ShoppingCartOutlined,
        Self::ShoppingOutlined,
        Self::ShrinkOutlined,
        Self::SignatureOutlined,
        Self::SisternodeOutlined,
        Self::SketchOutlined,
        Self::SkinOutlined,
        Self::SkypeOutlined,
        Self::SlackSquareOutlined,
        Self::SlackOutlined,
        Self::SlidersOutlined,
        Self::SmallDashOutlined,
        Self::SmileOutlined,
        Self::SnippetsOutlined,
        Self::SolutionOutlined,
        Self::SortAscendingOutlined,
        Self::SortDescendingOutlined,
        Self::SoundOutlined,
        Self::SplitCellsOutlined,
        Self::SpotifyOutlined,
        Self::StarOutlined,
        Self::StepBackwardOutlined,
        Self::StepForwardOutlined,
        Self::StockOutlined,
        Self::StopOutlined,
        Self::StrikethroughOutlined,
        Self::SubnodeOutlined,
        Self::SunOutlined,
        Self::SwapLeftOutlined,
        Self::SwapRightOutlined,
        Self::SwapOutlined,
        Self::SwitcherOutlined,
        Self::SyncOutlined,
        Self::TableOutlined,
        Self::TabletOutlined,
        Self::TagOutlined,
        Self::TagsOutlined,
        Self::TaobaoCircleOutlined,
        Self::TaobaoOutlined,
        Self::TeamOutlined,
        Self::ThunderboltOutlined,
        Self::TikTokOutlined,
        Self::ToTopOutlined,
        Self::ToolOutlined,
        Self::TrademarkCircleOutlined,
        Self::TrademarkOutlined,
        Self::TransactionOutlined,
        Self::TranslationOutlined,
        Self::TrophyOutlined,
        Self::TruckOutlined,
        Self::TwitchOutlined,
        Self::TwitterOutlined,
        Self::UnderlineOutlined,
        Self::UndoOutlined,
        Self::UngroupOutlined,
        Self::UnlockOutlined,
        Self::UnorderedListOutlined,
        Self::UpCircleOutlined,
        Self::UpSquareOutlined,
        Self::UpOutlined,
        Self::UploadOutlined,
        Self::UsbOutlined,
        Self::UserAddOutlined,
        Self::UserDeleteOutlined,
        Self::UserSwitchOutlined,
        Self::UserOutlined,
        Self::UsergroupAddOutlined,
        Self::UsergroupDeleteOutlined,
        Self::VerifiedOutlined,
        Self::VerticalAlignBottomOutlined,
        Self::VerticalAlignMiddleOutlined,
        Self::VerticalAlignTopOutlined,
        Self::VerticalLeftOutlined,
        Self::VerticalRightOutlined,
        Self::VideoCameraAddOutlined,
        Self::VideoCameraOutlined,
        Self::WalletOutlined,
        Self::WarningOutlined,
        Self::WechatWorkOutlined,
        Self::WechatOutlined,
        Self::WeiboCircleOutlined,
        Self::WeiboSquareOutlined,
        Self::WeiboOutlined,
        Self::WhatsAppOutlined,
        Self::WifiOutlined,
        Self::WindowsOutlined,
        Self::WomanOutlined,
        Self::XOutlined,
        Self::YahooOutlined,
        Self::YoutubeOutlined,
        Self::YuqueOutlined,
        Self::ZhihuOutlined,
        Self::ZoomInOutlined,
        Self::ZoomOutOutlined,
        Self::AccountBookFilled,
        Self::AlertFilled,
        Self::AlipayCircleFilled,
        Self::AlipaySquareFilled,
        Self::AliwangwangFilled,
        Self::AmazonCircleFilled,
        Self::AmazonSquareFilled,
        Self::AndroidFilled,
        Self::AnthropicFilled,
        Self::ApiFilled,
        Self::AppleFilled,
        Self::AppstoreFilled,
        Self::AudioFilled,
        Self::BackwardFilled,
        Self::BankFilled,
        Self::BehanceCircleFilled,
        Self::BehanceSquareFilled,
        Self::BellFilled,
        Self::BilibiliFilled,
        Self::BookFilled,
        Self::BoxPlotFilled,
        Self::BugFilled,
        Self::BuildFilled,
        Self::BulbFilled,
        Self::CalculatorFilled,
        Self::CalendarFilled,
        Self::CameraFilled,
        Self::CarFilled,
        Self::CaretDownFilled,
        Self::CaretLeftFilled,
        Self::CaretRightFilled,
        Self::CaretUpFilled,
        Self::CarryOutFilled,
        Self::CheckCircleFilled,
        Self::CheckSquareFilled,
        Self::ChromeFilled,
        Self::CiCircleFilled,
        Self::ClaudeFilled,
        Self::ClockCircleFilled,
        Self::CloseCircleFilled,
        Self::CloseSquareFilled,
        Self::CloudFilled,
        Self::CodeSandboxCircleFilled,
        Self::CodeSandboxSquareFilled,
        Self::CodeFilled,
        Self::CodepenCircleFilled,
        Self::CodepenSquareFilled,
        Self::CompassFilled,
        Self::ContactsFilled,
        Self::ContainerFilled,
        Self::ControlFilled,
        Self::CopyFilled,
        Self::CopyrightCircleFilled,
        Self::CreditCardFilled,
        Self::CrownFilled,
        Self::CustomerServiceFilled,
        Self::DashboardFilled,
        Self::DatabaseFilled,
        Self::DeepSeekFilled,
        Self::DeleteFilled,
        Self::DiffFilled,
        Self::DingtalkCircleFilled,
        Self::DingtalkSquareFilled,
        Self::DiscordFilled,
        Self::DislikeFilled,
        Self::DollarCircleFilled,
        Self::DownCircleFilled,
        Self::DownSquareFilled,
        Self::DribbbleCircleFilled,
        Self::DribbbleSquareFilled,
        Self::DropboxCircleFilled,
        Self::DropboxSquareFilled,
        Self::EditFilled,
        Self::ElevenLabsFilled,
        Self::EnvironmentFilled,
        Self::EuroCircleFilled,
        Self::ExclamationCircleFilled,
        Self::ExperimentFilled,
        Self::EyeInvisibleFilled,
        Self::EyeFilled,
        Self::FacebookFilled,
        Self::FastBackwardFilled,
        Self::FastForwardFilled,
        Self::FileAddFilled,
        Self::FileExcelFilled,
        Self::FileExclamationFilled,
        Self::FileImageFilled,
        Self::FileMarkdownFilled,
        Self::FilePdfFilled,
        Self::FilePptFilled,
        Self::FileTextFilled,
        Self::FileUnknownFilled,
        Self::FileWordFilled,
        Self::FileZipFilled,
        Self::FileFilled,
        Self::FilterFilled,
        Self::FireFilled,
        Self::FlagFilled,
        Self::FolderAddFilled,
        Self::FolderOpenFilled,
        Self::FolderFilled,
        Self::FormatPainterFilled,
        Self::ForwardFilled,
        Self::FrownFilled,
        Self::FundFilled,
        Self::FunnelPlotFilled,
        Self::GeminiFilled,
        Self::GiftFilled,
        Self::GithubFilled,
        Self::GitlabFilled,
        Self::GoldFilled,
        Self::GoldenFilled,
        Self::GoogleCircleFilled,
        Self::GooglePlusCircleFilled,
        Self::GooglePlusSquareFilled,
        Self::GoogleSquareFilled,
        Self::HddFilled,
        Self::HeartFilled,
        Self::HighlightFilled,
        Self::HomeFilled,
        Self::HourglassFilled,
        Self::Html5Filled,
        Self::HuggingFaceFilled,
        Self::IdcardFilled,
        Self::IeCircleFilled,
        Self::IeSquareFilled,
        Self::InfoCircleFilled,
        Self::InstagramFilled,
        Self::InsuranceFilled,
        Self::InteractionFilled,
        Self::LayoutFilled,
        Self::LeftCircleFilled,
        Self::LeftSquareFilled,
        Self::LikeFilled,
        Self::LinkedinFilled,
        Self::LockFilled,
        Self::MacCommandFilled,
        Self::MailFilled,
        Self::MastodonFilled,
        Self::MedicineBoxFilled,
        Self::MediumCircleFilled,
        Self::MediumSquareFilled,
        Self::MehFilled,
        Self::MergeFilled,
        Self::MessageFilled,
        Self::MetaFilled,
        Self::MinusCircleFilled,
        Self::MinusSquareFilled,
        Self::MistralFilled,
        Self::MobileFilled,
        Self::MoneyCollectFilled,
        Self::MoonFilled,
        Self::MutedFilled,
        Self::NetflixFilled,
        Self::NotificationFilled,
        Self::OllamaFilled,
        Self::OpenAIFilled,
        Self::PauseCircleFilled,
        Self::PayCircleFilled,
        Self::PerplexityFilled,
        Self::PhoneFilled,
        Self::PictureFilled,
        Self::PieChartFilled,
        Self::PinterestFilled,
        Self::PlayCircleFilled,
        Self::PlaySquareFilled,
        Self::PlusCircleFilled,
        Self::PlusSquareFilled,
        Self::PoundCircleFilled,
        Self::PrinterFilled,
        Self::ProductFilled,
        Self::ProfileFilled,
        Self::ProjectFilled,
        Self::PropertySafetyFilled,
        Self::PushpinFilled,
        Self::QqCircleFilled,
        Self::QqSquareFilled,
        Self::QuestionCircleFilled,
        Self::QwenFilled,
        Self::ReadFilled,
        Self::ReconciliationFilled,
        Self::RedEnvelopeFilled,
        Self::RedditCircleFilled,
        Self::RedditSquareFilled,
        Self::ReplicateFilled,
        Self::RestFilled,
        Self::RightCircleFilled,
        Self::RightSquareFilled,
        Self::RobotFilled,
        Self::RocketFilled,
        Self::SafetyCertificateFilled,
        Self::SaveFilled,
        Self::ScheduleFilled,
        Self::SecurityScanFilled,
        Self::SettingFilled,
        Self::ShopFilled,
        Self::ShoppingFilled,
        Self::SignalFilled,
        Self::SignatureFilled,
        Self::SketchCircleFilled,
        Self::SketchSquareFilled,
        Self::SkinFilled,
        Self::SkypeFilled,
        Self::SlackCircleFilled,
        Self::SlackSquareFilled,
        Self::SlidersFilled,
        Self::SmileFilled,
        Self::SnapchatFilled,
        Self::SnippetsFilled,
        Self::SoundFilled,
        Self::SpotifyFilled,
        Self::StarFilled,
        Self::StepBackwardFilled,
        Self::StepForwardFilled,
        Self::StopFilled,
        Self::SunFilled,
        Self::SwitcherFilled,
        Self::TabletFilled,
        Self::TagFilled,
        Self::TagsFilled,
        Self::TaobaoCircleFilled,
        Self::TaobaoSquareFilled,
        Self::TelegramFilled,
        Self::ThreadsFilled,
        Self::ThunderboltFilled,
        Self::TikTokFilled,
        Self::ToolFilled,
        Self::TrademarkCircleFilled,
        Self::TrophyFilled,
        Self::TruckFilled,
        Self::TwitchFilled,
        Self::TwitterCircleFilled,
        Self::TwitterSquareFilled,
        Self::UnlockFilled,
        Self::UpCircleFilled,
        Self::UpSquareFilled,
        Self::UsbFilled,
        Self::VideoCameraFilled,
        Self::WalletFilled,
        Self::WarningFilled,
        Self::WechatWorkFilled,
        Self::WechatFilled,
        Self::WeiboCircleFilled,
        Self::WeiboSquareFilled,
        Self::WindowsFilled,
        Self::XFilled,
        Self::YahooFilled,
        Self::YoutubeFilled,
        Self::YuqueFilled,
        Self::ZhihuCircleFilled,
        Self::ZhihuSquareFilled,
        Self::AccountBookTwoTone,
        Self::AlertTwoTone,
        Self::ApiTwoTone,
        Self::AppstoreTwoTone,
        Self::AudioTwoTone,
        Self::BankTwoTone,
        Self::BellTwoTone,
        Self::BookTwoTone,
        Self::BoxPlotTwoTone,
        Self::BugTwoTone,
        Self::BuildTwoTone,
        Self::BulbTwoTone,
        Self::CalculatorTwoTone,
        Self::CalendarTwoTone,
        Self::CameraTwoTone,
        Self::CarTwoTone,
        Self::CarryOutTwoTone,
        Self::CheckCircleTwoTone,
        Self::CheckSquareTwoTone,
        Self::CiCircleTwoTone,
        Self::CiTwoTone,
        Self::ClockCircleTwoTone,
        Self::CloseCircleTwoTone,
        Self::CloseSquareTwoTone,
        Self::CloudTwoTone,
        Self::CodeTwoTone,
        Self::CompassTwoTone,
        Self::ContactsTwoTone,
        Self::ContainerTwoTone,
        Self::ControlTwoTone,
        Self::CopyTwoTone,
        Self::CopyrightCircleTwoTone,
        Self::CopyrightTwoTone,
        Self::CreditCardTwoTone,
        Self::CrownTwoTone,
        Self::CustomerServiceTwoTone,
        Self::DashboardTwoTone,
        Self::DatabaseTwoTone,
        Self::DeleteTwoTone,
        Self::DiffTwoTone,
        Self::DislikeTwoTone,
        Self::DollarCircleTwoTone,
        Self::DollarTwoTone,
        Self::DownCircleTwoTone,
        Self::DownSquareTwoTone,
        Self::EditTwoTone,
        Self::EnvironmentTwoTone,
        Self::EuroCircleTwoTone,
        Self::EuroTwoTone,
        Self::ExclamationCircleTwoTone,
        Self::ExperimentTwoTone,
        Self::EyeInvisibleTwoTone,
        Self::EyeTwoTone,
        Self::FileAddTwoTone,
        Self::FileExcelTwoTone,
        Self::FileExclamationTwoTone,
        Self::FileImageTwoTone,
        Self::FileMarkdownTwoTone,
        Self::FilePdfTwoTone,
        Self::FilePptTwoTone,
        Self::FileTextTwoTone,
        Self::FileUnknownTwoTone,
        Self::FileWordTwoTone,
        Self::FileZipTwoTone,
        Self::FileTwoTone,
        Self::FilterTwoTone,
        Self::FireTwoTone,
        Self::FlagTwoTone,
        Self::FolderAddTwoTone,
        Self::FolderOpenTwoTone,
        Self::FolderTwoTone,
        Self::FrownTwoTone,
        Self::FundTwoTone,
        Self::FunnelPlotTwoTone,
        Self::GiftTwoTone,
        Self::GoldTwoTone,
        Self::HddTwoTone,
        Self::HeartTwoTone,
        Self::HighlightTwoTone,
        Self::HomeTwoTone,
        Self::HourglassTwoTone,
        Self::Html5TwoTone,
        Self::IdcardTwoTone,
        Self::InfoCircleTwoTone,
        Self::InsuranceTwoTone,
        Self::InteractionTwoTone,
        Self::LayoutTwoTone,
        Self::LeftCircleTwoTone,
        Self::LeftSquareTwoTone,
        Self::LikeTwoTone,
        Self::LockTwoTone,
        Self::MailTwoTone,
        Self::MedicineBoxTwoTone,
        Self::MehTwoTone,
        Self::MessageTwoTone,
        Self::MinusCircleTwoTone,
        Self::MinusSquareTwoTone,
        Self::MobileTwoTone,
        Self::MoneyCollectTwoTone,
        Self::NotificationTwoTone,
        Self::PauseCircleTwoTone,
        Self::PhoneTwoTone,
        Self::PictureTwoTone,
        Self::PieChartTwoTone,
        Self::PlayCircleTwoTone,
        Self::PlaySquareTwoTone,
        Self::PlusCircleTwoTone,
        Self::PlusSquareTwoTone,
        Self::PoundCircleTwoTone,
        Self::PrinterTwoTone,
        Self::ProfileTwoTone,
        Self::ProjectTwoTone,
        Self::PropertySafetyTwoTone,
        Self::PushpinTwoTone,
        Self::QuestionCircleTwoTone,
        Self::ReconciliationTwoTone,
        Self::RedEnvelopeTwoTone,
        Self::RestTwoTone,
        Self::RightCircleTwoTone,
        Self::RightSquareTwoTone,
        Self::RocketTwoTone,
        Self::SafetyCertificateTwoTone,
        Self::SaveTwoTone,
        Self::ScheduleTwoTone,
        Self::SecurityScanTwoTone,
        Self::SettingTwoTone,
        Self::ShopTwoTone,
        Self::ShoppingTwoTone,
        Self::SkinTwoTone,
        Self::SlidersTwoTone,
        Self::SmileTwoTone,
        Self::SnippetsTwoTone,
        Self::SoundTwoTone,
        Self::StarTwoTone,
        Self::StopTwoTone,
        Self::SwitcherTwoTone,
        Self::TabletTwoTone,
        Self::TagTwoTone,
        Self::TagsTwoTone,
        Self::ThunderboltTwoTone,
        Self::ToolTwoTone,
        Self::TrademarkCircleTwoTone,
        Self::TrophyTwoTone,
        Self::UnlockTwoTone,
        Self::UpCircleTwoTone,
        Self::UpSquareTwoTone,
        Self::UsbTwoTone,
        Self::VideoCameraTwoTone,
        Self::WalletTwoTone,
        Self::WarningTwoTone,
    ];

    pub const fn path(self) -> &'static str {
        match self {
            Self::AccountBookOutlined => "antd-icons/outlined/account-book.svg",
            Self::AimOutlined => "antd-icons/outlined/aim.svg",
            Self::AlertOutlined => "antd-icons/outlined/alert.svg",
            Self::AlibabaOutlined => "antd-icons/outlined/alibaba.svg",
            Self::AlignCenterOutlined => "antd-icons/outlined/align-center.svg",
            Self::AlignLeftOutlined => "antd-icons/outlined/align-left.svg",
            Self::AlignRightOutlined => "antd-icons/outlined/align-right.svg",
            Self::AlipayCircleOutlined => "antd-icons/outlined/alipay-circle.svg",
            Self::AlipayOutlined => "antd-icons/outlined/alipay.svg",
            Self::AliwangwangOutlined => "antd-icons/outlined/aliwangwang.svg",
            Self::AliyunOutlined => "antd-icons/outlined/aliyun.svg",
            Self::AmazonOutlined => "antd-icons/outlined/amazon.svg",
            Self::AndroidOutlined => "antd-icons/outlined/android.svg",
            Self::AntCloudOutlined => "antd-icons/outlined/ant-cloud.svg",
            Self::AntDesignOutlined => "antd-icons/outlined/ant-design.svg",
            Self::ApartmentOutlined => "antd-icons/outlined/apartment.svg",
            Self::ApiOutlined => "antd-icons/outlined/api.svg",
            Self::AppleOutlined => "antd-icons/outlined/apple.svg",
            Self::AppstoreAddOutlined => "antd-icons/outlined/appstore-add.svg",
            Self::AppstoreOutlined => "antd-icons/outlined/appstore.svg",
            Self::AreaChartOutlined => "antd-icons/outlined/area-chart.svg",
            Self::ArrowDownOutlined => "antd-icons/outlined/arrow-down.svg",
            Self::ArrowLeftOutlined => "antd-icons/outlined/arrow-left.svg",
            Self::ArrowRightOutlined => "antd-icons/outlined/arrow-right.svg",
            Self::ArrowUpOutlined => "antd-icons/outlined/arrow-up.svg",
            Self::ArrowsAltOutlined => "antd-icons/outlined/arrows-alt.svg",
            Self::AudioMutedOutlined => "antd-icons/outlined/audio-muted.svg",
            Self::AudioOutlined => "antd-icons/outlined/audio.svg",
            Self::AuditOutlined => "antd-icons/outlined/audit.svg",
            Self::BackwardOutlined => "antd-icons/outlined/backward.svg",
            Self::BaiduOutlined => "antd-icons/outlined/baidu.svg",
            Self::BankOutlined => "antd-icons/outlined/bank.svg",
            Self::BarChartOutlined => "antd-icons/outlined/bar-chart.svg",
            Self::BarcodeOutlined => "antd-icons/outlined/barcode.svg",
            Self::BarsOutlined => "antd-icons/outlined/bars.svg",
            Self::BehanceSquareOutlined => "antd-icons/outlined/behance-square.svg",
            Self::BehanceOutlined => "antd-icons/outlined/behance.svg",
            Self::BellOutlined => "antd-icons/outlined/bell.svg",
            Self::BgColorsOutlined => "antd-icons/outlined/bg-colors.svg",
            Self::BilibiliOutlined => "antd-icons/outlined/bilibili.svg",
            Self::BlockOutlined => "antd-icons/outlined/block.svg",
            Self::BoldOutlined => "antd-icons/outlined/bold.svg",
            Self::BookOutlined => "antd-icons/outlined/book.svg",
            Self::BorderBottomOutlined => "antd-icons/outlined/border-bottom.svg",
            Self::BorderHorizontalOutlined => "antd-icons/outlined/border-horizontal.svg",
            Self::BorderInnerOutlined => "antd-icons/outlined/border-inner.svg",
            Self::BorderLeftOutlined => "antd-icons/outlined/border-left.svg",
            Self::BorderOuterOutlined => "antd-icons/outlined/border-outer.svg",
            Self::BorderRightOutlined => "antd-icons/outlined/border-right.svg",
            Self::BorderTopOutlined => "antd-icons/outlined/border-top.svg",
            Self::BorderVerticleOutlined => "antd-icons/outlined/border-verticle.svg",
            Self::BorderOutlined => "antd-icons/outlined/border.svg",
            Self::BorderlessTableOutlined => "antd-icons/outlined/borderless-table.svg",
            Self::BoxPlotOutlined => "antd-icons/outlined/box-plot.svg",
            Self::BranchesOutlined => "antd-icons/outlined/branches.svg",
            Self::BugOutlined => "antd-icons/outlined/bug.svg",
            Self::BuildOutlined => "antd-icons/outlined/build.svg",
            Self::BulbOutlined => "antd-icons/outlined/bulb.svg",
            Self::CalculatorOutlined => "antd-icons/outlined/calculator.svg",
            Self::CalendarOutlined => "antd-icons/outlined/calendar.svg",
            Self::CameraOutlined => "antd-icons/outlined/camera.svg",
            Self::CarOutlined => "antd-icons/outlined/car.svg",
            Self::CaretDownOutlined => "antd-icons/outlined/caret-down.svg",
            Self::CaretLeftOutlined => "antd-icons/outlined/caret-left.svg",
            Self::CaretRightOutlined => "antd-icons/outlined/caret-right.svg",
            Self::CaretUpOutlined => "antd-icons/outlined/caret-up.svg",
            Self::CarryOutOutlined => "antd-icons/outlined/carry-out.svg",
            Self::CheckCircleOutlined => "antd-icons/outlined/check-circle.svg",
            Self::CheckSquareOutlined => "antd-icons/outlined/check-square.svg",
            Self::CheckOutlined => "antd-icons/outlined/check.svg",
            Self::ChromeOutlined => "antd-icons/outlined/chrome.svg",
            Self::CiCircleOutlined => "antd-icons/outlined/ci-circle.svg",
            Self::CiOutlined => "antd-icons/outlined/ci.svg",
            Self::ClearOutlined => "antd-icons/outlined/clear.svg",
            Self::ClockCircleOutlined => "antd-icons/outlined/clock-circle.svg",
            Self::CloseCircleOutlined => "antd-icons/outlined/close-circle.svg",
            Self::CloseSquareOutlined => "antd-icons/outlined/close-square.svg",
            Self::CloseOutlined => "antd-icons/outlined/close.svg",
            Self::CloudDownloadOutlined => "antd-icons/outlined/cloud-download.svg",
            Self::CloudServerOutlined => "antd-icons/outlined/cloud-server.svg",
            Self::CloudSyncOutlined => "antd-icons/outlined/cloud-sync.svg",
            Self::CloudUploadOutlined => "antd-icons/outlined/cloud-upload.svg",
            Self::CloudOutlined => "antd-icons/outlined/cloud.svg",
            Self::ClusterOutlined => "antd-icons/outlined/cluster.svg",
            Self::CodeSandboxOutlined => "antd-icons/outlined/code-sandbox.svg",
            Self::CodeOutlined => "antd-icons/outlined/code.svg",
            Self::CodepenCircleOutlined => "antd-icons/outlined/codepen-circle.svg",
            Self::CodepenOutlined => "antd-icons/outlined/codepen.svg",
            Self::CoffeeOutlined => "antd-icons/outlined/coffee.svg",
            Self::ColumnHeightOutlined => "antd-icons/outlined/column-height.svg",
            Self::ColumnWidthOutlined => "antd-icons/outlined/column-width.svg",
            Self::CommentOutlined => "antd-icons/outlined/comment.svg",
            Self::CompassOutlined => "antd-icons/outlined/compass.svg",
            Self::CompressOutlined => "antd-icons/outlined/compress.svg",
            Self::ConsoleSqlOutlined => "antd-icons/outlined/console-sql.svg",
            Self::ContactsOutlined => "antd-icons/outlined/contacts.svg",
            Self::ContainerOutlined => "antd-icons/outlined/container.svg",
            Self::ControlOutlined => "antd-icons/outlined/control.svg",
            Self::CopyOutlined => "antd-icons/outlined/copy.svg",
            Self::CopyrightCircleOutlined => "antd-icons/outlined/copyright-circle.svg",
            Self::CopyrightOutlined => "antd-icons/outlined/copyright.svg",
            Self::CreditCardOutlined => "antd-icons/outlined/credit-card.svg",
            Self::CrownOutlined => "antd-icons/outlined/crown.svg",
            Self::CustomerServiceOutlined => "antd-icons/outlined/customer-service.svg",
            Self::DashOutlined => "antd-icons/outlined/dash.svg",
            Self::DashboardOutlined => "antd-icons/outlined/dashboard.svg",
            Self::DatabaseOutlined => "antd-icons/outlined/database.svg",
            Self::DeleteColumnOutlined => "antd-icons/outlined/delete-column.svg",
            Self::DeleteRowOutlined => "antd-icons/outlined/delete-row.svg",
            Self::DeleteOutlined => "antd-icons/outlined/delete.svg",
            Self::DeliveredProcedureOutlined => "antd-icons/outlined/delivered-procedure.svg",
            Self::DeploymentUnitOutlined => "antd-icons/outlined/deployment-unit.svg",
            Self::DesktopOutlined => "antd-icons/outlined/desktop.svg",
            Self::DiffOutlined => "antd-icons/outlined/diff.svg",
            Self::DingdingOutlined => "antd-icons/outlined/dingding.svg",
            Self::DingtalkOutlined => "antd-icons/outlined/dingtalk.svg",
            Self::DisconnectOutlined => "antd-icons/outlined/disconnect.svg",
            Self::DiscordOutlined => "antd-icons/outlined/discord.svg",
            Self::DislikeOutlined => "antd-icons/outlined/dislike.svg",
            Self::DockerOutlined => "antd-icons/outlined/docker.svg",
            Self::DollarCircleOutlined => "antd-icons/outlined/dollar-circle.svg",
            Self::DollarOutlined => "antd-icons/outlined/dollar.svg",
            Self::DotChartOutlined => "antd-icons/outlined/dot-chart.svg",
            Self::DotNetOutlined => "antd-icons/outlined/dot-net.svg",
            Self::DoubleLeftOutlined => "antd-icons/outlined/double-left.svg",
            Self::DoubleRightOutlined => "antd-icons/outlined/double-right.svg",
            Self::DownCircleOutlined => "antd-icons/outlined/down-circle.svg",
            Self::DownSquareOutlined => "antd-icons/outlined/down-square.svg",
            Self::DownOutlined => "antd-icons/outlined/down.svg",
            Self::DownloadOutlined => "antd-icons/outlined/download.svg",
            Self::DragOutlined => "antd-icons/outlined/drag.svg",
            Self::DribbbleSquareOutlined => "antd-icons/outlined/dribbble-square.svg",
            Self::DribbbleOutlined => "antd-icons/outlined/dribbble.svg",
            Self::DropboxOutlined => "antd-icons/outlined/dropbox.svg",
            Self::EditOutlined => "antd-icons/outlined/edit.svg",
            Self::EllipsisOutlined => "antd-icons/outlined/ellipsis.svg",
            Self::EnterOutlined => "antd-icons/outlined/enter.svg",
            Self::EnvironmentOutlined => "antd-icons/outlined/environment.svg",
            Self::EuroCircleOutlined => "antd-icons/outlined/euro-circle.svg",
            Self::EuroOutlined => "antd-icons/outlined/euro.svg",
            Self::ExceptionOutlined => "antd-icons/outlined/exception.svg",
            Self::ExclamationCircleOutlined => "antd-icons/outlined/exclamation-circle.svg",
            Self::ExclamationOutlined => "antd-icons/outlined/exclamation.svg",
            Self::ExpandAltOutlined => "antd-icons/outlined/expand-alt.svg",
            Self::ExpandOutlined => "antd-icons/outlined/expand.svg",
            Self::ExperimentOutlined => "antd-icons/outlined/experiment.svg",
            Self::ExportOutlined => "antd-icons/outlined/export.svg",
            Self::EyeInvisibleOutlined => "antd-icons/outlined/eye-invisible.svg",
            Self::EyeOutlined => "antd-icons/outlined/eye.svg",
            Self::FacebookOutlined => "antd-icons/outlined/facebook.svg",
            Self::FallOutlined => "antd-icons/outlined/fall.svg",
            Self::FastBackwardOutlined => "antd-icons/outlined/fast-backward.svg",
            Self::FastForwardOutlined => "antd-icons/outlined/fast-forward.svg",
            Self::FieldBinaryOutlined => "antd-icons/outlined/field-binary.svg",
            Self::FieldNumberOutlined => "antd-icons/outlined/field-number.svg",
            Self::FieldStringOutlined => "antd-icons/outlined/field-string.svg",
            Self::FieldTimeOutlined => "antd-icons/outlined/field-time.svg",
            Self::FileAddOutlined => "antd-icons/outlined/file-add.svg",
            Self::FileDoneOutlined => "antd-icons/outlined/file-done.svg",
            Self::FileExcelOutlined => "antd-icons/outlined/file-excel.svg",
            Self::FileExclamationOutlined => "antd-icons/outlined/file-exclamation.svg",
            Self::FileGifOutlined => "antd-icons/outlined/file-gif.svg",
            Self::FileImageOutlined => "antd-icons/outlined/file-image.svg",
            Self::FileJpgOutlined => "antd-icons/outlined/file-jpg.svg",
            Self::FileMarkdownOutlined => "antd-icons/outlined/file-markdown.svg",
            Self::FilePdfOutlined => "antd-icons/outlined/file-pdf.svg",
            Self::FilePptOutlined => "antd-icons/outlined/file-ppt.svg",
            Self::FileProtectOutlined => "antd-icons/outlined/file-protect.svg",
            Self::FileSearchOutlined => "antd-icons/outlined/file-search.svg",
            Self::FileSyncOutlined => "antd-icons/outlined/file-sync.svg",
            Self::FileTextOutlined => "antd-icons/outlined/file-text.svg",
            Self::FileUnknownOutlined => "antd-icons/outlined/file-unknown.svg",
            Self::FileWordOutlined => "antd-icons/outlined/file-word.svg",
            Self::FileZipOutlined => "antd-icons/outlined/file-zip.svg",
            Self::FileOutlined => "antd-icons/outlined/file.svg",
            Self::FilterOutlined => "antd-icons/outlined/filter.svg",
            Self::FireOutlined => "antd-icons/outlined/fire.svg",
            Self::FlagOutlined => "antd-icons/outlined/flag.svg",
            Self::FolderAddOutlined => "antd-icons/outlined/folder-add.svg",
            Self::FolderOpenOutlined => "antd-icons/outlined/folder-open.svg",
            Self::FolderViewOutlined => "antd-icons/outlined/folder-view.svg",
            Self::FolderOutlined => "antd-icons/outlined/folder.svg",
            Self::FontColorsOutlined => "antd-icons/outlined/font-colors.svg",
            Self::FontSizeOutlined => "antd-icons/outlined/font-size.svg",
            Self::ForkOutlined => "antd-icons/outlined/fork.svg",
            Self::FormOutlined => "antd-icons/outlined/form.svg",
            Self::FormatPainterOutlined => "antd-icons/outlined/format-painter.svg",
            Self::ForwardOutlined => "antd-icons/outlined/forward.svg",
            Self::FrownOutlined => "antd-icons/outlined/frown.svg",
            Self::FullscreenExitOutlined => "antd-icons/outlined/fullscreen-exit.svg",
            Self::FullscreenOutlined => "antd-icons/outlined/fullscreen.svg",
            Self::FunctionOutlined => "antd-icons/outlined/function.svg",
            Self::FundProjectionScreenOutlined => "antd-icons/outlined/fund-projection-screen.svg",
            Self::FundViewOutlined => "antd-icons/outlined/fund-view.svg",
            Self::FundOutlined => "antd-icons/outlined/fund.svg",
            Self::FunnelPlotOutlined => "antd-icons/outlined/funnel-plot.svg",
            Self::GatewayOutlined => "antd-icons/outlined/gateway.svg",
            Self::GifOutlined => "antd-icons/outlined/gif.svg",
            Self::GiftOutlined => "antd-icons/outlined/gift.svg",
            Self::GithubOutlined => "antd-icons/outlined/github.svg",
            Self::GitlabOutlined => "antd-icons/outlined/gitlab.svg",
            Self::GlobalOutlined => "antd-icons/outlined/global.svg",
            Self::GoldOutlined => "antd-icons/outlined/gold.svg",
            Self::GooglePlusOutlined => "antd-icons/outlined/google-plus.svg",
            Self::GoogleOutlined => "antd-icons/outlined/google.svg",
            Self::GroupOutlined => "antd-icons/outlined/group.svg",
            Self::HarmonyOSOutlined => "antd-icons/outlined/harmony-o-s.svg",
            Self::HddOutlined => "antd-icons/outlined/hdd.svg",
            Self::HeartOutlined => "antd-icons/outlined/heart.svg",
            Self::HeatMapOutlined => "antd-icons/outlined/heat-map.svg",
            Self::HighlightOutlined => "antd-icons/outlined/highlight.svg",
            Self::HistoryOutlined => "antd-icons/outlined/history.svg",
            Self::HolderOutlined => "antd-icons/outlined/holder.svg",
            Self::HomeOutlined => "antd-icons/outlined/home.svg",
            Self::HourglassOutlined => "antd-icons/outlined/hourglass.svg",
            Self::Html5Outlined => "antd-icons/outlined/html5.svg",
            Self::IdcardOutlined => "antd-icons/outlined/idcard.svg",
            Self::IeOutlined => "antd-icons/outlined/ie.svg",
            Self::ImportOutlined => "antd-icons/outlined/import.svg",
            Self::InboxOutlined => "antd-icons/outlined/inbox.svg",
            Self::InfoCircleOutlined => "antd-icons/outlined/info-circle.svg",
            Self::InfoOutlined => "antd-icons/outlined/info.svg",
            Self::InsertRowAboveOutlined => "antd-icons/outlined/insert-row-above.svg",
            Self::InsertRowBelowOutlined => "antd-icons/outlined/insert-row-below.svg",
            Self::InsertRowLeftOutlined => "antd-icons/outlined/insert-row-left.svg",
            Self::InsertRowRightOutlined => "antd-icons/outlined/insert-row-right.svg",
            Self::InstagramOutlined => "antd-icons/outlined/instagram.svg",
            Self::InsuranceOutlined => "antd-icons/outlined/insurance.svg",
            Self::InteractionOutlined => "antd-icons/outlined/interaction.svg",
            Self::IssuesCloseOutlined => "antd-icons/outlined/issues-close.svg",
            Self::ItalicOutlined => "antd-icons/outlined/italic.svg",
            Self::JavaScriptOutlined => "antd-icons/outlined/java-script.svg",
            Self::JavaOutlined => "antd-icons/outlined/java.svg",
            Self::KeyOutlined => "antd-icons/outlined/key.svg",
            Self::KubernetesOutlined => "antd-icons/outlined/kubernetes.svg",
            Self::LaptopOutlined => "antd-icons/outlined/laptop.svg",
            Self::LayoutOutlined => "antd-icons/outlined/layout.svg",
            Self::LeftCircleOutlined => "antd-icons/outlined/left-circle.svg",
            Self::LeftSquareOutlined => "antd-icons/outlined/left-square.svg",
            Self::LeftOutlined => "antd-icons/outlined/left.svg",
            Self::LikeOutlined => "antd-icons/outlined/like.svg",
            Self::LineChartOutlined => "antd-icons/outlined/line-chart.svg",
            Self::LineHeightOutlined => "antd-icons/outlined/line-height.svg",
            Self::LineOutlined => "antd-icons/outlined/line.svg",
            Self::LinkOutlined => "antd-icons/outlined/link.svg",
            Self::LinkedinOutlined => "antd-icons/outlined/linkedin.svg",
            Self::LinuxOutlined => "antd-icons/outlined/linux.svg",
            Self::Loading3QuartersOutlined => "antd-icons/outlined/loading-3-quarters.svg",
            Self::LoadingOutlined => "antd-icons/outlined/loading.svg",
            Self::LockOutlined => "antd-icons/outlined/lock.svg",
            Self::LoginOutlined => "antd-icons/outlined/login.svg",
            Self::LogoutOutlined => "antd-icons/outlined/logout.svg",
            Self::MacCommandOutlined => "antd-icons/outlined/mac-command.svg",
            Self::MailOutlined => "antd-icons/outlined/mail.svg",
            Self::ManOutlined => "antd-icons/outlined/man.svg",
            Self::MedicineBoxOutlined => "antd-icons/outlined/medicine-box.svg",
            Self::MediumWorkmarkOutlined => "antd-icons/outlined/medium-workmark.svg",
            Self::MediumOutlined => "antd-icons/outlined/medium.svg",
            Self::MehOutlined => "antd-icons/outlined/meh.svg",
            Self::MenuFoldOutlined => "antd-icons/outlined/menu-fold.svg",
            Self::MenuUnfoldOutlined => "antd-icons/outlined/menu-unfold.svg",
            Self::MenuOutlined => "antd-icons/outlined/menu.svg",
            Self::MergeCellsOutlined => "antd-icons/outlined/merge-cells.svg",
            Self::MergeOutlined => "antd-icons/outlined/merge.svg",
            Self::MessageOutlined => "antd-icons/outlined/message.svg",
            Self::MinusCircleOutlined => "antd-icons/outlined/minus-circle.svg",
            Self::MinusSquareOutlined => "antd-icons/outlined/minus-square.svg",
            Self::MinusOutlined => "antd-icons/outlined/minus.svg",
            Self::MobileOutlined => "antd-icons/outlined/mobile.svg",
            Self::MoneyCollectOutlined => "antd-icons/outlined/money-collect.svg",
            Self::MonitorOutlined => "antd-icons/outlined/monitor.svg",
            Self::MoonOutlined => "antd-icons/outlined/moon.svg",
            Self::MoreOutlined => "antd-icons/outlined/more.svg",
            Self::MutedOutlined => "antd-icons/outlined/muted.svg",
            Self::NodeCollapseOutlined => "antd-icons/outlined/node-collapse.svg",
            Self::NodeExpandOutlined => "antd-icons/outlined/node-expand.svg",
            Self::NodeIndexOutlined => "antd-icons/outlined/node-index.svg",
            Self::NotificationOutlined => "antd-icons/outlined/notification.svg",
            Self::NumberOutlined => "antd-icons/outlined/number.svg",
            Self::OneToOneOutlined => "antd-icons/outlined/one-to-one.svg",
            Self::OpenAIOutlined => "antd-icons/outlined/open-a-i.svg",
            Self::OrderedListOutlined => "antd-icons/outlined/ordered-list.svg",
            Self::PaperClipOutlined => "antd-icons/outlined/paper-clip.svg",
            Self::PartitionOutlined => "antd-icons/outlined/partition.svg",
            Self::PauseCircleOutlined => "antd-icons/outlined/pause-circle.svg",
            Self::PauseOutlined => "antd-icons/outlined/pause.svg",
            Self::PayCircleOutlined => "antd-icons/outlined/pay-circle.svg",
            Self::PercentageOutlined => "antd-icons/outlined/percentage.svg",
            Self::PhoneOutlined => "antd-icons/outlined/phone.svg",
            Self::PicCenterOutlined => "antd-icons/outlined/pic-center.svg",
            Self::PicLeftOutlined => "antd-icons/outlined/pic-left.svg",
            Self::PicRightOutlined => "antd-icons/outlined/pic-right.svg",
            Self::PictureOutlined => "antd-icons/outlined/picture.svg",
            Self::PieChartOutlined => "antd-icons/outlined/pie-chart.svg",
            Self::PinterestOutlined => "antd-icons/outlined/pinterest.svg",
            Self::PlayCircleOutlined => "antd-icons/outlined/play-circle.svg",
            Self::PlaySquareOutlined => "antd-icons/outlined/play-square.svg",
            Self::PlusCircleOutlined => "antd-icons/outlined/plus-circle.svg",
            Self::PlusSquareOutlined => "antd-icons/outlined/plus-square.svg",
            Self::PlusOutlined => "antd-icons/outlined/plus.svg",
            Self::PoundCircleOutlined => "antd-icons/outlined/pound-circle.svg",
            Self::PoundOutlined => "antd-icons/outlined/pound.svg",
            Self::PoweroffOutlined => "antd-icons/outlined/poweroff.svg",
            Self::PrinterOutlined => "antd-icons/outlined/printer.svg",
            Self::ProductOutlined => "antd-icons/outlined/product.svg",
            Self::ProfileOutlined => "antd-icons/outlined/profile.svg",
            Self::ProjectOutlined => "antd-icons/outlined/project.svg",
            Self::PropertySafetyOutlined => "antd-icons/outlined/property-safety.svg",
            Self::PullRequestOutlined => "antd-icons/outlined/pull-request.svg",
            Self::PushpinOutlined => "antd-icons/outlined/pushpin.svg",
            Self::PythonOutlined => "antd-icons/outlined/python.svg",
            Self::QqOutlined => "antd-icons/outlined/qq.svg",
            Self::QrcodeOutlined => "antd-icons/outlined/qrcode.svg",
            Self::QuestionCircleOutlined => "antd-icons/outlined/question-circle.svg",
            Self::QuestionOutlined => "antd-icons/outlined/question.svg",
            Self::RadarChartOutlined => "antd-icons/outlined/radar-chart.svg",
            Self::RadiusBottomleftOutlined => "antd-icons/outlined/radius-bottomleft.svg",
            Self::RadiusBottomrightOutlined => "antd-icons/outlined/radius-bottomright.svg",
            Self::RadiusSettingOutlined => "antd-icons/outlined/radius-setting.svg",
            Self::RadiusUpleftOutlined => "antd-icons/outlined/radius-upleft.svg",
            Self::RadiusUprightOutlined => "antd-icons/outlined/radius-upright.svg",
            Self::ReadOutlined => "antd-icons/outlined/read.svg",
            Self::ReconciliationOutlined => "antd-icons/outlined/reconciliation.svg",
            Self::RedEnvelopeOutlined => "antd-icons/outlined/red-envelope.svg",
            Self::RedditOutlined => "antd-icons/outlined/reddit.svg",
            Self::RedoOutlined => "antd-icons/outlined/redo.svg",
            Self::ReloadOutlined => "antd-icons/outlined/reload.svg",
            Self::RestOutlined => "antd-icons/outlined/rest.svg",
            Self::RetweetOutlined => "antd-icons/outlined/retweet.svg",
            Self::RightCircleOutlined => "antd-icons/outlined/right-circle.svg",
            Self::RightSquareOutlined => "antd-icons/outlined/right-square.svg",
            Self::RightOutlined => "antd-icons/outlined/right.svg",
            Self::RiseOutlined => "antd-icons/outlined/rise.svg",
            Self::RobotOutlined => "antd-icons/outlined/robot.svg",
            Self::RocketOutlined => "antd-icons/outlined/rocket.svg",
            Self::RollbackOutlined => "antd-icons/outlined/rollback.svg",
            Self::RotateLeftOutlined => "antd-icons/outlined/rotate-left.svg",
            Self::RotateRightOutlined => "antd-icons/outlined/rotate-right.svg",
            Self::RubyOutlined => "antd-icons/outlined/ruby.svg",
            Self::SafetyCertificateOutlined => "antd-icons/outlined/safety-certificate.svg",
            Self::SafetyOutlined => "antd-icons/outlined/safety.svg",
            Self::SaveOutlined => "antd-icons/outlined/save.svg",
            Self::ScanOutlined => "antd-icons/outlined/scan.svg",
            Self::ScheduleOutlined => "antd-icons/outlined/schedule.svg",
            Self::ScissorOutlined => "antd-icons/outlined/scissor.svg",
            Self::SearchOutlined => "antd-icons/outlined/search.svg",
            Self::SecurityScanOutlined => "antd-icons/outlined/security-scan.svg",
            Self::SelectOutlined => "antd-icons/outlined/select.svg",
            Self::SendOutlined => "antd-icons/outlined/send.svg",
            Self::SettingOutlined => "antd-icons/outlined/setting.svg",
            Self::ShakeOutlined => "antd-icons/outlined/shake.svg",
            Self::ShareAltOutlined => "antd-icons/outlined/share-alt.svg",
            Self::ShopOutlined => "antd-icons/outlined/shop.svg",
            Self::ShoppingCartOutlined => "antd-icons/outlined/shopping-cart.svg",
            Self::ShoppingOutlined => "antd-icons/outlined/shopping.svg",
            Self::ShrinkOutlined => "antd-icons/outlined/shrink.svg",
            Self::SignatureOutlined => "antd-icons/outlined/signature.svg",
            Self::SisternodeOutlined => "antd-icons/outlined/sisternode.svg",
            Self::SketchOutlined => "antd-icons/outlined/sketch.svg",
            Self::SkinOutlined => "antd-icons/outlined/skin.svg",
            Self::SkypeOutlined => "antd-icons/outlined/skype.svg",
            Self::SlackSquareOutlined => "antd-icons/outlined/slack-square.svg",
            Self::SlackOutlined => "antd-icons/outlined/slack.svg",
            Self::SlidersOutlined => "antd-icons/outlined/sliders.svg",
            Self::SmallDashOutlined => "antd-icons/outlined/small-dash.svg",
            Self::SmileOutlined => "antd-icons/outlined/smile.svg",
            Self::SnippetsOutlined => "antd-icons/outlined/snippets.svg",
            Self::SolutionOutlined => "antd-icons/outlined/solution.svg",
            Self::SortAscendingOutlined => "antd-icons/outlined/sort-ascending.svg",
            Self::SortDescendingOutlined => "antd-icons/outlined/sort-descending.svg",
            Self::SoundOutlined => "antd-icons/outlined/sound.svg",
            Self::SplitCellsOutlined => "antd-icons/outlined/split-cells.svg",
            Self::SpotifyOutlined => "antd-icons/outlined/spotify.svg",
            Self::StarOutlined => "antd-icons/outlined/star.svg",
            Self::StepBackwardOutlined => "antd-icons/outlined/step-backward.svg",
            Self::StepForwardOutlined => "antd-icons/outlined/step-forward.svg",
            Self::StockOutlined => "antd-icons/outlined/stock.svg",
            Self::StopOutlined => "antd-icons/outlined/stop.svg",
            Self::StrikethroughOutlined => "antd-icons/outlined/strikethrough.svg",
            Self::SubnodeOutlined => "antd-icons/outlined/subnode.svg",
            Self::SunOutlined => "antd-icons/outlined/sun.svg",
            Self::SwapLeftOutlined => "antd-icons/outlined/swap-left.svg",
            Self::SwapRightOutlined => "antd-icons/outlined/swap-right.svg",
            Self::SwapOutlined => "antd-icons/outlined/swap.svg",
            Self::SwitcherOutlined => "antd-icons/outlined/switcher.svg",
            Self::SyncOutlined => "antd-icons/outlined/sync.svg",
            Self::TableOutlined => "antd-icons/outlined/table.svg",
            Self::TabletOutlined => "antd-icons/outlined/tablet.svg",
            Self::TagOutlined => "antd-icons/outlined/tag.svg",
            Self::TagsOutlined => "antd-icons/outlined/tags.svg",
            Self::TaobaoCircleOutlined => "antd-icons/outlined/taobao-circle.svg",
            Self::TaobaoOutlined => "antd-icons/outlined/taobao.svg",
            Self::TeamOutlined => "antd-icons/outlined/team.svg",
            Self::ThunderboltOutlined => "antd-icons/outlined/thunderbolt.svg",
            Self::TikTokOutlined => "antd-icons/outlined/tik-tok.svg",
            Self::ToTopOutlined => "antd-icons/outlined/to-top.svg",
            Self::ToolOutlined => "antd-icons/outlined/tool.svg",
            Self::TrademarkCircleOutlined => "antd-icons/outlined/trademark-circle.svg",
            Self::TrademarkOutlined => "antd-icons/outlined/trademark.svg",
            Self::TransactionOutlined => "antd-icons/outlined/transaction.svg",
            Self::TranslationOutlined => "antd-icons/outlined/translation.svg",
            Self::TrophyOutlined => "antd-icons/outlined/trophy.svg",
            Self::TruckOutlined => "antd-icons/outlined/truck.svg",
            Self::TwitchOutlined => "antd-icons/outlined/twitch.svg",
            Self::TwitterOutlined => "antd-icons/outlined/twitter.svg",
            Self::UnderlineOutlined => "antd-icons/outlined/underline.svg",
            Self::UndoOutlined => "antd-icons/outlined/undo.svg",
            Self::UngroupOutlined => "antd-icons/outlined/ungroup.svg",
            Self::UnlockOutlined => "antd-icons/outlined/unlock.svg",
            Self::UnorderedListOutlined => "antd-icons/outlined/unordered-list.svg",
            Self::UpCircleOutlined => "antd-icons/outlined/up-circle.svg",
            Self::UpSquareOutlined => "antd-icons/outlined/up-square.svg",
            Self::UpOutlined => "antd-icons/outlined/up.svg",
            Self::UploadOutlined => "antd-icons/outlined/upload.svg",
            Self::UsbOutlined => "antd-icons/outlined/usb.svg",
            Self::UserAddOutlined => "antd-icons/outlined/user-add.svg",
            Self::UserDeleteOutlined => "antd-icons/outlined/user-delete.svg",
            Self::UserSwitchOutlined => "antd-icons/outlined/user-switch.svg",
            Self::UserOutlined => "antd-icons/outlined/user.svg",
            Self::UsergroupAddOutlined => "antd-icons/outlined/usergroup-add.svg",
            Self::UsergroupDeleteOutlined => "antd-icons/outlined/usergroup-delete.svg",
            Self::VerifiedOutlined => "antd-icons/outlined/verified.svg",
            Self::VerticalAlignBottomOutlined => "antd-icons/outlined/vertical-align-bottom.svg",
            Self::VerticalAlignMiddleOutlined => "antd-icons/outlined/vertical-align-middle.svg",
            Self::VerticalAlignTopOutlined => "antd-icons/outlined/vertical-align-top.svg",
            Self::VerticalLeftOutlined => "antd-icons/outlined/vertical-left.svg",
            Self::VerticalRightOutlined => "antd-icons/outlined/vertical-right.svg",
            Self::VideoCameraAddOutlined => "antd-icons/outlined/video-camera-add.svg",
            Self::VideoCameraOutlined => "antd-icons/outlined/video-camera.svg",
            Self::WalletOutlined => "antd-icons/outlined/wallet.svg",
            Self::WarningOutlined => "antd-icons/outlined/warning.svg",
            Self::WechatWorkOutlined => "antd-icons/outlined/wechat-work.svg",
            Self::WechatOutlined => "antd-icons/outlined/wechat.svg",
            Self::WeiboCircleOutlined => "antd-icons/outlined/weibo-circle.svg",
            Self::WeiboSquareOutlined => "antd-icons/outlined/weibo-square.svg",
            Self::WeiboOutlined => "antd-icons/outlined/weibo.svg",
            Self::WhatsAppOutlined => "antd-icons/outlined/whats-app.svg",
            Self::WifiOutlined => "antd-icons/outlined/wifi.svg",
            Self::WindowsOutlined => "antd-icons/outlined/windows.svg",
            Self::WomanOutlined => "antd-icons/outlined/woman.svg",
            Self::XOutlined => "antd-icons/outlined/x.svg",
            Self::YahooOutlined => "antd-icons/outlined/yahoo.svg",
            Self::YoutubeOutlined => "antd-icons/outlined/youtube.svg",
            Self::YuqueOutlined => "antd-icons/outlined/yuque.svg",
            Self::ZhihuOutlined => "antd-icons/outlined/zhihu.svg",
            Self::ZoomInOutlined => "antd-icons/outlined/zoom-in.svg",
            Self::ZoomOutOutlined => "antd-icons/outlined/zoom-out.svg",
            Self::AccountBookFilled => "antd-icons/filled/account-book.svg",
            Self::AlertFilled => "antd-icons/filled/alert.svg",
            Self::AlipayCircleFilled => "antd-icons/filled/alipay-circle.svg",
            Self::AlipaySquareFilled => "antd-icons/filled/alipay-square.svg",
            Self::AliwangwangFilled => "antd-icons/filled/aliwangwang.svg",
            Self::AmazonCircleFilled => "antd-icons/filled/amazon-circle.svg",
            Self::AmazonSquareFilled => "antd-icons/filled/amazon-square.svg",
            Self::AndroidFilled => "antd-icons/filled/android.svg",
            Self::AnthropicFilled => "antd-icons/filled/anthropic.svg",
            Self::ApiFilled => "antd-icons/filled/api.svg",
            Self::AppleFilled => "antd-icons/filled/apple.svg",
            Self::AppstoreFilled => "antd-icons/filled/appstore.svg",
            Self::AudioFilled => "antd-icons/filled/audio.svg",
            Self::BackwardFilled => "antd-icons/filled/backward.svg",
            Self::BankFilled => "antd-icons/filled/bank.svg",
            Self::BehanceCircleFilled => "antd-icons/filled/behance-circle.svg",
            Self::BehanceSquareFilled => "antd-icons/filled/behance-square.svg",
            Self::BellFilled => "antd-icons/filled/bell.svg",
            Self::BilibiliFilled => "antd-icons/filled/bilibili.svg",
            Self::BookFilled => "antd-icons/filled/book.svg",
            Self::BoxPlotFilled => "antd-icons/filled/box-plot.svg",
            Self::BugFilled => "antd-icons/filled/bug.svg",
            Self::BuildFilled => "antd-icons/filled/build.svg",
            Self::BulbFilled => "antd-icons/filled/bulb.svg",
            Self::CalculatorFilled => "antd-icons/filled/calculator.svg",
            Self::CalendarFilled => "antd-icons/filled/calendar.svg",
            Self::CameraFilled => "antd-icons/filled/camera.svg",
            Self::CarFilled => "antd-icons/filled/car.svg",
            Self::CaretDownFilled => "antd-icons/filled/caret-down.svg",
            Self::CaretLeftFilled => "antd-icons/filled/caret-left.svg",
            Self::CaretRightFilled => "antd-icons/filled/caret-right.svg",
            Self::CaretUpFilled => "antd-icons/filled/caret-up.svg",
            Self::CarryOutFilled => "antd-icons/filled/carry-out.svg",
            Self::CheckCircleFilled => "antd-icons/filled/check-circle.svg",
            Self::CheckSquareFilled => "antd-icons/filled/check-square.svg",
            Self::ChromeFilled => "antd-icons/filled/chrome.svg",
            Self::CiCircleFilled => "antd-icons/filled/ci-circle.svg",
            Self::ClaudeFilled => "antd-icons/filled/claude.svg",
            Self::ClockCircleFilled => "antd-icons/filled/clock-circle.svg",
            Self::CloseCircleFilled => "antd-icons/filled/close-circle.svg",
            Self::CloseSquareFilled => "antd-icons/filled/close-square.svg",
            Self::CloudFilled => "antd-icons/filled/cloud.svg",
            Self::CodeSandboxCircleFilled => "antd-icons/filled/code-sandbox-circle.svg",
            Self::CodeSandboxSquareFilled => "antd-icons/filled/code-sandbox-square.svg",
            Self::CodeFilled => "antd-icons/filled/code.svg",
            Self::CodepenCircleFilled => "antd-icons/filled/codepen-circle.svg",
            Self::CodepenSquareFilled => "antd-icons/filled/codepen-square.svg",
            Self::CompassFilled => "antd-icons/filled/compass.svg",
            Self::ContactsFilled => "antd-icons/filled/contacts.svg",
            Self::ContainerFilled => "antd-icons/filled/container.svg",
            Self::ControlFilled => "antd-icons/filled/control.svg",
            Self::CopyFilled => "antd-icons/filled/copy.svg",
            Self::CopyrightCircleFilled => "antd-icons/filled/copyright-circle.svg",
            Self::CreditCardFilled => "antd-icons/filled/credit-card.svg",
            Self::CrownFilled => "antd-icons/filled/crown.svg",
            Self::CustomerServiceFilled => "antd-icons/filled/customer-service.svg",
            Self::DashboardFilled => "antd-icons/filled/dashboard.svg",
            Self::DatabaseFilled => "antd-icons/filled/database.svg",
            Self::DeepSeekFilled => "antd-icons/filled/deep-seek.svg",
            Self::DeleteFilled => "antd-icons/filled/delete.svg",
            Self::DiffFilled => "antd-icons/filled/diff.svg",
            Self::DingtalkCircleFilled => "antd-icons/filled/dingtalk-circle.svg",
            Self::DingtalkSquareFilled => "antd-icons/filled/dingtalk-square.svg",
            Self::DiscordFilled => "antd-icons/filled/discord.svg",
            Self::DislikeFilled => "antd-icons/filled/dislike.svg",
            Self::DollarCircleFilled => "antd-icons/filled/dollar-circle.svg",
            Self::DownCircleFilled => "antd-icons/filled/down-circle.svg",
            Self::DownSquareFilled => "antd-icons/filled/down-square.svg",
            Self::DribbbleCircleFilled => "antd-icons/filled/dribbble-circle.svg",
            Self::DribbbleSquareFilled => "antd-icons/filled/dribbble-square.svg",
            Self::DropboxCircleFilled => "antd-icons/filled/dropbox-circle.svg",
            Self::DropboxSquareFilled => "antd-icons/filled/dropbox-square.svg",
            Self::EditFilled => "antd-icons/filled/edit.svg",
            Self::ElevenLabsFilled => "antd-icons/filled/eleven-labs.svg",
            Self::EnvironmentFilled => "antd-icons/filled/environment.svg",
            Self::EuroCircleFilled => "antd-icons/filled/euro-circle.svg",
            Self::ExclamationCircleFilled => "antd-icons/filled/exclamation-circle.svg",
            Self::ExperimentFilled => "antd-icons/filled/experiment.svg",
            Self::EyeInvisibleFilled => "antd-icons/filled/eye-invisible.svg",
            Self::EyeFilled => "antd-icons/filled/eye.svg",
            Self::FacebookFilled => "antd-icons/filled/facebook.svg",
            Self::FastBackwardFilled => "antd-icons/filled/fast-backward.svg",
            Self::FastForwardFilled => "antd-icons/filled/fast-forward.svg",
            Self::FileAddFilled => "antd-icons/filled/file-add.svg",
            Self::FileExcelFilled => "antd-icons/filled/file-excel.svg",
            Self::FileExclamationFilled => "antd-icons/filled/file-exclamation.svg",
            Self::FileImageFilled => "antd-icons/filled/file-image.svg",
            Self::FileMarkdownFilled => "antd-icons/filled/file-markdown.svg",
            Self::FilePdfFilled => "antd-icons/filled/file-pdf.svg",
            Self::FilePptFilled => "antd-icons/filled/file-ppt.svg",
            Self::FileTextFilled => "antd-icons/filled/file-text.svg",
            Self::FileUnknownFilled => "antd-icons/filled/file-unknown.svg",
            Self::FileWordFilled => "antd-icons/filled/file-word.svg",
            Self::FileZipFilled => "antd-icons/filled/file-zip.svg",
            Self::FileFilled => "antd-icons/filled/file.svg",
            Self::FilterFilled => "antd-icons/filled/filter.svg",
            Self::FireFilled => "antd-icons/filled/fire.svg",
            Self::FlagFilled => "antd-icons/filled/flag.svg",
            Self::FolderAddFilled => "antd-icons/filled/folder-add.svg",
            Self::FolderOpenFilled => "antd-icons/filled/folder-open.svg",
            Self::FolderFilled => "antd-icons/filled/folder.svg",
            Self::FormatPainterFilled => "antd-icons/filled/format-painter.svg",
            Self::ForwardFilled => "antd-icons/filled/forward.svg",
            Self::FrownFilled => "antd-icons/filled/frown.svg",
            Self::FundFilled => "antd-icons/filled/fund.svg",
            Self::FunnelPlotFilled => "antd-icons/filled/funnel-plot.svg",
            Self::GeminiFilled => "antd-icons/filled/gemini.svg",
            Self::GiftFilled => "antd-icons/filled/gift.svg",
            Self::GithubFilled => "antd-icons/filled/github.svg",
            Self::GitlabFilled => "antd-icons/filled/gitlab.svg",
            Self::GoldFilled => "antd-icons/filled/gold.svg",
            Self::GoldenFilled => "antd-icons/filled/golden.svg",
            Self::GoogleCircleFilled => "antd-icons/filled/google-circle.svg",
            Self::GooglePlusCircleFilled => "antd-icons/filled/google-plus-circle.svg",
            Self::GooglePlusSquareFilled => "antd-icons/filled/google-plus-square.svg",
            Self::GoogleSquareFilled => "antd-icons/filled/google-square.svg",
            Self::HddFilled => "antd-icons/filled/hdd.svg",
            Self::HeartFilled => "antd-icons/filled/heart.svg",
            Self::HighlightFilled => "antd-icons/filled/highlight.svg",
            Self::HomeFilled => "antd-icons/filled/home.svg",
            Self::HourglassFilled => "antd-icons/filled/hourglass.svg",
            Self::Html5Filled => "antd-icons/filled/html5.svg",
            Self::HuggingFaceFilled => "antd-icons/filled/hugging-face.svg",
            Self::IdcardFilled => "antd-icons/filled/idcard.svg",
            Self::IeCircleFilled => "antd-icons/filled/ie-circle.svg",
            Self::IeSquareFilled => "antd-icons/filled/ie-square.svg",
            Self::InfoCircleFilled => "antd-icons/filled/info-circle.svg",
            Self::InstagramFilled => "antd-icons/filled/instagram.svg",
            Self::InsuranceFilled => "antd-icons/filled/insurance.svg",
            Self::InteractionFilled => "antd-icons/filled/interaction.svg",
            Self::LayoutFilled => "antd-icons/filled/layout.svg",
            Self::LeftCircleFilled => "antd-icons/filled/left-circle.svg",
            Self::LeftSquareFilled => "antd-icons/filled/left-square.svg",
            Self::LikeFilled => "antd-icons/filled/like.svg",
            Self::LinkedinFilled => "antd-icons/filled/linkedin.svg",
            Self::LockFilled => "antd-icons/filled/lock.svg",
            Self::MacCommandFilled => "antd-icons/filled/mac-command.svg",
            Self::MailFilled => "antd-icons/filled/mail.svg",
            Self::MastodonFilled => "antd-icons/filled/mastodon.svg",
            Self::MedicineBoxFilled => "antd-icons/filled/medicine-box.svg",
            Self::MediumCircleFilled => "antd-icons/filled/medium-circle.svg",
            Self::MediumSquareFilled => "antd-icons/filled/medium-square.svg",
            Self::MehFilled => "antd-icons/filled/meh.svg",
            Self::MergeFilled => "antd-icons/filled/merge.svg",
            Self::MessageFilled => "antd-icons/filled/message.svg",
            Self::MetaFilled => "antd-icons/filled/meta.svg",
            Self::MinusCircleFilled => "antd-icons/filled/minus-circle.svg",
            Self::MinusSquareFilled => "antd-icons/filled/minus-square.svg",
            Self::MistralFilled => "antd-icons/filled/mistral.svg",
            Self::MobileFilled => "antd-icons/filled/mobile.svg",
            Self::MoneyCollectFilled => "antd-icons/filled/money-collect.svg",
            Self::MoonFilled => "antd-icons/filled/moon.svg",
            Self::MutedFilled => "antd-icons/filled/muted.svg",
            Self::NetflixFilled => "antd-icons/filled/netflix.svg",
            Self::NotificationFilled => "antd-icons/filled/notification.svg",
            Self::OllamaFilled => "antd-icons/filled/ollama.svg",
            Self::OpenAIFilled => "antd-icons/filled/open-a-i.svg",
            Self::PauseCircleFilled => "antd-icons/filled/pause-circle.svg",
            Self::PayCircleFilled => "antd-icons/filled/pay-circle.svg",
            Self::PerplexityFilled => "antd-icons/filled/perplexity.svg",
            Self::PhoneFilled => "antd-icons/filled/phone.svg",
            Self::PictureFilled => "antd-icons/filled/picture.svg",
            Self::PieChartFilled => "antd-icons/filled/pie-chart.svg",
            Self::PinterestFilled => "antd-icons/filled/pinterest.svg",
            Self::PlayCircleFilled => "antd-icons/filled/play-circle.svg",
            Self::PlaySquareFilled => "antd-icons/filled/play-square.svg",
            Self::PlusCircleFilled => "antd-icons/filled/plus-circle.svg",
            Self::PlusSquareFilled => "antd-icons/filled/plus-square.svg",
            Self::PoundCircleFilled => "antd-icons/filled/pound-circle.svg",
            Self::PrinterFilled => "antd-icons/filled/printer.svg",
            Self::ProductFilled => "antd-icons/filled/product.svg",
            Self::ProfileFilled => "antd-icons/filled/profile.svg",
            Self::ProjectFilled => "antd-icons/filled/project.svg",
            Self::PropertySafetyFilled => "antd-icons/filled/property-safety.svg",
            Self::PushpinFilled => "antd-icons/filled/pushpin.svg",
            Self::QqCircleFilled => "antd-icons/filled/qq-circle.svg",
            Self::QqSquareFilled => "antd-icons/filled/qq-square.svg",
            Self::QuestionCircleFilled => "antd-icons/filled/question-circle.svg",
            Self::QwenFilled => "antd-icons/filled/qwen.svg",
            Self::ReadFilled => "antd-icons/filled/read.svg",
            Self::ReconciliationFilled => "antd-icons/filled/reconciliation.svg",
            Self::RedEnvelopeFilled => "antd-icons/filled/red-envelope.svg",
            Self::RedditCircleFilled => "antd-icons/filled/reddit-circle.svg",
            Self::RedditSquareFilled => "antd-icons/filled/reddit-square.svg",
            Self::ReplicateFilled => "antd-icons/filled/replicate.svg",
            Self::RestFilled => "antd-icons/filled/rest.svg",
            Self::RightCircleFilled => "antd-icons/filled/right-circle.svg",
            Self::RightSquareFilled => "antd-icons/filled/right-square.svg",
            Self::RobotFilled => "antd-icons/filled/robot.svg",
            Self::RocketFilled => "antd-icons/filled/rocket.svg",
            Self::SafetyCertificateFilled => "antd-icons/filled/safety-certificate.svg",
            Self::SaveFilled => "antd-icons/filled/save.svg",
            Self::ScheduleFilled => "antd-icons/filled/schedule.svg",
            Self::SecurityScanFilled => "antd-icons/filled/security-scan.svg",
            Self::SettingFilled => "antd-icons/filled/setting.svg",
            Self::ShopFilled => "antd-icons/filled/shop.svg",
            Self::ShoppingFilled => "antd-icons/filled/shopping.svg",
            Self::SignalFilled => "antd-icons/filled/signal.svg",
            Self::SignatureFilled => "antd-icons/filled/signature.svg",
            Self::SketchCircleFilled => "antd-icons/filled/sketch-circle.svg",
            Self::SketchSquareFilled => "antd-icons/filled/sketch-square.svg",
            Self::SkinFilled => "antd-icons/filled/skin.svg",
            Self::SkypeFilled => "antd-icons/filled/skype.svg",
            Self::SlackCircleFilled => "antd-icons/filled/slack-circle.svg",
            Self::SlackSquareFilled => "antd-icons/filled/slack-square.svg",
            Self::SlidersFilled => "antd-icons/filled/sliders.svg",
            Self::SmileFilled => "antd-icons/filled/smile.svg",
            Self::SnapchatFilled => "antd-icons/filled/snapchat.svg",
            Self::SnippetsFilled => "antd-icons/filled/snippets.svg",
            Self::SoundFilled => "antd-icons/filled/sound.svg",
            Self::SpotifyFilled => "antd-icons/filled/spotify.svg",
            Self::StarFilled => "antd-icons/filled/star.svg",
            Self::StepBackwardFilled => "antd-icons/filled/step-backward.svg",
            Self::StepForwardFilled => "antd-icons/filled/step-forward.svg",
            Self::StopFilled => "antd-icons/filled/stop.svg",
            Self::SunFilled => "antd-icons/filled/sun.svg",
            Self::SwitcherFilled => "antd-icons/filled/switcher.svg",
            Self::TabletFilled => "antd-icons/filled/tablet.svg",
            Self::TagFilled => "antd-icons/filled/tag.svg",
            Self::TagsFilled => "antd-icons/filled/tags.svg",
            Self::TaobaoCircleFilled => "antd-icons/filled/taobao-circle.svg",
            Self::TaobaoSquareFilled => "antd-icons/filled/taobao-square.svg",
            Self::TelegramFilled => "antd-icons/filled/telegram.svg",
            Self::ThreadsFilled => "antd-icons/filled/threads.svg",
            Self::ThunderboltFilled => "antd-icons/filled/thunderbolt.svg",
            Self::TikTokFilled => "antd-icons/filled/tik-tok.svg",
            Self::ToolFilled => "antd-icons/filled/tool.svg",
            Self::TrademarkCircleFilled => "antd-icons/filled/trademark-circle.svg",
            Self::TrophyFilled => "antd-icons/filled/trophy.svg",
            Self::TruckFilled => "antd-icons/filled/truck.svg",
            Self::TwitchFilled => "antd-icons/filled/twitch.svg",
            Self::TwitterCircleFilled => "antd-icons/filled/twitter-circle.svg",
            Self::TwitterSquareFilled => "antd-icons/filled/twitter-square.svg",
            Self::UnlockFilled => "antd-icons/filled/unlock.svg",
            Self::UpCircleFilled => "antd-icons/filled/up-circle.svg",
            Self::UpSquareFilled => "antd-icons/filled/up-square.svg",
            Self::UsbFilled => "antd-icons/filled/usb.svg",
            Self::VideoCameraFilled => "antd-icons/filled/video-camera.svg",
            Self::WalletFilled => "antd-icons/filled/wallet.svg",
            Self::WarningFilled => "antd-icons/filled/warning.svg",
            Self::WechatWorkFilled => "antd-icons/filled/wechat-work.svg",
            Self::WechatFilled => "antd-icons/filled/wechat.svg",
            Self::WeiboCircleFilled => "antd-icons/filled/weibo-circle.svg",
            Self::WeiboSquareFilled => "antd-icons/filled/weibo-square.svg",
            Self::WindowsFilled => "antd-icons/filled/windows.svg",
            Self::XFilled => "antd-icons/filled/x.svg",
            Self::YahooFilled => "antd-icons/filled/yahoo.svg",
            Self::YoutubeFilled => "antd-icons/filled/youtube.svg",
            Self::YuqueFilled => "antd-icons/filled/yuque.svg",
            Self::ZhihuCircleFilled => "antd-icons/filled/zhihu-circle.svg",
            Self::ZhihuSquareFilled => "antd-icons/filled/zhihu-square.svg",
            Self::AccountBookTwoTone => "antd-icons/twotone/account-book.svg",
            Self::AlertTwoTone => "antd-icons/twotone/alert.svg",
            Self::ApiTwoTone => "antd-icons/twotone/api.svg",
            Self::AppstoreTwoTone => "antd-icons/twotone/appstore.svg",
            Self::AudioTwoTone => "antd-icons/twotone/audio.svg",
            Self::BankTwoTone => "antd-icons/twotone/bank.svg",
            Self::BellTwoTone => "antd-icons/twotone/bell.svg",
            Self::BookTwoTone => "antd-icons/twotone/book.svg",
            Self::BoxPlotTwoTone => "antd-icons/twotone/box-plot.svg",
            Self::BugTwoTone => "antd-icons/twotone/bug.svg",
            Self::BuildTwoTone => "antd-icons/twotone/build.svg",
            Self::BulbTwoTone => "antd-icons/twotone/bulb.svg",
            Self::CalculatorTwoTone => "antd-icons/twotone/calculator.svg",
            Self::CalendarTwoTone => "antd-icons/twotone/calendar.svg",
            Self::CameraTwoTone => "antd-icons/twotone/camera.svg",
            Self::CarTwoTone => "antd-icons/twotone/car.svg",
            Self::CarryOutTwoTone => "antd-icons/twotone/carry-out.svg",
            Self::CheckCircleTwoTone => "antd-icons/twotone/check-circle.svg",
            Self::CheckSquareTwoTone => "antd-icons/twotone/check-square.svg",
            Self::CiCircleTwoTone => "antd-icons/twotone/ci-circle.svg",
            Self::CiTwoTone => "antd-icons/twotone/ci.svg",
            Self::ClockCircleTwoTone => "antd-icons/twotone/clock-circle.svg",
            Self::CloseCircleTwoTone => "antd-icons/twotone/close-circle.svg",
            Self::CloseSquareTwoTone => "antd-icons/twotone/close-square.svg",
            Self::CloudTwoTone => "antd-icons/twotone/cloud.svg",
            Self::CodeTwoTone => "antd-icons/twotone/code.svg",
            Self::CompassTwoTone => "antd-icons/twotone/compass.svg",
            Self::ContactsTwoTone => "antd-icons/twotone/contacts.svg",
            Self::ContainerTwoTone => "antd-icons/twotone/container.svg",
            Self::ControlTwoTone => "antd-icons/twotone/control.svg",
            Self::CopyTwoTone => "antd-icons/twotone/copy.svg",
            Self::CopyrightCircleTwoTone => "antd-icons/twotone/copyright-circle.svg",
            Self::CopyrightTwoTone => "antd-icons/twotone/copyright.svg",
            Self::CreditCardTwoTone => "antd-icons/twotone/credit-card.svg",
            Self::CrownTwoTone => "antd-icons/twotone/crown.svg",
            Self::CustomerServiceTwoTone => "antd-icons/twotone/customer-service.svg",
            Self::DashboardTwoTone => "antd-icons/twotone/dashboard.svg",
            Self::DatabaseTwoTone => "antd-icons/twotone/database.svg",
            Self::DeleteTwoTone => "antd-icons/twotone/delete.svg",
            Self::DiffTwoTone => "antd-icons/twotone/diff.svg",
            Self::DislikeTwoTone => "antd-icons/twotone/dislike.svg",
            Self::DollarCircleTwoTone => "antd-icons/twotone/dollar-circle.svg",
            Self::DollarTwoTone => "antd-icons/twotone/dollar.svg",
            Self::DownCircleTwoTone => "antd-icons/twotone/down-circle.svg",
            Self::DownSquareTwoTone => "antd-icons/twotone/down-square.svg",
            Self::EditTwoTone => "antd-icons/twotone/edit.svg",
            Self::EnvironmentTwoTone => "antd-icons/twotone/environment.svg",
            Self::EuroCircleTwoTone => "antd-icons/twotone/euro-circle.svg",
            Self::EuroTwoTone => "antd-icons/twotone/euro.svg",
            Self::ExclamationCircleTwoTone => "antd-icons/twotone/exclamation-circle.svg",
            Self::ExperimentTwoTone => "antd-icons/twotone/experiment.svg",
            Self::EyeInvisibleTwoTone => "antd-icons/twotone/eye-invisible.svg",
            Self::EyeTwoTone => "antd-icons/twotone/eye.svg",
            Self::FileAddTwoTone => "antd-icons/twotone/file-add.svg",
            Self::FileExcelTwoTone => "antd-icons/twotone/file-excel.svg",
            Self::FileExclamationTwoTone => "antd-icons/twotone/file-exclamation.svg",
            Self::FileImageTwoTone => "antd-icons/twotone/file-image.svg",
            Self::FileMarkdownTwoTone => "antd-icons/twotone/file-markdown.svg",
            Self::FilePdfTwoTone => "antd-icons/twotone/file-pdf.svg",
            Self::FilePptTwoTone => "antd-icons/twotone/file-ppt.svg",
            Self::FileTextTwoTone => "antd-icons/twotone/file-text.svg",
            Self::FileUnknownTwoTone => "antd-icons/twotone/file-unknown.svg",
            Self::FileWordTwoTone => "antd-icons/twotone/file-word.svg",
            Self::FileZipTwoTone => "antd-icons/twotone/file-zip.svg",
            Self::FileTwoTone => "antd-icons/twotone/file.svg",
            Self::FilterTwoTone => "antd-icons/twotone/filter.svg",
            Self::FireTwoTone => "antd-icons/twotone/fire.svg",
            Self::FlagTwoTone => "antd-icons/twotone/flag.svg",
            Self::FolderAddTwoTone => "antd-icons/twotone/folder-add.svg",
            Self::FolderOpenTwoTone => "antd-icons/twotone/folder-open.svg",
            Self::FolderTwoTone => "antd-icons/twotone/folder.svg",
            Self::FrownTwoTone => "antd-icons/twotone/frown.svg",
            Self::FundTwoTone => "antd-icons/twotone/fund.svg",
            Self::FunnelPlotTwoTone => "antd-icons/twotone/funnel-plot.svg",
            Self::GiftTwoTone => "antd-icons/twotone/gift.svg",
            Self::GoldTwoTone => "antd-icons/twotone/gold.svg",
            Self::HddTwoTone => "antd-icons/twotone/hdd.svg",
            Self::HeartTwoTone => "antd-icons/twotone/heart.svg",
            Self::HighlightTwoTone => "antd-icons/twotone/highlight.svg",
            Self::HomeTwoTone => "antd-icons/twotone/home.svg",
            Self::HourglassTwoTone => "antd-icons/twotone/hourglass.svg",
            Self::Html5TwoTone => "antd-icons/twotone/html5.svg",
            Self::IdcardTwoTone => "antd-icons/twotone/idcard.svg",
            Self::InfoCircleTwoTone => "antd-icons/twotone/info-circle.svg",
            Self::InsuranceTwoTone => "antd-icons/twotone/insurance.svg",
            Self::InteractionTwoTone => "antd-icons/twotone/interaction.svg",
            Self::LayoutTwoTone => "antd-icons/twotone/layout.svg",
            Self::LeftCircleTwoTone => "antd-icons/twotone/left-circle.svg",
            Self::LeftSquareTwoTone => "antd-icons/twotone/left-square.svg",
            Self::LikeTwoTone => "antd-icons/twotone/like.svg",
            Self::LockTwoTone => "antd-icons/twotone/lock.svg",
            Self::MailTwoTone => "antd-icons/twotone/mail.svg",
            Self::MedicineBoxTwoTone => "antd-icons/twotone/medicine-box.svg",
            Self::MehTwoTone => "antd-icons/twotone/meh.svg",
            Self::MessageTwoTone => "antd-icons/twotone/message.svg",
            Self::MinusCircleTwoTone => "antd-icons/twotone/minus-circle.svg",
            Self::MinusSquareTwoTone => "antd-icons/twotone/minus-square.svg",
            Self::MobileTwoTone => "antd-icons/twotone/mobile.svg",
            Self::MoneyCollectTwoTone => "antd-icons/twotone/money-collect.svg",
            Self::NotificationTwoTone => "antd-icons/twotone/notification.svg",
            Self::PauseCircleTwoTone => "antd-icons/twotone/pause-circle.svg",
            Self::PhoneTwoTone => "antd-icons/twotone/phone.svg",
            Self::PictureTwoTone => "antd-icons/twotone/picture.svg",
            Self::PieChartTwoTone => "antd-icons/twotone/pie-chart.svg",
            Self::PlayCircleTwoTone => "antd-icons/twotone/play-circle.svg",
            Self::PlaySquareTwoTone => "antd-icons/twotone/play-square.svg",
            Self::PlusCircleTwoTone => "antd-icons/twotone/plus-circle.svg",
            Self::PlusSquareTwoTone => "antd-icons/twotone/plus-square.svg",
            Self::PoundCircleTwoTone => "antd-icons/twotone/pound-circle.svg",
            Self::PrinterTwoTone => "antd-icons/twotone/printer.svg",
            Self::ProfileTwoTone => "antd-icons/twotone/profile.svg",
            Self::ProjectTwoTone => "antd-icons/twotone/project.svg",
            Self::PropertySafetyTwoTone => "antd-icons/twotone/property-safety.svg",
            Self::PushpinTwoTone => "antd-icons/twotone/pushpin.svg",
            Self::QuestionCircleTwoTone => "antd-icons/twotone/question-circle.svg",
            Self::ReconciliationTwoTone => "antd-icons/twotone/reconciliation.svg",
            Self::RedEnvelopeTwoTone => "antd-icons/twotone/red-envelope.svg",
            Self::RestTwoTone => "antd-icons/twotone/rest.svg",
            Self::RightCircleTwoTone => "antd-icons/twotone/right-circle.svg",
            Self::RightSquareTwoTone => "antd-icons/twotone/right-square.svg",
            Self::RocketTwoTone => "antd-icons/twotone/rocket.svg",
            Self::SafetyCertificateTwoTone => "antd-icons/twotone/safety-certificate.svg",
            Self::SaveTwoTone => "antd-icons/twotone/save.svg",
            Self::ScheduleTwoTone => "antd-icons/twotone/schedule.svg",
            Self::SecurityScanTwoTone => "antd-icons/twotone/security-scan.svg",
            Self::SettingTwoTone => "antd-icons/twotone/setting.svg",
            Self::ShopTwoTone => "antd-icons/twotone/shop.svg",
            Self::ShoppingTwoTone => "antd-icons/twotone/shopping.svg",
            Self::SkinTwoTone => "antd-icons/twotone/skin.svg",
            Self::SlidersTwoTone => "antd-icons/twotone/sliders.svg",
            Self::SmileTwoTone => "antd-icons/twotone/smile.svg",
            Self::SnippetsTwoTone => "antd-icons/twotone/snippets.svg",
            Self::SoundTwoTone => "antd-icons/twotone/sound.svg",
            Self::StarTwoTone => "antd-icons/twotone/star.svg",
            Self::StopTwoTone => "antd-icons/twotone/stop.svg",
            Self::SwitcherTwoTone => "antd-icons/twotone/switcher.svg",
            Self::TabletTwoTone => "antd-icons/twotone/tablet.svg",
            Self::TagTwoTone => "antd-icons/twotone/tag.svg",
            Self::TagsTwoTone => "antd-icons/twotone/tags.svg",
            Self::ThunderboltTwoTone => "antd-icons/twotone/thunderbolt.svg",
            Self::ToolTwoTone => "antd-icons/twotone/tool.svg",
            Self::TrademarkCircleTwoTone => "antd-icons/twotone/trademark-circle.svg",
            Self::TrophyTwoTone => "antd-icons/twotone/trophy.svg",
            Self::UnlockTwoTone => "antd-icons/twotone/unlock.svg",
            Self::UpCircleTwoTone => "antd-icons/twotone/up-circle.svg",
            Self::UpSquareTwoTone => "antd-icons/twotone/up-square.svg",
            Self::UsbTwoTone => "antd-icons/twotone/usb.svg",
            Self::VideoCameraTwoTone => "antd-icons/twotone/video-camera.svg",
            Self::WalletTwoTone => "antd-icons/twotone/wallet.svg",
            Self::WarningTwoTone => "antd-icons/twotone/warning.svg",
        }
    }

    pub const fn ant_name(self) -> &'static str {
        match self {
            Self::AccountBookOutlined => "AccountBookOutlined",
            Self::AimOutlined => "AimOutlined",
            Self::AlertOutlined => "AlertOutlined",
            Self::AlibabaOutlined => "AlibabaOutlined",
            Self::AlignCenterOutlined => "AlignCenterOutlined",
            Self::AlignLeftOutlined => "AlignLeftOutlined",
            Self::AlignRightOutlined => "AlignRightOutlined",
            Self::AlipayCircleOutlined => "AlipayCircleOutlined",
            Self::AlipayOutlined => "AlipayOutlined",
            Self::AliwangwangOutlined => "AliwangwangOutlined",
            Self::AliyunOutlined => "AliyunOutlined",
            Self::AmazonOutlined => "AmazonOutlined",
            Self::AndroidOutlined => "AndroidOutlined",
            Self::AntCloudOutlined => "AntCloudOutlined",
            Self::AntDesignOutlined => "AntDesignOutlined",
            Self::ApartmentOutlined => "ApartmentOutlined",
            Self::ApiOutlined => "ApiOutlined",
            Self::AppleOutlined => "AppleOutlined",
            Self::AppstoreAddOutlined => "AppstoreAddOutlined",
            Self::AppstoreOutlined => "AppstoreOutlined",
            Self::AreaChartOutlined => "AreaChartOutlined",
            Self::ArrowDownOutlined => "ArrowDownOutlined",
            Self::ArrowLeftOutlined => "ArrowLeftOutlined",
            Self::ArrowRightOutlined => "ArrowRightOutlined",
            Self::ArrowUpOutlined => "ArrowUpOutlined",
            Self::ArrowsAltOutlined => "ArrowsAltOutlined",
            Self::AudioMutedOutlined => "AudioMutedOutlined",
            Self::AudioOutlined => "AudioOutlined",
            Self::AuditOutlined => "AuditOutlined",
            Self::BackwardOutlined => "BackwardOutlined",
            Self::BaiduOutlined => "BaiduOutlined",
            Self::BankOutlined => "BankOutlined",
            Self::BarChartOutlined => "BarChartOutlined",
            Self::BarcodeOutlined => "BarcodeOutlined",
            Self::BarsOutlined => "BarsOutlined",
            Self::BehanceSquareOutlined => "BehanceSquareOutlined",
            Self::BehanceOutlined => "BehanceOutlined",
            Self::BellOutlined => "BellOutlined",
            Self::BgColorsOutlined => "BgColorsOutlined",
            Self::BilibiliOutlined => "BilibiliOutlined",
            Self::BlockOutlined => "BlockOutlined",
            Self::BoldOutlined => "BoldOutlined",
            Self::BookOutlined => "BookOutlined",
            Self::BorderBottomOutlined => "BorderBottomOutlined",
            Self::BorderHorizontalOutlined => "BorderHorizontalOutlined",
            Self::BorderInnerOutlined => "BorderInnerOutlined",
            Self::BorderLeftOutlined => "BorderLeftOutlined",
            Self::BorderOuterOutlined => "BorderOuterOutlined",
            Self::BorderRightOutlined => "BorderRightOutlined",
            Self::BorderTopOutlined => "BorderTopOutlined",
            Self::BorderVerticleOutlined => "BorderVerticleOutlined",
            Self::BorderOutlined => "BorderOutlined",
            Self::BorderlessTableOutlined => "BorderlessTableOutlined",
            Self::BoxPlotOutlined => "BoxPlotOutlined",
            Self::BranchesOutlined => "BranchesOutlined",
            Self::BugOutlined => "BugOutlined",
            Self::BuildOutlined => "BuildOutlined",
            Self::BulbOutlined => "BulbOutlined",
            Self::CalculatorOutlined => "CalculatorOutlined",
            Self::CalendarOutlined => "CalendarOutlined",
            Self::CameraOutlined => "CameraOutlined",
            Self::CarOutlined => "CarOutlined",
            Self::CaretDownOutlined => "CaretDownOutlined",
            Self::CaretLeftOutlined => "CaretLeftOutlined",
            Self::CaretRightOutlined => "CaretRightOutlined",
            Self::CaretUpOutlined => "CaretUpOutlined",
            Self::CarryOutOutlined => "CarryOutOutlined",
            Self::CheckCircleOutlined => "CheckCircleOutlined",
            Self::CheckSquareOutlined => "CheckSquareOutlined",
            Self::CheckOutlined => "CheckOutlined",
            Self::ChromeOutlined => "ChromeOutlined",
            Self::CiCircleOutlined => "CiCircleOutlined",
            Self::CiOutlined => "CiOutlined",
            Self::ClearOutlined => "ClearOutlined",
            Self::ClockCircleOutlined => "ClockCircleOutlined",
            Self::CloseCircleOutlined => "CloseCircleOutlined",
            Self::CloseSquareOutlined => "CloseSquareOutlined",
            Self::CloseOutlined => "CloseOutlined",
            Self::CloudDownloadOutlined => "CloudDownloadOutlined",
            Self::CloudServerOutlined => "CloudServerOutlined",
            Self::CloudSyncOutlined => "CloudSyncOutlined",
            Self::CloudUploadOutlined => "CloudUploadOutlined",
            Self::CloudOutlined => "CloudOutlined",
            Self::ClusterOutlined => "ClusterOutlined",
            Self::CodeSandboxOutlined => "CodeSandboxOutlined",
            Self::CodeOutlined => "CodeOutlined",
            Self::CodepenCircleOutlined => "CodepenCircleOutlined",
            Self::CodepenOutlined => "CodepenOutlined",
            Self::CoffeeOutlined => "CoffeeOutlined",
            Self::ColumnHeightOutlined => "ColumnHeightOutlined",
            Self::ColumnWidthOutlined => "ColumnWidthOutlined",
            Self::CommentOutlined => "CommentOutlined",
            Self::CompassOutlined => "CompassOutlined",
            Self::CompressOutlined => "CompressOutlined",
            Self::ConsoleSqlOutlined => "ConsoleSqlOutlined",
            Self::ContactsOutlined => "ContactsOutlined",
            Self::ContainerOutlined => "ContainerOutlined",
            Self::ControlOutlined => "ControlOutlined",
            Self::CopyOutlined => "CopyOutlined",
            Self::CopyrightCircleOutlined => "CopyrightCircleOutlined",
            Self::CopyrightOutlined => "CopyrightOutlined",
            Self::CreditCardOutlined => "CreditCardOutlined",
            Self::CrownOutlined => "CrownOutlined",
            Self::CustomerServiceOutlined => "CustomerServiceOutlined",
            Self::DashOutlined => "DashOutlined",
            Self::DashboardOutlined => "DashboardOutlined",
            Self::DatabaseOutlined => "DatabaseOutlined",
            Self::DeleteColumnOutlined => "DeleteColumnOutlined",
            Self::DeleteRowOutlined => "DeleteRowOutlined",
            Self::DeleteOutlined => "DeleteOutlined",
            Self::DeliveredProcedureOutlined => "DeliveredProcedureOutlined",
            Self::DeploymentUnitOutlined => "DeploymentUnitOutlined",
            Self::DesktopOutlined => "DesktopOutlined",
            Self::DiffOutlined => "DiffOutlined",
            Self::DingdingOutlined => "DingdingOutlined",
            Self::DingtalkOutlined => "DingtalkOutlined",
            Self::DisconnectOutlined => "DisconnectOutlined",
            Self::DiscordOutlined => "DiscordOutlined",
            Self::DislikeOutlined => "DislikeOutlined",
            Self::DockerOutlined => "DockerOutlined",
            Self::DollarCircleOutlined => "DollarCircleOutlined",
            Self::DollarOutlined => "DollarOutlined",
            Self::DotChartOutlined => "DotChartOutlined",
            Self::DotNetOutlined => "DotNetOutlined",
            Self::DoubleLeftOutlined => "DoubleLeftOutlined",
            Self::DoubleRightOutlined => "DoubleRightOutlined",
            Self::DownCircleOutlined => "DownCircleOutlined",
            Self::DownSquareOutlined => "DownSquareOutlined",
            Self::DownOutlined => "DownOutlined",
            Self::DownloadOutlined => "DownloadOutlined",
            Self::DragOutlined => "DragOutlined",
            Self::DribbbleSquareOutlined => "DribbbleSquareOutlined",
            Self::DribbbleOutlined => "DribbbleOutlined",
            Self::DropboxOutlined => "DropboxOutlined",
            Self::EditOutlined => "EditOutlined",
            Self::EllipsisOutlined => "EllipsisOutlined",
            Self::EnterOutlined => "EnterOutlined",
            Self::EnvironmentOutlined => "EnvironmentOutlined",
            Self::EuroCircleOutlined => "EuroCircleOutlined",
            Self::EuroOutlined => "EuroOutlined",
            Self::ExceptionOutlined => "ExceptionOutlined",
            Self::ExclamationCircleOutlined => "ExclamationCircleOutlined",
            Self::ExclamationOutlined => "ExclamationOutlined",
            Self::ExpandAltOutlined => "ExpandAltOutlined",
            Self::ExpandOutlined => "ExpandOutlined",
            Self::ExperimentOutlined => "ExperimentOutlined",
            Self::ExportOutlined => "ExportOutlined",
            Self::EyeInvisibleOutlined => "EyeInvisibleOutlined",
            Self::EyeOutlined => "EyeOutlined",
            Self::FacebookOutlined => "FacebookOutlined",
            Self::FallOutlined => "FallOutlined",
            Self::FastBackwardOutlined => "FastBackwardOutlined",
            Self::FastForwardOutlined => "FastForwardOutlined",
            Self::FieldBinaryOutlined => "FieldBinaryOutlined",
            Self::FieldNumberOutlined => "FieldNumberOutlined",
            Self::FieldStringOutlined => "FieldStringOutlined",
            Self::FieldTimeOutlined => "FieldTimeOutlined",
            Self::FileAddOutlined => "FileAddOutlined",
            Self::FileDoneOutlined => "FileDoneOutlined",
            Self::FileExcelOutlined => "FileExcelOutlined",
            Self::FileExclamationOutlined => "FileExclamationOutlined",
            Self::FileGifOutlined => "FileGifOutlined",
            Self::FileImageOutlined => "FileImageOutlined",
            Self::FileJpgOutlined => "FileJpgOutlined",
            Self::FileMarkdownOutlined => "FileMarkdownOutlined",
            Self::FilePdfOutlined => "FilePdfOutlined",
            Self::FilePptOutlined => "FilePptOutlined",
            Self::FileProtectOutlined => "FileProtectOutlined",
            Self::FileSearchOutlined => "FileSearchOutlined",
            Self::FileSyncOutlined => "FileSyncOutlined",
            Self::FileTextOutlined => "FileTextOutlined",
            Self::FileUnknownOutlined => "FileUnknownOutlined",
            Self::FileWordOutlined => "FileWordOutlined",
            Self::FileZipOutlined => "FileZipOutlined",
            Self::FileOutlined => "FileOutlined",
            Self::FilterOutlined => "FilterOutlined",
            Self::FireOutlined => "FireOutlined",
            Self::FlagOutlined => "FlagOutlined",
            Self::FolderAddOutlined => "FolderAddOutlined",
            Self::FolderOpenOutlined => "FolderOpenOutlined",
            Self::FolderViewOutlined => "FolderViewOutlined",
            Self::FolderOutlined => "FolderOutlined",
            Self::FontColorsOutlined => "FontColorsOutlined",
            Self::FontSizeOutlined => "FontSizeOutlined",
            Self::ForkOutlined => "ForkOutlined",
            Self::FormOutlined => "FormOutlined",
            Self::FormatPainterOutlined => "FormatPainterOutlined",
            Self::ForwardOutlined => "ForwardOutlined",
            Self::FrownOutlined => "FrownOutlined",
            Self::FullscreenExitOutlined => "FullscreenExitOutlined",
            Self::FullscreenOutlined => "FullscreenOutlined",
            Self::FunctionOutlined => "FunctionOutlined",
            Self::FundProjectionScreenOutlined => "FundProjectionScreenOutlined",
            Self::FundViewOutlined => "FundViewOutlined",
            Self::FundOutlined => "FundOutlined",
            Self::FunnelPlotOutlined => "FunnelPlotOutlined",
            Self::GatewayOutlined => "GatewayOutlined",
            Self::GifOutlined => "GifOutlined",
            Self::GiftOutlined => "GiftOutlined",
            Self::GithubOutlined => "GithubOutlined",
            Self::GitlabOutlined => "GitlabOutlined",
            Self::GlobalOutlined => "GlobalOutlined",
            Self::GoldOutlined => "GoldOutlined",
            Self::GooglePlusOutlined => "GooglePlusOutlined",
            Self::GoogleOutlined => "GoogleOutlined",
            Self::GroupOutlined => "GroupOutlined",
            Self::HarmonyOSOutlined => "HarmonyOSOutlined",
            Self::HddOutlined => "HddOutlined",
            Self::HeartOutlined => "HeartOutlined",
            Self::HeatMapOutlined => "HeatMapOutlined",
            Self::HighlightOutlined => "HighlightOutlined",
            Self::HistoryOutlined => "HistoryOutlined",
            Self::HolderOutlined => "HolderOutlined",
            Self::HomeOutlined => "HomeOutlined",
            Self::HourglassOutlined => "HourglassOutlined",
            Self::Html5Outlined => "Html5Outlined",
            Self::IdcardOutlined => "IdcardOutlined",
            Self::IeOutlined => "IeOutlined",
            Self::ImportOutlined => "ImportOutlined",
            Self::InboxOutlined => "InboxOutlined",
            Self::InfoCircleOutlined => "InfoCircleOutlined",
            Self::InfoOutlined => "InfoOutlined",
            Self::InsertRowAboveOutlined => "InsertRowAboveOutlined",
            Self::InsertRowBelowOutlined => "InsertRowBelowOutlined",
            Self::InsertRowLeftOutlined => "InsertRowLeftOutlined",
            Self::InsertRowRightOutlined => "InsertRowRightOutlined",
            Self::InstagramOutlined => "InstagramOutlined",
            Self::InsuranceOutlined => "InsuranceOutlined",
            Self::InteractionOutlined => "InteractionOutlined",
            Self::IssuesCloseOutlined => "IssuesCloseOutlined",
            Self::ItalicOutlined => "ItalicOutlined",
            Self::JavaScriptOutlined => "JavaScriptOutlined",
            Self::JavaOutlined => "JavaOutlined",
            Self::KeyOutlined => "KeyOutlined",
            Self::KubernetesOutlined => "KubernetesOutlined",
            Self::LaptopOutlined => "LaptopOutlined",
            Self::LayoutOutlined => "LayoutOutlined",
            Self::LeftCircleOutlined => "LeftCircleOutlined",
            Self::LeftSquareOutlined => "LeftSquareOutlined",
            Self::LeftOutlined => "LeftOutlined",
            Self::LikeOutlined => "LikeOutlined",
            Self::LineChartOutlined => "LineChartOutlined",
            Self::LineHeightOutlined => "LineHeightOutlined",
            Self::LineOutlined => "LineOutlined",
            Self::LinkOutlined => "LinkOutlined",
            Self::LinkedinOutlined => "LinkedinOutlined",
            Self::LinuxOutlined => "LinuxOutlined",
            Self::Loading3QuartersOutlined => "Loading3QuartersOutlined",
            Self::LoadingOutlined => "LoadingOutlined",
            Self::LockOutlined => "LockOutlined",
            Self::LoginOutlined => "LoginOutlined",
            Self::LogoutOutlined => "LogoutOutlined",
            Self::MacCommandOutlined => "MacCommandOutlined",
            Self::MailOutlined => "MailOutlined",
            Self::ManOutlined => "ManOutlined",
            Self::MedicineBoxOutlined => "MedicineBoxOutlined",
            Self::MediumWorkmarkOutlined => "MediumWorkmarkOutlined",
            Self::MediumOutlined => "MediumOutlined",
            Self::MehOutlined => "MehOutlined",
            Self::MenuFoldOutlined => "MenuFoldOutlined",
            Self::MenuUnfoldOutlined => "MenuUnfoldOutlined",
            Self::MenuOutlined => "MenuOutlined",
            Self::MergeCellsOutlined => "MergeCellsOutlined",
            Self::MergeOutlined => "MergeOutlined",
            Self::MessageOutlined => "MessageOutlined",
            Self::MinusCircleOutlined => "MinusCircleOutlined",
            Self::MinusSquareOutlined => "MinusSquareOutlined",
            Self::MinusOutlined => "MinusOutlined",
            Self::MobileOutlined => "MobileOutlined",
            Self::MoneyCollectOutlined => "MoneyCollectOutlined",
            Self::MonitorOutlined => "MonitorOutlined",
            Self::MoonOutlined => "MoonOutlined",
            Self::MoreOutlined => "MoreOutlined",
            Self::MutedOutlined => "MutedOutlined",
            Self::NodeCollapseOutlined => "NodeCollapseOutlined",
            Self::NodeExpandOutlined => "NodeExpandOutlined",
            Self::NodeIndexOutlined => "NodeIndexOutlined",
            Self::NotificationOutlined => "NotificationOutlined",
            Self::NumberOutlined => "NumberOutlined",
            Self::OneToOneOutlined => "OneToOneOutlined",
            Self::OpenAIOutlined => "OpenAIOutlined",
            Self::OrderedListOutlined => "OrderedListOutlined",
            Self::PaperClipOutlined => "PaperClipOutlined",
            Self::PartitionOutlined => "PartitionOutlined",
            Self::PauseCircleOutlined => "PauseCircleOutlined",
            Self::PauseOutlined => "PauseOutlined",
            Self::PayCircleOutlined => "PayCircleOutlined",
            Self::PercentageOutlined => "PercentageOutlined",
            Self::PhoneOutlined => "PhoneOutlined",
            Self::PicCenterOutlined => "PicCenterOutlined",
            Self::PicLeftOutlined => "PicLeftOutlined",
            Self::PicRightOutlined => "PicRightOutlined",
            Self::PictureOutlined => "PictureOutlined",
            Self::PieChartOutlined => "PieChartOutlined",
            Self::PinterestOutlined => "PinterestOutlined",
            Self::PlayCircleOutlined => "PlayCircleOutlined",
            Self::PlaySquareOutlined => "PlaySquareOutlined",
            Self::PlusCircleOutlined => "PlusCircleOutlined",
            Self::PlusSquareOutlined => "PlusSquareOutlined",
            Self::PlusOutlined => "PlusOutlined",
            Self::PoundCircleOutlined => "PoundCircleOutlined",
            Self::PoundOutlined => "PoundOutlined",
            Self::PoweroffOutlined => "PoweroffOutlined",
            Self::PrinterOutlined => "PrinterOutlined",
            Self::ProductOutlined => "ProductOutlined",
            Self::ProfileOutlined => "ProfileOutlined",
            Self::ProjectOutlined => "ProjectOutlined",
            Self::PropertySafetyOutlined => "PropertySafetyOutlined",
            Self::PullRequestOutlined => "PullRequestOutlined",
            Self::PushpinOutlined => "PushpinOutlined",
            Self::PythonOutlined => "PythonOutlined",
            Self::QqOutlined => "QqOutlined",
            Self::QrcodeOutlined => "QrcodeOutlined",
            Self::QuestionCircleOutlined => "QuestionCircleOutlined",
            Self::QuestionOutlined => "QuestionOutlined",
            Self::RadarChartOutlined => "RadarChartOutlined",
            Self::RadiusBottomleftOutlined => "RadiusBottomleftOutlined",
            Self::RadiusBottomrightOutlined => "RadiusBottomrightOutlined",
            Self::RadiusSettingOutlined => "RadiusSettingOutlined",
            Self::RadiusUpleftOutlined => "RadiusUpleftOutlined",
            Self::RadiusUprightOutlined => "RadiusUprightOutlined",
            Self::ReadOutlined => "ReadOutlined",
            Self::ReconciliationOutlined => "ReconciliationOutlined",
            Self::RedEnvelopeOutlined => "RedEnvelopeOutlined",
            Self::RedditOutlined => "RedditOutlined",
            Self::RedoOutlined => "RedoOutlined",
            Self::ReloadOutlined => "ReloadOutlined",
            Self::RestOutlined => "RestOutlined",
            Self::RetweetOutlined => "RetweetOutlined",
            Self::RightCircleOutlined => "RightCircleOutlined",
            Self::RightSquareOutlined => "RightSquareOutlined",
            Self::RightOutlined => "RightOutlined",
            Self::RiseOutlined => "RiseOutlined",
            Self::RobotOutlined => "RobotOutlined",
            Self::RocketOutlined => "RocketOutlined",
            Self::RollbackOutlined => "RollbackOutlined",
            Self::RotateLeftOutlined => "RotateLeftOutlined",
            Self::RotateRightOutlined => "RotateRightOutlined",
            Self::RubyOutlined => "RubyOutlined",
            Self::SafetyCertificateOutlined => "SafetyCertificateOutlined",
            Self::SafetyOutlined => "SafetyOutlined",
            Self::SaveOutlined => "SaveOutlined",
            Self::ScanOutlined => "ScanOutlined",
            Self::ScheduleOutlined => "ScheduleOutlined",
            Self::ScissorOutlined => "ScissorOutlined",
            Self::SearchOutlined => "SearchOutlined",
            Self::SecurityScanOutlined => "SecurityScanOutlined",
            Self::SelectOutlined => "SelectOutlined",
            Self::SendOutlined => "SendOutlined",
            Self::SettingOutlined => "SettingOutlined",
            Self::ShakeOutlined => "ShakeOutlined",
            Self::ShareAltOutlined => "ShareAltOutlined",
            Self::ShopOutlined => "ShopOutlined",
            Self::ShoppingCartOutlined => "ShoppingCartOutlined",
            Self::ShoppingOutlined => "ShoppingOutlined",
            Self::ShrinkOutlined => "ShrinkOutlined",
            Self::SignatureOutlined => "SignatureOutlined",
            Self::SisternodeOutlined => "SisternodeOutlined",
            Self::SketchOutlined => "SketchOutlined",
            Self::SkinOutlined => "SkinOutlined",
            Self::SkypeOutlined => "SkypeOutlined",
            Self::SlackSquareOutlined => "SlackSquareOutlined",
            Self::SlackOutlined => "SlackOutlined",
            Self::SlidersOutlined => "SlidersOutlined",
            Self::SmallDashOutlined => "SmallDashOutlined",
            Self::SmileOutlined => "SmileOutlined",
            Self::SnippetsOutlined => "SnippetsOutlined",
            Self::SolutionOutlined => "SolutionOutlined",
            Self::SortAscendingOutlined => "SortAscendingOutlined",
            Self::SortDescendingOutlined => "SortDescendingOutlined",
            Self::SoundOutlined => "SoundOutlined",
            Self::SplitCellsOutlined => "SplitCellsOutlined",
            Self::SpotifyOutlined => "SpotifyOutlined",
            Self::StarOutlined => "StarOutlined",
            Self::StepBackwardOutlined => "StepBackwardOutlined",
            Self::StepForwardOutlined => "StepForwardOutlined",
            Self::StockOutlined => "StockOutlined",
            Self::StopOutlined => "StopOutlined",
            Self::StrikethroughOutlined => "StrikethroughOutlined",
            Self::SubnodeOutlined => "SubnodeOutlined",
            Self::SunOutlined => "SunOutlined",
            Self::SwapLeftOutlined => "SwapLeftOutlined",
            Self::SwapRightOutlined => "SwapRightOutlined",
            Self::SwapOutlined => "SwapOutlined",
            Self::SwitcherOutlined => "SwitcherOutlined",
            Self::SyncOutlined => "SyncOutlined",
            Self::TableOutlined => "TableOutlined",
            Self::TabletOutlined => "TabletOutlined",
            Self::TagOutlined => "TagOutlined",
            Self::TagsOutlined => "TagsOutlined",
            Self::TaobaoCircleOutlined => "TaobaoCircleOutlined",
            Self::TaobaoOutlined => "TaobaoOutlined",
            Self::TeamOutlined => "TeamOutlined",
            Self::ThunderboltOutlined => "ThunderboltOutlined",
            Self::TikTokOutlined => "TikTokOutlined",
            Self::ToTopOutlined => "ToTopOutlined",
            Self::ToolOutlined => "ToolOutlined",
            Self::TrademarkCircleOutlined => "TrademarkCircleOutlined",
            Self::TrademarkOutlined => "TrademarkOutlined",
            Self::TransactionOutlined => "TransactionOutlined",
            Self::TranslationOutlined => "TranslationOutlined",
            Self::TrophyOutlined => "TrophyOutlined",
            Self::TruckOutlined => "TruckOutlined",
            Self::TwitchOutlined => "TwitchOutlined",
            Self::TwitterOutlined => "TwitterOutlined",
            Self::UnderlineOutlined => "UnderlineOutlined",
            Self::UndoOutlined => "UndoOutlined",
            Self::UngroupOutlined => "UngroupOutlined",
            Self::UnlockOutlined => "UnlockOutlined",
            Self::UnorderedListOutlined => "UnorderedListOutlined",
            Self::UpCircleOutlined => "UpCircleOutlined",
            Self::UpSquareOutlined => "UpSquareOutlined",
            Self::UpOutlined => "UpOutlined",
            Self::UploadOutlined => "UploadOutlined",
            Self::UsbOutlined => "UsbOutlined",
            Self::UserAddOutlined => "UserAddOutlined",
            Self::UserDeleteOutlined => "UserDeleteOutlined",
            Self::UserSwitchOutlined => "UserSwitchOutlined",
            Self::UserOutlined => "UserOutlined",
            Self::UsergroupAddOutlined => "UsergroupAddOutlined",
            Self::UsergroupDeleteOutlined => "UsergroupDeleteOutlined",
            Self::VerifiedOutlined => "VerifiedOutlined",
            Self::VerticalAlignBottomOutlined => "VerticalAlignBottomOutlined",
            Self::VerticalAlignMiddleOutlined => "VerticalAlignMiddleOutlined",
            Self::VerticalAlignTopOutlined => "VerticalAlignTopOutlined",
            Self::VerticalLeftOutlined => "VerticalLeftOutlined",
            Self::VerticalRightOutlined => "VerticalRightOutlined",
            Self::VideoCameraAddOutlined => "VideoCameraAddOutlined",
            Self::VideoCameraOutlined => "VideoCameraOutlined",
            Self::WalletOutlined => "WalletOutlined",
            Self::WarningOutlined => "WarningOutlined",
            Self::WechatWorkOutlined => "WechatWorkOutlined",
            Self::WechatOutlined => "WechatOutlined",
            Self::WeiboCircleOutlined => "WeiboCircleOutlined",
            Self::WeiboSquareOutlined => "WeiboSquareOutlined",
            Self::WeiboOutlined => "WeiboOutlined",
            Self::WhatsAppOutlined => "WhatsAppOutlined",
            Self::WifiOutlined => "WifiOutlined",
            Self::WindowsOutlined => "WindowsOutlined",
            Self::WomanOutlined => "WomanOutlined",
            Self::XOutlined => "XOutlined",
            Self::YahooOutlined => "YahooOutlined",
            Self::YoutubeOutlined => "YoutubeOutlined",
            Self::YuqueOutlined => "YuqueOutlined",
            Self::ZhihuOutlined => "ZhihuOutlined",
            Self::ZoomInOutlined => "ZoomInOutlined",
            Self::ZoomOutOutlined => "ZoomOutOutlined",
            Self::AccountBookFilled => "AccountBookFilled",
            Self::AlertFilled => "AlertFilled",
            Self::AlipayCircleFilled => "AlipayCircleFilled",
            Self::AlipaySquareFilled => "AlipaySquareFilled",
            Self::AliwangwangFilled => "AliwangwangFilled",
            Self::AmazonCircleFilled => "AmazonCircleFilled",
            Self::AmazonSquareFilled => "AmazonSquareFilled",
            Self::AndroidFilled => "AndroidFilled",
            Self::AnthropicFilled => "AnthropicFilled",
            Self::ApiFilled => "ApiFilled",
            Self::AppleFilled => "AppleFilled",
            Self::AppstoreFilled => "AppstoreFilled",
            Self::AudioFilled => "AudioFilled",
            Self::BackwardFilled => "BackwardFilled",
            Self::BankFilled => "BankFilled",
            Self::BehanceCircleFilled => "BehanceCircleFilled",
            Self::BehanceSquareFilled => "BehanceSquareFilled",
            Self::BellFilled => "BellFilled",
            Self::BilibiliFilled => "BilibiliFilled",
            Self::BookFilled => "BookFilled",
            Self::BoxPlotFilled => "BoxPlotFilled",
            Self::BugFilled => "BugFilled",
            Self::BuildFilled => "BuildFilled",
            Self::BulbFilled => "BulbFilled",
            Self::CalculatorFilled => "CalculatorFilled",
            Self::CalendarFilled => "CalendarFilled",
            Self::CameraFilled => "CameraFilled",
            Self::CarFilled => "CarFilled",
            Self::CaretDownFilled => "CaretDownFilled",
            Self::CaretLeftFilled => "CaretLeftFilled",
            Self::CaretRightFilled => "CaretRightFilled",
            Self::CaretUpFilled => "CaretUpFilled",
            Self::CarryOutFilled => "CarryOutFilled",
            Self::CheckCircleFilled => "CheckCircleFilled",
            Self::CheckSquareFilled => "CheckSquareFilled",
            Self::ChromeFilled => "ChromeFilled",
            Self::CiCircleFilled => "CiCircleFilled",
            Self::ClaudeFilled => "ClaudeFilled",
            Self::ClockCircleFilled => "ClockCircleFilled",
            Self::CloseCircleFilled => "CloseCircleFilled",
            Self::CloseSquareFilled => "CloseSquareFilled",
            Self::CloudFilled => "CloudFilled",
            Self::CodeSandboxCircleFilled => "CodeSandboxCircleFilled",
            Self::CodeSandboxSquareFilled => "CodeSandboxSquareFilled",
            Self::CodeFilled => "CodeFilled",
            Self::CodepenCircleFilled => "CodepenCircleFilled",
            Self::CodepenSquareFilled => "CodepenSquareFilled",
            Self::CompassFilled => "CompassFilled",
            Self::ContactsFilled => "ContactsFilled",
            Self::ContainerFilled => "ContainerFilled",
            Self::ControlFilled => "ControlFilled",
            Self::CopyFilled => "CopyFilled",
            Self::CopyrightCircleFilled => "CopyrightCircleFilled",
            Self::CreditCardFilled => "CreditCardFilled",
            Self::CrownFilled => "CrownFilled",
            Self::CustomerServiceFilled => "CustomerServiceFilled",
            Self::DashboardFilled => "DashboardFilled",
            Self::DatabaseFilled => "DatabaseFilled",
            Self::DeepSeekFilled => "DeepSeekFilled",
            Self::DeleteFilled => "DeleteFilled",
            Self::DiffFilled => "DiffFilled",
            Self::DingtalkCircleFilled => "DingtalkCircleFilled",
            Self::DingtalkSquareFilled => "DingtalkSquareFilled",
            Self::DiscordFilled => "DiscordFilled",
            Self::DislikeFilled => "DislikeFilled",
            Self::DollarCircleFilled => "DollarCircleFilled",
            Self::DownCircleFilled => "DownCircleFilled",
            Self::DownSquareFilled => "DownSquareFilled",
            Self::DribbbleCircleFilled => "DribbbleCircleFilled",
            Self::DribbbleSquareFilled => "DribbbleSquareFilled",
            Self::DropboxCircleFilled => "DropboxCircleFilled",
            Self::DropboxSquareFilled => "DropboxSquareFilled",
            Self::EditFilled => "EditFilled",
            Self::ElevenLabsFilled => "ElevenLabsFilled",
            Self::EnvironmentFilled => "EnvironmentFilled",
            Self::EuroCircleFilled => "EuroCircleFilled",
            Self::ExclamationCircleFilled => "ExclamationCircleFilled",
            Self::ExperimentFilled => "ExperimentFilled",
            Self::EyeInvisibleFilled => "EyeInvisibleFilled",
            Self::EyeFilled => "EyeFilled",
            Self::FacebookFilled => "FacebookFilled",
            Self::FastBackwardFilled => "FastBackwardFilled",
            Self::FastForwardFilled => "FastForwardFilled",
            Self::FileAddFilled => "FileAddFilled",
            Self::FileExcelFilled => "FileExcelFilled",
            Self::FileExclamationFilled => "FileExclamationFilled",
            Self::FileImageFilled => "FileImageFilled",
            Self::FileMarkdownFilled => "FileMarkdownFilled",
            Self::FilePdfFilled => "FilePdfFilled",
            Self::FilePptFilled => "FilePptFilled",
            Self::FileTextFilled => "FileTextFilled",
            Self::FileUnknownFilled => "FileUnknownFilled",
            Self::FileWordFilled => "FileWordFilled",
            Self::FileZipFilled => "FileZipFilled",
            Self::FileFilled => "FileFilled",
            Self::FilterFilled => "FilterFilled",
            Self::FireFilled => "FireFilled",
            Self::FlagFilled => "FlagFilled",
            Self::FolderAddFilled => "FolderAddFilled",
            Self::FolderOpenFilled => "FolderOpenFilled",
            Self::FolderFilled => "FolderFilled",
            Self::FormatPainterFilled => "FormatPainterFilled",
            Self::ForwardFilled => "ForwardFilled",
            Self::FrownFilled => "FrownFilled",
            Self::FundFilled => "FundFilled",
            Self::FunnelPlotFilled => "FunnelPlotFilled",
            Self::GeminiFilled => "GeminiFilled",
            Self::GiftFilled => "GiftFilled",
            Self::GithubFilled => "GithubFilled",
            Self::GitlabFilled => "GitlabFilled",
            Self::GoldFilled => "GoldFilled",
            Self::GoldenFilled => "GoldenFilled",
            Self::GoogleCircleFilled => "GoogleCircleFilled",
            Self::GooglePlusCircleFilled => "GooglePlusCircleFilled",
            Self::GooglePlusSquareFilled => "GooglePlusSquareFilled",
            Self::GoogleSquareFilled => "GoogleSquareFilled",
            Self::HddFilled => "HddFilled",
            Self::HeartFilled => "HeartFilled",
            Self::HighlightFilled => "HighlightFilled",
            Self::HomeFilled => "HomeFilled",
            Self::HourglassFilled => "HourglassFilled",
            Self::Html5Filled => "Html5Filled",
            Self::HuggingFaceFilled => "HuggingFaceFilled",
            Self::IdcardFilled => "IdcardFilled",
            Self::IeCircleFilled => "IeCircleFilled",
            Self::IeSquareFilled => "IeSquareFilled",
            Self::InfoCircleFilled => "InfoCircleFilled",
            Self::InstagramFilled => "InstagramFilled",
            Self::InsuranceFilled => "InsuranceFilled",
            Self::InteractionFilled => "InteractionFilled",
            Self::LayoutFilled => "LayoutFilled",
            Self::LeftCircleFilled => "LeftCircleFilled",
            Self::LeftSquareFilled => "LeftSquareFilled",
            Self::LikeFilled => "LikeFilled",
            Self::LinkedinFilled => "LinkedinFilled",
            Self::LockFilled => "LockFilled",
            Self::MacCommandFilled => "MacCommandFilled",
            Self::MailFilled => "MailFilled",
            Self::MastodonFilled => "MastodonFilled",
            Self::MedicineBoxFilled => "MedicineBoxFilled",
            Self::MediumCircleFilled => "MediumCircleFilled",
            Self::MediumSquareFilled => "MediumSquareFilled",
            Self::MehFilled => "MehFilled",
            Self::MergeFilled => "MergeFilled",
            Self::MessageFilled => "MessageFilled",
            Self::MetaFilled => "MetaFilled",
            Self::MinusCircleFilled => "MinusCircleFilled",
            Self::MinusSquareFilled => "MinusSquareFilled",
            Self::MistralFilled => "MistralFilled",
            Self::MobileFilled => "MobileFilled",
            Self::MoneyCollectFilled => "MoneyCollectFilled",
            Self::MoonFilled => "MoonFilled",
            Self::MutedFilled => "MutedFilled",
            Self::NetflixFilled => "NetflixFilled",
            Self::NotificationFilled => "NotificationFilled",
            Self::OllamaFilled => "OllamaFilled",
            Self::OpenAIFilled => "OpenAIFilled",
            Self::PauseCircleFilled => "PauseCircleFilled",
            Self::PayCircleFilled => "PayCircleFilled",
            Self::PerplexityFilled => "PerplexityFilled",
            Self::PhoneFilled => "PhoneFilled",
            Self::PictureFilled => "PictureFilled",
            Self::PieChartFilled => "PieChartFilled",
            Self::PinterestFilled => "PinterestFilled",
            Self::PlayCircleFilled => "PlayCircleFilled",
            Self::PlaySquareFilled => "PlaySquareFilled",
            Self::PlusCircleFilled => "PlusCircleFilled",
            Self::PlusSquareFilled => "PlusSquareFilled",
            Self::PoundCircleFilled => "PoundCircleFilled",
            Self::PrinterFilled => "PrinterFilled",
            Self::ProductFilled => "ProductFilled",
            Self::ProfileFilled => "ProfileFilled",
            Self::ProjectFilled => "ProjectFilled",
            Self::PropertySafetyFilled => "PropertySafetyFilled",
            Self::PushpinFilled => "PushpinFilled",
            Self::QqCircleFilled => "QqCircleFilled",
            Self::QqSquareFilled => "QqSquareFilled",
            Self::QuestionCircleFilled => "QuestionCircleFilled",
            Self::QwenFilled => "QwenFilled",
            Self::ReadFilled => "ReadFilled",
            Self::ReconciliationFilled => "ReconciliationFilled",
            Self::RedEnvelopeFilled => "RedEnvelopeFilled",
            Self::RedditCircleFilled => "RedditCircleFilled",
            Self::RedditSquareFilled => "RedditSquareFilled",
            Self::ReplicateFilled => "ReplicateFilled",
            Self::RestFilled => "RestFilled",
            Self::RightCircleFilled => "RightCircleFilled",
            Self::RightSquareFilled => "RightSquareFilled",
            Self::RobotFilled => "RobotFilled",
            Self::RocketFilled => "RocketFilled",
            Self::SafetyCertificateFilled => "SafetyCertificateFilled",
            Self::SaveFilled => "SaveFilled",
            Self::ScheduleFilled => "ScheduleFilled",
            Self::SecurityScanFilled => "SecurityScanFilled",
            Self::SettingFilled => "SettingFilled",
            Self::ShopFilled => "ShopFilled",
            Self::ShoppingFilled => "ShoppingFilled",
            Self::SignalFilled => "SignalFilled",
            Self::SignatureFilled => "SignatureFilled",
            Self::SketchCircleFilled => "SketchCircleFilled",
            Self::SketchSquareFilled => "SketchSquareFilled",
            Self::SkinFilled => "SkinFilled",
            Self::SkypeFilled => "SkypeFilled",
            Self::SlackCircleFilled => "SlackCircleFilled",
            Self::SlackSquareFilled => "SlackSquareFilled",
            Self::SlidersFilled => "SlidersFilled",
            Self::SmileFilled => "SmileFilled",
            Self::SnapchatFilled => "SnapchatFilled",
            Self::SnippetsFilled => "SnippetsFilled",
            Self::SoundFilled => "SoundFilled",
            Self::SpotifyFilled => "SpotifyFilled",
            Self::StarFilled => "StarFilled",
            Self::StepBackwardFilled => "StepBackwardFilled",
            Self::StepForwardFilled => "StepForwardFilled",
            Self::StopFilled => "StopFilled",
            Self::SunFilled => "SunFilled",
            Self::SwitcherFilled => "SwitcherFilled",
            Self::TabletFilled => "TabletFilled",
            Self::TagFilled => "TagFilled",
            Self::TagsFilled => "TagsFilled",
            Self::TaobaoCircleFilled => "TaobaoCircleFilled",
            Self::TaobaoSquareFilled => "TaobaoSquareFilled",
            Self::TelegramFilled => "TelegramFilled",
            Self::ThreadsFilled => "ThreadsFilled",
            Self::ThunderboltFilled => "ThunderboltFilled",
            Self::TikTokFilled => "TikTokFilled",
            Self::ToolFilled => "ToolFilled",
            Self::TrademarkCircleFilled => "TrademarkCircleFilled",
            Self::TrophyFilled => "TrophyFilled",
            Self::TruckFilled => "TruckFilled",
            Self::TwitchFilled => "TwitchFilled",
            Self::TwitterCircleFilled => "TwitterCircleFilled",
            Self::TwitterSquareFilled => "TwitterSquareFilled",
            Self::UnlockFilled => "UnlockFilled",
            Self::UpCircleFilled => "UpCircleFilled",
            Self::UpSquareFilled => "UpSquareFilled",
            Self::UsbFilled => "UsbFilled",
            Self::VideoCameraFilled => "VideoCameraFilled",
            Self::WalletFilled => "WalletFilled",
            Self::WarningFilled => "WarningFilled",
            Self::WechatWorkFilled => "WechatWorkFilled",
            Self::WechatFilled => "WechatFilled",
            Self::WeiboCircleFilled => "WeiboCircleFilled",
            Self::WeiboSquareFilled => "WeiboSquareFilled",
            Self::WindowsFilled => "WindowsFilled",
            Self::XFilled => "XFilled",
            Self::YahooFilled => "YahooFilled",
            Self::YoutubeFilled => "YoutubeFilled",
            Self::YuqueFilled => "YuqueFilled",
            Self::ZhihuCircleFilled => "ZhihuCircleFilled",
            Self::ZhihuSquareFilled => "ZhihuSquareFilled",
            Self::AccountBookTwoTone => "AccountBookTwoTone",
            Self::AlertTwoTone => "AlertTwoTone",
            Self::ApiTwoTone => "ApiTwoTone",
            Self::AppstoreTwoTone => "AppstoreTwoTone",
            Self::AudioTwoTone => "AudioTwoTone",
            Self::BankTwoTone => "BankTwoTone",
            Self::BellTwoTone => "BellTwoTone",
            Self::BookTwoTone => "BookTwoTone",
            Self::BoxPlotTwoTone => "BoxPlotTwoTone",
            Self::BugTwoTone => "BugTwoTone",
            Self::BuildTwoTone => "BuildTwoTone",
            Self::BulbTwoTone => "BulbTwoTone",
            Self::CalculatorTwoTone => "CalculatorTwoTone",
            Self::CalendarTwoTone => "CalendarTwoTone",
            Self::CameraTwoTone => "CameraTwoTone",
            Self::CarTwoTone => "CarTwoTone",
            Self::CarryOutTwoTone => "CarryOutTwoTone",
            Self::CheckCircleTwoTone => "CheckCircleTwoTone",
            Self::CheckSquareTwoTone => "CheckSquareTwoTone",
            Self::CiCircleTwoTone => "CiCircleTwoTone",
            Self::CiTwoTone => "CiTwoTone",
            Self::ClockCircleTwoTone => "ClockCircleTwoTone",
            Self::CloseCircleTwoTone => "CloseCircleTwoTone",
            Self::CloseSquareTwoTone => "CloseSquareTwoTone",
            Self::CloudTwoTone => "CloudTwoTone",
            Self::CodeTwoTone => "CodeTwoTone",
            Self::CompassTwoTone => "CompassTwoTone",
            Self::ContactsTwoTone => "ContactsTwoTone",
            Self::ContainerTwoTone => "ContainerTwoTone",
            Self::ControlTwoTone => "ControlTwoTone",
            Self::CopyTwoTone => "CopyTwoTone",
            Self::CopyrightCircleTwoTone => "CopyrightCircleTwoTone",
            Self::CopyrightTwoTone => "CopyrightTwoTone",
            Self::CreditCardTwoTone => "CreditCardTwoTone",
            Self::CrownTwoTone => "CrownTwoTone",
            Self::CustomerServiceTwoTone => "CustomerServiceTwoTone",
            Self::DashboardTwoTone => "DashboardTwoTone",
            Self::DatabaseTwoTone => "DatabaseTwoTone",
            Self::DeleteTwoTone => "DeleteTwoTone",
            Self::DiffTwoTone => "DiffTwoTone",
            Self::DislikeTwoTone => "DislikeTwoTone",
            Self::DollarCircleTwoTone => "DollarCircleTwoTone",
            Self::DollarTwoTone => "DollarTwoTone",
            Self::DownCircleTwoTone => "DownCircleTwoTone",
            Self::DownSquareTwoTone => "DownSquareTwoTone",
            Self::EditTwoTone => "EditTwoTone",
            Self::EnvironmentTwoTone => "EnvironmentTwoTone",
            Self::EuroCircleTwoTone => "EuroCircleTwoTone",
            Self::EuroTwoTone => "EuroTwoTone",
            Self::ExclamationCircleTwoTone => "ExclamationCircleTwoTone",
            Self::ExperimentTwoTone => "ExperimentTwoTone",
            Self::EyeInvisibleTwoTone => "EyeInvisibleTwoTone",
            Self::EyeTwoTone => "EyeTwoTone",
            Self::FileAddTwoTone => "FileAddTwoTone",
            Self::FileExcelTwoTone => "FileExcelTwoTone",
            Self::FileExclamationTwoTone => "FileExclamationTwoTone",
            Self::FileImageTwoTone => "FileImageTwoTone",
            Self::FileMarkdownTwoTone => "FileMarkdownTwoTone",
            Self::FilePdfTwoTone => "FilePdfTwoTone",
            Self::FilePptTwoTone => "FilePptTwoTone",
            Self::FileTextTwoTone => "FileTextTwoTone",
            Self::FileUnknownTwoTone => "FileUnknownTwoTone",
            Self::FileWordTwoTone => "FileWordTwoTone",
            Self::FileZipTwoTone => "FileZipTwoTone",
            Self::FileTwoTone => "FileTwoTone",
            Self::FilterTwoTone => "FilterTwoTone",
            Self::FireTwoTone => "FireTwoTone",
            Self::FlagTwoTone => "FlagTwoTone",
            Self::FolderAddTwoTone => "FolderAddTwoTone",
            Self::FolderOpenTwoTone => "FolderOpenTwoTone",
            Self::FolderTwoTone => "FolderTwoTone",
            Self::FrownTwoTone => "FrownTwoTone",
            Self::FundTwoTone => "FundTwoTone",
            Self::FunnelPlotTwoTone => "FunnelPlotTwoTone",
            Self::GiftTwoTone => "GiftTwoTone",
            Self::GoldTwoTone => "GoldTwoTone",
            Self::HddTwoTone => "HddTwoTone",
            Self::HeartTwoTone => "HeartTwoTone",
            Self::HighlightTwoTone => "HighlightTwoTone",
            Self::HomeTwoTone => "HomeTwoTone",
            Self::HourglassTwoTone => "HourglassTwoTone",
            Self::Html5TwoTone => "Html5TwoTone",
            Self::IdcardTwoTone => "IdcardTwoTone",
            Self::InfoCircleTwoTone => "InfoCircleTwoTone",
            Self::InsuranceTwoTone => "InsuranceTwoTone",
            Self::InteractionTwoTone => "InteractionTwoTone",
            Self::LayoutTwoTone => "LayoutTwoTone",
            Self::LeftCircleTwoTone => "LeftCircleTwoTone",
            Self::LeftSquareTwoTone => "LeftSquareTwoTone",
            Self::LikeTwoTone => "LikeTwoTone",
            Self::LockTwoTone => "LockTwoTone",
            Self::MailTwoTone => "MailTwoTone",
            Self::MedicineBoxTwoTone => "MedicineBoxTwoTone",
            Self::MehTwoTone => "MehTwoTone",
            Self::MessageTwoTone => "MessageTwoTone",
            Self::MinusCircleTwoTone => "MinusCircleTwoTone",
            Self::MinusSquareTwoTone => "MinusSquareTwoTone",
            Self::MobileTwoTone => "MobileTwoTone",
            Self::MoneyCollectTwoTone => "MoneyCollectTwoTone",
            Self::NotificationTwoTone => "NotificationTwoTone",
            Self::PauseCircleTwoTone => "PauseCircleTwoTone",
            Self::PhoneTwoTone => "PhoneTwoTone",
            Self::PictureTwoTone => "PictureTwoTone",
            Self::PieChartTwoTone => "PieChartTwoTone",
            Self::PlayCircleTwoTone => "PlayCircleTwoTone",
            Self::PlaySquareTwoTone => "PlaySquareTwoTone",
            Self::PlusCircleTwoTone => "PlusCircleTwoTone",
            Self::PlusSquareTwoTone => "PlusSquareTwoTone",
            Self::PoundCircleTwoTone => "PoundCircleTwoTone",
            Self::PrinterTwoTone => "PrinterTwoTone",
            Self::ProfileTwoTone => "ProfileTwoTone",
            Self::ProjectTwoTone => "ProjectTwoTone",
            Self::PropertySafetyTwoTone => "PropertySafetyTwoTone",
            Self::PushpinTwoTone => "PushpinTwoTone",
            Self::QuestionCircleTwoTone => "QuestionCircleTwoTone",
            Self::ReconciliationTwoTone => "ReconciliationTwoTone",
            Self::RedEnvelopeTwoTone => "RedEnvelopeTwoTone",
            Self::RestTwoTone => "RestTwoTone",
            Self::RightCircleTwoTone => "RightCircleTwoTone",
            Self::RightSquareTwoTone => "RightSquareTwoTone",
            Self::RocketTwoTone => "RocketTwoTone",
            Self::SafetyCertificateTwoTone => "SafetyCertificateTwoTone",
            Self::SaveTwoTone => "SaveTwoTone",
            Self::ScheduleTwoTone => "ScheduleTwoTone",
            Self::SecurityScanTwoTone => "SecurityScanTwoTone",
            Self::SettingTwoTone => "SettingTwoTone",
            Self::ShopTwoTone => "ShopTwoTone",
            Self::ShoppingTwoTone => "ShoppingTwoTone",
            Self::SkinTwoTone => "SkinTwoTone",
            Self::SlidersTwoTone => "SlidersTwoTone",
            Self::SmileTwoTone => "SmileTwoTone",
            Self::SnippetsTwoTone => "SnippetsTwoTone",
            Self::SoundTwoTone => "SoundTwoTone",
            Self::StarTwoTone => "StarTwoTone",
            Self::StopTwoTone => "StopTwoTone",
            Self::SwitcherTwoTone => "SwitcherTwoTone",
            Self::TabletTwoTone => "TabletTwoTone",
            Self::TagTwoTone => "TagTwoTone",
            Self::TagsTwoTone => "TagsTwoTone",
            Self::ThunderboltTwoTone => "ThunderboltTwoTone",
            Self::ToolTwoTone => "ToolTwoTone",
            Self::TrademarkCircleTwoTone => "TrademarkCircleTwoTone",
            Self::TrophyTwoTone => "TrophyTwoTone",
            Self::UnlockTwoTone => "UnlockTwoTone",
            Self::UpCircleTwoTone => "UpCircleTwoTone",
            Self::UpSquareTwoTone => "UpSquareTwoTone",
            Self::UsbTwoTone => "UsbTwoTone",
            Self::VideoCameraTwoTone => "VideoCameraTwoTone",
            Self::WalletTwoTone => "WalletTwoTone",
            Self::WarningTwoTone => "WarningTwoTone",
        }
    }

    pub const fn style(self) -> &'static str {
        match self {
            Self::AccountBookOutlined
            | Self::AimOutlined
            | Self::AlertOutlined
            | Self::AlibabaOutlined
            | Self::AlignCenterOutlined
            | Self::AlignLeftOutlined
            | Self::AlignRightOutlined
            | Self::AlipayCircleOutlined
            | Self::AlipayOutlined
            | Self::AliwangwangOutlined
            | Self::AliyunOutlined
            | Self::AmazonOutlined
            | Self::AndroidOutlined
            | Self::AntCloudOutlined
            | Self::AntDesignOutlined
            | Self::ApartmentOutlined
            | Self::ApiOutlined
            | Self::AppleOutlined
            | Self::AppstoreAddOutlined
            | Self::AppstoreOutlined
            | Self::AreaChartOutlined
            | Self::ArrowDownOutlined
            | Self::ArrowLeftOutlined
            | Self::ArrowRightOutlined
            | Self::ArrowUpOutlined
            | Self::ArrowsAltOutlined
            | Self::AudioMutedOutlined
            | Self::AudioOutlined
            | Self::AuditOutlined
            | Self::BackwardOutlined
            | Self::BaiduOutlined
            | Self::BankOutlined
            | Self::BarChartOutlined
            | Self::BarcodeOutlined
            | Self::BarsOutlined
            | Self::BehanceSquareOutlined
            | Self::BehanceOutlined
            | Self::BellOutlined
            | Self::BgColorsOutlined
            | Self::BilibiliOutlined
            | Self::BlockOutlined
            | Self::BoldOutlined
            | Self::BookOutlined
            | Self::BorderBottomOutlined
            | Self::BorderHorizontalOutlined
            | Self::BorderInnerOutlined
            | Self::BorderLeftOutlined
            | Self::BorderOuterOutlined
            | Self::BorderRightOutlined
            | Self::BorderTopOutlined
            | Self::BorderVerticleOutlined
            | Self::BorderOutlined
            | Self::BorderlessTableOutlined
            | Self::BoxPlotOutlined
            | Self::BranchesOutlined
            | Self::BugOutlined
            | Self::BuildOutlined
            | Self::BulbOutlined
            | Self::CalculatorOutlined
            | Self::CalendarOutlined
            | Self::CameraOutlined
            | Self::CarOutlined
            | Self::CaretDownOutlined
            | Self::CaretLeftOutlined
            | Self::CaretRightOutlined
            | Self::CaretUpOutlined
            | Self::CarryOutOutlined
            | Self::CheckCircleOutlined
            | Self::CheckSquareOutlined
            | Self::CheckOutlined
            | Self::ChromeOutlined
            | Self::CiCircleOutlined
            | Self::CiOutlined
            | Self::ClearOutlined
            | Self::ClockCircleOutlined
            | Self::CloseCircleOutlined
            | Self::CloseSquareOutlined
            | Self::CloseOutlined
            | Self::CloudDownloadOutlined
            | Self::CloudServerOutlined
            | Self::CloudSyncOutlined
            | Self::CloudUploadOutlined
            | Self::CloudOutlined
            | Self::ClusterOutlined
            | Self::CodeSandboxOutlined
            | Self::CodeOutlined
            | Self::CodepenCircleOutlined
            | Self::CodepenOutlined
            | Self::CoffeeOutlined
            | Self::ColumnHeightOutlined
            | Self::ColumnWidthOutlined
            | Self::CommentOutlined
            | Self::CompassOutlined
            | Self::CompressOutlined
            | Self::ConsoleSqlOutlined
            | Self::ContactsOutlined
            | Self::ContainerOutlined
            | Self::ControlOutlined
            | Self::CopyOutlined
            | Self::CopyrightCircleOutlined
            | Self::CopyrightOutlined
            | Self::CreditCardOutlined
            | Self::CrownOutlined
            | Self::CustomerServiceOutlined
            | Self::DashOutlined
            | Self::DashboardOutlined
            | Self::DatabaseOutlined
            | Self::DeleteColumnOutlined
            | Self::DeleteRowOutlined
            | Self::DeleteOutlined
            | Self::DeliveredProcedureOutlined
            | Self::DeploymentUnitOutlined
            | Self::DesktopOutlined
            | Self::DiffOutlined
            | Self::DingdingOutlined
            | Self::DingtalkOutlined
            | Self::DisconnectOutlined
            | Self::DiscordOutlined
            | Self::DislikeOutlined
            | Self::DockerOutlined
            | Self::DollarCircleOutlined
            | Self::DollarOutlined
            | Self::DotChartOutlined
            | Self::DotNetOutlined
            | Self::DoubleLeftOutlined
            | Self::DoubleRightOutlined
            | Self::DownCircleOutlined
            | Self::DownSquareOutlined
            | Self::DownOutlined
            | Self::DownloadOutlined
            | Self::DragOutlined
            | Self::DribbbleSquareOutlined
            | Self::DribbbleOutlined
            | Self::DropboxOutlined
            | Self::EditOutlined
            | Self::EllipsisOutlined
            | Self::EnterOutlined
            | Self::EnvironmentOutlined
            | Self::EuroCircleOutlined
            | Self::EuroOutlined
            | Self::ExceptionOutlined
            | Self::ExclamationCircleOutlined
            | Self::ExclamationOutlined
            | Self::ExpandAltOutlined
            | Self::ExpandOutlined
            | Self::ExperimentOutlined
            | Self::ExportOutlined
            | Self::EyeInvisibleOutlined
            | Self::EyeOutlined
            | Self::FacebookOutlined
            | Self::FallOutlined
            | Self::FastBackwardOutlined
            | Self::FastForwardOutlined
            | Self::FieldBinaryOutlined
            | Self::FieldNumberOutlined
            | Self::FieldStringOutlined
            | Self::FieldTimeOutlined
            | Self::FileAddOutlined
            | Self::FileDoneOutlined
            | Self::FileExcelOutlined
            | Self::FileExclamationOutlined
            | Self::FileGifOutlined
            | Self::FileImageOutlined
            | Self::FileJpgOutlined
            | Self::FileMarkdownOutlined
            | Self::FilePdfOutlined
            | Self::FilePptOutlined
            | Self::FileProtectOutlined
            | Self::FileSearchOutlined
            | Self::FileSyncOutlined
            | Self::FileTextOutlined
            | Self::FileUnknownOutlined
            | Self::FileWordOutlined
            | Self::FileZipOutlined
            | Self::FileOutlined
            | Self::FilterOutlined
            | Self::FireOutlined
            | Self::FlagOutlined
            | Self::FolderAddOutlined
            | Self::FolderOpenOutlined
            | Self::FolderViewOutlined
            | Self::FolderOutlined
            | Self::FontColorsOutlined
            | Self::FontSizeOutlined
            | Self::ForkOutlined
            | Self::FormOutlined
            | Self::FormatPainterOutlined
            | Self::ForwardOutlined
            | Self::FrownOutlined
            | Self::FullscreenExitOutlined
            | Self::FullscreenOutlined
            | Self::FunctionOutlined
            | Self::FundProjectionScreenOutlined
            | Self::FundViewOutlined
            | Self::FundOutlined
            | Self::FunnelPlotOutlined
            | Self::GatewayOutlined
            | Self::GifOutlined
            | Self::GiftOutlined
            | Self::GithubOutlined
            | Self::GitlabOutlined
            | Self::GlobalOutlined
            | Self::GoldOutlined
            | Self::GooglePlusOutlined
            | Self::GoogleOutlined
            | Self::GroupOutlined
            | Self::HarmonyOSOutlined
            | Self::HddOutlined
            | Self::HeartOutlined
            | Self::HeatMapOutlined
            | Self::HighlightOutlined
            | Self::HistoryOutlined
            | Self::HolderOutlined
            | Self::HomeOutlined
            | Self::HourglassOutlined
            | Self::Html5Outlined
            | Self::IdcardOutlined
            | Self::IeOutlined
            | Self::ImportOutlined
            | Self::InboxOutlined
            | Self::InfoCircleOutlined
            | Self::InfoOutlined
            | Self::InsertRowAboveOutlined
            | Self::InsertRowBelowOutlined
            | Self::InsertRowLeftOutlined
            | Self::InsertRowRightOutlined
            | Self::InstagramOutlined
            | Self::InsuranceOutlined
            | Self::InteractionOutlined
            | Self::IssuesCloseOutlined
            | Self::ItalicOutlined
            | Self::JavaScriptOutlined
            | Self::JavaOutlined
            | Self::KeyOutlined
            | Self::KubernetesOutlined
            | Self::LaptopOutlined
            | Self::LayoutOutlined
            | Self::LeftCircleOutlined
            | Self::LeftSquareOutlined
            | Self::LeftOutlined
            | Self::LikeOutlined
            | Self::LineChartOutlined
            | Self::LineHeightOutlined
            | Self::LineOutlined
            | Self::LinkOutlined
            | Self::LinkedinOutlined
            | Self::LinuxOutlined
            | Self::Loading3QuartersOutlined
            | Self::LoadingOutlined
            | Self::LockOutlined
            | Self::LoginOutlined
            | Self::LogoutOutlined
            | Self::MacCommandOutlined
            | Self::MailOutlined
            | Self::ManOutlined
            | Self::MedicineBoxOutlined
            | Self::MediumWorkmarkOutlined
            | Self::MediumOutlined
            | Self::MehOutlined
            | Self::MenuFoldOutlined
            | Self::MenuUnfoldOutlined
            | Self::MenuOutlined
            | Self::MergeCellsOutlined
            | Self::MergeOutlined
            | Self::MessageOutlined
            | Self::MinusCircleOutlined
            | Self::MinusSquareOutlined
            | Self::MinusOutlined
            | Self::MobileOutlined
            | Self::MoneyCollectOutlined
            | Self::MonitorOutlined
            | Self::MoonOutlined
            | Self::MoreOutlined
            | Self::MutedOutlined
            | Self::NodeCollapseOutlined
            | Self::NodeExpandOutlined
            | Self::NodeIndexOutlined
            | Self::NotificationOutlined
            | Self::NumberOutlined
            | Self::OneToOneOutlined
            | Self::OpenAIOutlined
            | Self::OrderedListOutlined
            | Self::PaperClipOutlined
            | Self::PartitionOutlined
            | Self::PauseCircleOutlined
            | Self::PauseOutlined
            | Self::PayCircleOutlined
            | Self::PercentageOutlined
            | Self::PhoneOutlined
            | Self::PicCenterOutlined
            | Self::PicLeftOutlined
            | Self::PicRightOutlined
            | Self::PictureOutlined
            | Self::PieChartOutlined
            | Self::PinterestOutlined
            | Self::PlayCircleOutlined
            | Self::PlaySquareOutlined
            | Self::PlusCircleOutlined
            | Self::PlusSquareOutlined
            | Self::PlusOutlined
            | Self::PoundCircleOutlined
            | Self::PoundOutlined
            | Self::PoweroffOutlined
            | Self::PrinterOutlined
            | Self::ProductOutlined
            | Self::ProfileOutlined
            | Self::ProjectOutlined
            | Self::PropertySafetyOutlined
            | Self::PullRequestOutlined
            | Self::PushpinOutlined
            | Self::PythonOutlined
            | Self::QqOutlined
            | Self::QrcodeOutlined
            | Self::QuestionCircleOutlined
            | Self::QuestionOutlined
            | Self::RadarChartOutlined
            | Self::RadiusBottomleftOutlined
            | Self::RadiusBottomrightOutlined
            | Self::RadiusSettingOutlined
            | Self::RadiusUpleftOutlined
            | Self::RadiusUprightOutlined
            | Self::ReadOutlined
            | Self::ReconciliationOutlined
            | Self::RedEnvelopeOutlined
            | Self::RedditOutlined
            | Self::RedoOutlined
            | Self::ReloadOutlined
            | Self::RestOutlined
            | Self::RetweetOutlined
            | Self::RightCircleOutlined
            | Self::RightSquareOutlined
            | Self::RightOutlined
            | Self::RiseOutlined
            | Self::RobotOutlined
            | Self::RocketOutlined
            | Self::RollbackOutlined
            | Self::RotateLeftOutlined
            | Self::RotateRightOutlined
            | Self::RubyOutlined
            | Self::SafetyCertificateOutlined
            | Self::SafetyOutlined
            | Self::SaveOutlined
            | Self::ScanOutlined
            | Self::ScheduleOutlined
            | Self::ScissorOutlined
            | Self::SearchOutlined
            | Self::SecurityScanOutlined
            | Self::SelectOutlined
            | Self::SendOutlined
            | Self::SettingOutlined
            | Self::ShakeOutlined
            | Self::ShareAltOutlined
            | Self::ShopOutlined
            | Self::ShoppingCartOutlined
            | Self::ShoppingOutlined
            | Self::ShrinkOutlined
            | Self::SignatureOutlined
            | Self::SisternodeOutlined
            | Self::SketchOutlined
            | Self::SkinOutlined
            | Self::SkypeOutlined
            | Self::SlackSquareOutlined
            | Self::SlackOutlined
            | Self::SlidersOutlined
            | Self::SmallDashOutlined
            | Self::SmileOutlined
            | Self::SnippetsOutlined
            | Self::SolutionOutlined
            | Self::SortAscendingOutlined
            | Self::SortDescendingOutlined
            | Self::SoundOutlined
            | Self::SplitCellsOutlined
            | Self::SpotifyOutlined
            | Self::StarOutlined
            | Self::StepBackwardOutlined
            | Self::StepForwardOutlined
            | Self::StockOutlined
            | Self::StopOutlined
            | Self::StrikethroughOutlined
            | Self::SubnodeOutlined
            | Self::SunOutlined
            | Self::SwapLeftOutlined
            | Self::SwapRightOutlined
            | Self::SwapOutlined
            | Self::SwitcherOutlined
            | Self::SyncOutlined
            | Self::TableOutlined
            | Self::TabletOutlined
            | Self::TagOutlined
            | Self::TagsOutlined
            | Self::TaobaoCircleOutlined
            | Self::TaobaoOutlined
            | Self::TeamOutlined
            | Self::ThunderboltOutlined
            | Self::TikTokOutlined
            | Self::ToTopOutlined
            | Self::ToolOutlined
            | Self::TrademarkCircleOutlined
            | Self::TrademarkOutlined
            | Self::TransactionOutlined
            | Self::TranslationOutlined
            | Self::TrophyOutlined
            | Self::TruckOutlined
            | Self::TwitchOutlined
            | Self::TwitterOutlined
            | Self::UnderlineOutlined
            | Self::UndoOutlined
            | Self::UngroupOutlined
            | Self::UnlockOutlined
            | Self::UnorderedListOutlined
            | Self::UpCircleOutlined
            | Self::UpSquareOutlined
            | Self::UpOutlined
            | Self::UploadOutlined
            | Self::UsbOutlined
            | Self::UserAddOutlined
            | Self::UserDeleteOutlined
            | Self::UserSwitchOutlined
            | Self::UserOutlined
            | Self::UsergroupAddOutlined
            | Self::UsergroupDeleteOutlined
            | Self::VerifiedOutlined
            | Self::VerticalAlignBottomOutlined
            | Self::VerticalAlignMiddleOutlined
            | Self::VerticalAlignTopOutlined
            | Self::VerticalLeftOutlined
            | Self::VerticalRightOutlined
            | Self::VideoCameraAddOutlined
            | Self::VideoCameraOutlined
            | Self::WalletOutlined
            | Self::WarningOutlined
            | Self::WechatWorkOutlined
            | Self::WechatOutlined
            | Self::WeiboCircleOutlined
            | Self::WeiboSquareOutlined
            | Self::WeiboOutlined
            | Self::WhatsAppOutlined
            | Self::WifiOutlined
            | Self::WindowsOutlined
            | Self::WomanOutlined
            | Self::XOutlined
            | Self::YahooOutlined
            | Self::YoutubeOutlined
            | Self::YuqueOutlined
            | Self::ZhihuOutlined
            | Self::ZoomInOutlined
            | Self::ZoomOutOutlined => "outlined",
            Self::AccountBookFilled
            | Self::AlertFilled
            | Self::AlipayCircleFilled
            | Self::AlipaySquareFilled
            | Self::AliwangwangFilled
            | Self::AmazonCircleFilled
            | Self::AmazonSquareFilled
            | Self::AndroidFilled
            | Self::AnthropicFilled
            | Self::ApiFilled
            | Self::AppleFilled
            | Self::AppstoreFilled
            | Self::AudioFilled
            | Self::BackwardFilled
            | Self::BankFilled
            | Self::BehanceCircleFilled
            | Self::BehanceSquareFilled
            | Self::BellFilled
            | Self::BilibiliFilled
            | Self::BookFilled
            | Self::BoxPlotFilled
            | Self::BugFilled
            | Self::BuildFilled
            | Self::BulbFilled
            | Self::CalculatorFilled
            | Self::CalendarFilled
            | Self::CameraFilled
            | Self::CarFilled
            | Self::CaretDownFilled
            | Self::CaretLeftFilled
            | Self::CaretRightFilled
            | Self::CaretUpFilled
            | Self::CarryOutFilled
            | Self::CheckCircleFilled
            | Self::CheckSquareFilled
            | Self::ChromeFilled
            | Self::CiCircleFilled
            | Self::ClaudeFilled
            | Self::ClockCircleFilled
            | Self::CloseCircleFilled
            | Self::CloseSquareFilled
            | Self::CloudFilled
            | Self::CodeSandboxCircleFilled
            | Self::CodeSandboxSquareFilled
            | Self::CodeFilled
            | Self::CodepenCircleFilled
            | Self::CodepenSquareFilled
            | Self::CompassFilled
            | Self::ContactsFilled
            | Self::ContainerFilled
            | Self::ControlFilled
            | Self::CopyFilled
            | Self::CopyrightCircleFilled
            | Self::CreditCardFilled
            | Self::CrownFilled
            | Self::CustomerServiceFilled
            | Self::DashboardFilled
            | Self::DatabaseFilled
            | Self::DeepSeekFilled
            | Self::DeleteFilled
            | Self::DiffFilled
            | Self::DingtalkCircleFilled
            | Self::DingtalkSquareFilled
            | Self::DiscordFilled
            | Self::DislikeFilled
            | Self::DollarCircleFilled
            | Self::DownCircleFilled
            | Self::DownSquareFilled
            | Self::DribbbleCircleFilled
            | Self::DribbbleSquareFilled
            | Self::DropboxCircleFilled
            | Self::DropboxSquareFilled
            | Self::EditFilled
            | Self::ElevenLabsFilled
            | Self::EnvironmentFilled
            | Self::EuroCircleFilled
            | Self::ExclamationCircleFilled
            | Self::ExperimentFilled
            | Self::EyeInvisibleFilled
            | Self::EyeFilled
            | Self::FacebookFilled
            | Self::FastBackwardFilled
            | Self::FastForwardFilled
            | Self::FileAddFilled
            | Self::FileExcelFilled
            | Self::FileExclamationFilled
            | Self::FileImageFilled
            | Self::FileMarkdownFilled
            | Self::FilePdfFilled
            | Self::FilePptFilled
            | Self::FileTextFilled
            | Self::FileUnknownFilled
            | Self::FileWordFilled
            | Self::FileZipFilled
            | Self::FileFilled
            | Self::FilterFilled
            | Self::FireFilled
            | Self::FlagFilled
            | Self::FolderAddFilled
            | Self::FolderOpenFilled
            | Self::FolderFilled
            | Self::FormatPainterFilled
            | Self::ForwardFilled
            | Self::FrownFilled
            | Self::FundFilled
            | Self::FunnelPlotFilled
            | Self::GeminiFilled
            | Self::GiftFilled
            | Self::GithubFilled
            | Self::GitlabFilled
            | Self::GoldFilled
            | Self::GoldenFilled
            | Self::GoogleCircleFilled
            | Self::GooglePlusCircleFilled
            | Self::GooglePlusSquareFilled
            | Self::GoogleSquareFilled
            | Self::HddFilled
            | Self::HeartFilled
            | Self::HighlightFilled
            | Self::HomeFilled
            | Self::HourglassFilled
            | Self::Html5Filled
            | Self::HuggingFaceFilled
            | Self::IdcardFilled
            | Self::IeCircleFilled
            | Self::IeSquareFilled
            | Self::InfoCircleFilled
            | Self::InstagramFilled
            | Self::InsuranceFilled
            | Self::InteractionFilled
            | Self::LayoutFilled
            | Self::LeftCircleFilled
            | Self::LeftSquareFilled
            | Self::LikeFilled
            | Self::LinkedinFilled
            | Self::LockFilled
            | Self::MacCommandFilled
            | Self::MailFilled
            | Self::MastodonFilled
            | Self::MedicineBoxFilled
            | Self::MediumCircleFilled
            | Self::MediumSquareFilled
            | Self::MehFilled
            | Self::MergeFilled
            | Self::MessageFilled
            | Self::MetaFilled
            | Self::MinusCircleFilled
            | Self::MinusSquareFilled
            | Self::MistralFilled
            | Self::MobileFilled
            | Self::MoneyCollectFilled
            | Self::MoonFilled
            | Self::MutedFilled
            | Self::NetflixFilled
            | Self::NotificationFilled
            | Self::OllamaFilled
            | Self::OpenAIFilled
            | Self::PauseCircleFilled
            | Self::PayCircleFilled
            | Self::PerplexityFilled
            | Self::PhoneFilled
            | Self::PictureFilled
            | Self::PieChartFilled
            | Self::PinterestFilled
            | Self::PlayCircleFilled
            | Self::PlaySquareFilled
            | Self::PlusCircleFilled
            | Self::PlusSquareFilled
            | Self::PoundCircleFilled
            | Self::PrinterFilled
            | Self::ProductFilled
            | Self::ProfileFilled
            | Self::ProjectFilled
            | Self::PropertySafetyFilled
            | Self::PushpinFilled
            | Self::QqCircleFilled
            | Self::QqSquareFilled
            | Self::QuestionCircleFilled
            | Self::QwenFilled
            | Self::ReadFilled
            | Self::ReconciliationFilled
            | Self::RedEnvelopeFilled
            | Self::RedditCircleFilled
            | Self::RedditSquareFilled
            | Self::ReplicateFilled
            | Self::RestFilled
            | Self::RightCircleFilled
            | Self::RightSquareFilled
            | Self::RobotFilled
            | Self::RocketFilled
            | Self::SafetyCertificateFilled
            | Self::SaveFilled
            | Self::ScheduleFilled
            | Self::SecurityScanFilled
            | Self::SettingFilled
            | Self::ShopFilled
            | Self::ShoppingFilled
            | Self::SignalFilled
            | Self::SignatureFilled
            | Self::SketchCircleFilled
            | Self::SketchSquareFilled
            | Self::SkinFilled
            | Self::SkypeFilled
            | Self::SlackCircleFilled
            | Self::SlackSquareFilled
            | Self::SlidersFilled
            | Self::SmileFilled
            | Self::SnapchatFilled
            | Self::SnippetsFilled
            | Self::SoundFilled
            | Self::SpotifyFilled
            | Self::StarFilled
            | Self::StepBackwardFilled
            | Self::StepForwardFilled
            | Self::StopFilled
            | Self::SunFilled
            | Self::SwitcherFilled
            | Self::TabletFilled
            | Self::TagFilled
            | Self::TagsFilled
            | Self::TaobaoCircleFilled
            | Self::TaobaoSquareFilled
            | Self::TelegramFilled
            | Self::ThreadsFilled
            | Self::ThunderboltFilled
            | Self::TikTokFilled
            | Self::ToolFilled
            | Self::TrademarkCircleFilled
            | Self::TrophyFilled
            | Self::TruckFilled
            | Self::TwitchFilled
            | Self::TwitterCircleFilled
            | Self::TwitterSquareFilled
            | Self::UnlockFilled
            | Self::UpCircleFilled
            | Self::UpSquareFilled
            | Self::UsbFilled
            | Self::VideoCameraFilled
            | Self::WalletFilled
            | Self::WarningFilled
            | Self::WechatWorkFilled
            | Self::WechatFilled
            | Self::WeiboCircleFilled
            | Self::WeiboSquareFilled
            | Self::WindowsFilled
            | Self::XFilled
            | Self::YahooFilled
            | Self::YoutubeFilled
            | Self::YuqueFilled
            | Self::ZhihuCircleFilled
            | Self::ZhihuSquareFilled => "filled",
            Self::AccountBookTwoTone
            | Self::AlertTwoTone
            | Self::ApiTwoTone
            | Self::AppstoreTwoTone
            | Self::AudioTwoTone
            | Self::BankTwoTone
            | Self::BellTwoTone
            | Self::BookTwoTone
            | Self::BoxPlotTwoTone
            | Self::BugTwoTone
            | Self::BuildTwoTone
            | Self::BulbTwoTone
            | Self::CalculatorTwoTone
            | Self::CalendarTwoTone
            | Self::CameraTwoTone
            | Self::CarTwoTone
            | Self::CarryOutTwoTone
            | Self::CheckCircleTwoTone
            | Self::CheckSquareTwoTone
            | Self::CiCircleTwoTone
            | Self::CiTwoTone
            | Self::ClockCircleTwoTone
            | Self::CloseCircleTwoTone
            | Self::CloseSquareTwoTone
            | Self::CloudTwoTone
            | Self::CodeTwoTone
            | Self::CompassTwoTone
            | Self::ContactsTwoTone
            | Self::ContainerTwoTone
            | Self::ControlTwoTone
            | Self::CopyTwoTone
            | Self::CopyrightCircleTwoTone
            | Self::CopyrightTwoTone
            | Self::CreditCardTwoTone
            | Self::CrownTwoTone
            | Self::CustomerServiceTwoTone
            | Self::DashboardTwoTone
            | Self::DatabaseTwoTone
            | Self::DeleteTwoTone
            | Self::DiffTwoTone
            | Self::DislikeTwoTone
            | Self::DollarCircleTwoTone
            | Self::DollarTwoTone
            | Self::DownCircleTwoTone
            | Self::DownSquareTwoTone
            | Self::EditTwoTone
            | Self::EnvironmentTwoTone
            | Self::EuroCircleTwoTone
            | Self::EuroTwoTone
            | Self::ExclamationCircleTwoTone
            | Self::ExperimentTwoTone
            | Self::EyeInvisibleTwoTone
            | Self::EyeTwoTone
            | Self::FileAddTwoTone
            | Self::FileExcelTwoTone
            | Self::FileExclamationTwoTone
            | Self::FileImageTwoTone
            | Self::FileMarkdownTwoTone
            | Self::FilePdfTwoTone
            | Self::FilePptTwoTone
            | Self::FileTextTwoTone
            | Self::FileUnknownTwoTone
            | Self::FileWordTwoTone
            | Self::FileZipTwoTone
            | Self::FileTwoTone
            | Self::FilterTwoTone
            | Self::FireTwoTone
            | Self::FlagTwoTone
            | Self::FolderAddTwoTone
            | Self::FolderOpenTwoTone
            | Self::FolderTwoTone
            | Self::FrownTwoTone
            | Self::FundTwoTone
            | Self::FunnelPlotTwoTone
            | Self::GiftTwoTone
            | Self::GoldTwoTone
            | Self::HddTwoTone
            | Self::HeartTwoTone
            | Self::HighlightTwoTone
            | Self::HomeTwoTone
            | Self::HourglassTwoTone
            | Self::Html5TwoTone
            | Self::IdcardTwoTone
            | Self::InfoCircleTwoTone
            | Self::InsuranceTwoTone
            | Self::InteractionTwoTone
            | Self::LayoutTwoTone
            | Self::LeftCircleTwoTone
            | Self::LeftSquareTwoTone
            | Self::LikeTwoTone
            | Self::LockTwoTone
            | Self::MailTwoTone
            | Self::MedicineBoxTwoTone
            | Self::MehTwoTone
            | Self::MessageTwoTone
            | Self::MinusCircleTwoTone
            | Self::MinusSquareTwoTone
            | Self::MobileTwoTone
            | Self::MoneyCollectTwoTone
            | Self::NotificationTwoTone
            | Self::PauseCircleTwoTone
            | Self::PhoneTwoTone
            | Self::PictureTwoTone
            | Self::PieChartTwoTone
            | Self::PlayCircleTwoTone
            | Self::PlaySquareTwoTone
            | Self::PlusCircleTwoTone
            | Self::PlusSquareTwoTone
            | Self::PoundCircleTwoTone
            | Self::PrinterTwoTone
            | Self::ProfileTwoTone
            | Self::ProjectTwoTone
            | Self::PropertySafetyTwoTone
            | Self::PushpinTwoTone
            | Self::QuestionCircleTwoTone
            | Self::ReconciliationTwoTone
            | Self::RedEnvelopeTwoTone
            | Self::RestTwoTone
            | Self::RightCircleTwoTone
            | Self::RightSquareTwoTone
            | Self::RocketTwoTone
            | Self::SafetyCertificateTwoTone
            | Self::SaveTwoTone
            | Self::ScheduleTwoTone
            | Self::SecurityScanTwoTone
            | Self::SettingTwoTone
            | Self::ShopTwoTone
            | Self::ShoppingTwoTone
            | Self::SkinTwoTone
            | Self::SlidersTwoTone
            | Self::SmileTwoTone
            | Self::SnippetsTwoTone
            | Self::SoundTwoTone
            | Self::StarTwoTone
            | Self::StopTwoTone
            | Self::SwitcherTwoTone
            | Self::TabletTwoTone
            | Self::TagTwoTone
            | Self::TagsTwoTone
            | Self::ThunderboltTwoTone
            | Self::ToolTwoTone
            | Self::TrademarkCircleTwoTone
            | Self::TrophyTwoTone
            | Self::UnlockTwoTone
            | Self::UpCircleTwoTone
            | Self::UpSquareTwoTone
            | Self::UsbTwoTone
            | Self::VideoCameraTwoTone
            | Self::WalletTwoTone
            | Self::WarningTwoTone => "twotone",
        }
    }

    /// Resolve the Rust enum by official name, such as SearchOutlined.
    pub fn from_ant_name(name: &str) -> Option<Self> {
        match name {
            "AccountBookOutlined" => Some(Self::AccountBookOutlined),
            "AimOutlined" => Some(Self::AimOutlined),
            "AlertOutlined" => Some(Self::AlertOutlined),
            "AlibabaOutlined" => Some(Self::AlibabaOutlined),
            "AlignCenterOutlined" => Some(Self::AlignCenterOutlined),
            "AlignLeftOutlined" => Some(Self::AlignLeftOutlined),
            "AlignRightOutlined" => Some(Self::AlignRightOutlined),
            "AlipayCircleOutlined" => Some(Self::AlipayCircleOutlined),
            "AlipayOutlined" => Some(Self::AlipayOutlined),
            "AliwangwangOutlined" => Some(Self::AliwangwangOutlined),
            "AliyunOutlined" => Some(Self::AliyunOutlined),
            "AmazonOutlined" => Some(Self::AmazonOutlined),
            "AndroidOutlined" => Some(Self::AndroidOutlined),
            "AntCloudOutlined" => Some(Self::AntCloudOutlined),
            "AntDesignOutlined" => Some(Self::AntDesignOutlined),
            "ApartmentOutlined" => Some(Self::ApartmentOutlined),
            "ApiOutlined" => Some(Self::ApiOutlined),
            "AppleOutlined" => Some(Self::AppleOutlined),
            "AppstoreAddOutlined" => Some(Self::AppstoreAddOutlined),
            "AppstoreOutlined" => Some(Self::AppstoreOutlined),
            "AreaChartOutlined" => Some(Self::AreaChartOutlined),
            "ArrowDownOutlined" => Some(Self::ArrowDownOutlined),
            "ArrowLeftOutlined" => Some(Self::ArrowLeftOutlined),
            "ArrowRightOutlined" => Some(Self::ArrowRightOutlined),
            "ArrowUpOutlined" => Some(Self::ArrowUpOutlined),
            "ArrowsAltOutlined" => Some(Self::ArrowsAltOutlined),
            "AudioMutedOutlined" => Some(Self::AudioMutedOutlined),
            "AudioOutlined" => Some(Self::AudioOutlined),
            "AuditOutlined" => Some(Self::AuditOutlined),
            "BackwardOutlined" => Some(Self::BackwardOutlined),
            "BaiduOutlined" => Some(Self::BaiduOutlined),
            "BankOutlined" => Some(Self::BankOutlined),
            "BarChartOutlined" => Some(Self::BarChartOutlined),
            "BarcodeOutlined" => Some(Self::BarcodeOutlined),
            "BarsOutlined" => Some(Self::BarsOutlined),
            "BehanceSquareOutlined" => Some(Self::BehanceSquareOutlined),
            "BehanceOutlined" => Some(Self::BehanceOutlined),
            "BellOutlined" => Some(Self::BellOutlined),
            "BgColorsOutlined" => Some(Self::BgColorsOutlined),
            "BilibiliOutlined" => Some(Self::BilibiliOutlined),
            "BlockOutlined" => Some(Self::BlockOutlined),
            "BoldOutlined" => Some(Self::BoldOutlined),
            "BookOutlined" => Some(Self::BookOutlined),
            "BorderBottomOutlined" => Some(Self::BorderBottomOutlined),
            "BorderHorizontalOutlined" => Some(Self::BorderHorizontalOutlined),
            "BorderInnerOutlined" => Some(Self::BorderInnerOutlined),
            "BorderLeftOutlined" => Some(Self::BorderLeftOutlined),
            "BorderOuterOutlined" => Some(Self::BorderOuterOutlined),
            "BorderRightOutlined" => Some(Self::BorderRightOutlined),
            "BorderTopOutlined" => Some(Self::BorderTopOutlined),
            "BorderVerticleOutlined" => Some(Self::BorderVerticleOutlined),
            "BorderOutlined" => Some(Self::BorderOutlined),
            "BorderlessTableOutlined" => Some(Self::BorderlessTableOutlined),
            "BoxPlotOutlined" => Some(Self::BoxPlotOutlined),
            "BranchesOutlined" => Some(Self::BranchesOutlined),
            "BugOutlined" => Some(Self::BugOutlined),
            "BuildOutlined" => Some(Self::BuildOutlined),
            "BulbOutlined" => Some(Self::BulbOutlined),
            "CalculatorOutlined" => Some(Self::CalculatorOutlined),
            "CalendarOutlined" => Some(Self::CalendarOutlined),
            "CameraOutlined" => Some(Self::CameraOutlined),
            "CarOutlined" => Some(Self::CarOutlined),
            "CaretDownOutlined" => Some(Self::CaretDownOutlined),
            "CaretLeftOutlined" => Some(Self::CaretLeftOutlined),
            "CaretRightOutlined" => Some(Self::CaretRightOutlined),
            "CaretUpOutlined" => Some(Self::CaretUpOutlined),
            "CarryOutOutlined" => Some(Self::CarryOutOutlined),
            "CheckCircleOutlined" => Some(Self::CheckCircleOutlined),
            "CheckSquareOutlined" => Some(Self::CheckSquareOutlined),
            "CheckOutlined" => Some(Self::CheckOutlined),
            "ChromeOutlined" => Some(Self::ChromeOutlined),
            "CiCircleOutlined" => Some(Self::CiCircleOutlined),
            "CiOutlined" => Some(Self::CiOutlined),
            "ClearOutlined" => Some(Self::ClearOutlined),
            "ClockCircleOutlined" => Some(Self::ClockCircleOutlined),
            "CloseCircleOutlined" => Some(Self::CloseCircleOutlined),
            "CloseSquareOutlined" => Some(Self::CloseSquareOutlined),
            "CloseOutlined" => Some(Self::CloseOutlined),
            "CloudDownloadOutlined" => Some(Self::CloudDownloadOutlined),
            "CloudServerOutlined" => Some(Self::CloudServerOutlined),
            "CloudSyncOutlined" => Some(Self::CloudSyncOutlined),
            "CloudUploadOutlined" => Some(Self::CloudUploadOutlined),
            "CloudOutlined" => Some(Self::CloudOutlined),
            "ClusterOutlined" => Some(Self::ClusterOutlined),
            "CodeSandboxOutlined" => Some(Self::CodeSandboxOutlined),
            "CodeOutlined" => Some(Self::CodeOutlined),
            "CodepenCircleOutlined" => Some(Self::CodepenCircleOutlined),
            "CodepenOutlined" => Some(Self::CodepenOutlined),
            "CoffeeOutlined" => Some(Self::CoffeeOutlined),
            "ColumnHeightOutlined" => Some(Self::ColumnHeightOutlined),
            "ColumnWidthOutlined" => Some(Self::ColumnWidthOutlined),
            "CommentOutlined" => Some(Self::CommentOutlined),
            "CompassOutlined" => Some(Self::CompassOutlined),
            "CompressOutlined" => Some(Self::CompressOutlined),
            "ConsoleSqlOutlined" => Some(Self::ConsoleSqlOutlined),
            "ContactsOutlined" => Some(Self::ContactsOutlined),
            "ContainerOutlined" => Some(Self::ContainerOutlined),
            "ControlOutlined" => Some(Self::ControlOutlined),
            "CopyOutlined" => Some(Self::CopyOutlined),
            "CopyrightCircleOutlined" => Some(Self::CopyrightCircleOutlined),
            "CopyrightOutlined" => Some(Self::CopyrightOutlined),
            "CreditCardOutlined" => Some(Self::CreditCardOutlined),
            "CrownOutlined" => Some(Self::CrownOutlined),
            "CustomerServiceOutlined" => Some(Self::CustomerServiceOutlined),
            "DashOutlined" => Some(Self::DashOutlined),
            "DashboardOutlined" => Some(Self::DashboardOutlined),
            "DatabaseOutlined" => Some(Self::DatabaseOutlined),
            "DeleteColumnOutlined" => Some(Self::DeleteColumnOutlined),
            "DeleteRowOutlined" => Some(Self::DeleteRowOutlined),
            "DeleteOutlined" => Some(Self::DeleteOutlined),
            "DeliveredProcedureOutlined" => Some(Self::DeliveredProcedureOutlined),
            "DeploymentUnitOutlined" => Some(Self::DeploymentUnitOutlined),
            "DesktopOutlined" => Some(Self::DesktopOutlined),
            "DiffOutlined" => Some(Self::DiffOutlined),
            "DingdingOutlined" => Some(Self::DingdingOutlined),
            "DingtalkOutlined" => Some(Self::DingtalkOutlined),
            "DisconnectOutlined" => Some(Self::DisconnectOutlined),
            "DiscordOutlined" => Some(Self::DiscordOutlined),
            "DislikeOutlined" => Some(Self::DislikeOutlined),
            "DockerOutlined" => Some(Self::DockerOutlined),
            "DollarCircleOutlined" => Some(Self::DollarCircleOutlined),
            "DollarOutlined" => Some(Self::DollarOutlined),
            "DotChartOutlined" => Some(Self::DotChartOutlined),
            "DotNetOutlined" => Some(Self::DotNetOutlined),
            "DoubleLeftOutlined" => Some(Self::DoubleLeftOutlined),
            "DoubleRightOutlined" => Some(Self::DoubleRightOutlined),
            "DownCircleOutlined" => Some(Self::DownCircleOutlined),
            "DownSquareOutlined" => Some(Self::DownSquareOutlined),
            "DownOutlined" => Some(Self::DownOutlined),
            "DownloadOutlined" => Some(Self::DownloadOutlined),
            "DragOutlined" => Some(Self::DragOutlined),
            "DribbbleSquareOutlined" => Some(Self::DribbbleSquareOutlined),
            "DribbbleOutlined" => Some(Self::DribbbleOutlined),
            "DropboxOutlined" => Some(Self::DropboxOutlined),
            "EditOutlined" => Some(Self::EditOutlined),
            "EllipsisOutlined" => Some(Self::EllipsisOutlined),
            "EnterOutlined" => Some(Self::EnterOutlined),
            "EnvironmentOutlined" => Some(Self::EnvironmentOutlined),
            "EuroCircleOutlined" => Some(Self::EuroCircleOutlined),
            "EuroOutlined" => Some(Self::EuroOutlined),
            "ExceptionOutlined" => Some(Self::ExceptionOutlined),
            "ExclamationCircleOutlined" => Some(Self::ExclamationCircleOutlined),
            "ExclamationOutlined" => Some(Self::ExclamationOutlined),
            "ExpandAltOutlined" => Some(Self::ExpandAltOutlined),
            "ExpandOutlined" => Some(Self::ExpandOutlined),
            "ExperimentOutlined" => Some(Self::ExperimentOutlined),
            "ExportOutlined" => Some(Self::ExportOutlined),
            "EyeInvisibleOutlined" => Some(Self::EyeInvisibleOutlined),
            "EyeOutlined" => Some(Self::EyeOutlined),
            "FacebookOutlined" => Some(Self::FacebookOutlined),
            "FallOutlined" => Some(Self::FallOutlined),
            "FastBackwardOutlined" => Some(Self::FastBackwardOutlined),
            "FastForwardOutlined" => Some(Self::FastForwardOutlined),
            "FieldBinaryOutlined" => Some(Self::FieldBinaryOutlined),
            "FieldNumberOutlined" => Some(Self::FieldNumberOutlined),
            "FieldStringOutlined" => Some(Self::FieldStringOutlined),
            "FieldTimeOutlined" => Some(Self::FieldTimeOutlined),
            "FileAddOutlined" => Some(Self::FileAddOutlined),
            "FileDoneOutlined" => Some(Self::FileDoneOutlined),
            "FileExcelOutlined" => Some(Self::FileExcelOutlined),
            "FileExclamationOutlined" => Some(Self::FileExclamationOutlined),
            "FileGifOutlined" => Some(Self::FileGifOutlined),
            "FileImageOutlined" => Some(Self::FileImageOutlined),
            "FileJpgOutlined" => Some(Self::FileJpgOutlined),
            "FileMarkdownOutlined" => Some(Self::FileMarkdownOutlined),
            "FilePdfOutlined" => Some(Self::FilePdfOutlined),
            "FilePptOutlined" => Some(Self::FilePptOutlined),
            "FileProtectOutlined" => Some(Self::FileProtectOutlined),
            "FileSearchOutlined" => Some(Self::FileSearchOutlined),
            "FileSyncOutlined" => Some(Self::FileSyncOutlined),
            "FileTextOutlined" => Some(Self::FileTextOutlined),
            "FileUnknownOutlined" => Some(Self::FileUnknownOutlined),
            "FileWordOutlined" => Some(Self::FileWordOutlined),
            "FileZipOutlined" => Some(Self::FileZipOutlined),
            "FileOutlined" => Some(Self::FileOutlined),
            "FilterOutlined" => Some(Self::FilterOutlined),
            "FireOutlined" => Some(Self::FireOutlined),
            "FlagOutlined" => Some(Self::FlagOutlined),
            "FolderAddOutlined" => Some(Self::FolderAddOutlined),
            "FolderOpenOutlined" => Some(Self::FolderOpenOutlined),
            "FolderViewOutlined" => Some(Self::FolderViewOutlined),
            "FolderOutlined" => Some(Self::FolderOutlined),
            "FontColorsOutlined" => Some(Self::FontColorsOutlined),
            "FontSizeOutlined" => Some(Self::FontSizeOutlined),
            "ForkOutlined" => Some(Self::ForkOutlined),
            "FormOutlined" => Some(Self::FormOutlined),
            "FormatPainterOutlined" => Some(Self::FormatPainterOutlined),
            "ForwardOutlined" => Some(Self::ForwardOutlined),
            "FrownOutlined" => Some(Self::FrownOutlined),
            "FullscreenExitOutlined" => Some(Self::FullscreenExitOutlined),
            "FullscreenOutlined" => Some(Self::FullscreenOutlined),
            "FunctionOutlined" => Some(Self::FunctionOutlined),
            "FundProjectionScreenOutlined" => Some(Self::FundProjectionScreenOutlined),
            "FundViewOutlined" => Some(Self::FundViewOutlined),
            "FundOutlined" => Some(Self::FundOutlined),
            "FunnelPlotOutlined" => Some(Self::FunnelPlotOutlined),
            "GatewayOutlined" => Some(Self::GatewayOutlined),
            "GifOutlined" => Some(Self::GifOutlined),
            "GiftOutlined" => Some(Self::GiftOutlined),
            "GithubOutlined" => Some(Self::GithubOutlined),
            "GitlabOutlined" => Some(Self::GitlabOutlined),
            "GlobalOutlined" => Some(Self::GlobalOutlined),
            "GoldOutlined" => Some(Self::GoldOutlined),
            "GooglePlusOutlined" => Some(Self::GooglePlusOutlined),
            "GoogleOutlined" => Some(Self::GoogleOutlined),
            "GroupOutlined" => Some(Self::GroupOutlined),
            "HarmonyOSOutlined" => Some(Self::HarmonyOSOutlined),
            "HddOutlined" => Some(Self::HddOutlined),
            "HeartOutlined" => Some(Self::HeartOutlined),
            "HeatMapOutlined" => Some(Self::HeatMapOutlined),
            "HighlightOutlined" => Some(Self::HighlightOutlined),
            "HistoryOutlined" => Some(Self::HistoryOutlined),
            "HolderOutlined" => Some(Self::HolderOutlined),
            "HomeOutlined" => Some(Self::HomeOutlined),
            "HourglassOutlined" => Some(Self::HourglassOutlined),
            "Html5Outlined" => Some(Self::Html5Outlined),
            "IdcardOutlined" => Some(Self::IdcardOutlined),
            "IeOutlined" => Some(Self::IeOutlined),
            "ImportOutlined" => Some(Self::ImportOutlined),
            "InboxOutlined" => Some(Self::InboxOutlined),
            "InfoCircleOutlined" => Some(Self::InfoCircleOutlined),
            "InfoOutlined" => Some(Self::InfoOutlined),
            "InsertRowAboveOutlined" => Some(Self::InsertRowAboveOutlined),
            "InsertRowBelowOutlined" => Some(Self::InsertRowBelowOutlined),
            "InsertRowLeftOutlined" => Some(Self::InsertRowLeftOutlined),
            "InsertRowRightOutlined" => Some(Self::InsertRowRightOutlined),
            "InstagramOutlined" => Some(Self::InstagramOutlined),
            "InsuranceOutlined" => Some(Self::InsuranceOutlined),
            "InteractionOutlined" => Some(Self::InteractionOutlined),
            "IssuesCloseOutlined" => Some(Self::IssuesCloseOutlined),
            "ItalicOutlined" => Some(Self::ItalicOutlined),
            "JavaScriptOutlined" => Some(Self::JavaScriptOutlined),
            "JavaOutlined" => Some(Self::JavaOutlined),
            "KeyOutlined" => Some(Self::KeyOutlined),
            "KubernetesOutlined" => Some(Self::KubernetesOutlined),
            "LaptopOutlined" => Some(Self::LaptopOutlined),
            "LayoutOutlined" => Some(Self::LayoutOutlined),
            "LeftCircleOutlined" => Some(Self::LeftCircleOutlined),
            "LeftSquareOutlined" => Some(Self::LeftSquareOutlined),
            "LeftOutlined" => Some(Self::LeftOutlined),
            "LikeOutlined" => Some(Self::LikeOutlined),
            "LineChartOutlined" => Some(Self::LineChartOutlined),
            "LineHeightOutlined" => Some(Self::LineHeightOutlined),
            "LineOutlined" => Some(Self::LineOutlined),
            "LinkOutlined" => Some(Self::LinkOutlined),
            "LinkedinOutlined" => Some(Self::LinkedinOutlined),
            "LinuxOutlined" => Some(Self::LinuxOutlined),
            "Loading3QuartersOutlined" => Some(Self::Loading3QuartersOutlined),
            "LoadingOutlined" => Some(Self::LoadingOutlined),
            "LockOutlined" => Some(Self::LockOutlined),
            "LoginOutlined" => Some(Self::LoginOutlined),
            "LogoutOutlined" => Some(Self::LogoutOutlined),
            "MacCommandOutlined" => Some(Self::MacCommandOutlined),
            "MailOutlined" => Some(Self::MailOutlined),
            "ManOutlined" => Some(Self::ManOutlined),
            "MedicineBoxOutlined" => Some(Self::MedicineBoxOutlined),
            "MediumWorkmarkOutlined" => Some(Self::MediumWorkmarkOutlined),
            "MediumOutlined" => Some(Self::MediumOutlined),
            "MehOutlined" => Some(Self::MehOutlined),
            "MenuFoldOutlined" => Some(Self::MenuFoldOutlined),
            "MenuUnfoldOutlined" => Some(Self::MenuUnfoldOutlined),
            "MenuOutlined" => Some(Self::MenuOutlined),
            "MergeCellsOutlined" => Some(Self::MergeCellsOutlined),
            "MergeOutlined" => Some(Self::MergeOutlined),
            "MessageOutlined" => Some(Self::MessageOutlined),
            "MinusCircleOutlined" => Some(Self::MinusCircleOutlined),
            "MinusSquareOutlined" => Some(Self::MinusSquareOutlined),
            "MinusOutlined" => Some(Self::MinusOutlined),
            "MobileOutlined" => Some(Self::MobileOutlined),
            "MoneyCollectOutlined" => Some(Self::MoneyCollectOutlined),
            "MonitorOutlined" => Some(Self::MonitorOutlined),
            "MoonOutlined" => Some(Self::MoonOutlined),
            "MoreOutlined" => Some(Self::MoreOutlined),
            "MutedOutlined" => Some(Self::MutedOutlined),
            "NodeCollapseOutlined" => Some(Self::NodeCollapseOutlined),
            "NodeExpandOutlined" => Some(Self::NodeExpandOutlined),
            "NodeIndexOutlined" => Some(Self::NodeIndexOutlined),
            "NotificationOutlined" => Some(Self::NotificationOutlined),
            "NumberOutlined" => Some(Self::NumberOutlined),
            "OneToOneOutlined" => Some(Self::OneToOneOutlined),
            "OpenAIOutlined" => Some(Self::OpenAIOutlined),
            "OrderedListOutlined" => Some(Self::OrderedListOutlined),
            "PaperClipOutlined" => Some(Self::PaperClipOutlined),
            "PartitionOutlined" => Some(Self::PartitionOutlined),
            "PauseCircleOutlined" => Some(Self::PauseCircleOutlined),
            "PauseOutlined" => Some(Self::PauseOutlined),
            "PayCircleOutlined" => Some(Self::PayCircleOutlined),
            "PercentageOutlined" => Some(Self::PercentageOutlined),
            "PhoneOutlined" => Some(Self::PhoneOutlined),
            "PicCenterOutlined" => Some(Self::PicCenterOutlined),
            "PicLeftOutlined" => Some(Self::PicLeftOutlined),
            "PicRightOutlined" => Some(Self::PicRightOutlined),
            "PictureOutlined" => Some(Self::PictureOutlined),
            "PieChartOutlined" => Some(Self::PieChartOutlined),
            "PinterestOutlined" => Some(Self::PinterestOutlined),
            "PlayCircleOutlined" => Some(Self::PlayCircleOutlined),
            "PlaySquareOutlined" => Some(Self::PlaySquareOutlined),
            "PlusCircleOutlined" => Some(Self::PlusCircleOutlined),
            "PlusSquareOutlined" => Some(Self::PlusSquareOutlined),
            "PlusOutlined" => Some(Self::PlusOutlined),
            "PoundCircleOutlined" => Some(Self::PoundCircleOutlined),
            "PoundOutlined" => Some(Self::PoundOutlined),
            "PoweroffOutlined" => Some(Self::PoweroffOutlined),
            "PrinterOutlined" => Some(Self::PrinterOutlined),
            "ProductOutlined" => Some(Self::ProductOutlined),
            "ProfileOutlined" => Some(Self::ProfileOutlined),
            "ProjectOutlined" => Some(Self::ProjectOutlined),
            "PropertySafetyOutlined" => Some(Self::PropertySafetyOutlined),
            "PullRequestOutlined" => Some(Self::PullRequestOutlined),
            "PushpinOutlined" => Some(Self::PushpinOutlined),
            "PythonOutlined" => Some(Self::PythonOutlined),
            "QqOutlined" => Some(Self::QqOutlined),
            "QrcodeOutlined" => Some(Self::QrcodeOutlined),
            "QuestionCircleOutlined" => Some(Self::QuestionCircleOutlined),
            "QuestionOutlined" => Some(Self::QuestionOutlined),
            "RadarChartOutlined" => Some(Self::RadarChartOutlined),
            "RadiusBottomleftOutlined" => Some(Self::RadiusBottomleftOutlined),
            "RadiusBottomrightOutlined" => Some(Self::RadiusBottomrightOutlined),
            "RadiusSettingOutlined" => Some(Self::RadiusSettingOutlined),
            "RadiusUpleftOutlined" => Some(Self::RadiusUpleftOutlined),
            "RadiusUprightOutlined" => Some(Self::RadiusUprightOutlined),
            "ReadOutlined" => Some(Self::ReadOutlined),
            "ReconciliationOutlined" => Some(Self::ReconciliationOutlined),
            "RedEnvelopeOutlined" => Some(Self::RedEnvelopeOutlined),
            "RedditOutlined" => Some(Self::RedditOutlined),
            "RedoOutlined" => Some(Self::RedoOutlined),
            "ReloadOutlined" => Some(Self::ReloadOutlined),
            "RestOutlined" => Some(Self::RestOutlined),
            "RetweetOutlined" => Some(Self::RetweetOutlined),
            "RightCircleOutlined" => Some(Self::RightCircleOutlined),
            "RightSquareOutlined" => Some(Self::RightSquareOutlined),
            "RightOutlined" => Some(Self::RightOutlined),
            "RiseOutlined" => Some(Self::RiseOutlined),
            "RobotOutlined" => Some(Self::RobotOutlined),
            "RocketOutlined" => Some(Self::RocketOutlined),
            "RollbackOutlined" => Some(Self::RollbackOutlined),
            "RotateLeftOutlined" => Some(Self::RotateLeftOutlined),
            "RotateRightOutlined" => Some(Self::RotateRightOutlined),
            "RubyOutlined" => Some(Self::RubyOutlined),
            "SafetyCertificateOutlined" => Some(Self::SafetyCertificateOutlined),
            "SafetyOutlined" => Some(Self::SafetyOutlined),
            "SaveOutlined" => Some(Self::SaveOutlined),
            "ScanOutlined" => Some(Self::ScanOutlined),
            "ScheduleOutlined" => Some(Self::ScheduleOutlined),
            "ScissorOutlined" => Some(Self::ScissorOutlined),
            "SearchOutlined" => Some(Self::SearchOutlined),
            "SecurityScanOutlined" => Some(Self::SecurityScanOutlined),
            "SelectOutlined" => Some(Self::SelectOutlined),
            "SendOutlined" => Some(Self::SendOutlined),
            "SettingOutlined" => Some(Self::SettingOutlined),
            "ShakeOutlined" => Some(Self::ShakeOutlined),
            "ShareAltOutlined" => Some(Self::ShareAltOutlined),
            "ShopOutlined" => Some(Self::ShopOutlined),
            "ShoppingCartOutlined" => Some(Self::ShoppingCartOutlined),
            "ShoppingOutlined" => Some(Self::ShoppingOutlined),
            "ShrinkOutlined" => Some(Self::ShrinkOutlined),
            "SignatureOutlined" => Some(Self::SignatureOutlined),
            "SisternodeOutlined" => Some(Self::SisternodeOutlined),
            "SketchOutlined" => Some(Self::SketchOutlined),
            "SkinOutlined" => Some(Self::SkinOutlined),
            "SkypeOutlined" => Some(Self::SkypeOutlined),
            "SlackSquareOutlined" => Some(Self::SlackSquareOutlined),
            "SlackOutlined" => Some(Self::SlackOutlined),
            "SlidersOutlined" => Some(Self::SlidersOutlined),
            "SmallDashOutlined" => Some(Self::SmallDashOutlined),
            "SmileOutlined" => Some(Self::SmileOutlined),
            "SnippetsOutlined" => Some(Self::SnippetsOutlined),
            "SolutionOutlined" => Some(Self::SolutionOutlined),
            "SortAscendingOutlined" => Some(Self::SortAscendingOutlined),
            "SortDescendingOutlined" => Some(Self::SortDescendingOutlined),
            "SoundOutlined" => Some(Self::SoundOutlined),
            "SplitCellsOutlined" => Some(Self::SplitCellsOutlined),
            "SpotifyOutlined" => Some(Self::SpotifyOutlined),
            "StarOutlined" => Some(Self::StarOutlined),
            "StepBackwardOutlined" => Some(Self::StepBackwardOutlined),
            "StepForwardOutlined" => Some(Self::StepForwardOutlined),
            "StockOutlined" => Some(Self::StockOutlined),
            "StopOutlined" => Some(Self::StopOutlined),
            "StrikethroughOutlined" => Some(Self::StrikethroughOutlined),
            "SubnodeOutlined" => Some(Self::SubnodeOutlined),
            "SunOutlined" => Some(Self::SunOutlined),
            "SwapLeftOutlined" => Some(Self::SwapLeftOutlined),
            "SwapRightOutlined" => Some(Self::SwapRightOutlined),
            "SwapOutlined" => Some(Self::SwapOutlined),
            "SwitcherOutlined" => Some(Self::SwitcherOutlined),
            "SyncOutlined" => Some(Self::SyncOutlined),
            "TableOutlined" => Some(Self::TableOutlined),
            "TabletOutlined" => Some(Self::TabletOutlined),
            "TagOutlined" => Some(Self::TagOutlined),
            "TagsOutlined" => Some(Self::TagsOutlined),
            "TaobaoCircleOutlined" => Some(Self::TaobaoCircleOutlined),
            "TaobaoOutlined" => Some(Self::TaobaoOutlined),
            "TeamOutlined" => Some(Self::TeamOutlined),
            "ThunderboltOutlined" => Some(Self::ThunderboltOutlined),
            "TikTokOutlined" => Some(Self::TikTokOutlined),
            "ToTopOutlined" => Some(Self::ToTopOutlined),
            "ToolOutlined" => Some(Self::ToolOutlined),
            "TrademarkCircleOutlined" => Some(Self::TrademarkCircleOutlined),
            "TrademarkOutlined" => Some(Self::TrademarkOutlined),
            "TransactionOutlined" => Some(Self::TransactionOutlined),
            "TranslationOutlined" => Some(Self::TranslationOutlined),
            "TrophyOutlined" => Some(Self::TrophyOutlined),
            "TruckOutlined" => Some(Self::TruckOutlined),
            "TwitchOutlined" => Some(Self::TwitchOutlined),
            "TwitterOutlined" => Some(Self::TwitterOutlined),
            "UnderlineOutlined" => Some(Self::UnderlineOutlined),
            "UndoOutlined" => Some(Self::UndoOutlined),
            "UngroupOutlined" => Some(Self::UngroupOutlined),
            "UnlockOutlined" => Some(Self::UnlockOutlined),
            "UnorderedListOutlined" => Some(Self::UnorderedListOutlined),
            "UpCircleOutlined" => Some(Self::UpCircleOutlined),
            "UpSquareOutlined" => Some(Self::UpSquareOutlined),
            "UpOutlined" => Some(Self::UpOutlined),
            "UploadOutlined" => Some(Self::UploadOutlined),
            "UsbOutlined" => Some(Self::UsbOutlined),
            "UserAddOutlined" => Some(Self::UserAddOutlined),
            "UserDeleteOutlined" => Some(Self::UserDeleteOutlined),
            "UserSwitchOutlined" => Some(Self::UserSwitchOutlined),
            "UserOutlined" => Some(Self::UserOutlined),
            "UsergroupAddOutlined" => Some(Self::UsergroupAddOutlined),
            "UsergroupDeleteOutlined" => Some(Self::UsergroupDeleteOutlined),
            "VerifiedOutlined" => Some(Self::VerifiedOutlined),
            "VerticalAlignBottomOutlined" => Some(Self::VerticalAlignBottomOutlined),
            "VerticalAlignMiddleOutlined" => Some(Self::VerticalAlignMiddleOutlined),
            "VerticalAlignTopOutlined" => Some(Self::VerticalAlignTopOutlined),
            "VerticalLeftOutlined" => Some(Self::VerticalLeftOutlined),
            "VerticalRightOutlined" => Some(Self::VerticalRightOutlined),
            "VideoCameraAddOutlined" => Some(Self::VideoCameraAddOutlined),
            "VideoCameraOutlined" => Some(Self::VideoCameraOutlined),
            "WalletOutlined" => Some(Self::WalletOutlined),
            "WarningOutlined" => Some(Self::WarningOutlined),
            "WechatWorkOutlined" => Some(Self::WechatWorkOutlined),
            "WechatOutlined" => Some(Self::WechatOutlined),
            "WeiboCircleOutlined" => Some(Self::WeiboCircleOutlined),
            "WeiboSquareOutlined" => Some(Self::WeiboSquareOutlined),
            "WeiboOutlined" => Some(Self::WeiboOutlined),
            "WhatsAppOutlined" => Some(Self::WhatsAppOutlined),
            "WifiOutlined" => Some(Self::WifiOutlined),
            "WindowsOutlined" => Some(Self::WindowsOutlined),
            "WomanOutlined" => Some(Self::WomanOutlined),
            "XOutlined" => Some(Self::XOutlined),
            "YahooOutlined" => Some(Self::YahooOutlined),
            "YoutubeOutlined" => Some(Self::YoutubeOutlined),
            "YuqueOutlined" => Some(Self::YuqueOutlined),
            "ZhihuOutlined" => Some(Self::ZhihuOutlined),
            "ZoomInOutlined" => Some(Self::ZoomInOutlined),
            "ZoomOutOutlined" => Some(Self::ZoomOutOutlined),
            "AccountBookFilled" => Some(Self::AccountBookFilled),
            "AlertFilled" => Some(Self::AlertFilled),
            "AlipayCircleFilled" => Some(Self::AlipayCircleFilled),
            "AlipaySquareFilled" => Some(Self::AlipaySquareFilled),
            "AliwangwangFilled" => Some(Self::AliwangwangFilled),
            "AmazonCircleFilled" => Some(Self::AmazonCircleFilled),
            "AmazonSquareFilled" => Some(Self::AmazonSquareFilled),
            "AndroidFilled" => Some(Self::AndroidFilled),
            "AnthropicFilled" => Some(Self::AnthropicFilled),
            "ApiFilled" => Some(Self::ApiFilled),
            "AppleFilled" => Some(Self::AppleFilled),
            "AppstoreFilled" => Some(Self::AppstoreFilled),
            "AudioFilled" => Some(Self::AudioFilled),
            "BackwardFilled" => Some(Self::BackwardFilled),
            "BankFilled" => Some(Self::BankFilled),
            "BehanceCircleFilled" => Some(Self::BehanceCircleFilled),
            "BehanceSquareFilled" => Some(Self::BehanceSquareFilled),
            "BellFilled" => Some(Self::BellFilled),
            "BilibiliFilled" => Some(Self::BilibiliFilled),
            "BookFilled" => Some(Self::BookFilled),
            "BoxPlotFilled" => Some(Self::BoxPlotFilled),
            "BugFilled" => Some(Self::BugFilled),
            "BuildFilled" => Some(Self::BuildFilled),
            "BulbFilled" => Some(Self::BulbFilled),
            "CalculatorFilled" => Some(Self::CalculatorFilled),
            "CalendarFilled" => Some(Self::CalendarFilled),
            "CameraFilled" => Some(Self::CameraFilled),
            "CarFilled" => Some(Self::CarFilled),
            "CaretDownFilled" => Some(Self::CaretDownFilled),
            "CaretLeftFilled" => Some(Self::CaretLeftFilled),
            "CaretRightFilled" => Some(Self::CaretRightFilled),
            "CaretUpFilled" => Some(Self::CaretUpFilled),
            "CarryOutFilled" => Some(Self::CarryOutFilled),
            "CheckCircleFilled" => Some(Self::CheckCircleFilled),
            "CheckSquareFilled" => Some(Self::CheckSquareFilled),
            "ChromeFilled" => Some(Self::ChromeFilled),
            "CiCircleFilled" => Some(Self::CiCircleFilled),
            "ClaudeFilled" => Some(Self::ClaudeFilled),
            "ClockCircleFilled" => Some(Self::ClockCircleFilled),
            "CloseCircleFilled" => Some(Self::CloseCircleFilled),
            "CloseSquareFilled" => Some(Self::CloseSquareFilled),
            "CloudFilled" => Some(Self::CloudFilled),
            "CodeSandboxCircleFilled" => Some(Self::CodeSandboxCircleFilled),
            "CodeSandboxSquareFilled" => Some(Self::CodeSandboxSquareFilled),
            "CodeFilled" => Some(Self::CodeFilled),
            "CodepenCircleFilled" => Some(Self::CodepenCircleFilled),
            "CodepenSquareFilled" => Some(Self::CodepenSquareFilled),
            "CompassFilled" => Some(Self::CompassFilled),
            "ContactsFilled" => Some(Self::ContactsFilled),
            "ContainerFilled" => Some(Self::ContainerFilled),
            "ControlFilled" => Some(Self::ControlFilled),
            "CopyFilled" => Some(Self::CopyFilled),
            "CopyrightCircleFilled" => Some(Self::CopyrightCircleFilled),
            "CreditCardFilled" => Some(Self::CreditCardFilled),
            "CrownFilled" => Some(Self::CrownFilled),
            "CustomerServiceFilled" => Some(Self::CustomerServiceFilled),
            "DashboardFilled" => Some(Self::DashboardFilled),
            "DatabaseFilled" => Some(Self::DatabaseFilled),
            "DeepSeekFilled" => Some(Self::DeepSeekFilled),
            "DeleteFilled" => Some(Self::DeleteFilled),
            "DiffFilled" => Some(Self::DiffFilled),
            "DingtalkCircleFilled" => Some(Self::DingtalkCircleFilled),
            "DingtalkSquareFilled" => Some(Self::DingtalkSquareFilled),
            "DiscordFilled" => Some(Self::DiscordFilled),
            "DislikeFilled" => Some(Self::DislikeFilled),
            "DollarCircleFilled" => Some(Self::DollarCircleFilled),
            "DownCircleFilled" => Some(Self::DownCircleFilled),
            "DownSquareFilled" => Some(Self::DownSquareFilled),
            "DribbbleCircleFilled" => Some(Self::DribbbleCircleFilled),
            "DribbbleSquareFilled" => Some(Self::DribbbleSquareFilled),
            "DropboxCircleFilled" => Some(Self::DropboxCircleFilled),
            "DropboxSquareFilled" => Some(Self::DropboxSquareFilled),
            "EditFilled" => Some(Self::EditFilled),
            "ElevenLabsFilled" => Some(Self::ElevenLabsFilled),
            "EnvironmentFilled" => Some(Self::EnvironmentFilled),
            "EuroCircleFilled" => Some(Self::EuroCircleFilled),
            "ExclamationCircleFilled" => Some(Self::ExclamationCircleFilled),
            "ExperimentFilled" => Some(Self::ExperimentFilled),
            "EyeInvisibleFilled" => Some(Self::EyeInvisibleFilled),
            "EyeFilled" => Some(Self::EyeFilled),
            "FacebookFilled" => Some(Self::FacebookFilled),
            "FastBackwardFilled" => Some(Self::FastBackwardFilled),
            "FastForwardFilled" => Some(Self::FastForwardFilled),
            "FileAddFilled" => Some(Self::FileAddFilled),
            "FileExcelFilled" => Some(Self::FileExcelFilled),
            "FileExclamationFilled" => Some(Self::FileExclamationFilled),
            "FileImageFilled" => Some(Self::FileImageFilled),
            "FileMarkdownFilled" => Some(Self::FileMarkdownFilled),
            "FilePdfFilled" => Some(Self::FilePdfFilled),
            "FilePptFilled" => Some(Self::FilePptFilled),
            "FileTextFilled" => Some(Self::FileTextFilled),
            "FileUnknownFilled" => Some(Self::FileUnknownFilled),
            "FileWordFilled" => Some(Self::FileWordFilled),
            "FileZipFilled" => Some(Self::FileZipFilled),
            "FileFilled" => Some(Self::FileFilled),
            "FilterFilled" => Some(Self::FilterFilled),
            "FireFilled" => Some(Self::FireFilled),
            "FlagFilled" => Some(Self::FlagFilled),
            "FolderAddFilled" => Some(Self::FolderAddFilled),
            "FolderOpenFilled" => Some(Self::FolderOpenFilled),
            "FolderFilled" => Some(Self::FolderFilled),
            "FormatPainterFilled" => Some(Self::FormatPainterFilled),
            "ForwardFilled" => Some(Self::ForwardFilled),
            "FrownFilled" => Some(Self::FrownFilled),
            "FundFilled" => Some(Self::FundFilled),
            "FunnelPlotFilled" => Some(Self::FunnelPlotFilled),
            "GeminiFilled" => Some(Self::GeminiFilled),
            "GiftFilled" => Some(Self::GiftFilled),
            "GithubFilled" => Some(Self::GithubFilled),
            "GitlabFilled" => Some(Self::GitlabFilled),
            "GoldFilled" => Some(Self::GoldFilled),
            "GoldenFilled" => Some(Self::GoldenFilled),
            "GoogleCircleFilled" => Some(Self::GoogleCircleFilled),
            "GooglePlusCircleFilled" => Some(Self::GooglePlusCircleFilled),
            "GooglePlusSquareFilled" => Some(Self::GooglePlusSquareFilled),
            "GoogleSquareFilled" => Some(Self::GoogleSquareFilled),
            "HddFilled" => Some(Self::HddFilled),
            "HeartFilled" => Some(Self::HeartFilled),
            "HighlightFilled" => Some(Self::HighlightFilled),
            "HomeFilled" => Some(Self::HomeFilled),
            "HourglassFilled" => Some(Self::HourglassFilled),
            "Html5Filled" => Some(Self::Html5Filled),
            "HuggingFaceFilled" => Some(Self::HuggingFaceFilled),
            "IdcardFilled" => Some(Self::IdcardFilled),
            "IeCircleFilled" => Some(Self::IeCircleFilled),
            "IeSquareFilled" => Some(Self::IeSquareFilled),
            "InfoCircleFilled" => Some(Self::InfoCircleFilled),
            "InstagramFilled" => Some(Self::InstagramFilled),
            "InsuranceFilled" => Some(Self::InsuranceFilled),
            "InteractionFilled" => Some(Self::InteractionFilled),
            "LayoutFilled" => Some(Self::LayoutFilled),
            "LeftCircleFilled" => Some(Self::LeftCircleFilled),
            "LeftSquareFilled" => Some(Self::LeftSquareFilled),
            "LikeFilled" => Some(Self::LikeFilled),
            "LinkedinFilled" => Some(Self::LinkedinFilled),
            "LockFilled" => Some(Self::LockFilled),
            "MacCommandFilled" => Some(Self::MacCommandFilled),
            "MailFilled" => Some(Self::MailFilled),
            "MastodonFilled" => Some(Self::MastodonFilled),
            "MedicineBoxFilled" => Some(Self::MedicineBoxFilled),
            "MediumCircleFilled" => Some(Self::MediumCircleFilled),
            "MediumSquareFilled" => Some(Self::MediumSquareFilled),
            "MehFilled" => Some(Self::MehFilled),
            "MergeFilled" => Some(Self::MergeFilled),
            "MessageFilled" => Some(Self::MessageFilled),
            "MetaFilled" => Some(Self::MetaFilled),
            "MinusCircleFilled" => Some(Self::MinusCircleFilled),
            "MinusSquareFilled" => Some(Self::MinusSquareFilled),
            "MistralFilled" => Some(Self::MistralFilled),
            "MobileFilled" => Some(Self::MobileFilled),
            "MoneyCollectFilled" => Some(Self::MoneyCollectFilled),
            "MoonFilled" => Some(Self::MoonFilled),
            "MutedFilled" => Some(Self::MutedFilled),
            "NetflixFilled" => Some(Self::NetflixFilled),
            "NotificationFilled" => Some(Self::NotificationFilled),
            "OllamaFilled" => Some(Self::OllamaFilled),
            "OpenAIFilled" => Some(Self::OpenAIFilled),
            "PauseCircleFilled" => Some(Self::PauseCircleFilled),
            "PayCircleFilled" => Some(Self::PayCircleFilled),
            "PerplexityFilled" => Some(Self::PerplexityFilled),
            "PhoneFilled" => Some(Self::PhoneFilled),
            "PictureFilled" => Some(Self::PictureFilled),
            "PieChartFilled" => Some(Self::PieChartFilled),
            "PinterestFilled" => Some(Self::PinterestFilled),
            "PlayCircleFilled" => Some(Self::PlayCircleFilled),
            "PlaySquareFilled" => Some(Self::PlaySquareFilled),
            "PlusCircleFilled" => Some(Self::PlusCircleFilled),
            "PlusSquareFilled" => Some(Self::PlusSquareFilled),
            "PoundCircleFilled" => Some(Self::PoundCircleFilled),
            "PrinterFilled" => Some(Self::PrinterFilled),
            "ProductFilled" => Some(Self::ProductFilled),
            "ProfileFilled" => Some(Self::ProfileFilled),
            "ProjectFilled" => Some(Self::ProjectFilled),
            "PropertySafetyFilled" => Some(Self::PropertySafetyFilled),
            "PushpinFilled" => Some(Self::PushpinFilled),
            "QqCircleFilled" => Some(Self::QqCircleFilled),
            "QqSquareFilled" => Some(Self::QqSquareFilled),
            "QuestionCircleFilled" => Some(Self::QuestionCircleFilled),
            "QwenFilled" => Some(Self::QwenFilled),
            "ReadFilled" => Some(Self::ReadFilled),
            "ReconciliationFilled" => Some(Self::ReconciliationFilled),
            "RedEnvelopeFilled" => Some(Self::RedEnvelopeFilled),
            "RedditCircleFilled" => Some(Self::RedditCircleFilled),
            "RedditSquareFilled" => Some(Self::RedditSquareFilled),
            "ReplicateFilled" => Some(Self::ReplicateFilled),
            "RestFilled" => Some(Self::RestFilled),
            "RightCircleFilled" => Some(Self::RightCircleFilled),
            "RightSquareFilled" => Some(Self::RightSquareFilled),
            "RobotFilled" => Some(Self::RobotFilled),
            "RocketFilled" => Some(Self::RocketFilled),
            "SafetyCertificateFilled" => Some(Self::SafetyCertificateFilled),
            "SaveFilled" => Some(Self::SaveFilled),
            "ScheduleFilled" => Some(Self::ScheduleFilled),
            "SecurityScanFilled" => Some(Self::SecurityScanFilled),
            "SettingFilled" => Some(Self::SettingFilled),
            "ShopFilled" => Some(Self::ShopFilled),
            "ShoppingFilled" => Some(Self::ShoppingFilled),
            "SignalFilled" => Some(Self::SignalFilled),
            "SignatureFilled" => Some(Self::SignatureFilled),
            "SketchCircleFilled" => Some(Self::SketchCircleFilled),
            "SketchSquareFilled" => Some(Self::SketchSquareFilled),
            "SkinFilled" => Some(Self::SkinFilled),
            "SkypeFilled" => Some(Self::SkypeFilled),
            "SlackCircleFilled" => Some(Self::SlackCircleFilled),
            "SlackSquareFilled" => Some(Self::SlackSquareFilled),
            "SlidersFilled" => Some(Self::SlidersFilled),
            "SmileFilled" => Some(Self::SmileFilled),
            "SnapchatFilled" => Some(Self::SnapchatFilled),
            "SnippetsFilled" => Some(Self::SnippetsFilled),
            "SoundFilled" => Some(Self::SoundFilled),
            "SpotifyFilled" => Some(Self::SpotifyFilled),
            "StarFilled" => Some(Self::StarFilled),
            "StepBackwardFilled" => Some(Self::StepBackwardFilled),
            "StepForwardFilled" => Some(Self::StepForwardFilled),
            "StopFilled" => Some(Self::StopFilled),
            "SunFilled" => Some(Self::SunFilled),
            "SwitcherFilled" => Some(Self::SwitcherFilled),
            "TabletFilled" => Some(Self::TabletFilled),
            "TagFilled" => Some(Self::TagFilled),
            "TagsFilled" => Some(Self::TagsFilled),
            "TaobaoCircleFilled" => Some(Self::TaobaoCircleFilled),
            "TaobaoSquareFilled" => Some(Self::TaobaoSquareFilled),
            "TelegramFilled" => Some(Self::TelegramFilled),
            "ThreadsFilled" => Some(Self::ThreadsFilled),
            "ThunderboltFilled" => Some(Self::ThunderboltFilled),
            "TikTokFilled" => Some(Self::TikTokFilled),
            "ToolFilled" => Some(Self::ToolFilled),
            "TrademarkCircleFilled" => Some(Self::TrademarkCircleFilled),
            "TrophyFilled" => Some(Self::TrophyFilled),
            "TruckFilled" => Some(Self::TruckFilled),
            "TwitchFilled" => Some(Self::TwitchFilled),
            "TwitterCircleFilled" => Some(Self::TwitterCircleFilled),
            "TwitterSquareFilled" => Some(Self::TwitterSquareFilled),
            "UnlockFilled" => Some(Self::UnlockFilled),
            "UpCircleFilled" => Some(Self::UpCircleFilled),
            "UpSquareFilled" => Some(Self::UpSquareFilled),
            "UsbFilled" => Some(Self::UsbFilled),
            "VideoCameraFilled" => Some(Self::VideoCameraFilled),
            "WalletFilled" => Some(Self::WalletFilled),
            "WarningFilled" => Some(Self::WarningFilled),
            "WechatWorkFilled" => Some(Self::WechatWorkFilled),
            "WechatFilled" => Some(Self::WechatFilled),
            "WeiboCircleFilled" => Some(Self::WeiboCircleFilled),
            "WeiboSquareFilled" => Some(Self::WeiboSquareFilled),
            "WindowsFilled" => Some(Self::WindowsFilled),
            "XFilled" => Some(Self::XFilled),
            "YahooFilled" => Some(Self::YahooFilled),
            "YoutubeFilled" => Some(Self::YoutubeFilled),
            "YuqueFilled" => Some(Self::YuqueFilled),
            "ZhihuCircleFilled" => Some(Self::ZhihuCircleFilled),
            "ZhihuSquareFilled" => Some(Self::ZhihuSquareFilled),
            "AccountBookTwoTone" => Some(Self::AccountBookTwoTone),
            "AlertTwoTone" => Some(Self::AlertTwoTone),
            "ApiTwoTone" => Some(Self::ApiTwoTone),
            "AppstoreTwoTone" => Some(Self::AppstoreTwoTone),
            "AudioTwoTone" => Some(Self::AudioTwoTone),
            "BankTwoTone" => Some(Self::BankTwoTone),
            "BellTwoTone" => Some(Self::BellTwoTone),
            "BookTwoTone" => Some(Self::BookTwoTone),
            "BoxPlotTwoTone" => Some(Self::BoxPlotTwoTone),
            "BugTwoTone" => Some(Self::BugTwoTone),
            "BuildTwoTone" => Some(Self::BuildTwoTone),
            "BulbTwoTone" => Some(Self::BulbTwoTone),
            "CalculatorTwoTone" => Some(Self::CalculatorTwoTone),
            "CalendarTwoTone" => Some(Self::CalendarTwoTone),
            "CameraTwoTone" => Some(Self::CameraTwoTone),
            "CarTwoTone" => Some(Self::CarTwoTone),
            "CarryOutTwoTone" => Some(Self::CarryOutTwoTone),
            "CheckCircleTwoTone" => Some(Self::CheckCircleTwoTone),
            "CheckSquareTwoTone" => Some(Self::CheckSquareTwoTone),
            "CiCircleTwoTone" => Some(Self::CiCircleTwoTone),
            "CiTwoTone" => Some(Self::CiTwoTone),
            "ClockCircleTwoTone" => Some(Self::ClockCircleTwoTone),
            "CloseCircleTwoTone" => Some(Self::CloseCircleTwoTone),
            "CloseSquareTwoTone" => Some(Self::CloseSquareTwoTone),
            "CloudTwoTone" => Some(Self::CloudTwoTone),
            "CodeTwoTone" => Some(Self::CodeTwoTone),
            "CompassTwoTone" => Some(Self::CompassTwoTone),
            "ContactsTwoTone" => Some(Self::ContactsTwoTone),
            "ContainerTwoTone" => Some(Self::ContainerTwoTone),
            "ControlTwoTone" => Some(Self::ControlTwoTone),
            "CopyTwoTone" => Some(Self::CopyTwoTone),
            "CopyrightCircleTwoTone" => Some(Self::CopyrightCircleTwoTone),
            "CopyrightTwoTone" => Some(Self::CopyrightTwoTone),
            "CreditCardTwoTone" => Some(Self::CreditCardTwoTone),
            "CrownTwoTone" => Some(Self::CrownTwoTone),
            "CustomerServiceTwoTone" => Some(Self::CustomerServiceTwoTone),
            "DashboardTwoTone" => Some(Self::DashboardTwoTone),
            "DatabaseTwoTone" => Some(Self::DatabaseTwoTone),
            "DeleteTwoTone" => Some(Self::DeleteTwoTone),
            "DiffTwoTone" => Some(Self::DiffTwoTone),
            "DislikeTwoTone" => Some(Self::DislikeTwoTone),
            "DollarCircleTwoTone" => Some(Self::DollarCircleTwoTone),
            "DollarTwoTone" => Some(Self::DollarTwoTone),
            "DownCircleTwoTone" => Some(Self::DownCircleTwoTone),
            "DownSquareTwoTone" => Some(Self::DownSquareTwoTone),
            "EditTwoTone" => Some(Self::EditTwoTone),
            "EnvironmentTwoTone" => Some(Self::EnvironmentTwoTone),
            "EuroCircleTwoTone" => Some(Self::EuroCircleTwoTone),
            "EuroTwoTone" => Some(Self::EuroTwoTone),
            "ExclamationCircleTwoTone" => Some(Self::ExclamationCircleTwoTone),
            "ExperimentTwoTone" => Some(Self::ExperimentTwoTone),
            "EyeInvisibleTwoTone" => Some(Self::EyeInvisibleTwoTone),
            "EyeTwoTone" => Some(Self::EyeTwoTone),
            "FileAddTwoTone" => Some(Self::FileAddTwoTone),
            "FileExcelTwoTone" => Some(Self::FileExcelTwoTone),
            "FileExclamationTwoTone" => Some(Self::FileExclamationTwoTone),
            "FileImageTwoTone" => Some(Self::FileImageTwoTone),
            "FileMarkdownTwoTone" => Some(Self::FileMarkdownTwoTone),
            "FilePdfTwoTone" => Some(Self::FilePdfTwoTone),
            "FilePptTwoTone" => Some(Self::FilePptTwoTone),
            "FileTextTwoTone" => Some(Self::FileTextTwoTone),
            "FileUnknownTwoTone" => Some(Self::FileUnknownTwoTone),
            "FileWordTwoTone" => Some(Self::FileWordTwoTone),
            "FileZipTwoTone" => Some(Self::FileZipTwoTone),
            "FileTwoTone" => Some(Self::FileTwoTone),
            "FilterTwoTone" => Some(Self::FilterTwoTone),
            "FireTwoTone" => Some(Self::FireTwoTone),
            "FlagTwoTone" => Some(Self::FlagTwoTone),
            "FolderAddTwoTone" => Some(Self::FolderAddTwoTone),
            "FolderOpenTwoTone" => Some(Self::FolderOpenTwoTone),
            "FolderTwoTone" => Some(Self::FolderTwoTone),
            "FrownTwoTone" => Some(Self::FrownTwoTone),
            "FundTwoTone" => Some(Self::FundTwoTone),
            "FunnelPlotTwoTone" => Some(Self::FunnelPlotTwoTone),
            "GiftTwoTone" => Some(Self::GiftTwoTone),
            "GoldTwoTone" => Some(Self::GoldTwoTone),
            "HddTwoTone" => Some(Self::HddTwoTone),
            "HeartTwoTone" => Some(Self::HeartTwoTone),
            "HighlightTwoTone" => Some(Self::HighlightTwoTone),
            "HomeTwoTone" => Some(Self::HomeTwoTone),
            "HourglassTwoTone" => Some(Self::HourglassTwoTone),
            "Html5TwoTone" => Some(Self::Html5TwoTone),
            "IdcardTwoTone" => Some(Self::IdcardTwoTone),
            "InfoCircleTwoTone" => Some(Self::InfoCircleTwoTone),
            "InsuranceTwoTone" => Some(Self::InsuranceTwoTone),
            "InteractionTwoTone" => Some(Self::InteractionTwoTone),
            "LayoutTwoTone" => Some(Self::LayoutTwoTone),
            "LeftCircleTwoTone" => Some(Self::LeftCircleTwoTone),
            "LeftSquareTwoTone" => Some(Self::LeftSquareTwoTone),
            "LikeTwoTone" => Some(Self::LikeTwoTone),
            "LockTwoTone" => Some(Self::LockTwoTone),
            "MailTwoTone" => Some(Self::MailTwoTone),
            "MedicineBoxTwoTone" => Some(Self::MedicineBoxTwoTone),
            "MehTwoTone" => Some(Self::MehTwoTone),
            "MessageTwoTone" => Some(Self::MessageTwoTone),
            "MinusCircleTwoTone" => Some(Self::MinusCircleTwoTone),
            "MinusSquareTwoTone" => Some(Self::MinusSquareTwoTone),
            "MobileTwoTone" => Some(Self::MobileTwoTone),
            "MoneyCollectTwoTone" => Some(Self::MoneyCollectTwoTone),
            "NotificationTwoTone" => Some(Self::NotificationTwoTone),
            "PauseCircleTwoTone" => Some(Self::PauseCircleTwoTone),
            "PhoneTwoTone" => Some(Self::PhoneTwoTone),
            "PictureTwoTone" => Some(Self::PictureTwoTone),
            "PieChartTwoTone" => Some(Self::PieChartTwoTone),
            "PlayCircleTwoTone" => Some(Self::PlayCircleTwoTone),
            "PlaySquareTwoTone" => Some(Self::PlaySquareTwoTone),
            "PlusCircleTwoTone" => Some(Self::PlusCircleTwoTone),
            "PlusSquareTwoTone" => Some(Self::PlusSquareTwoTone),
            "PoundCircleTwoTone" => Some(Self::PoundCircleTwoTone),
            "PrinterTwoTone" => Some(Self::PrinterTwoTone),
            "ProfileTwoTone" => Some(Self::ProfileTwoTone),
            "ProjectTwoTone" => Some(Self::ProjectTwoTone),
            "PropertySafetyTwoTone" => Some(Self::PropertySafetyTwoTone),
            "PushpinTwoTone" => Some(Self::PushpinTwoTone),
            "QuestionCircleTwoTone" => Some(Self::QuestionCircleTwoTone),
            "ReconciliationTwoTone" => Some(Self::ReconciliationTwoTone),
            "RedEnvelopeTwoTone" => Some(Self::RedEnvelopeTwoTone),
            "RestTwoTone" => Some(Self::RestTwoTone),
            "RightCircleTwoTone" => Some(Self::RightCircleTwoTone),
            "RightSquareTwoTone" => Some(Self::RightSquareTwoTone),
            "RocketTwoTone" => Some(Self::RocketTwoTone),
            "SafetyCertificateTwoTone" => Some(Self::SafetyCertificateTwoTone),
            "SaveTwoTone" => Some(Self::SaveTwoTone),
            "ScheduleTwoTone" => Some(Self::ScheduleTwoTone),
            "SecurityScanTwoTone" => Some(Self::SecurityScanTwoTone),
            "SettingTwoTone" => Some(Self::SettingTwoTone),
            "ShopTwoTone" => Some(Self::ShopTwoTone),
            "ShoppingTwoTone" => Some(Self::ShoppingTwoTone),
            "SkinTwoTone" => Some(Self::SkinTwoTone),
            "SlidersTwoTone" => Some(Self::SlidersTwoTone),
            "SmileTwoTone" => Some(Self::SmileTwoTone),
            "SnippetsTwoTone" => Some(Self::SnippetsTwoTone),
            "SoundTwoTone" => Some(Self::SoundTwoTone),
            "StarTwoTone" => Some(Self::StarTwoTone),
            "StopTwoTone" => Some(Self::StopTwoTone),
            "SwitcherTwoTone" => Some(Self::SwitcherTwoTone),
            "TabletTwoTone" => Some(Self::TabletTwoTone),
            "TagTwoTone" => Some(Self::TagTwoTone),
            "TagsTwoTone" => Some(Self::TagsTwoTone),
            "ThunderboltTwoTone" => Some(Self::ThunderboltTwoTone),
            "ToolTwoTone" => Some(Self::ToolTwoTone),
            "TrademarkCircleTwoTone" => Some(Self::TrademarkCircleTwoTone),
            "TrophyTwoTone" => Some(Self::TrophyTwoTone),
            "UnlockTwoTone" => Some(Self::UnlockTwoTone),
            "UpCircleTwoTone" => Some(Self::UpCircleTwoTone),
            "UpSquareTwoTone" => Some(Self::UpSquareTwoTone),
            "UsbTwoTone" => Some(Self::UsbTwoTone),
            "VideoCameraTwoTone" => Some(Self::VideoCameraTwoTone),
            "WalletTwoTone" => Some(Self::WalletTwoTone),
            "WarningTwoTone" => Some(Self::WarningTwoTone),
            _ => None,
        }
    }
}

/// Render an actual official Ant Design SVG using GPUI's SVG painter.
///
/// Prefer `AntIcon::new(AntIconName::SearchOutlined)` in buttons and text.
/// Outlined/Filled icons use a theme-tinted SVG mask. TwoTone icons retain
/// both SVG fill layers and follow light/dark tokens by default.
#[derive(IntoElement)]
pub struct AntIcon {
    name: AntIconName,
    size: f32,
    color: Option<Hsla>,
    two_tone_secondary: Option<Hsla>,
    theme: AntTheme,
    spin: bool,
}

impl AntIcon {
    pub fn new(name: AntIconName) -> Self {
        Self {
            name,
            size: 16.,
            color: None,
            two_tone_secondary: None,
            theme: AntTheme::default(),
            spin: false,
        }
    }
    pub fn loading() -> Self {
        Self::new(AntIconName::LoadingOutlined).spin(true)
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(0.);
        self
    }
    /// Tint Outlined/Filled icons; for TwoTone, set the primary SVG fill.
    /// The TwoTone secondary fill follows `theme.info_bg` unless overridden.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Override both colors of official TwoTone artwork without altering SVG files.
    /// On Outlined/Filled icons, only the primary color is used as a tint.
    /// Explicit TwoTone colors override theme defaults.
    pub fn two_tone_colors(mut self, primary: Hsla, secondary: Hsla) -> Self {
        self.color = Some(primary);
        self.two_tone_secondary = Some(secondary);
        self
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    /// Rotate an icon, including a TwoTone icon's two color layers in sync.
    /// Respects GPUI's reduced-motion setting automatically.
    pub fn spin(mut self, spin: bool) -> Self {
        self.spin = spin;
        self
    }
    pub fn name(&self) -> AntIconName {
        self.name
    }
}
impl RenderOnce for AntIcon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        if self.name.style() == "twotone" {
            let primary = self.color.unwrap_or(self.theme.primary);
            let secondary = self.two_tone_secondary.unwrap_or(self.theme.info_bg);
            if self.spin {
                // GPUI img() is full-color but cannot be rotated. Split the
                // official artwork into two alpha masks for native GPUI svg()
                // transforms, phase-locked to the same global animation clock.
                // A single image is still used when the icon is static.
                let animation = || Animation::new(Duration::from_millis(900)).repeat_synced();
                let path = self.name.path();
                let primary_mask = svg()
                    .path(layer_asset_key(path, TwoToneLayer::Primary))
                    .size(px(self.size))
                    .text_color(primary)
                    .with_animation(
                        format!("ant-icon-twotone-spin-{path}-primary"),
                        animation(),
                        |icon, delta| {
                            icon.with_transformation(Transformation::rotate(percentage(delta)))
                        },
                    );
                let secondary_mask = svg()
                    .path(layer_asset_key(path, TwoToneLayer::Secondary))
                    .absolute()
                    .top(px(0.))
                    .left(px(0.))
                    .size(px(self.size))
                    .text_color(secondary)
                    .with_animation(
                        format!("ant-icon-twotone-spin-{path}-secondary"),
                        animation(),
                        |icon, delta| {
                            icon.with_transformation(Transformation::rotate(percentage(delta)))
                        },
                    );
                return div()
                    .relative()
                    .size(px(self.size))
                    .flex_shrink_0()
                    .child(secondary_mask)
                    .child(primary_mask)
                    .into_any_element();
            }
            // svg()/KitIcon use single-color alpha masks and would discard
            // the pale secondary fill; retain img() for full-color raster.
            let key = palette_asset_key(self.name.path(), primary, secondary);
            return img(key)
                .size(px(self.size))
                .flex_shrink_0()
                .into_any_element();
        }
        let icon = KitIcon::default()
            .path(self.name.path())
            .with_size(Size::Size(px(self.size)))
            .text_color(self.color.unwrap_or(self.theme.text));
        if self.spin {
            icon.with_animation(
                "ant-icon-spin",
                Animation::new(Duration::from_millis(900)).repeat(),
                |this, delta| this.transform(Transformation::rotate(percentage(delta))),
            )
            .into_any_element()
        } else {
            icon.into_any_element()
        }
    }
}

// GPUI's img() rasterizes SVGs at twice their intrinsic dimensions. The
// official 1024x1024 viewBox must remain unchanged, but loading that same size
// for each 16-64px icon would allocate large textures (especially in WASM).
// Supply a smaller intrinsic image size without modifying any artwork paths.
const TWO_TONE_IMAGE_DIMENSIONS: &[u8] = b" width=\"128\" height=\"128\"";

fn two_tone_image_svg(bytes: &'static [u8]) -> Cow<'static, [u8]> {
    let start = bytes
        .windows(4)
        .position(|part| part == b"<svg")
        .expect("SVG root");
    let end = start
        + bytes[start..]
            .iter()
            .position(|byte| *byte == b'>')
            .expect("SVG root end");
    let root = &bytes[start..end];
    if root.windows(7).any(|part| part == b" width=")
        || root.windows(8).any(|part| part == b" height=")
    {
        return Cow::Owned(normalize_two_tone_svg(bytes));
    }
    let mut resized = Vec::with_capacity(bytes.len() + TWO_TONE_IMAGE_DIMENSIONS.len());
    resized.extend_from_slice(&bytes[..start + 4]);
    resized.extend_from_slice(TWO_TONE_IMAGE_DIMENSIONS);
    resized.extend_from_slice(&bytes[start + 4..]);
    Cow::Owned(normalize_two_tone_svg(&resized))
}

/// Embedded source for every official Ant Design SVG. No network or
/// font installation is required in desktop apps or WASM.
/// Register `AntIconAssets` or a composite source alongside GPUI Kit assets.
#[derive(Debug, Clone, Copy, Default)]
pub struct AntIconAssets;
impl AssetSource for AntIconAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let layer = parse_layer_asset_key(path);
        let (asset_path, palette) = match layer {
            Some((asset_path, _)) => (asset_path, None),
            None => {
                let Some(parsed) = parse_palette_asset_key(path) else {
                    return Ok(None);
                };
                parsed
            }
        };
        let svg: Option<&'static [u8]> = match asset_path {
            "antd-icons/outlined/account-book.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/account-book.svg").as_slice())
            }
            "antd-icons/outlined/aim.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/aim.svg").as_slice())
            }
            "antd-icons/outlined/alert.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/alert.svg").as_slice())
            }
            "antd-icons/outlined/alibaba.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/alibaba.svg").as_slice())
            }
            "antd-icons/outlined/align-center.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/align-center.svg").as_slice())
            }
            "antd-icons/outlined/align-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/align-left.svg").as_slice())
            }
            "antd-icons/outlined/align-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/align-right.svg").as_slice())
            }
            "antd-icons/outlined/alipay-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/alipay-circle.svg").as_slice())
            }
            "antd-icons/outlined/alipay.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/alipay.svg").as_slice())
            }
            "antd-icons/outlined/aliwangwang.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/aliwangwang.svg").as_slice())
            }
            "antd-icons/outlined/aliyun.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/aliyun.svg").as_slice())
            }
            "antd-icons/outlined/amazon.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/amazon.svg").as_slice())
            }
            "antd-icons/outlined/android.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/android.svg").as_slice())
            }
            "antd-icons/outlined/ant-cloud.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/ant-cloud.svg").as_slice())
            }
            "antd-icons/outlined/ant-design.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/ant-design.svg").as_slice())
            }
            "antd-icons/outlined/apartment.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/apartment.svg").as_slice())
            }
            "antd-icons/outlined/api.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/api.svg").as_slice())
            }
            "antd-icons/outlined/apple.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/apple.svg").as_slice())
            }
            "antd-icons/outlined/appstore-add.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/appstore-add.svg").as_slice())
            }
            "antd-icons/outlined/appstore.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/appstore.svg").as_slice())
            }
            "antd-icons/outlined/area-chart.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/area-chart.svg").as_slice())
            }
            "antd-icons/outlined/arrow-down.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/arrow-down.svg").as_slice())
            }
            "antd-icons/outlined/arrow-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/arrow-left.svg").as_slice())
            }
            "antd-icons/outlined/arrow-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/arrow-right.svg").as_slice())
            }
            "antd-icons/outlined/arrow-up.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/arrow-up.svg").as_slice())
            }
            "antd-icons/outlined/arrows-alt.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/arrows-alt.svg").as_slice())
            }
            "antd-icons/outlined/audio-muted.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/audio-muted.svg").as_slice())
            }
            "antd-icons/outlined/audio.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/audio.svg").as_slice())
            }
            "antd-icons/outlined/audit.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/audit.svg").as_slice())
            }
            "antd-icons/outlined/backward.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/backward.svg").as_slice())
            }
            "antd-icons/outlined/baidu.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/baidu.svg").as_slice())
            }
            "antd-icons/outlined/bank.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/bank.svg").as_slice())
            }
            "antd-icons/outlined/bar-chart.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/bar-chart.svg").as_slice())
            }
            "antd-icons/outlined/barcode.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/barcode.svg").as_slice())
            }
            "antd-icons/outlined/bars.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/bars.svg").as_slice())
            }
            "antd-icons/outlined/behance-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/behance-square.svg").as_slice())
            }
            "antd-icons/outlined/behance.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/behance.svg").as_slice())
            }
            "antd-icons/outlined/bell.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/bell.svg").as_slice())
            }
            "antd-icons/outlined/bg-colors.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/bg-colors.svg").as_slice())
            }
            "antd-icons/outlined/bilibili.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/bilibili.svg").as_slice())
            }
            "antd-icons/outlined/block.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/block.svg").as_slice())
            }
            "antd-icons/outlined/bold.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/bold.svg").as_slice())
            }
            "antd-icons/outlined/book.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/book.svg").as_slice())
            }
            "antd-icons/outlined/border-bottom.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/border-bottom.svg").as_slice())
            }
            "antd-icons/outlined/border-horizontal.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/border-horizontal.svg").as_slice(),
            ),
            "antd-icons/outlined/border-inner.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/border-inner.svg").as_slice())
            }
            "antd-icons/outlined/border-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/border-left.svg").as_slice())
            }
            "antd-icons/outlined/border-outer.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/border-outer.svg").as_slice())
            }
            "antd-icons/outlined/border-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/border-right.svg").as_slice())
            }
            "antd-icons/outlined/border-top.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/border-top.svg").as_slice())
            }
            "antd-icons/outlined/border-verticle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/border-verticle.svg").as_slice())
            }
            "antd-icons/outlined/border.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/border.svg").as_slice())
            }
            "antd-icons/outlined/borderless-table.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/borderless-table.svg").as_slice(),
            ),
            "antd-icons/outlined/box-plot.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/box-plot.svg").as_slice())
            }
            "antd-icons/outlined/branches.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/branches.svg").as_slice())
            }
            "antd-icons/outlined/bug.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/bug.svg").as_slice())
            }
            "antd-icons/outlined/build.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/build.svg").as_slice())
            }
            "antd-icons/outlined/bulb.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/bulb.svg").as_slice())
            }
            "antd-icons/outlined/calculator.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/calculator.svg").as_slice())
            }
            "antd-icons/outlined/calendar.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/calendar.svg").as_slice())
            }
            "antd-icons/outlined/camera.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/camera.svg").as_slice())
            }
            "antd-icons/outlined/car.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/car.svg").as_slice())
            }
            "antd-icons/outlined/caret-down.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/caret-down.svg").as_slice())
            }
            "antd-icons/outlined/caret-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/caret-left.svg").as_slice())
            }
            "antd-icons/outlined/caret-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/caret-right.svg").as_slice())
            }
            "antd-icons/outlined/caret-up.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/caret-up.svg").as_slice())
            }
            "antd-icons/outlined/carry-out.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/carry-out.svg").as_slice())
            }
            "antd-icons/outlined/check-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/check-circle.svg").as_slice())
            }
            "antd-icons/outlined/check-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/check-square.svg").as_slice())
            }
            "antd-icons/outlined/check.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/check.svg").as_slice())
            }
            "antd-icons/outlined/chrome.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/chrome.svg").as_slice())
            }
            "antd-icons/outlined/ci-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/ci-circle.svg").as_slice())
            }
            "antd-icons/outlined/ci.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/ci.svg").as_slice())
            }
            "antd-icons/outlined/clear.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/clear.svg").as_slice())
            }
            "antd-icons/outlined/clock-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/clock-circle.svg").as_slice())
            }
            "antd-icons/outlined/close-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/close-circle.svg").as_slice())
            }
            "antd-icons/outlined/close-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/close-square.svg").as_slice())
            }
            "antd-icons/outlined/close.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/close.svg").as_slice())
            }
            "antd-icons/outlined/cloud-download.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/cloud-download.svg").as_slice())
            }
            "antd-icons/outlined/cloud-server.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/cloud-server.svg").as_slice())
            }
            "antd-icons/outlined/cloud-sync.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/cloud-sync.svg").as_slice())
            }
            "antd-icons/outlined/cloud-upload.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/cloud-upload.svg").as_slice())
            }
            "antd-icons/outlined/cloud.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/cloud.svg").as_slice())
            }
            "antd-icons/outlined/cluster.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/cluster.svg").as_slice())
            }
            "antd-icons/outlined/code-sandbox.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/code-sandbox.svg").as_slice())
            }
            "antd-icons/outlined/code.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/code.svg").as_slice())
            }
            "antd-icons/outlined/codepen-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/codepen-circle.svg").as_slice())
            }
            "antd-icons/outlined/codepen.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/codepen.svg").as_slice())
            }
            "antd-icons/outlined/coffee.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/coffee.svg").as_slice())
            }
            "antd-icons/outlined/column-height.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/column-height.svg").as_slice())
            }
            "antd-icons/outlined/column-width.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/column-width.svg").as_slice())
            }
            "antd-icons/outlined/comment.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/comment.svg").as_slice())
            }
            "antd-icons/outlined/compass.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/compass.svg").as_slice())
            }
            "antd-icons/outlined/compress.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/compress.svg").as_slice())
            }
            "antd-icons/outlined/console-sql.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/console-sql.svg").as_slice())
            }
            "antd-icons/outlined/contacts.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/contacts.svg").as_slice())
            }
            "antd-icons/outlined/container.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/container.svg").as_slice())
            }
            "antd-icons/outlined/control.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/control.svg").as_slice())
            }
            "antd-icons/outlined/copy.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/copy.svg").as_slice())
            }
            "antd-icons/outlined/copyright-circle.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/copyright-circle.svg").as_slice(),
            ),
            "antd-icons/outlined/copyright.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/copyright.svg").as_slice())
            }
            "antd-icons/outlined/credit-card.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/credit-card.svg").as_slice())
            }
            "antd-icons/outlined/crown.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/crown.svg").as_slice())
            }
            "antd-icons/outlined/customer-service.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/customer-service.svg").as_slice(),
            ),
            "antd-icons/outlined/dash.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dash.svg").as_slice())
            }
            "antd-icons/outlined/dashboard.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dashboard.svg").as_slice())
            }
            "antd-icons/outlined/database.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/database.svg").as_slice())
            }
            "antd-icons/outlined/delete-column.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/delete-column.svg").as_slice())
            }
            "antd-icons/outlined/delete-row.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/delete-row.svg").as_slice())
            }
            "antd-icons/outlined/delete.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/delete.svg").as_slice())
            }
            "antd-icons/outlined/delivered-procedure.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/delivered-procedure.svg").as_slice(),
            ),
            "antd-icons/outlined/deployment-unit.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/deployment-unit.svg").as_slice())
            }
            "antd-icons/outlined/desktop.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/desktop.svg").as_slice())
            }
            "antd-icons/outlined/diff.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/diff.svg").as_slice())
            }
            "antd-icons/outlined/dingding.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dingding.svg").as_slice())
            }
            "antd-icons/outlined/dingtalk.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dingtalk.svg").as_slice())
            }
            "antd-icons/outlined/disconnect.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/disconnect.svg").as_slice())
            }
            "antd-icons/outlined/discord.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/discord.svg").as_slice())
            }
            "antd-icons/outlined/dislike.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dislike.svg").as_slice())
            }
            "antd-icons/outlined/docker.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/docker.svg").as_slice())
            }
            "antd-icons/outlined/dollar-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dollar-circle.svg").as_slice())
            }
            "antd-icons/outlined/dollar.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dollar.svg").as_slice())
            }
            "antd-icons/outlined/dot-chart.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dot-chart.svg").as_slice())
            }
            "antd-icons/outlined/dot-net.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dot-net.svg").as_slice())
            }
            "antd-icons/outlined/double-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/double-left.svg").as_slice())
            }
            "antd-icons/outlined/double-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/double-right.svg").as_slice())
            }
            "antd-icons/outlined/down-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/down-circle.svg").as_slice())
            }
            "antd-icons/outlined/down-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/down-square.svg").as_slice())
            }
            "antd-icons/outlined/down.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/down.svg").as_slice())
            }
            "antd-icons/outlined/download.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/download.svg").as_slice())
            }
            "antd-icons/outlined/drag.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/drag.svg").as_slice())
            }
            "antd-icons/outlined/dribbble-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dribbble-square.svg").as_slice())
            }
            "antd-icons/outlined/dribbble.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dribbble.svg").as_slice())
            }
            "antd-icons/outlined/dropbox.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/dropbox.svg").as_slice())
            }
            "antd-icons/outlined/edit.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/edit.svg").as_slice())
            }
            "antd-icons/outlined/ellipsis.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/ellipsis.svg").as_slice())
            }
            "antd-icons/outlined/enter.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/enter.svg").as_slice())
            }
            "antd-icons/outlined/environment.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/environment.svg").as_slice())
            }
            "antd-icons/outlined/euro-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/euro-circle.svg").as_slice())
            }
            "antd-icons/outlined/euro.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/euro.svg").as_slice())
            }
            "antd-icons/outlined/exception.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/exception.svg").as_slice())
            }
            "antd-icons/outlined/exclamation-circle.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/exclamation-circle.svg").as_slice(),
            ),
            "antd-icons/outlined/exclamation.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/exclamation.svg").as_slice())
            }
            "antd-icons/outlined/expand-alt.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/expand-alt.svg").as_slice())
            }
            "antd-icons/outlined/expand.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/expand.svg").as_slice())
            }
            "antd-icons/outlined/experiment.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/experiment.svg").as_slice())
            }
            "antd-icons/outlined/export.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/export.svg").as_slice())
            }
            "antd-icons/outlined/eye-invisible.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/eye-invisible.svg").as_slice())
            }
            "antd-icons/outlined/eye.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/eye.svg").as_slice())
            }
            "antd-icons/outlined/facebook.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/facebook.svg").as_slice())
            }
            "antd-icons/outlined/fall.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/fall.svg").as_slice())
            }
            "antd-icons/outlined/fast-backward.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/fast-backward.svg").as_slice())
            }
            "antd-icons/outlined/fast-forward.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/fast-forward.svg").as_slice())
            }
            "antd-icons/outlined/field-binary.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/field-binary.svg").as_slice())
            }
            "antd-icons/outlined/field-number.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/field-number.svg").as_slice())
            }
            "antd-icons/outlined/field-string.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/field-string.svg").as_slice())
            }
            "antd-icons/outlined/field-time.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/field-time.svg").as_slice())
            }
            "antd-icons/outlined/file-add.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-add.svg").as_slice())
            }
            "antd-icons/outlined/file-done.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-done.svg").as_slice())
            }
            "antd-icons/outlined/file-excel.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-excel.svg").as_slice())
            }
            "antd-icons/outlined/file-exclamation.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/file-exclamation.svg").as_slice(),
            ),
            "antd-icons/outlined/file-gif.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-gif.svg").as_slice())
            }
            "antd-icons/outlined/file-image.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-image.svg").as_slice())
            }
            "antd-icons/outlined/file-jpg.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-jpg.svg").as_slice())
            }
            "antd-icons/outlined/file-markdown.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-markdown.svg").as_slice())
            }
            "antd-icons/outlined/file-pdf.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-pdf.svg").as_slice())
            }
            "antd-icons/outlined/file-ppt.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-ppt.svg").as_slice())
            }
            "antd-icons/outlined/file-protect.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-protect.svg").as_slice())
            }
            "antd-icons/outlined/file-search.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-search.svg").as_slice())
            }
            "antd-icons/outlined/file-sync.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-sync.svg").as_slice())
            }
            "antd-icons/outlined/file-text.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-text.svg").as_slice())
            }
            "antd-icons/outlined/file-unknown.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-unknown.svg").as_slice())
            }
            "antd-icons/outlined/file-word.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-word.svg").as_slice())
            }
            "antd-icons/outlined/file-zip.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file-zip.svg").as_slice())
            }
            "antd-icons/outlined/file.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/file.svg").as_slice())
            }
            "antd-icons/outlined/filter.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/filter.svg").as_slice())
            }
            "antd-icons/outlined/fire.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/fire.svg").as_slice())
            }
            "antd-icons/outlined/flag.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/flag.svg").as_slice())
            }
            "antd-icons/outlined/folder-add.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/folder-add.svg").as_slice())
            }
            "antd-icons/outlined/folder-open.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/folder-open.svg").as_slice())
            }
            "antd-icons/outlined/folder-view.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/folder-view.svg").as_slice())
            }
            "antd-icons/outlined/folder.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/folder.svg").as_slice())
            }
            "antd-icons/outlined/font-colors.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/font-colors.svg").as_slice())
            }
            "antd-icons/outlined/font-size.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/font-size.svg").as_slice())
            }
            "antd-icons/outlined/fork.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/fork.svg").as_slice())
            }
            "antd-icons/outlined/form.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/form.svg").as_slice())
            }
            "antd-icons/outlined/format-painter.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/format-painter.svg").as_slice())
            }
            "antd-icons/outlined/forward.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/forward.svg").as_slice())
            }
            "antd-icons/outlined/frown.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/frown.svg").as_slice())
            }
            "antd-icons/outlined/fullscreen-exit.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/fullscreen-exit.svg").as_slice())
            }
            "antd-icons/outlined/fullscreen.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/fullscreen.svg").as_slice())
            }
            "antd-icons/outlined/function.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/function.svg").as_slice())
            }
            "antd-icons/outlined/fund-projection-screen.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/fund-projection-screen.svg")
                    .as_slice(),
            ),
            "antd-icons/outlined/fund-view.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/fund-view.svg").as_slice())
            }
            "antd-icons/outlined/fund.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/fund.svg").as_slice())
            }
            "antd-icons/outlined/funnel-plot.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/funnel-plot.svg").as_slice())
            }
            "antd-icons/outlined/gateway.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/gateway.svg").as_slice())
            }
            "antd-icons/outlined/gif.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/gif.svg").as_slice())
            }
            "antd-icons/outlined/gift.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/gift.svg").as_slice())
            }
            "antd-icons/outlined/github.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/github.svg").as_slice())
            }
            "antd-icons/outlined/gitlab.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/gitlab.svg").as_slice())
            }
            "antd-icons/outlined/global.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/global.svg").as_slice())
            }
            "antd-icons/outlined/gold.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/gold.svg").as_slice())
            }
            "antd-icons/outlined/google-plus.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/google-plus.svg").as_slice())
            }
            "antd-icons/outlined/google.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/google.svg").as_slice())
            }
            "antd-icons/outlined/group.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/group.svg").as_slice())
            }
            "antd-icons/outlined/harmony-o-s.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/harmony-o-s.svg").as_slice())
            }
            "antd-icons/outlined/hdd.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/hdd.svg").as_slice())
            }
            "antd-icons/outlined/heart.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/heart.svg").as_slice())
            }
            "antd-icons/outlined/heat-map.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/heat-map.svg").as_slice())
            }
            "antd-icons/outlined/highlight.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/highlight.svg").as_slice())
            }
            "antd-icons/outlined/history.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/history.svg").as_slice())
            }
            "antd-icons/outlined/holder.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/holder.svg").as_slice())
            }
            "antd-icons/outlined/home.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/home.svg").as_slice())
            }
            "antd-icons/outlined/hourglass.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/hourglass.svg").as_slice())
            }
            "antd-icons/outlined/html5.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/html5.svg").as_slice())
            }
            "antd-icons/outlined/idcard.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/idcard.svg").as_slice())
            }
            "antd-icons/outlined/ie.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/ie.svg").as_slice())
            }
            "antd-icons/outlined/import.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/import.svg").as_slice())
            }
            "antd-icons/outlined/inbox.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/inbox.svg").as_slice())
            }
            "antd-icons/outlined/info-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/info-circle.svg").as_slice())
            }
            "antd-icons/outlined/info.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/info.svg").as_slice())
            }
            "antd-icons/outlined/insert-row-above.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/insert-row-above.svg").as_slice(),
            ),
            "antd-icons/outlined/insert-row-below.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/insert-row-below.svg").as_slice(),
            ),
            "antd-icons/outlined/insert-row-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/insert-row-left.svg").as_slice())
            }
            "antd-icons/outlined/insert-row-right.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/insert-row-right.svg").as_slice(),
            ),
            "antd-icons/outlined/instagram.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/instagram.svg").as_slice())
            }
            "antd-icons/outlined/insurance.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/insurance.svg").as_slice())
            }
            "antd-icons/outlined/interaction.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/interaction.svg").as_slice())
            }
            "antd-icons/outlined/issues-close.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/issues-close.svg").as_slice())
            }
            "antd-icons/outlined/italic.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/italic.svg").as_slice())
            }
            "antd-icons/outlined/java-script.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/java-script.svg").as_slice())
            }
            "antd-icons/outlined/java.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/java.svg").as_slice())
            }
            "antd-icons/outlined/key.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/key.svg").as_slice())
            }
            "antd-icons/outlined/kubernetes.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/kubernetes.svg").as_slice())
            }
            "antd-icons/outlined/laptop.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/laptop.svg").as_slice())
            }
            "antd-icons/outlined/layout.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/layout.svg").as_slice())
            }
            "antd-icons/outlined/left-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/left-circle.svg").as_slice())
            }
            "antd-icons/outlined/left-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/left-square.svg").as_slice())
            }
            "antd-icons/outlined/left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/left.svg").as_slice())
            }
            "antd-icons/outlined/like.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/like.svg").as_slice())
            }
            "antd-icons/outlined/line-chart.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/line-chart.svg").as_slice())
            }
            "antd-icons/outlined/line-height.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/line-height.svg").as_slice())
            }
            "antd-icons/outlined/line.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/line.svg").as_slice())
            }
            "antd-icons/outlined/link.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/link.svg").as_slice())
            }
            "antd-icons/outlined/linkedin.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/linkedin.svg").as_slice())
            }
            "antd-icons/outlined/linux.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/linux.svg").as_slice())
            }
            "antd-icons/outlined/loading-3-quarters.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/loading-3-quarters.svg").as_slice(),
            ),
            "antd-icons/outlined/loading.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/loading.svg").as_slice())
            }
            "antd-icons/outlined/lock.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/lock.svg").as_slice())
            }
            "antd-icons/outlined/login.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/login.svg").as_slice())
            }
            "antd-icons/outlined/logout.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/logout.svg").as_slice())
            }
            "antd-icons/outlined/mac-command.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/mac-command.svg").as_slice())
            }
            "antd-icons/outlined/mail.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/mail.svg").as_slice())
            }
            "antd-icons/outlined/man.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/man.svg").as_slice())
            }
            "antd-icons/outlined/medicine-box.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/medicine-box.svg").as_slice())
            }
            "antd-icons/outlined/medium-workmark.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/medium-workmark.svg").as_slice())
            }
            "antd-icons/outlined/medium.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/medium.svg").as_slice())
            }
            "antd-icons/outlined/meh.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/meh.svg").as_slice())
            }
            "antd-icons/outlined/menu-fold.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/menu-fold.svg").as_slice())
            }
            "antd-icons/outlined/menu-unfold.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/menu-unfold.svg").as_slice())
            }
            "antd-icons/outlined/menu.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/menu.svg").as_slice())
            }
            "antd-icons/outlined/merge-cells.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/merge-cells.svg").as_slice())
            }
            "antd-icons/outlined/merge.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/merge.svg").as_slice())
            }
            "antd-icons/outlined/message.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/message.svg").as_slice())
            }
            "antd-icons/outlined/minus-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/minus-circle.svg").as_slice())
            }
            "antd-icons/outlined/minus-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/minus-square.svg").as_slice())
            }
            "antd-icons/outlined/minus.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/minus.svg").as_slice())
            }
            "antd-icons/outlined/mobile.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/mobile.svg").as_slice())
            }
            "antd-icons/outlined/money-collect.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/money-collect.svg").as_slice())
            }
            "antd-icons/outlined/monitor.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/monitor.svg").as_slice())
            }
            "antd-icons/outlined/moon.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/moon.svg").as_slice())
            }
            "antd-icons/outlined/more.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/more.svg").as_slice())
            }
            "antd-icons/outlined/muted.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/muted.svg").as_slice())
            }
            "antd-icons/outlined/node-collapse.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/node-collapse.svg").as_slice())
            }
            "antd-icons/outlined/node-expand.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/node-expand.svg").as_slice())
            }
            "antd-icons/outlined/node-index.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/node-index.svg").as_slice())
            }
            "antd-icons/outlined/notification.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/notification.svg").as_slice())
            }
            "antd-icons/outlined/number.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/number.svg").as_slice())
            }
            "antd-icons/outlined/one-to-one.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/one-to-one.svg").as_slice())
            }
            "antd-icons/outlined/open-a-i.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/open-a-i.svg").as_slice())
            }
            "antd-icons/outlined/ordered-list.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/ordered-list.svg").as_slice())
            }
            "antd-icons/outlined/paper-clip.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/paper-clip.svg").as_slice())
            }
            "antd-icons/outlined/partition.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/partition.svg").as_slice())
            }
            "antd-icons/outlined/pause-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pause-circle.svg").as_slice())
            }
            "antd-icons/outlined/pause.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pause.svg").as_slice())
            }
            "antd-icons/outlined/pay-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pay-circle.svg").as_slice())
            }
            "antd-icons/outlined/percentage.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/percentage.svg").as_slice())
            }
            "antd-icons/outlined/phone.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/phone.svg").as_slice())
            }
            "antd-icons/outlined/pic-center.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pic-center.svg").as_slice())
            }
            "antd-icons/outlined/pic-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pic-left.svg").as_slice())
            }
            "antd-icons/outlined/pic-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pic-right.svg").as_slice())
            }
            "antd-icons/outlined/picture.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/picture.svg").as_slice())
            }
            "antd-icons/outlined/pie-chart.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pie-chart.svg").as_slice())
            }
            "antd-icons/outlined/pinterest.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pinterest.svg").as_slice())
            }
            "antd-icons/outlined/play-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/play-circle.svg").as_slice())
            }
            "antd-icons/outlined/play-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/play-square.svg").as_slice())
            }
            "antd-icons/outlined/plus-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/plus-circle.svg").as_slice())
            }
            "antd-icons/outlined/plus-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/plus-square.svg").as_slice())
            }
            "antd-icons/outlined/plus.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/plus.svg").as_slice())
            }
            "antd-icons/outlined/pound-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pound-circle.svg").as_slice())
            }
            "antd-icons/outlined/pound.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pound.svg").as_slice())
            }
            "antd-icons/outlined/poweroff.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/poweroff.svg").as_slice())
            }
            "antd-icons/outlined/printer.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/printer.svg").as_slice())
            }
            "antd-icons/outlined/product.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/product.svg").as_slice())
            }
            "antd-icons/outlined/profile.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/profile.svg").as_slice())
            }
            "antd-icons/outlined/project.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/project.svg").as_slice())
            }
            "antd-icons/outlined/property-safety.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/property-safety.svg").as_slice())
            }
            "antd-icons/outlined/pull-request.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pull-request.svg").as_slice())
            }
            "antd-icons/outlined/pushpin.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/pushpin.svg").as_slice())
            }
            "antd-icons/outlined/python.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/python.svg").as_slice())
            }
            "antd-icons/outlined/qq.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/qq.svg").as_slice())
            }
            "antd-icons/outlined/qrcode.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/qrcode.svg").as_slice())
            }
            "antd-icons/outlined/question-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/question-circle.svg").as_slice())
            }
            "antd-icons/outlined/question.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/question.svg").as_slice())
            }
            "antd-icons/outlined/radar-chart.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/radar-chart.svg").as_slice())
            }
            "antd-icons/outlined/radius-bottomleft.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/radius-bottomleft.svg").as_slice(),
            ),
            "antd-icons/outlined/radius-bottomright.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/radius-bottomright.svg").as_slice(),
            ),
            "antd-icons/outlined/radius-setting.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/radius-setting.svg").as_slice())
            }
            "antd-icons/outlined/radius-upleft.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/radius-upleft.svg").as_slice())
            }
            "antd-icons/outlined/radius-upright.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/radius-upright.svg").as_slice())
            }
            "antd-icons/outlined/read.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/read.svg").as_slice())
            }
            "antd-icons/outlined/reconciliation.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/reconciliation.svg").as_slice())
            }
            "antd-icons/outlined/red-envelope.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/red-envelope.svg").as_slice())
            }
            "antd-icons/outlined/reddit.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/reddit.svg").as_slice())
            }
            "antd-icons/outlined/redo.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/redo.svg").as_slice())
            }
            "antd-icons/outlined/reload.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/reload.svg").as_slice())
            }
            "antd-icons/outlined/rest.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/rest.svg").as_slice())
            }
            "antd-icons/outlined/retweet.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/retweet.svg").as_slice())
            }
            "antd-icons/outlined/right-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/right-circle.svg").as_slice())
            }
            "antd-icons/outlined/right-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/right-square.svg").as_slice())
            }
            "antd-icons/outlined/right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/right.svg").as_slice())
            }
            "antd-icons/outlined/rise.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/rise.svg").as_slice())
            }
            "antd-icons/outlined/robot.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/robot.svg").as_slice())
            }
            "antd-icons/outlined/rocket.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/rocket.svg").as_slice())
            }
            "antd-icons/outlined/rollback.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/rollback.svg").as_slice())
            }
            "antd-icons/outlined/rotate-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/rotate-left.svg").as_slice())
            }
            "antd-icons/outlined/rotate-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/rotate-right.svg").as_slice())
            }
            "antd-icons/outlined/ruby.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/ruby.svg").as_slice())
            }
            "antd-icons/outlined/safety-certificate.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/safety-certificate.svg").as_slice(),
            ),
            "antd-icons/outlined/safety.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/safety.svg").as_slice())
            }
            "antd-icons/outlined/save.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/save.svg").as_slice())
            }
            "antd-icons/outlined/scan.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/scan.svg").as_slice())
            }
            "antd-icons/outlined/schedule.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/schedule.svg").as_slice())
            }
            "antd-icons/outlined/scissor.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/scissor.svg").as_slice())
            }
            "antd-icons/outlined/search.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/search.svg").as_slice())
            }
            "antd-icons/outlined/security-scan.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/security-scan.svg").as_slice())
            }
            "antd-icons/outlined/select.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/select.svg").as_slice())
            }
            "antd-icons/outlined/send.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/send.svg").as_slice())
            }
            "antd-icons/outlined/setting.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/setting.svg").as_slice())
            }
            "antd-icons/outlined/shake.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/shake.svg").as_slice())
            }
            "antd-icons/outlined/share-alt.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/share-alt.svg").as_slice())
            }
            "antd-icons/outlined/shop.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/shop.svg").as_slice())
            }
            "antd-icons/outlined/shopping-cart.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/shopping-cart.svg").as_slice())
            }
            "antd-icons/outlined/shopping.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/shopping.svg").as_slice())
            }
            "antd-icons/outlined/shrink.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/shrink.svg").as_slice())
            }
            "antd-icons/outlined/signature.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/signature.svg").as_slice())
            }
            "antd-icons/outlined/sisternode.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/sisternode.svg").as_slice())
            }
            "antd-icons/outlined/sketch.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/sketch.svg").as_slice())
            }
            "antd-icons/outlined/skin.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/skin.svg").as_slice())
            }
            "antd-icons/outlined/skype.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/skype.svg").as_slice())
            }
            "antd-icons/outlined/slack-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/slack-square.svg").as_slice())
            }
            "antd-icons/outlined/slack.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/slack.svg").as_slice())
            }
            "antd-icons/outlined/sliders.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/sliders.svg").as_slice())
            }
            "antd-icons/outlined/small-dash.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/small-dash.svg").as_slice())
            }
            "antd-icons/outlined/smile.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/smile.svg").as_slice())
            }
            "antd-icons/outlined/snippets.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/snippets.svg").as_slice())
            }
            "antd-icons/outlined/solution.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/solution.svg").as_slice())
            }
            "antd-icons/outlined/sort-ascending.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/sort-ascending.svg").as_slice())
            }
            "antd-icons/outlined/sort-descending.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/sort-descending.svg").as_slice())
            }
            "antd-icons/outlined/sound.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/sound.svg").as_slice())
            }
            "antd-icons/outlined/split-cells.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/split-cells.svg").as_slice())
            }
            "antd-icons/outlined/spotify.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/spotify.svg").as_slice())
            }
            "antd-icons/outlined/star.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/star.svg").as_slice())
            }
            "antd-icons/outlined/step-backward.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/step-backward.svg").as_slice())
            }
            "antd-icons/outlined/step-forward.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/step-forward.svg").as_slice())
            }
            "antd-icons/outlined/stock.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/stock.svg").as_slice())
            }
            "antd-icons/outlined/stop.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/stop.svg").as_slice())
            }
            "antd-icons/outlined/strikethrough.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/strikethrough.svg").as_slice())
            }
            "antd-icons/outlined/subnode.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/subnode.svg").as_slice())
            }
            "antd-icons/outlined/sun.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/sun.svg").as_slice())
            }
            "antd-icons/outlined/swap-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/swap-left.svg").as_slice())
            }
            "antd-icons/outlined/swap-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/swap-right.svg").as_slice())
            }
            "antd-icons/outlined/swap.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/swap.svg").as_slice())
            }
            "antd-icons/outlined/switcher.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/switcher.svg").as_slice())
            }
            "antd-icons/outlined/sync.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/sync.svg").as_slice())
            }
            "antd-icons/outlined/table.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/table.svg").as_slice())
            }
            "antd-icons/outlined/tablet.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/tablet.svg").as_slice())
            }
            "antd-icons/outlined/tag.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/tag.svg").as_slice())
            }
            "antd-icons/outlined/tags.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/tags.svg").as_slice())
            }
            "antd-icons/outlined/taobao-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/taobao-circle.svg").as_slice())
            }
            "antd-icons/outlined/taobao.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/taobao.svg").as_slice())
            }
            "antd-icons/outlined/team.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/team.svg").as_slice())
            }
            "antd-icons/outlined/thunderbolt.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/thunderbolt.svg").as_slice())
            }
            "antd-icons/outlined/tik-tok.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/tik-tok.svg").as_slice())
            }
            "antd-icons/outlined/to-top.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/to-top.svg").as_slice())
            }
            "antd-icons/outlined/tool.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/tool.svg").as_slice())
            }
            "antd-icons/outlined/trademark-circle.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/trademark-circle.svg").as_slice(),
            ),
            "antd-icons/outlined/trademark.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/trademark.svg").as_slice())
            }
            "antd-icons/outlined/transaction.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/transaction.svg").as_slice())
            }
            "antd-icons/outlined/translation.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/translation.svg").as_slice())
            }
            "antd-icons/outlined/trophy.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/trophy.svg").as_slice())
            }
            "antd-icons/outlined/truck.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/truck.svg").as_slice())
            }
            "antd-icons/outlined/twitch.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/twitch.svg").as_slice())
            }
            "antd-icons/outlined/twitter.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/twitter.svg").as_slice())
            }
            "antd-icons/outlined/underline.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/underline.svg").as_slice())
            }
            "antd-icons/outlined/undo.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/undo.svg").as_slice())
            }
            "antd-icons/outlined/ungroup.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/ungroup.svg").as_slice())
            }
            "antd-icons/outlined/unlock.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/unlock.svg").as_slice())
            }
            "antd-icons/outlined/unordered-list.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/unordered-list.svg").as_slice())
            }
            "antd-icons/outlined/up-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/up-circle.svg").as_slice())
            }
            "antd-icons/outlined/up-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/up-square.svg").as_slice())
            }
            "antd-icons/outlined/up.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/up.svg").as_slice())
            }
            "antd-icons/outlined/upload.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/upload.svg").as_slice())
            }
            "antd-icons/outlined/usb.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/usb.svg").as_slice())
            }
            "antd-icons/outlined/user-add.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/user-add.svg").as_slice())
            }
            "antd-icons/outlined/user-delete.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/user-delete.svg").as_slice())
            }
            "antd-icons/outlined/user-switch.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/user-switch.svg").as_slice())
            }
            "antd-icons/outlined/user.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/user.svg").as_slice())
            }
            "antd-icons/outlined/usergroup-add.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/usergroup-add.svg").as_slice())
            }
            "antd-icons/outlined/usergroup-delete.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/usergroup-delete.svg").as_slice(),
            ),
            "antd-icons/outlined/verified.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/verified.svg").as_slice())
            }
            "antd-icons/outlined/vertical-align-bottom.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/vertical-align-bottom.svg")
                    .as_slice(),
            ),
            "antd-icons/outlined/vertical-align-middle.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/vertical-align-middle.svg")
                    .as_slice(),
            ),
            "antd-icons/outlined/vertical-align-top.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/vertical-align-top.svg").as_slice(),
            ),
            "antd-icons/outlined/vertical-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/vertical-left.svg").as_slice())
            }
            "antd-icons/outlined/vertical-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/vertical-right.svg").as_slice())
            }
            "antd-icons/outlined/video-camera-add.svg" => Some(
                include_bytes!("../assets/antd-icons/outlined/video-camera-add.svg").as_slice(),
            ),
            "antd-icons/outlined/video-camera.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/video-camera.svg").as_slice())
            }
            "antd-icons/outlined/wallet.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/wallet.svg").as_slice())
            }
            "antd-icons/outlined/warning.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/warning.svg").as_slice())
            }
            "antd-icons/outlined/wechat-work.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/wechat-work.svg").as_slice())
            }
            "antd-icons/outlined/wechat.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/wechat.svg").as_slice())
            }
            "antd-icons/outlined/weibo-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/weibo-circle.svg").as_slice())
            }
            "antd-icons/outlined/weibo-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/weibo-square.svg").as_slice())
            }
            "antd-icons/outlined/weibo.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/weibo.svg").as_slice())
            }
            "antd-icons/outlined/whats-app.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/whats-app.svg").as_slice())
            }
            "antd-icons/outlined/wifi.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/wifi.svg").as_slice())
            }
            "antd-icons/outlined/windows.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/windows.svg").as_slice())
            }
            "antd-icons/outlined/woman.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/woman.svg").as_slice())
            }
            "antd-icons/outlined/x.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/x.svg").as_slice())
            }
            "antd-icons/outlined/yahoo.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/yahoo.svg").as_slice())
            }
            "antd-icons/outlined/youtube.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/youtube.svg").as_slice())
            }
            "antd-icons/outlined/yuque.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/yuque.svg").as_slice())
            }
            "antd-icons/outlined/zhihu.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/zhihu.svg").as_slice())
            }
            "antd-icons/outlined/zoom-in.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/zoom-in.svg").as_slice())
            }
            "antd-icons/outlined/zoom-out.svg" => {
                Some(include_bytes!("../assets/antd-icons/outlined/zoom-out.svg").as_slice())
            }
            "antd-icons/filled/account-book.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/account-book.svg").as_slice())
            }
            "antd-icons/filled/alert.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/alert.svg").as_slice())
            }
            "antd-icons/filled/alipay-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/alipay-circle.svg").as_slice())
            }
            "antd-icons/filled/alipay-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/alipay-square.svg").as_slice())
            }
            "antd-icons/filled/aliwangwang.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/aliwangwang.svg").as_slice())
            }
            "antd-icons/filled/amazon-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/amazon-circle.svg").as_slice())
            }
            "antd-icons/filled/amazon-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/amazon-square.svg").as_slice())
            }
            "antd-icons/filled/android.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/android.svg").as_slice())
            }
            "antd-icons/filled/anthropic.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/anthropic.svg").as_slice())
            }
            "antd-icons/filled/api.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/api.svg").as_slice())
            }
            "antd-icons/filled/apple.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/apple.svg").as_slice())
            }
            "antd-icons/filled/appstore.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/appstore.svg").as_slice())
            }
            "antd-icons/filled/audio.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/audio.svg").as_slice())
            }
            "antd-icons/filled/backward.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/backward.svg").as_slice())
            }
            "antd-icons/filled/bank.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/bank.svg").as_slice())
            }
            "antd-icons/filled/behance-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/behance-circle.svg").as_slice())
            }
            "antd-icons/filled/behance-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/behance-square.svg").as_slice())
            }
            "antd-icons/filled/bell.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/bell.svg").as_slice())
            }
            "antd-icons/filled/bilibili.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/bilibili.svg").as_slice())
            }
            "antd-icons/filled/book.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/book.svg").as_slice())
            }
            "antd-icons/filled/box-plot.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/box-plot.svg").as_slice())
            }
            "antd-icons/filled/bug.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/bug.svg").as_slice())
            }
            "antd-icons/filled/build.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/build.svg").as_slice())
            }
            "antd-icons/filled/bulb.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/bulb.svg").as_slice())
            }
            "antd-icons/filled/calculator.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/calculator.svg").as_slice())
            }
            "antd-icons/filled/calendar.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/calendar.svg").as_slice())
            }
            "antd-icons/filled/camera.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/camera.svg").as_slice())
            }
            "antd-icons/filled/car.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/car.svg").as_slice())
            }
            "antd-icons/filled/caret-down.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/caret-down.svg").as_slice())
            }
            "antd-icons/filled/caret-left.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/caret-left.svg").as_slice())
            }
            "antd-icons/filled/caret-right.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/caret-right.svg").as_slice())
            }
            "antd-icons/filled/caret-up.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/caret-up.svg").as_slice())
            }
            "antd-icons/filled/carry-out.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/carry-out.svg").as_slice())
            }
            "antd-icons/filled/check-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/check-circle.svg").as_slice())
            }
            "antd-icons/filled/check-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/check-square.svg").as_slice())
            }
            "antd-icons/filled/chrome.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/chrome.svg").as_slice())
            }
            "antd-icons/filled/ci-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/ci-circle.svg").as_slice())
            }
            "antd-icons/filled/claude.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/claude.svg").as_slice())
            }
            "antd-icons/filled/clock-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/clock-circle.svg").as_slice())
            }
            "antd-icons/filled/close-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/close-circle.svg").as_slice())
            }
            "antd-icons/filled/close-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/close-square.svg").as_slice())
            }
            "antd-icons/filled/cloud.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/cloud.svg").as_slice())
            }
            "antd-icons/filled/code-sandbox-circle.svg" => Some(
                include_bytes!("../assets/antd-icons/filled/code-sandbox-circle.svg").as_slice(),
            ),
            "antd-icons/filled/code-sandbox-square.svg" => Some(
                include_bytes!("../assets/antd-icons/filled/code-sandbox-square.svg").as_slice(),
            ),
            "antd-icons/filled/code.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/code.svg").as_slice())
            }
            "antd-icons/filled/codepen-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/codepen-circle.svg").as_slice())
            }
            "antd-icons/filled/codepen-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/codepen-square.svg").as_slice())
            }
            "antd-icons/filled/compass.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/compass.svg").as_slice())
            }
            "antd-icons/filled/contacts.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/contacts.svg").as_slice())
            }
            "antd-icons/filled/container.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/container.svg").as_slice())
            }
            "antd-icons/filled/control.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/control.svg").as_slice())
            }
            "antd-icons/filled/copy.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/copy.svg").as_slice())
            }
            "antd-icons/filled/copyright-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/copyright-circle.svg").as_slice())
            }
            "antd-icons/filled/credit-card.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/credit-card.svg").as_slice())
            }
            "antd-icons/filled/crown.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/crown.svg").as_slice())
            }
            "antd-icons/filled/customer-service.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/customer-service.svg").as_slice())
            }
            "antd-icons/filled/dashboard.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/dashboard.svg").as_slice())
            }
            "antd-icons/filled/database.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/database.svg").as_slice())
            }
            "antd-icons/filled/deep-seek.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/deep-seek.svg").as_slice())
            }
            "antd-icons/filled/delete.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/delete.svg").as_slice())
            }
            "antd-icons/filled/diff.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/diff.svg").as_slice())
            }
            "antd-icons/filled/dingtalk-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/dingtalk-circle.svg").as_slice())
            }
            "antd-icons/filled/dingtalk-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/dingtalk-square.svg").as_slice())
            }
            "antd-icons/filled/discord.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/discord.svg").as_slice())
            }
            "antd-icons/filled/dislike.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/dislike.svg").as_slice())
            }
            "antd-icons/filled/dollar-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/dollar-circle.svg").as_slice())
            }
            "antd-icons/filled/down-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/down-circle.svg").as_slice())
            }
            "antd-icons/filled/down-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/down-square.svg").as_slice())
            }
            "antd-icons/filled/dribbble-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/dribbble-circle.svg").as_slice())
            }
            "antd-icons/filled/dribbble-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/dribbble-square.svg").as_slice())
            }
            "antd-icons/filled/dropbox-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/dropbox-circle.svg").as_slice())
            }
            "antd-icons/filled/dropbox-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/dropbox-square.svg").as_slice())
            }
            "antd-icons/filled/edit.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/edit.svg").as_slice())
            }
            "antd-icons/filled/eleven-labs.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/eleven-labs.svg").as_slice())
            }
            "antd-icons/filled/environment.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/environment.svg").as_slice())
            }
            "antd-icons/filled/euro-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/euro-circle.svg").as_slice())
            }
            "antd-icons/filled/exclamation-circle.svg" => Some(
                include_bytes!("../assets/antd-icons/filled/exclamation-circle.svg").as_slice(),
            ),
            "antd-icons/filled/experiment.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/experiment.svg").as_slice())
            }
            "antd-icons/filled/eye-invisible.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/eye-invisible.svg").as_slice())
            }
            "antd-icons/filled/eye.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/eye.svg").as_slice())
            }
            "antd-icons/filled/facebook.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/facebook.svg").as_slice())
            }
            "antd-icons/filled/fast-backward.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/fast-backward.svg").as_slice())
            }
            "antd-icons/filled/fast-forward.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/fast-forward.svg").as_slice())
            }
            "antd-icons/filled/file-add.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-add.svg").as_slice())
            }
            "antd-icons/filled/file-excel.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-excel.svg").as_slice())
            }
            "antd-icons/filled/file-exclamation.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-exclamation.svg").as_slice())
            }
            "antd-icons/filled/file-image.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-image.svg").as_slice())
            }
            "antd-icons/filled/file-markdown.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-markdown.svg").as_slice())
            }
            "antd-icons/filled/file-pdf.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-pdf.svg").as_slice())
            }
            "antd-icons/filled/file-ppt.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-ppt.svg").as_slice())
            }
            "antd-icons/filled/file-text.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-text.svg").as_slice())
            }
            "antd-icons/filled/file-unknown.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-unknown.svg").as_slice())
            }
            "antd-icons/filled/file-word.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-word.svg").as_slice())
            }
            "antd-icons/filled/file-zip.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file-zip.svg").as_slice())
            }
            "antd-icons/filled/file.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/file.svg").as_slice())
            }
            "antd-icons/filled/filter.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/filter.svg").as_slice())
            }
            "antd-icons/filled/fire.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/fire.svg").as_slice())
            }
            "antd-icons/filled/flag.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/flag.svg").as_slice())
            }
            "antd-icons/filled/folder-add.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/folder-add.svg").as_slice())
            }
            "antd-icons/filled/folder-open.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/folder-open.svg").as_slice())
            }
            "antd-icons/filled/folder.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/folder.svg").as_slice())
            }
            "antd-icons/filled/format-painter.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/format-painter.svg").as_slice())
            }
            "antd-icons/filled/forward.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/forward.svg").as_slice())
            }
            "antd-icons/filled/frown.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/frown.svg").as_slice())
            }
            "antd-icons/filled/fund.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/fund.svg").as_slice())
            }
            "antd-icons/filled/funnel-plot.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/funnel-plot.svg").as_slice())
            }
            "antd-icons/filled/gemini.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/gemini.svg").as_slice())
            }
            "antd-icons/filled/gift.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/gift.svg").as_slice())
            }
            "antd-icons/filled/github.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/github.svg").as_slice())
            }
            "antd-icons/filled/gitlab.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/gitlab.svg").as_slice())
            }
            "antd-icons/filled/gold.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/gold.svg").as_slice())
            }
            "antd-icons/filled/golden.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/golden.svg").as_slice())
            }
            "antd-icons/filled/google-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/google-circle.svg").as_slice())
            }
            "antd-icons/filled/google-plus-circle.svg" => Some(
                include_bytes!("../assets/antd-icons/filled/google-plus-circle.svg").as_slice(),
            ),
            "antd-icons/filled/google-plus-square.svg" => Some(
                include_bytes!("../assets/antd-icons/filled/google-plus-square.svg").as_slice(),
            ),
            "antd-icons/filled/google-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/google-square.svg").as_slice())
            }
            "antd-icons/filled/hdd.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/hdd.svg").as_slice())
            }
            "antd-icons/filled/heart.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/heart.svg").as_slice())
            }
            "antd-icons/filled/highlight.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/highlight.svg").as_slice())
            }
            "antd-icons/filled/home.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/home.svg").as_slice())
            }
            "antd-icons/filled/hourglass.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/hourglass.svg").as_slice())
            }
            "antd-icons/filled/html5.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/html5.svg").as_slice())
            }
            "antd-icons/filled/hugging-face.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/hugging-face.svg").as_slice())
            }
            "antd-icons/filled/idcard.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/idcard.svg").as_slice())
            }
            "antd-icons/filled/ie-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/ie-circle.svg").as_slice())
            }
            "antd-icons/filled/ie-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/ie-square.svg").as_slice())
            }
            "antd-icons/filled/info-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/info-circle.svg").as_slice())
            }
            "antd-icons/filled/instagram.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/instagram.svg").as_slice())
            }
            "antd-icons/filled/insurance.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/insurance.svg").as_slice())
            }
            "antd-icons/filled/interaction.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/interaction.svg").as_slice())
            }
            "antd-icons/filled/layout.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/layout.svg").as_slice())
            }
            "antd-icons/filled/left-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/left-circle.svg").as_slice())
            }
            "antd-icons/filled/left-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/left-square.svg").as_slice())
            }
            "antd-icons/filled/like.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/like.svg").as_slice())
            }
            "antd-icons/filled/linkedin.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/linkedin.svg").as_slice())
            }
            "antd-icons/filled/lock.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/lock.svg").as_slice())
            }
            "antd-icons/filled/mac-command.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/mac-command.svg").as_slice())
            }
            "antd-icons/filled/mail.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/mail.svg").as_slice())
            }
            "antd-icons/filled/mastodon.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/mastodon.svg").as_slice())
            }
            "antd-icons/filled/medicine-box.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/medicine-box.svg").as_slice())
            }
            "antd-icons/filled/medium-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/medium-circle.svg").as_slice())
            }
            "antd-icons/filled/medium-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/medium-square.svg").as_slice())
            }
            "antd-icons/filled/meh.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/meh.svg").as_slice())
            }
            "antd-icons/filled/merge.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/merge.svg").as_slice())
            }
            "antd-icons/filled/message.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/message.svg").as_slice())
            }
            "antd-icons/filled/meta.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/meta.svg").as_slice())
            }
            "antd-icons/filled/minus-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/minus-circle.svg").as_slice())
            }
            "antd-icons/filled/minus-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/minus-square.svg").as_slice())
            }
            "antd-icons/filled/mistral.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/mistral.svg").as_slice())
            }
            "antd-icons/filled/mobile.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/mobile.svg").as_slice())
            }
            "antd-icons/filled/money-collect.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/money-collect.svg").as_slice())
            }
            "antd-icons/filled/moon.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/moon.svg").as_slice())
            }
            "antd-icons/filled/muted.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/muted.svg").as_slice())
            }
            "antd-icons/filled/netflix.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/netflix.svg").as_slice())
            }
            "antd-icons/filled/notification.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/notification.svg").as_slice())
            }
            "antd-icons/filled/ollama.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/ollama.svg").as_slice())
            }
            "antd-icons/filled/open-a-i.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/open-a-i.svg").as_slice())
            }
            "antd-icons/filled/pause-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/pause-circle.svg").as_slice())
            }
            "antd-icons/filled/pay-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/pay-circle.svg").as_slice())
            }
            "antd-icons/filled/perplexity.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/perplexity.svg").as_slice())
            }
            "antd-icons/filled/phone.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/phone.svg").as_slice())
            }
            "antd-icons/filled/picture.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/picture.svg").as_slice())
            }
            "antd-icons/filled/pie-chart.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/pie-chart.svg").as_slice())
            }
            "antd-icons/filled/pinterest.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/pinterest.svg").as_slice())
            }
            "antd-icons/filled/play-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/play-circle.svg").as_slice())
            }
            "antd-icons/filled/play-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/play-square.svg").as_slice())
            }
            "antd-icons/filled/plus-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/plus-circle.svg").as_slice())
            }
            "antd-icons/filled/plus-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/plus-square.svg").as_slice())
            }
            "antd-icons/filled/pound-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/pound-circle.svg").as_slice())
            }
            "antd-icons/filled/printer.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/printer.svg").as_slice())
            }
            "antd-icons/filled/product.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/product.svg").as_slice())
            }
            "antd-icons/filled/profile.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/profile.svg").as_slice())
            }
            "antd-icons/filled/project.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/project.svg").as_slice())
            }
            "antd-icons/filled/property-safety.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/property-safety.svg").as_slice())
            }
            "antd-icons/filled/pushpin.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/pushpin.svg").as_slice())
            }
            "antd-icons/filled/qq-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/qq-circle.svg").as_slice())
            }
            "antd-icons/filled/qq-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/qq-square.svg").as_slice())
            }
            "antd-icons/filled/question-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/question-circle.svg").as_slice())
            }
            "antd-icons/filled/qwen.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/qwen.svg").as_slice())
            }
            "antd-icons/filled/read.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/read.svg").as_slice())
            }
            "antd-icons/filled/reconciliation.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/reconciliation.svg").as_slice())
            }
            "antd-icons/filled/red-envelope.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/red-envelope.svg").as_slice())
            }
            "antd-icons/filled/reddit-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/reddit-circle.svg").as_slice())
            }
            "antd-icons/filled/reddit-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/reddit-square.svg").as_slice())
            }
            "antd-icons/filled/replicate.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/replicate.svg").as_slice())
            }
            "antd-icons/filled/rest.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/rest.svg").as_slice())
            }
            "antd-icons/filled/right-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/right-circle.svg").as_slice())
            }
            "antd-icons/filled/right-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/right-square.svg").as_slice())
            }
            "antd-icons/filled/robot.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/robot.svg").as_slice())
            }
            "antd-icons/filled/rocket.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/rocket.svg").as_slice())
            }
            "antd-icons/filled/safety-certificate.svg" => Some(
                include_bytes!("../assets/antd-icons/filled/safety-certificate.svg").as_slice(),
            ),
            "antd-icons/filled/save.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/save.svg").as_slice())
            }
            "antd-icons/filled/schedule.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/schedule.svg").as_slice())
            }
            "antd-icons/filled/security-scan.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/security-scan.svg").as_slice())
            }
            "antd-icons/filled/setting.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/setting.svg").as_slice())
            }
            "antd-icons/filled/shop.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/shop.svg").as_slice())
            }
            "antd-icons/filled/shopping.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/shopping.svg").as_slice())
            }
            "antd-icons/filled/signal.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/signal.svg").as_slice())
            }
            "antd-icons/filled/signature.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/signature.svg").as_slice())
            }
            "antd-icons/filled/sketch-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/sketch-circle.svg").as_slice())
            }
            "antd-icons/filled/sketch-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/sketch-square.svg").as_slice())
            }
            "antd-icons/filled/skin.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/skin.svg").as_slice())
            }
            "antd-icons/filled/skype.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/skype.svg").as_slice())
            }
            "antd-icons/filled/slack-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/slack-circle.svg").as_slice())
            }
            "antd-icons/filled/slack-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/slack-square.svg").as_slice())
            }
            "antd-icons/filled/sliders.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/sliders.svg").as_slice())
            }
            "antd-icons/filled/smile.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/smile.svg").as_slice())
            }
            "antd-icons/filled/snapchat.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/snapchat.svg").as_slice())
            }
            "antd-icons/filled/snippets.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/snippets.svg").as_slice())
            }
            "antd-icons/filled/sound.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/sound.svg").as_slice())
            }
            "antd-icons/filled/spotify.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/spotify.svg").as_slice())
            }
            "antd-icons/filled/star.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/star.svg").as_slice())
            }
            "antd-icons/filled/step-backward.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/step-backward.svg").as_slice())
            }
            "antd-icons/filled/step-forward.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/step-forward.svg").as_slice())
            }
            "antd-icons/filled/stop.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/stop.svg").as_slice())
            }
            "antd-icons/filled/sun.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/sun.svg").as_slice())
            }
            "antd-icons/filled/switcher.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/switcher.svg").as_slice())
            }
            "antd-icons/filled/tablet.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/tablet.svg").as_slice())
            }
            "antd-icons/filled/tag.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/tag.svg").as_slice())
            }
            "antd-icons/filled/tags.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/tags.svg").as_slice())
            }
            "antd-icons/filled/taobao-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/taobao-circle.svg").as_slice())
            }
            "antd-icons/filled/taobao-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/taobao-square.svg").as_slice())
            }
            "antd-icons/filled/telegram.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/telegram.svg").as_slice())
            }
            "antd-icons/filled/threads.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/threads.svg").as_slice())
            }
            "antd-icons/filled/thunderbolt.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/thunderbolt.svg").as_slice())
            }
            "antd-icons/filled/tik-tok.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/tik-tok.svg").as_slice())
            }
            "antd-icons/filled/tool.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/tool.svg").as_slice())
            }
            "antd-icons/filled/trademark-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/trademark-circle.svg").as_slice())
            }
            "antd-icons/filled/trophy.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/trophy.svg").as_slice())
            }
            "antd-icons/filled/truck.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/truck.svg").as_slice())
            }
            "antd-icons/filled/twitch.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/twitch.svg").as_slice())
            }
            "antd-icons/filled/twitter-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/twitter-circle.svg").as_slice())
            }
            "antd-icons/filled/twitter-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/twitter-square.svg").as_slice())
            }
            "antd-icons/filled/unlock.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/unlock.svg").as_slice())
            }
            "antd-icons/filled/up-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/up-circle.svg").as_slice())
            }
            "antd-icons/filled/up-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/up-square.svg").as_slice())
            }
            "antd-icons/filled/usb.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/usb.svg").as_slice())
            }
            "antd-icons/filled/video-camera.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/video-camera.svg").as_slice())
            }
            "antd-icons/filled/wallet.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/wallet.svg").as_slice())
            }
            "antd-icons/filled/warning.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/warning.svg").as_slice())
            }
            "antd-icons/filled/wechat-work.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/wechat-work.svg").as_slice())
            }
            "antd-icons/filled/wechat.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/wechat.svg").as_slice())
            }
            "antd-icons/filled/weibo-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/weibo-circle.svg").as_slice())
            }
            "antd-icons/filled/weibo-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/weibo-square.svg").as_slice())
            }
            "antd-icons/filled/windows.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/windows.svg").as_slice())
            }
            "antd-icons/filled/x.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/x.svg").as_slice())
            }
            "antd-icons/filled/yahoo.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/yahoo.svg").as_slice())
            }
            "antd-icons/filled/youtube.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/youtube.svg").as_slice())
            }
            "antd-icons/filled/yuque.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/yuque.svg").as_slice())
            }
            "antd-icons/filled/zhihu-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/zhihu-circle.svg").as_slice())
            }
            "antd-icons/filled/zhihu-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/filled/zhihu-square.svg").as_slice())
            }
            "antd-icons/twotone/account-book.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/account-book.svg").as_slice())
            }
            "antd-icons/twotone/alert.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/alert.svg").as_slice())
            }
            "antd-icons/twotone/api.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/api.svg").as_slice())
            }
            "antd-icons/twotone/appstore.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/appstore.svg").as_slice())
            }
            "antd-icons/twotone/audio.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/audio.svg").as_slice())
            }
            "antd-icons/twotone/bank.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/bank.svg").as_slice())
            }
            "antd-icons/twotone/bell.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/bell.svg").as_slice())
            }
            "antd-icons/twotone/book.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/book.svg").as_slice())
            }
            "antd-icons/twotone/box-plot.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/box-plot.svg").as_slice())
            }
            "antd-icons/twotone/bug.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/bug.svg").as_slice())
            }
            "antd-icons/twotone/build.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/build.svg").as_slice())
            }
            "antd-icons/twotone/bulb.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/bulb.svg").as_slice())
            }
            "antd-icons/twotone/calculator.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/calculator.svg").as_slice())
            }
            "antd-icons/twotone/calendar.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/calendar.svg").as_slice())
            }
            "antd-icons/twotone/camera.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/camera.svg").as_slice())
            }
            "antd-icons/twotone/car.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/car.svg").as_slice())
            }
            "antd-icons/twotone/carry-out.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/carry-out.svg").as_slice())
            }
            "antd-icons/twotone/check-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/check-circle.svg").as_slice())
            }
            "antd-icons/twotone/check-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/check-square.svg").as_slice())
            }
            "antd-icons/twotone/ci-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/ci-circle.svg").as_slice())
            }
            "antd-icons/twotone/ci.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/ci.svg").as_slice())
            }
            "antd-icons/twotone/clock-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/clock-circle.svg").as_slice())
            }
            "antd-icons/twotone/close-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/close-circle.svg").as_slice())
            }
            "antd-icons/twotone/close-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/close-square.svg").as_slice())
            }
            "antd-icons/twotone/cloud.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/cloud.svg").as_slice())
            }
            "antd-icons/twotone/code.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/code.svg").as_slice())
            }
            "antd-icons/twotone/compass.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/compass.svg").as_slice())
            }
            "antd-icons/twotone/contacts.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/contacts.svg").as_slice())
            }
            "antd-icons/twotone/container.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/container.svg").as_slice())
            }
            "antd-icons/twotone/control.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/control.svg").as_slice())
            }
            "antd-icons/twotone/copy.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/copy.svg").as_slice())
            }
            "antd-icons/twotone/copyright-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/copyright-circle.svg").as_slice())
            }
            "antd-icons/twotone/copyright.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/copyright.svg").as_slice())
            }
            "antd-icons/twotone/credit-card.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/credit-card.svg").as_slice())
            }
            "antd-icons/twotone/crown.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/crown.svg").as_slice())
            }
            "antd-icons/twotone/customer-service.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/customer-service.svg").as_slice())
            }
            "antd-icons/twotone/dashboard.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/dashboard.svg").as_slice())
            }
            "antd-icons/twotone/database.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/database.svg").as_slice())
            }
            "antd-icons/twotone/delete.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/delete.svg").as_slice())
            }
            "antd-icons/twotone/diff.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/diff.svg").as_slice())
            }
            "antd-icons/twotone/dislike.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/dislike.svg").as_slice())
            }
            "antd-icons/twotone/dollar-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/dollar-circle.svg").as_slice())
            }
            "antd-icons/twotone/dollar.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/dollar.svg").as_slice())
            }
            "antd-icons/twotone/down-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/down-circle.svg").as_slice())
            }
            "antd-icons/twotone/down-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/down-square.svg").as_slice())
            }
            "antd-icons/twotone/edit.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/edit.svg").as_slice())
            }
            "antd-icons/twotone/environment.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/environment.svg").as_slice())
            }
            "antd-icons/twotone/euro-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/euro-circle.svg").as_slice())
            }
            "antd-icons/twotone/euro.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/euro.svg").as_slice())
            }
            "antd-icons/twotone/exclamation-circle.svg" => Some(
                include_bytes!("../assets/antd-icons/twotone/exclamation-circle.svg").as_slice(),
            ),
            "antd-icons/twotone/experiment.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/experiment.svg").as_slice())
            }
            "antd-icons/twotone/eye-invisible.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/eye-invisible.svg").as_slice())
            }
            "antd-icons/twotone/eye.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/eye.svg").as_slice())
            }
            "antd-icons/twotone/file-add.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-add.svg").as_slice())
            }
            "antd-icons/twotone/file-excel.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-excel.svg").as_slice())
            }
            "antd-icons/twotone/file-exclamation.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-exclamation.svg").as_slice())
            }
            "antd-icons/twotone/file-image.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-image.svg").as_slice())
            }
            "antd-icons/twotone/file-markdown.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-markdown.svg").as_slice())
            }
            "antd-icons/twotone/file-pdf.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-pdf.svg").as_slice())
            }
            "antd-icons/twotone/file-ppt.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-ppt.svg").as_slice())
            }
            "antd-icons/twotone/file-text.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-text.svg").as_slice())
            }
            "antd-icons/twotone/file-unknown.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-unknown.svg").as_slice())
            }
            "antd-icons/twotone/file-word.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-word.svg").as_slice())
            }
            "antd-icons/twotone/file-zip.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file-zip.svg").as_slice())
            }
            "antd-icons/twotone/file.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/file.svg").as_slice())
            }
            "antd-icons/twotone/filter.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/filter.svg").as_slice())
            }
            "antd-icons/twotone/fire.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/fire.svg").as_slice())
            }
            "antd-icons/twotone/flag.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/flag.svg").as_slice())
            }
            "antd-icons/twotone/folder-add.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/folder-add.svg").as_slice())
            }
            "antd-icons/twotone/folder-open.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/folder-open.svg").as_slice())
            }
            "antd-icons/twotone/folder.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/folder.svg").as_slice())
            }
            "antd-icons/twotone/frown.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/frown.svg").as_slice())
            }
            "antd-icons/twotone/fund.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/fund.svg").as_slice())
            }
            "antd-icons/twotone/funnel-plot.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/funnel-plot.svg").as_slice())
            }
            "antd-icons/twotone/gift.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/gift.svg").as_slice())
            }
            "antd-icons/twotone/gold.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/gold.svg").as_slice())
            }
            "antd-icons/twotone/hdd.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/hdd.svg").as_slice())
            }
            "antd-icons/twotone/heart.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/heart.svg").as_slice())
            }
            "antd-icons/twotone/highlight.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/highlight.svg").as_slice())
            }
            "antd-icons/twotone/home.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/home.svg").as_slice())
            }
            "antd-icons/twotone/hourglass.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/hourglass.svg").as_slice())
            }
            "antd-icons/twotone/html5.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/html5.svg").as_slice())
            }
            "antd-icons/twotone/idcard.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/idcard.svg").as_slice())
            }
            "antd-icons/twotone/info-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/info-circle.svg").as_slice())
            }
            "antd-icons/twotone/insurance.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/insurance.svg").as_slice())
            }
            "antd-icons/twotone/interaction.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/interaction.svg").as_slice())
            }
            "antd-icons/twotone/layout.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/layout.svg").as_slice())
            }
            "antd-icons/twotone/left-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/left-circle.svg").as_slice())
            }
            "antd-icons/twotone/left-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/left-square.svg").as_slice())
            }
            "antd-icons/twotone/like.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/like.svg").as_slice())
            }
            "antd-icons/twotone/lock.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/lock.svg").as_slice())
            }
            "antd-icons/twotone/mail.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/mail.svg").as_slice())
            }
            "antd-icons/twotone/medicine-box.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/medicine-box.svg").as_slice())
            }
            "antd-icons/twotone/meh.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/meh.svg").as_slice())
            }
            "antd-icons/twotone/message.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/message.svg").as_slice())
            }
            "antd-icons/twotone/minus-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/minus-circle.svg").as_slice())
            }
            "antd-icons/twotone/minus-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/minus-square.svg").as_slice())
            }
            "antd-icons/twotone/mobile.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/mobile.svg").as_slice())
            }
            "antd-icons/twotone/money-collect.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/money-collect.svg").as_slice())
            }
            "antd-icons/twotone/notification.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/notification.svg").as_slice())
            }
            "antd-icons/twotone/pause-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/pause-circle.svg").as_slice())
            }
            "antd-icons/twotone/phone.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/phone.svg").as_slice())
            }
            "antd-icons/twotone/picture.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/picture.svg").as_slice())
            }
            "antd-icons/twotone/pie-chart.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/pie-chart.svg").as_slice())
            }
            "antd-icons/twotone/play-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/play-circle.svg").as_slice())
            }
            "antd-icons/twotone/play-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/play-square.svg").as_slice())
            }
            "antd-icons/twotone/plus-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/plus-circle.svg").as_slice())
            }
            "antd-icons/twotone/plus-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/plus-square.svg").as_slice())
            }
            "antd-icons/twotone/pound-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/pound-circle.svg").as_slice())
            }
            "antd-icons/twotone/printer.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/printer.svg").as_slice())
            }
            "antd-icons/twotone/profile.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/profile.svg").as_slice())
            }
            "antd-icons/twotone/project.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/project.svg").as_slice())
            }
            "antd-icons/twotone/property-safety.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/property-safety.svg").as_slice())
            }
            "antd-icons/twotone/pushpin.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/pushpin.svg").as_slice())
            }
            "antd-icons/twotone/question-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/question-circle.svg").as_slice())
            }
            "antd-icons/twotone/reconciliation.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/reconciliation.svg").as_slice())
            }
            "antd-icons/twotone/red-envelope.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/red-envelope.svg").as_slice())
            }
            "antd-icons/twotone/rest.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/rest.svg").as_slice())
            }
            "antd-icons/twotone/right-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/right-circle.svg").as_slice())
            }
            "antd-icons/twotone/right-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/right-square.svg").as_slice())
            }
            "antd-icons/twotone/rocket.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/rocket.svg").as_slice())
            }
            "antd-icons/twotone/safety-certificate.svg" => Some(
                include_bytes!("../assets/antd-icons/twotone/safety-certificate.svg").as_slice(),
            ),
            "antd-icons/twotone/save.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/save.svg").as_slice())
            }
            "antd-icons/twotone/schedule.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/schedule.svg").as_slice())
            }
            "antd-icons/twotone/security-scan.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/security-scan.svg").as_slice())
            }
            "antd-icons/twotone/setting.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/setting.svg").as_slice())
            }
            "antd-icons/twotone/shop.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/shop.svg").as_slice())
            }
            "antd-icons/twotone/shopping.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/shopping.svg").as_slice())
            }
            "antd-icons/twotone/skin.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/skin.svg").as_slice())
            }
            "antd-icons/twotone/sliders.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/sliders.svg").as_slice())
            }
            "antd-icons/twotone/smile.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/smile.svg").as_slice())
            }
            "antd-icons/twotone/snippets.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/snippets.svg").as_slice())
            }
            "antd-icons/twotone/sound.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/sound.svg").as_slice())
            }
            "antd-icons/twotone/star.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/star.svg").as_slice())
            }
            "antd-icons/twotone/stop.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/stop.svg").as_slice())
            }
            "antd-icons/twotone/switcher.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/switcher.svg").as_slice())
            }
            "antd-icons/twotone/tablet.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/tablet.svg").as_slice())
            }
            "antd-icons/twotone/tag.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/tag.svg").as_slice())
            }
            "antd-icons/twotone/tags.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/tags.svg").as_slice())
            }
            "antd-icons/twotone/thunderbolt.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/thunderbolt.svg").as_slice())
            }
            "antd-icons/twotone/tool.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/tool.svg").as_slice())
            }
            "antd-icons/twotone/trademark-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/trademark-circle.svg").as_slice())
            }
            "antd-icons/twotone/trophy.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/trophy.svg").as_slice())
            }
            "antd-icons/twotone/unlock.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/unlock.svg").as_slice())
            }
            "antd-icons/twotone/up-circle.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/up-circle.svg").as_slice())
            }
            "antd-icons/twotone/up-square.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/up-square.svg").as_slice())
            }
            "antd-icons/twotone/usb.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/usb.svg").as_slice())
            }
            "antd-icons/twotone/video-camera.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/video-camera.svg").as_slice())
            }
            "antd-icons/twotone/wallet.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/wallet.svg").as_slice())
            }
            "antd-icons/twotone/warning.svg" => {
                Some(include_bytes!("../assets/antd-icons/twotone/warning.svg").as_slice())
            }
            _ => None,
        };
        if asset_path.starts_with("antd-icons/twotone/") {
            Ok(svg.map(|bytes| {
                let resized = two_tone_image_svg(bytes);
                if let Some((_, layer)) = layer {
                    Cow::Owned(svg_layer_mask(&resized, layer))
                } else {
                    match palette {
                        Some((primary, secondary)) => {
                            Cow::Owned(recolor_svg(&resized, primary, secondary))
                        }
                        None => resized,
                    }
                }
            }))
        } else {
            Ok(svg.map(Cow::Borrowed))
        }
    }

    fn list(&self, prefix: &str) -> Result<Vec<SharedString>> {
        Ok(AntIconName::ALL
            .iter()
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
        assert_eq!(AntIconName::ALL.len(), 848);
        for name in AntIconName::ALL {
            let bytes = source.load(name.path()).unwrap().expect(name.path());
            assert!(bytes.windows(4).any(|w| w == b"<svg"), "{}", name.path());
            assert_eq!(name.path().starts_with("antd-icons/"), true);
        }
        assert_eq!(
            source.list("antd-icons/").unwrap().len(),
            AntIconName::ALL.len()
        );
        assert!(source.load("icons/unknown.svg").unwrap().is_none());
        assert_eq!(source.list("unknown").unwrap().len(), 0);
    }

    #[test]
    fn filled_twotone_and_outlined_present() {
        assert!(AntIconName::CheckOutlined.path().contains("/outlined/"));
        assert!(AntIconName::CheckCircleFilled.path().contains("/filled/"));
        assert!(AntIconName::CheckCircleTwoTone.path().contains("/twotone/"));
        assert_eq!(AntIcon::loading().name(), AntIconName::LoadingOutlined);
    }

    #[test]
    fn two_tone_icons_retain_distinct_blue_and_light_fills() {
        let source = AntIconAssets;
        for name in [
            AntIconName::CarryOutTwoTone,
            AntIconName::CheckCircleTwoTone,
            AntIconName::CheckSquareTwoTone,
            AntIconName::CiCircleTwoTone,
            AntIconName::CiTwoTone,
        ] {
            assert_eq!(name.style(), "twotone");
            let bytes = source.load(name.path()).unwrap().expect(name.path());
            let svg = std::str::from_utf8(&bytes).unwrap();
            assert!(
                svg.contains(r#"<svg width="128" height="128""#),
                "{}",
                name.path()
            );
            assert!(
                svg.contains(r#"viewBox="0 0 1024 1024""#),
                "{}",
                name.path()
            );
            assert!(svg.contains(r##"fill="#1677ff""##), "{}", name.path());
            assert!(svg.contains(r##"fill="#e6f4ff""##), "{}", name.path());
        }
        assert_eq!(
            AntIconName::ALL
                .iter()
                .filter(|name| name.style() == "twotone")
                .count(),
            150
        );
        assert!(matches!(
            source.load(AntIconName::CheckCircleFilled.path()).unwrap(),
            Some(Cow::Borrowed(_))
        ));
    }
    #[test]
    fn two_tone_builder_prioritizes_explicit_colors() {
        let icon = AntIcon::new(AntIconName::CheckCircleTwoTone)
            .theme(AntTheme::dark())
            .two_tone_colors(
                crate::tokens::color(0x52c41a),
                crate::tokens::color(0xf6ffed),
            )
            .color(crate::tokens::color(0xff4d4f));
        assert_eq!(
            palette_asset_key(
                icon.name.path(),
                icon.color.unwrap_or(icon.theme.primary),
                icon.two_tone_secondary.unwrap_or(icon.theme.info_bg),
            ),
            "antd-icons/twotone/check-circle.svg?primary=ff4d4fff&secondary=f6ffedff"
        );
    }
}
