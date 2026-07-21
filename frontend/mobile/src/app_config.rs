use dioxus::prelude::*;

use crate::theme;

pub(crate) const DEFAULT_API_BASE: &str = "http://127.0.0.1:8080/api";
pub(crate) const LOGO: Asset = asset!("/assets/jalanaman-logo.png");
pub(crate) const TUTORIAL_VIDEO: Asset = asset!("/assets/video/videotutorial.mp4");

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum MobileTab {
    Map,
    Route,
    Report,
    Contacts,
    Profile,
    Help,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum MapPresentation {
    TwoDimensional,
    ThreeDimensional,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    pub(crate) fn storage_value(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    pub(crate) fn from_storage(value: &str) -> Self {
        match value {
            "light" => Self::Light,
            _ => Self::Dark,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum NavIconKind {
    Map,
    Route,
    Contacts,
    Profile,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Language {
    Indonesian,
    English,
}

impl Language {
    pub(crate) const fn toggled(self) -> Self {
        match self {
            Self::Indonesian => Self::English,
            Self::English => Self::Indonesian,
        }
    }

    pub(crate) const fn is_indonesian(self) -> bool {
        matches!(self, Self::Indonesian)
    }

    pub(crate) fn text(self, key: CopyKey) -> &'static str {
        app_spec().copy.text(self, key)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CopyKey {
    HeaderSubtitle,
    LanguageToggleTitle,
    SplashSubtitle,
    MapNav,
    RouteNav,
    ContactsNav,
    AccountNav,
    HeaderHelpTitle,
    HelpTitle,
    HelpSubtitle,
    TutorialVideo,
    VideoUnavailable,
    MapLoading,
    ReportsActiveRadius,
    GpsUnavailable,
    LiveMap,
    QuickReportTitle,
    ManualLocation,
    UsePhoneCoordinates,
    Fallback,
    ManualLocationBody,
    UseThisLocation,
    NearbyReports,
    Loading,
    Live,
    NoNearbyReports,
    RefreshGpsReports,
    LastRouteOverlay,
    RouteReportsSuffix,
    SearchDestination,
    SearchDestinationPlaceholder,
    SearchingPlaces,
    CheckSafeRoute,
    CheckingRoute,
    SafeRouteMapTitle,
    LiveRoute,
    RouteScore,
    RouteSafetyStatus,
    NotChecked,
    Weight,
    Status,
    Reports,
    EnterDestinationHint,
    RouteDetails,
    Distance,
    Estimate,
    Mode,
    Walking,
    GpsReady,
    GpsNotReady,
    QuickReport,
    ReportCategory,
    OptionalNote,
    Max100,
    Sending,
    SubmitReport,
    ReportPreview,
    EmergencyContacts,
    ContactsSaved,
    NoContacts,
    RefreshContacts,
    AddSosContact,
    ContactName,
    ContactEmail,
    ContactPhone,
    Saving,
    AddContact,
    AccountPrivacy,
    Anonymous,
    Protected,
    Location,
    Nearby,
    SosContacts,
    SafetySettings,
    SettingsTitle,
    AssistanceMode,
    VoiceAndChat,
    ChatOnly,
    VoiceModeBody,
    VoiceShortcutReady,
    VoiceShortcutDisabled,
    MicrophoneAccess,
    EnableMicrophone,
    TestVoiceSos,
    MicrophoneRequested,
    Appearance,
    DarkMode,
    LightMode,
    NearbyMap,
    Active,
    SosAlerts,
    ReadyToUse,
    LocationNotice,
    SosActiveTitle,
    SosStatusTitle,
    SosActiveBody,
    SosClosedBody,
    StopAlarm,
    Close,
    SosStillActive,
    PreparingHelp,
    SosStopped,
    AppPreparing,
    PreparingSosLocation,
    SosLocationMissing,
    SosActiveSending,
    WhatsappSent,
    WhatsappFallback,
    SosNotified,
    SosNoChannel,
    SosBackendFallback,
    SosStaySafe,
    DestinationMin,
    RouteNeedsLocation,
    RouteFallbackScore,
    ReportNeedsLocation,
    ContactNameMin,
    ContactChannelRequired,
    SelectedReportPoint,
    CurrentGpsLocation,
    DistanceUnavailable,
    ContactPushReady,
    ContactEmailReady,
    ContactWaReady,
    ContactPending,
    ContactNotConnected,
    DeleteContactTitle,
    DashboardBack,
    DashboardTitle,
    DashboardNoteEyebrow,
    DashboardNoteTitle,
    DashboardNoteBody,
    SosAlarmPermissionMissing,
    MapSrcLoadingTitle,
    MapSrcLoadingBody,
    MapSrcFailedTitle,
    MapSrcFailedBody,
    MapSrcThreeDFailedTitle,
    MapSrcThreeDFailedBody,
    MapSrcMarkHint,
    MapSrcMarkConfirm,
    MapSrcMarkCancel,
    MapSrcMarkAccept,
    Latitude,
    Longitude,
    VoiceSosTitle,
    VoiceListening,
    VoiceMatched,
    VoiceNotRecognized,
    VoicePermissionMissing,
    SosButton,
    StopSosButton,
    ContactChannels,
    MediaMp4,
    Map2D,
    Map3D,
    LatitudeInvalid,
    LongitudeInvalid,
    LatitudeRange,
    LongitudeRange,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ReportCategory {
    Lighting,
    Crime,
    Accident,
    Other,
}

impl ReportCategory {
    pub(crate) const fn api_value(self) -> &'static str {
        self.spec().api_value
    }

    pub(crate) const fn label_for(self, language: Language) -> &'static str {
        self.spec().label.text(language)
    }

    pub(crate) const fn short_label_for(self, language: Language) -> &'static str {
        self.spec().short_label.text(language)
    }

    pub(crate) const fn color(self) -> &'static str {
        self.spec().color
    }

    pub(crate) fn from_api(value: &str) -> Self {
        match value {
            "lighting" => Self::Lighting,
            "crime" => Self::Crime,
            "accident" => Self::Accident,
            _ => Self::Other,
        }
    }

    const fn spec(self) -> &'static CategorySpec {
        match self {
            Self::Lighting => &REPORT_CATEGORIES[0],
            Self::Crime => &REPORT_CATEGORIES[1],
            Self::Accident => &REPORT_CATEGORIES[2],
            Self::Other => &REPORT_CATEGORIES[3],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LocalizedText {
    pub(crate) id: &'static str,
    pub(crate) en: &'static str,
}

impl LocalizedText {
    pub(crate) const fn text(self, language: Language) -> &'static str {
        match language {
            Language::Indonesian => self.id,
            Language::English => self.en,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LocalizedCopy {
    pub(crate) key: CopyKey,
    pub(crate) text: LocalizedText,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CopySpec {
    pub(crate) entries: &'static [LocalizedCopy],
}

impl CopySpec {
    pub(crate) fn text(&self, language: Language, key: CopyKey) -> &'static str {
        self.entries
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.text.text(language))
            .unwrap_or("")
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CategorySpec {
    pub(crate) category: ReportCategory,
    pub(crate) api_value: &'static str,
    pub(crate) label: LocalizedText,
    pub(crate) short_label: LocalizedText,
    pub(crate) color: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BrandSpec {
    pub(crate) name: &'static str,
    pub(crate) logo_alt: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct AssetSpec {
    pub(crate) logo: Asset,
    pub(crate) tutorial_video: Asset,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ApiSpec {
    pub(crate) default_base: &'static str,
    pub(crate) report_radius_m: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LimitsSpec {
    pub(crate) place_query_min: usize,
    pub(crate) destination_min: usize,
    pub(crate) destination_max: usize,
    pub(crate) report_note_max: usize,
    pub(crate) contact_name_min: usize,
    pub(crate) contact_name_max: usize,
    pub(crate) contact_email_max: usize,
    pub(crate) contact_phone_max: usize,
    pub(crate) manual_coordinate_max: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct NavSpec {
    pub(crate) icon: NavIconKind,
    pub(crate) label: CopyKey,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SectionKind {
    MapPanel,
    Card,
    MetricGrid,
    Form,
    Notice,
    VideoPanel,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SectionSpec {
    pub(crate) id: &'static str,
    pub(crate) kind: SectionKind,
    pub(crate) title: Option<CopyKey>,
    pub(crate) action: Option<ActionId>,
    pub(crate) fields: &'static [FieldSpec],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FieldKind {
    Text,
    TextArea,
    Tel,
    Coordinate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FieldSpec {
    pub(crate) id: &'static str,
    pub(crate) kind: FieldKind,
    pub(crate) label: CopyKey,
    pub(crate) max_chars: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ActionId {
    RefreshReports,
    SearchRoute,
    SubmitReport,
    RefreshContacts,
    AddContact,
    DeleteContact,
    TriggerSos,
    StopSos,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ActionSpec {
    pub(crate) id: ActionId,
    pub(crate) label: Option<CopyKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ScreenSpec {
    pub(crate) tab: MobileTab,
    pub(crate) nav: Option<NavSpec>,
    pub(crate) nav_slot: Option<usize>,
    pub(crate) title: CopyKey,
    pub(crate) sections: &'static [SectionSpec],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ReportSpec {
    pub(crate) categories: &'static [CategorySpec],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ThemeSpec {
    pub(crate) app: &'static str,
    pub(crate) screen: &'static str,
    pub(crate) header: &'static str,
    pub(crate) brand_wrap: &'static str,
    pub(crate) brand: &'static str,
    pub(crate) subtitle: &'static str,
    pub(crate) header_logo: &'static str,
    pub(crate) icon_button: &'static str,
    pub(crate) header_actions: &'static str,
    pub(crate) language_toggle: &'static str,
    pub(crate) language_segment_active: &'static str,
    pub(crate) language_segment_idle: &'static str,
    pub(crate) content: &'static str,
    pub(crate) map_card: &'static str,
    pub(crate) route_map_card: &'static str,
    pub(crate) map_iframe: &'static str,
    pub(crate) map_label: &'static str,
    pub(crate) map_provider: &'static str,
    pub(crate) card: &'static str,
    pub(crate) card_tight: &'static str,
    pub(crate) row: &'static str,
    pub(crate) eyebrow: &'static str,
    pub(crate) title: &'static str,
    pub(crate) body: &'static str,
    pub(crate) meta_grid: &'static str,
    pub(crate) meta_cell: &'static str,
    pub(crate) meta_value: &'static str,
    pub(crate) meta_label: &'static str,
    pub(crate) field_grid: &'static str,
    pub(crate) input: &'static str,
    pub(crate) textarea: &'static str,
    pub(crate) primary_button: &'static str,
    pub(crate) secondary_button: &'static str,
    pub(crate) category_grid: &'static str,
    pub(crate) category_button: &'static str,
    pub(crate) category_button_active: &'static str,
    pub(crate) bottom_bar: &'static str,
    pub(crate) nav_button: &'static str,
    pub(crate) nav_button_active: &'static str,
    pub(crate) nav_icon: &'static str,
    pub(crate) sos_button: &'static str,
    pub(crate) sos_button_active: &'static str,
    pub(crate) dashboard_wrap: &'static str,
    pub(crate) back_link: &'static str,
    pub(crate) dash_title: &'static str,
    pub(crate) motion_css: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MobileAppSpec {
    pub(crate) brand: BrandSpec,
    pub(crate) assets: AssetSpec,
    pub(crate) api: ApiSpec,
    pub(crate) limits: LimitsSpec,
    pub(crate) copy: CopySpec,
    pub(crate) default_tab: MobileTab,
    pub(crate) default_language: Language,
    pub(crate) screens: &'static [ScreenSpec],
    pub(crate) report: ReportSpec,
    pub(crate) actions: &'static [ActionSpec],
    pub(crate) theme: ThemeSpec,
}

impl MobileAppSpec {
    pub(crate) const fn nav_screens(&self) -> &'static [ScreenSpec] {
        self.screens
    }
}

pub(crate) fn app_spec() -> &'static MobileAppSpec {
    &APP_SPEC
}

macro_rules! copy_entry {
    ($key:ident, $id:expr, $en:expr) => {
        LocalizedCopy {
            key: CopyKey::$key,
            text: LocalizedText { id: $id, en: $en },
        }
    };
}

const COPY_ENTRIES: &[LocalizedCopy] = &[
    copy_entry!(HeaderSubtitle, "Temukan rute yang lebih aman", "Find safer routes"),
    copy_entry!(LanguageToggleTitle, "Ganti bahasa", "Switch language"),
    copy_entry!(SplashSubtitle, "Melangkah dengan rasa aman.", "Move with confidence."),
    copy_entry!(MapNav, "Peta", "Map"),
    copy_entry!(RouteNav, "Rute", "Route"),
    copy_entry!(ContactsNav, "Kontak", "Contacts"),
    copy_entry!(AccountNav, "Akun", "Account"),
    copy_entry!(HeaderHelpTitle, "Bantuan", "Help"),
    copy_entry!(HelpTitle, "Bantuan JalanAman", "JalanAman Help"),
    copy_entry!(HelpSubtitle, "Video tutorial singkat untuk memakai fitur utama.", "A short tutorial video for the main features."),
    copy_entry!(TutorialVideo, "Video tutorial", "Tutorial video"),
    copy_entry!(VideoUnavailable, "Video tidak dapat diputar di perangkat ini.", "Video cannot be played on this device."),
    copy_entry!(MapLoading, "Memuat peta dan laporan", "Loading map and reports"),
    copy_entry!(ReportsActiveRadius, "laporan aktif radius 800 m", "active reports within 800 m"),
    copy_entry!(GpsUnavailable, "GPS belum tersedia", "GPS unavailable"),
    copy_entry!(LiveMap, "Peta langsung", "Live map"),
    copy_entry!(QuickReportTitle, "Lapor cepat", "Quick report"),
    copy_entry!(ManualLocation, "Lokasi manual", "Manual location"),
    copy_entry!(UsePhoneCoordinates, "Pakai koordinat HP", "Use phone coordinates"),
    copy_entry!(Fallback, "Fallback", "Fallback"),
    copy_entry!(ManualLocationBody, "Lokasi belum tersedia. Aktifkan izin lokasi atau masukkan koordinat untuk melanjutkan.", "Location is unavailable. Enable location permission or enter coordinates to continue."),
    copy_entry!(UseThisLocation, "Pakai lokasi ini", "Use this location"),
    copy_entry!(NearbyReports, "Laporan terdekat", "Nearby reports"),
    copy_entry!(Loading, "Loading", "Loading"),
    copy_entry!(Live, "Live", "Live"),
    copy_entry!(NoNearbyReports, "Belum ada laporan aktif dari user lain di radius ini.", "No active reports from other users in this radius."),
    copy_entry!(RefreshGpsReports, "Refresh GPS & laporan", "Refresh GPS & reports"),
    copy_entry!(LastRouteOverlay, "Overlay rute terakhir", "Last route overlay"),
    copy_entry!(RouteReportsSuffix, "laporan di rute", "reports on route"),
    copy_entry!(SearchDestination, "Cari tujuan", "Search destination"),
    copy_entry!(SearchDestinationPlaceholder, "Cari tempat atau alamat", "Search place or address"),
    copy_entry!(SearchingPlaces, "Mencari tempat...", "Searching places..."),
    copy_entry!(CheckSafeRoute, "Cek rute aman", "Check safer route"),
    copy_entry!(CheckingRoute, "Mengecek rute...", "Checking route..."),
    copy_entry!(SafeRouteMapTitle, "Peta rute aman", "Safe route map"),
    copy_entry!(LiveRoute, "Rute langsung", "Live route"),
    copy_entry!(RouteScore, "Skor rute", "Route score"),
    copy_entry!(RouteSafetyStatus, "Status keamanan rute", "Route safety status"),
    copy_entry!(NotChecked, "Belum dicek", "Not checked"),
    copy_entry!(Weight, "Bobot", "Weight"),
    copy_entry!(Status, "Status", "Status"),
    copy_entry!(Reports, "Laporan", "Reports"),
    copy_entry!(EnterDestinationHint, "Masukkan tujuan untuk melihat skor keamanan rute.", "Enter a destination to see the route safety score."),
    copy_entry!(RouteDetails, "Detail rute", "Route details"),
    copy_entry!(Distance, "Jarak", "Distance"),
    copy_entry!(Estimate, "Estimasi", "Estimate"),
    copy_entry!(Mode, "Mode", "Mode"),
    copy_entry!(Walking, "Jalan kaki", "Walking"),
    copy_entry!(GpsReady, "GPS siap", "GPS ready"),
    copy_entry!(GpsNotReady, "GPS belum siap", "GPS not ready"),
    copy_entry!(QuickReport, "Lapor cepat", "Quick report"),
    copy_entry!(ReportCategory, "Kategori laporan", "Report category"),
    copy_entry!(OptionalNote, "Catatan opsional", "Optional note"),
    copy_entry!(Max100, "Maks 100 karakter", "Max 100 characters"),
    copy_entry!(Sending, "Mengirim...", "Sending..."),
    copy_entry!(SubmitReport, "Kirim laporan", "Submit report"),
    copy_entry!(ReportPreview, "Preview laporan", "Report preview"),
    copy_entry!(EmergencyContacts, "Kontak darurat", "Emergency contacts"),
    copy_entry!(ContactsSaved, "kontak tersimpan", "saved contacts"),
    copy_entry!(NoContacts, "Belum ada kontak. Tambahkan email atau nomor WhatsApp agar SOS bisa mengirim alert.", "No contacts yet. Add an email or WhatsApp number so SOS can send alerts."),
    copy_entry!(RefreshContacts, "Refresh kontak", "Refresh contacts"),
    copy_entry!(AddSosContact, "Tambah kontak SOS", "Add SOS contact"),
    copy_entry!(ContactName, "Nama kontak", "Contact name"),
    copy_entry!(ContactEmail, "Email kontak", "Contact email"),
    copy_entry!(ContactPhone, "Nomor WhatsApp, contoh 08123456789", "WhatsApp number, e.g. 08123456789"),
    copy_entry!(Saving, "Menyimpan...", "Saving..."),
    copy_entry!(AddContact, "Tambah kontak", "Add contact"),
    copy_entry!(AccountPrivacy, "Akun & privasi", "Account & privacy"),
    copy_entry!(Anonymous, "Kamu tetap anonim", "You stay anonymous"),
    copy_entry!(Protected, "Terlindungi", "Protected"),
    copy_entry!(Location, "Lokasi", "Location"),
    copy_entry!(Nearby, "Di sekitar", "Nearby"),
    copy_entry!(SosContacts, "Kontak SOS", "SOS contacts"),
    copy_entry!(SafetySettings, "Pengaturan keamanan", "Safety settings"),
    copy_entry!(SettingsTitle, "Settings", "Settings"),
    copy_entry!(AssistanceMode, "Mode bantuan", "Assistance mode"),
    copy_entry!(VoiceAndChat, "Voice + Chat", "Voice + Chat"),
    copy_entry!(ChatOnly, "Chat saja", "Chat only"),
    copy_entry!(VoiceModeBody, "Voice dipakai saat widget atau shortcut Voice SOS membuka aplikasi. Chat saja tetap memakai tombol SOS, email, dan WhatsApp fallback.", "Voice is used when a Voice SOS widget or shortcut opens the app. Chat only still uses the SOS button, email, and WhatsApp fallback."),
    copy_entry!(VoiceShortcutReady, "Voice SOS siap untuk widget atau shortcut.", "Voice SOS is ready for a widget or shortcut."),
    copy_entry!(VoiceShortcutDisabled, "Voice SOS belum aktif. Pilih Voice + Chat di Settings dulu.", "Voice SOS is not enabled. Choose Voice + Chat in Settings first."),
    copy_entry!(MicrophoneAccess, "Akses mikrofon", "Microphone access"),
    copy_entry!(EnableMicrophone, "Aktifkan mikrofon", "Enable microphone"),
    copy_entry!(TestVoiceSos, "Coba Voice SOS", "Test Voice SOS"),
    copy_entry!(MicrophoneRequested, "Izin mikrofon diminta. Jika prompt muncul, pilih Izinkan.", "Microphone permission requested. If a prompt appears, choose Allow."),
    copy_entry!(Appearance, "Tampilan", "Appearance"),
    copy_entry!(DarkMode, "Dark", "Dark"),
    copy_entry!(LightMode, "Light", "Light"),
    copy_entry!(NearbyMap, "Peta sekitar", "Nearby map"),
    copy_entry!(Active, "Aktif", "Active"),
    copy_entry!(SosAlerts, "Peringatan SOS", "SOS alerts"),
    copy_entry!(ReadyToUse, "Siap digunakan", "Ready"),
    copy_entry!(LocationNotice, "Lokasi belum tersedia. Aktifkan izin lokasi agar peta dan SOS bekerja akurat.", "Location is unavailable. Enable location permission so map and SOS work accurately."),
    copy_entry!(SosActiveTitle, "SOS sedang aktif", "SOS is active"),
    copy_entry!(SosStatusTitle, "Status SOS", "SOS status"),
    copy_entry!(SosActiveBody, "Alarm dan getaran akan terus aktif sampai dihentikan.", "Alarm and vibration will stay active until stopped."),
    copy_entry!(SosClosedBody, "Kamu dapat menutup pemberitahuan ini.", "You can close this notice."),
    copy_entry!(StopAlarm, "Hentikan alarm", "Stop alarm"),
    copy_entry!(Close, "Tutup", "Close"),
    copy_entry!(SosStillActive, "Alarm SOS masih aktif. Hentikan lewat tombol ini atau notifikasi sistem.", "SOS alarm is still active. Stop it here or from the system notification."),
    copy_entry!(PreparingHelp, "Kami sedang menyiapkan bantuan untukmu.", "Preparing help for you."),
    copy_entry!(SosStopped, "Alarm SOS sudah dihentikan.", "SOS alarm has been stopped."),
    copy_entry!(AppPreparing, "Aplikasi sedang disiapkan. Coba lagi sebentar.", "The app is still preparing. Try again shortly."),
    copy_entry!(PreparingSosLocation, "Menyiapkan SOS dengan lokasi terkini...", "Preparing SOS with your latest location..."),
    copy_entry!(SosLocationMissing, "Lokasi belum didapatkan. Aktifkan izin lokasi, lalu coba SOS lagi.", "Location is not available yet. Enable location permission, then try SOS again."),
    copy_entry!(SosActiveSending, "Alarm SOS aktif. Lokasi dan permintaan bantuan sedang dikirim ke kontak darurat.", "SOS alarm is active. Your location and help request are being sent to emergency contacts."),
    copy_entry!(WhatsappSent, "Permintaan bantuan otomatis terkirim via WhatsApp. Alarm tetap aktif sampai dihentikan.", "Help request was sent automatically via WhatsApp. Alarm stays active until stopped."),
    copy_entry!(WhatsappFallback, "Kontak darurat sudah diproses. WhatsApp dibuka sebagai fallback karena auto-send belum tersedia untuk nomor ini. Alarm tetap aktif sampai dihentikan.", "Emergency contacts were processed. WhatsApp opened as fallback because auto-send is not available for this number. Alarm stays active until stopped."),
    copy_entry!(SosNotified, "Permintaan bantuan dikirim ke kontak darurat. Alarm tetap aktif sampai dihentikan.", "Help request was sent to emergency contacts. Alarm stays active until stopped."),
    copy_entry!(SosNoChannel, "Alarm tetap aktif, tetapi belum ada kanal otomatis yang berhasil. Pastikan backend/WhatsApp API aktif atau hubungi orang terdekat.", "Alarm stays active, but no automatic channel succeeded. Check backend/WhatsApp API or contact someone nearby."),
    copy_entry!(SosBackendFallback, "Backend belum terhubung. WhatsApp dibuka sebagai fallback agar pesan bisa segera dikirim. Alarm tetap aktif sampai dihentikan.", "Backend is not connected. WhatsApp opened as fallback so the message can be sent quickly. Alarm stays active until stopped."),
    copy_entry!(SosStaySafe, "Alarm tetap aktif. Pastikan koneksi internet lalu hubungi orang terdekat di sekitarmu.", "Alarm stays active. Check your internet connection and contact someone nearby."),
    copy_entry!(DestinationMin, "Tujuan minimal 3 karakter.", "Destination needs at least 3 characters."),
    copy_entry!(RouteNeedsLocation, "Lokasi belum tersedia. Isi koordinat manual di tab Peta dulu.", "Location is unavailable. Enter manual coordinates in the Map tab first."),
    copy_entry!(RouteFallbackScore, "Rute tetap tampil. Skor sementara dihitung dari laporan di sekitar kamu.", "Route is still shown. Temporary score is calculated from nearby reports."),
    copy_entry!(ReportNeedsLocation, "Lokasi belum tersedia. Isi koordinat manual di tab Peta dulu.", "Location is unavailable. Enter manual coordinates in the Map tab first."),
    copy_entry!(ContactNameMin, "Nama kontak minimal 2 karakter.", "Contact name needs at least 2 characters."),
    copy_entry!(ContactChannelRequired, "Isi email atau nomor WhatsApp kontak.", "Enter the contact email or WhatsApp number."),
    copy_entry!(SelectedReportPoint, "Titik laporan yang dipilih", "Selected report point"),
    copy_entry!(CurrentGpsLocation, "Lokasi GPS saat ini", "Current GPS location"),
    copy_entry!(DistanceUnavailable, "jarak belum ada", "distance unavailable"),
    copy_entry!(ContactPushReady, "Push siap", "Push ready"),
    copy_entry!(ContactEmailReady, "Email siap", "Email ready"),
    copy_entry!(ContactWaReady, "WA siap", "WA ready"),
    copy_entry!(ContactPending, "Menunggu", "Pending"),
    copy_entry!(ContactNotConnected, "Belum tersambung", "Not connected"),
    copy_entry!(DeleteContactTitle, "Hapus kontak", "Delete contact"),
    copy_entry!(DashboardBack, "< Kembali", "< Back"),
    copy_entry!(DashboardTitle, "Dashboard mobile", "Mobile dashboard"),
    copy_entry!(DashboardNoteEyebrow, "Catatan", "Note"),
    copy_entry!(DashboardNoteTitle, "Dashboard stakeholder ada di frontend/web", "Stakeholder dashboard is in frontend/web"),
    copy_entry!(DashboardNoteBody, "Mobile fokus untuk user lapangan: peta, rute, laporan cepat, kontak, dan SOS.", "Mobile focuses on field users: map, route, quick reports, contacts, and SOS."),
    copy_entry!(SosAlarmPermissionMissing, "Alarm SOS belum dapat dimulai. Pastikan izin notifikasi dan suara untuk JalanAman sudah diizinkan, lalu coba lagi.", "SOS alarm could not start. Allow notifications and sound for JalanAman, then try again."),
    copy_entry!(MapSrcLoadingTitle, "Memuat peta", "Loading map"),
    copy_entry!(MapSrcLoadingBody, "Menyiapkan peta interaktif...", "Preparing the interactive map..."),
    copy_entry!(MapSrcFailedTitle, "Peta belum tersedia", "Map unavailable"),
    copy_entry!(MapSrcFailedBody, "Periksa koneksi internet lalu coba refresh.", "Check your internet connection and refresh."),
    copy_entry!(MapSrcThreeDFailedTitle, "Mode 3D belum tersedia", "3D mode unavailable"),
    copy_entry!(MapSrcThreeDFailedBody, "Peta 2D tetap dapat digunakan.", "The 2D map is still available."),
    copy_entry!(MapSrcMarkHint, "Geser pejalan kaki untuk memilih titik laporan", "Drag the walker to choose a report point"),
    copy_entry!(MapSrcMarkConfirm, "Gunakan titik ini untuk laporan?", "Use this point for the report?"),
    copy_entry!(MapSrcMarkCancel, "Batal", "Cancel"),
    copy_entry!(MapSrcMarkAccept, "Gunakan titik", "Use point"),
    copy_entry!(Latitude, "Latitude", "Latitude"),
    copy_entry!(Longitude, "Longitude", "Longitude"),
    copy_entry!(VoiceSosTitle, "Voice SOS", "Voice SOS"),
    copy_entry!(VoiceListening, "Mendengarkan perintah suara...", "Listening for a voice command..."),
    copy_entry!(VoiceMatched, "Perintah suara dikenali. Menyiapkan SOS...", "Voice command recognized. Preparing SOS..."),
    copy_entry!(VoiceNotRecognized, "Perintah suara tidak dikenali. Ucapkan JalanAman SOS, tolong, atau bantuan.", "Voice command was not recognized. Say JalanAman SOS, help, or emergency."),
    copy_entry!(VoicePermissionMissing, "Izin mikrofon belum aktif. Izinkan mikrofon untuk memakai Voice SOS.", "Microphone permission is not enabled. Allow microphone access to use Voice SOS."),
    copy_entry!(SosButton, "SOS", "SOS"),
    copy_entry!(StopSosButton, "STOP", "STOP"),
    copy_entry!(ContactChannels, "Email/WA/SOS", "Email/WA/SOS"),
    copy_entry!(MediaMp4, "MP4", "MP4"),
    copy_entry!(Map2D, "2D", "2D"),
    copy_entry!(Map3D, "3D", "3D"),
    copy_entry!(LatitudeInvalid, "Latitude belum valid.", "Latitude is invalid."),
    copy_entry!(LongitudeInvalid, "Longitude belum valid.", "Longitude is invalid."),
    copy_entry!(LatitudeRange, "Latitude harus antara -90 sampai 90.", "Latitude must be between -90 and 90."),
    copy_entry!(LongitudeRange, "Longitude harus antara -180 sampai 180.", "Longitude must be between -180 and 180."),
];

const REPORT_CATEGORIES: [CategorySpec; 4] = [
    CategorySpec {
        category: ReportCategory::Lighting,
        api_value: "lighting",
        label: LocalizedText {
            id: "Pencahayaan buruk",
            en: "Poor lighting",
        },
        short_label: LocalizedText {
            id: "Gelap",
            en: "Dark",
        },
        color: "#f59e0b",
    },
    CategorySpec {
        category: ReportCategory::Crime,
        api_value: "crime",
        label: LocalizedText {
            id: "Rawan kriminal",
            en: "Crime risk",
        },
        short_label: LocalizedText {
            id: "Kriminal",
            en: "Crime",
        },
        color: "#ef4444",
    },
    CategorySpec {
        category: ReportCategory::Accident,
        api_value: "accident",
        label: LocalizedText {
            id: "Rawan kecelakaan",
            en: "Accident risk",
        },
        short_label: LocalizedText {
            id: "Kecelakaan",
            en: "Accident",
        },
        color: "#f97316",
    },
    CategorySpec {
        category: ReportCategory::Other,
        api_value: "other",
        label: LocalizedText {
            id: "Lainnya",
            en: "Other",
        },
        short_label: LocalizedText {
            id: "Lainnya",
            en: "Other",
        },
        color: "#94a3b8",
    },
];

const MAP_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        id: "latitude",
        kind: FieldKind::Coordinate,
        label: CopyKey::Latitude,
        max_chars: Some(24),
    },
    FieldSpec {
        id: "longitude",
        kind: FieldKind::Coordinate,
        label: CopyKey::Longitude,
        max_chars: Some(24),
    },
];

const ROUTE_FIELDS: &[FieldSpec] = &[FieldSpec {
    id: "destination",
    kind: FieldKind::Text,
    label: CopyKey::SearchDestination,
    max_chars: Some(80),
}];

const REPORT_FIELDS: &[FieldSpec] = &[FieldSpec {
    id: "note",
    kind: FieldKind::TextArea,
    label: CopyKey::OptionalNote,
    max_chars: Some(100),
}];

const CONTACT_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        id: "name",
        kind: FieldKind::Text,
        label: CopyKey::ContactName,
        max_chars: Some(48),
    },
    FieldSpec {
        id: "email",
        kind: FieldKind::Text,
        label: CopyKey::ContactEmail,
        max_chars: Some(90),
    },
    FieldSpec {
        id: "phone",
        kind: FieldKind::Tel,
        label: CopyKey::ContactPhone,
        max_chars: Some(24),
    },
];

const MAP_SECTIONS: &[SectionSpec] = &[
    SectionSpec {
        id: "live-map",
        kind: SectionKind::MapPanel,
        title: Some(CopyKey::LiveMap),
        action: Some(ActionId::RefreshReports),
        fields: MAP_FIELDS,
    },
    SectionSpec {
        id: "nearby-reports",
        kind: SectionKind::MetricGrid,
        title: Some(CopyKey::NearbyReports),
        action: None,
        fields: &[],
    },
    SectionSpec {
        id: "location-notice",
        kind: SectionKind::Notice,
        title: Some(CopyKey::LocationNotice),
        action: None,
        fields: &[],
    },
];

const ROUTE_SECTIONS: &[SectionSpec] = &[
    SectionSpec {
        id: "route-search",
        kind: SectionKind::Form,
        title: Some(CopyKey::SearchDestination),
        action: Some(ActionId::SearchRoute),
        fields: ROUTE_FIELDS,
    },
    SectionSpec {
        id: "route-map",
        kind: SectionKind::MapPanel,
        title: Some(CopyKey::SafeRouteMapTitle),
        action: None,
        fields: &[],
    },
];

const REPORT_SECTIONS: &[SectionSpec] = &[SectionSpec {
    id: "report-form",
    kind: SectionKind::Form,
    title: Some(CopyKey::QuickReport),
    action: Some(ActionId::SubmitReport),
    fields: REPORT_FIELDS,
}];

const CONTACT_SECTIONS: &[SectionSpec] = &[
    SectionSpec {
        id: "contact-list",
        kind: SectionKind::Card,
        title: Some(CopyKey::EmergencyContacts),
        action: Some(ActionId::RefreshContacts),
        fields: &[],
    },
    SectionSpec {
        id: "contact-form",
        kind: SectionKind::Form,
        title: Some(CopyKey::AddSosContact),
        action: Some(ActionId::AddContact),
        fields: CONTACT_FIELDS,
    },
];

const PROFILE_SECTIONS: &[SectionSpec] = &[SectionSpec {
    id: "profile-summary",
    kind: SectionKind::MetricGrid,
    title: Some(CopyKey::AccountPrivacy),
    action: None,
    fields: &[],
}];

const HELP_SECTIONS: &[SectionSpec] = &[SectionSpec {
    id: "tutorial",
    kind: SectionKind::VideoPanel,
    title: Some(CopyKey::TutorialVideo),
    action: None,
    fields: &[],
}];

const ACTIONS: &[ActionSpec] = &[
    ActionSpec {
        id: ActionId::RefreshReports,
        label: Some(CopyKey::RefreshGpsReports),
    },
    ActionSpec {
        id: ActionId::SearchRoute,
        label: Some(CopyKey::CheckSafeRoute),
    },
    ActionSpec {
        id: ActionId::SubmitReport,
        label: Some(CopyKey::SubmitReport),
    },
    ActionSpec {
        id: ActionId::RefreshContacts,
        label: Some(CopyKey::RefreshContacts),
    },
    ActionSpec {
        id: ActionId::AddContact,
        label: Some(CopyKey::AddContact),
    },
    ActionSpec {
        id: ActionId::DeleteContact,
        label: Some(CopyKey::DeleteContactTitle),
    },
    ActionSpec {
        id: ActionId::TriggerSos,
        label: Some(CopyKey::SosButton),
    },
    ActionSpec {
        id: ActionId::StopSos,
        label: Some(CopyKey::StopSosButton),
    },
];

const SCREENS: &[ScreenSpec] = &[
    ScreenSpec {
        tab: MobileTab::Map,
        nav: Some(NavSpec {
            icon: NavIconKind::Map,
            label: CopyKey::MapNav,
        }),
        nav_slot: Some(0),
        title: CopyKey::MapNav,
        sections: MAP_SECTIONS,
    },
    ScreenSpec {
        tab: MobileTab::Route,
        nav: Some(NavSpec {
            icon: NavIconKind::Route,
            label: CopyKey::RouteNav,
        }),
        nav_slot: Some(1),
        title: CopyKey::RouteNav,
        sections: ROUTE_SECTIONS,
    },
    ScreenSpec {
        tab: MobileTab::Report,
        nav: None,
        nav_slot: None,
        title: CopyKey::QuickReport,
        sections: REPORT_SECTIONS,
    },
    ScreenSpec {
        tab: MobileTab::Contacts,
        nav: Some(NavSpec {
            icon: NavIconKind::Contacts,
            label: CopyKey::ContactsNav,
        }),
        nav_slot: Some(3),
        title: CopyKey::ContactsNav,
        sections: CONTACT_SECTIONS,
    },
    ScreenSpec {
        tab: MobileTab::Profile,
        nav: Some(NavSpec {
            icon: NavIconKind::Profile,
            label: CopyKey::AccountNav,
        }),
        nav_slot: Some(4),
        title: CopyKey::AccountNav,
        sections: PROFILE_SECTIONS,
    },
    ScreenSpec {
        tab: MobileTab::Help,
        nav: None,
        nav_slot: None,
        title: CopyKey::HelpTitle,
        sections: HELP_SECTIONS,
    },
];

pub(crate) const APP_SPEC: MobileAppSpec = MobileAppSpec {
    brand: BrandSpec {
        name: "JalanAman",
        logo_alt: "Logo JalanAman",
    },
    assets: AssetSpec {
        logo: LOGO,
        tutorial_video: TUTORIAL_VIDEO,
    },
    api: ApiSpec {
        default_base: DEFAULT_API_BASE,
        report_radius_m: 800,
    },
    limits: LimitsSpec {
        place_query_min: 2,
        destination_min: 3,
        destination_max: 80,
        report_note_max: 100,
        contact_name_min: 2,
        contact_name_max: 48,
        contact_email_max: 90,
        contact_phone_max: 24,
        manual_coordinate_max: 24,
    },
    copy: CopySpec {
        entries: COPY_ENTRIES,
    },
    default_tab: MobileTab::Map,
    default_language: Language::Indonesian,
    screens: SCREENS,
    report: ReportSpec {
        categories: &REPORT_CATEGORIES,
    },
    actions: ACTIONS,
    theme: theme::THEME,
};
