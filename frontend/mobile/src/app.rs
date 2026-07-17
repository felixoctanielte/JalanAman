use dioxus::prelude::*;
use jalanaman_shared::{
    AddContactPayload, CreateReportPayload, DirectionsResponse, EmergencyContact, PlaceSuggestion,
    Report, RouteScorePayload, RouteScoreResponse, SosTriggerPayload, SosTriggerResponse, Waypoint,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_API_BASE: &str = "http://127.0.0.1:8080/api";

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/:..segments")]
    Fallback { segments: Vec<String> },
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MobileTab {
    Map,
    Route,
    Report,
    Contacts,
    Profile,
    Help,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapPresentation {
    TwoDimensional,
    ThreeDimensional,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum NavIconKind {
    Map,
    Route,
    Contacts,
    Profile,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Language {
    Indonesian,
    English,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CopyKey {
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
}

impl Language {
    const fn toggled(self) -> Self {
        match self {
            Self::Indonesian => Self::English,
            Self::English => Self::Indonesian,
        }
    }

    const fn is_indonesian(self) -> bool {
        matches!(self, Self::Indonesian)
    }

    const fn text(self, key: CopyKey) -> &'static str {
        match self {
            Self::Indonesian => match key {
                CopyKey::HeaderSubtitle => "Temukan rute yang lebih aman",
                CopyKey::LanguageToggleTitle => "Ganti bahasa",
                CopyKey::SplashSubtitle => "Melangkah dengan rasa aman.",
                CopyKey::MapNav => "Peta",
                CopyKey::RouteNav => "Rute",
                CopyKey::ContactsNav => "Kontak",
                CopyKey::AccountNav => "Akun",
                CopyKey::HeaderHelpTitle => "Bantuan",
                CopyKey::HelpTitle => "Bantuan JalanAman",
                CopyKey::HelpSubtitle => "Video tutorial singkat untuk memakai fitur utama.",
                CopyKey::TutorialVideo => "Video tutorial",
                CopyKey::VideoUnavailable => "Video tidak dapat diputar di perangkat ini.",
                CopyKey::MapLoading => "Memuat peta dan laporan",
                CopyKey::ReportsActiveRadius => "laporan aktif radius 800 m",
                CopyKey::GpsUnavailable => "GPS belum tersedia",
                CopyKey::LiveMap => "Peta langsung",
                CopyKey::QuickReportTitle => "Lapor cepat",
                CopyKey::ManualLocation => "Lokasi manual",
                CopyKey::UsePhoneCoordinates => "Pakai koordinat HP",
                CopyKey::Fallback => "Fallback",
                CopyKey::ManualLocationBody => {
                    "Lokasi belum tersedia. Aktifkan izin lokasi atau masukkan koordinat untuk melanjutkan."
                }
                CopyKey::UseThisLocation => "Pakai lokasi ini",
                CopyKey::NearbyReports => "Laporan terdekat",
                CopyKey::Loading => "Loading",
                CopyKey::Live => "Live",
                CopyKey::NoNearbyReports => "Belum ada laporan aktif dari user lain di radius ini.",
                CopyKey::RefreshGpsReports => "Refresh GPS & laporan",
                CopyKey::LastRouteOverlay => "Overlay rute terakhir",
                CopyKey::RouteReportsSuffix => "laporan di rute",
                CopyKey::SearchDestination => "Cari tujuan",
                CopyKey::SearchDestinationPlaceholder => "Cari tempat atau alamat",
                CopyKey::SearchingPlaces => "Mencari tempat...",
                CopyKey::CheckSafeRoute => "Cek rute aman",
                CopyKey::CheckingRoute => "Mengecek rute...",
                CopyKey::SafeRouteMapTitle => "Peta rute aman",
                CopyKey::LiveRoute => "Rute langsung",
                CopyKey::RouteScore => "Skor rute",
                CopyKey::RouteSafetyStatus => "Status keamanan rute",
                CopyKey::NotChecked => "Belum dicek",
                CopyKey::Weight => "Bobot",
                CopyKey::Status => "Status",
                CopyKey::Reports => "Laporan",
                CopyKey::EnterDestinationHint => "Masukkan tujuan untuk melihat skor keamanan rute.",
                CopyKey::RouteDetails => "Detail rute",
                CopyKey::Distance => "Jarak",
                CopyKey::Estimate => "Estimasi",
                CopyKey::Mode => "Mode",
                CopyKey::Walking => "Jalan kaki",
                CopyKey::GpsReady => "GPS siap",
                CopyKey::GpsNotReady => "GPS belum siap",
                CopyKey::QuickReport => "Lapor cepat",
                CopyKey::ReportCategory => "Kategori laporan",
                CopyKey::OptionalNote => "Catatan opsional",
                CopyKey::Max100 => "Maks 100 karakter",
                CopyKey::Sending => "Mengirim...",
                CopyKey::SubmitReport => "Kirim laporan",
                CopyKey::ReportPreview => "Preview laporan",
                CopyKey::EmergencyContacts => "Kontak darurat",
                CopyKey::ContactsSaved => "kontak tersimpan",
                CopyKey::NoContacts => {
                    "Belum ada kontak. Tambahkan email atau nomor WhatsApp agar SOS bisa mengirim alert."
                }
                CopyKey::RefreshContacts => "Refresh kontak",
                CopyKey::AddSosContact => "Tambah kontak SOS",
                CopyKey::ContactName => "Nama kontak",
                CopyKey::ContactEmail => "Email kontak",
                CopyKey::ContactPhone => "Nomor WhatsApp, contoh 08123456789",
                CopyKey::Saving => "Menyimpan...",
                CopyKey::AddContact => "Tambah kontak",
                CopyKey::AccountPrivacy => "Akun & privasi",
                CopyKey::Anonymous => "Kamu tetap anonim",
                CopyKey::Protected => "Terlindungi",
                CopyKey::Location => "Lokasi",
                CopyKey::Nearby => "Di sekitar",
                CopyKey::SosContacts => "Kontak SOS",
                CopyKey::SafetySettings => "Pengaturan keamanan",
                CopyKey::NearbyMap => "Peta sekitar",
                CopyKey::Active => "Aktif",
                CopyKey::SosAlerts => "Peringatan SOS",
                CopyKey::ReadyToUse => "Siap digunakan",
                CopyKey::LocationNotice => {
                    "Lokasi belum tersedia. Aktifkan izin lokasi agar peta dan SOS bekerja akurat."
                }
                CopyKey::SosActiveTitle => "SOS sedang aktif",
                CopyKey::SosStatusTitle => "Status SOS",
                CopyKey::SosActiveBody => "Alarm dan getaran akan terus aktif sampai dihentikan.",
                CopyKey::SosClosedBody => "Kamu dapat menutup pemberitahuan ini.",
                CopyKey::StopAlarm => "Hentikan alarm",
                CopyKey::Close => "Tutup",
                CopyKey::SosStillActive => {
                    "Alarm SOS masih aktif. Hentikan lewat tombol ini atau notifikasi sistem."
                }
                CopyKey::PreparingHelp => "Kami sedang menyiapkan bantuan untukmu.",
                CopyKey::SosStopped => "Alarm SOS sudah dihentikan.",
                CopyKey::AppPreparing => "Aplikasi sedang disiapkan. Coba lagi sebentar.",
                CopyKey::PreparingSosLocation => "Menyiapkan SOS dengan lokasi terkini...",
                CopyKey::SosLocationMissing => {
                    "Lokasi belum didapatkan. Aktifkan izin lokasi, lalu coba SOS lagi."
                }
                CopyKey::SosActiveSending => {
                    "Alarm SOS aktif. Lokasi dan permintaan bantuan sedang dikirim ke kontak darurat."
                }
                CopyKey::WhatsappSent => {
                    "Permintaan bantuan otomatis terkirim via WhatsApp. Alarm tetap aktif sampai dihentikan."
                }
                CopyKey::WhatsappFallback => {
                    "Kontak darurat sudah diproses. WhatsApp dibuka sebagai fallback karena auto-send belum tersedia untuk nomor ini. Alarm tetap aktif sampai dihentikan."
                }
                CopyKey::SosNotified => {
                    "Permintaan bantuan dikirim ke kontak darurat. Alarm tetap aktif sampai dihentikan."
                }
                CopyKey::SosNoChannel => {
                    "Alarm tetap aktif, tetapi belum ada kanal otomatis yang berhasil. Pastikan backend/WhatsApp API aktif atau hubungi orang terdekat."
                }
                CopyKey::SosBackendFallback => {
                    "Backend belum terhubung. WhatsApp dibuka sebagai fallback agar pesan bisa segera dikirim. Alarm tetap aktif sampai dihentikan."
                }
                CopyKey::SosStaySafe => {
                    "Alarm tetap aktif. Pastikan koneksi internet lalu hubungi orang terdekat di sekitarmu."
                }
                CopyKey::DestinationMin => "Tujuan minimal 3 karakter.",
                CopyKey::RouteNeedsLocation => {
                    "Lokasi belum tersedia. Isi koordinat manual di tab Peta dulu."
                }
                CopyKey::RouteFallbackScore => {
                    "Rute tetap tampil. Skor sementara dihitung dari laporan di sekitar kamu."
                }
                CopyKey::ReportNeedsLocation => {
                    "Lokasi belum tersedia. Isi koordinat manual di tab Peta dulu."
                }
                CopyKey::ContactNameMin => "Nama kontak minimal 2 karakter.",
                CopyKey::ContactChannelRequired => "Isi email atau nomor WhatsApp kontak.",
            },
            Self::English => match key {
                CopyKey::HeaderSubtitle => "Find safer routes",
                CopyKey::LanguageToggleTitle => "Switch language",
                CopyKey::SplashSubtitle => "Move with confidence.",
                CopyKey::MapNav => "Map",
                CopyKey::RouteNav => "Route",
                CopyKey::ContactsNav => "Contacts",
                CopyKey::AccountNav => "Account",
                CopyKey::HeaderHelpTitle => "Help",
                CopyKey::HelpTitle => "JalanAman Help",
                CopyKey::HelpSubtitle => "A short tutorial video for the main features.",
                CopyKey::TutorialVideo => "Tutorial video",
                CopyKey::VideoUnavailable => "Video cannot be played on this device.",
                CopyKey::MapLoading => "Loading map and reports",
                CopyKey::ReportsActiveRadius => "active reports within 800 m",
                CopyKey::GpsUnavailable => "GPS unavailable",
                CopyKey::LiveMap => "Live map",
                CopyKey::QuickReportTitle => "Quick report",
                CopyKey::ManualLocation => "Manual location",
                CopyKey::UsePhoneCoordinates => "Use phone coordinates",
                CopyKey::Fallback => "Fallback",
                CopyKey::ManualLocationBody => {
                    "Location is unavailable. Enable location permission or enter coordinates to continue."
                }
                CopyKey::UseThisLocation => "Use this location",
                CopyKey::NearbyReports => "Nearby reports",
                CopyKey::Loading => "Loading",
                CopyKey::Live => "Live",
                CopyKey::NoNearbyReports => "No active reports from other users in this radius.",
                CopyKey::RefreshGpsReports => "Refresh GPS & reports",
                CopyKey::LastRouteOverlay => "Last route overlay",
                CopyKey::RouteReportsSuffix => "reports on route",
                CopyKey::SearchDestination => "Search destination",
                CopyKey::SearchDestinationPlaceholder => "Search place or address",
                CopyKey::SearchingPlaces => "Searching places...",
                CopyKey::CheckSafeRoute => "Check safer route",
                CopyKey::CheckingRoute => "Checking route...",
                CopyKey::SafeRouteMapTitle => "Safe route map",
                CopyKey::LiveRoute => "Live route",
                CopyKey::RouteScore => "Route score",
                CopyKey::RouteSafetyStatus => "Route safety status",
                CopyKey::NotChecked => "Not checked",
                CopyKey::Weight => "Weight",
                CopyKey::Status => "Status",
                CopyKey::Reports => "Reports",
                CopyKey::EnterDestinationHint => "Enter a destination to see the route safety score.",
                CopyKey::RouteDetails => "Route details",
                CopyKey::Distance => "Distance",
                CopyKey::Estimate => "Estimate",
                CopyKey::Mode => "Mode",
                CopyKey::Walking => "Walking",
                CopyKey::GpsReady => "GPS ready",
                CopyKey::GpsNotReady => "GPS not ready",
                CopyKey::QuickReport => "Quick report",
                CopyKey::ReportCategory => "Report category",
                CopyKey::OptionalNote => "Optional note",
                CopyKey::Max100 => "Max 100 characters",
                CopyKey::Sending => "Sending...",
                CopyKey::SubmitReport => "Submit report",
                CopyKey::ReportPreview => "Report preview",
                CopyKey::EmergencyContacts => "Emergency contacts",
                CopyKey::ContactsSaved => "saved contacts",
                CopyKey::NoContacts => {
                    "No contacts yet. Add an email or WhatsApp number so SOS can send alerts."
                }
                CopyKey::RefreshContacts => "Refresh contacts",
                CopyKey::AddSosContact => "Add SOS contact",
                CopyKey::ContactName => "Contact name",
                CopyKey::ContactEmail => "Contact email",
                CopyKey::ContactPhone => "WhatsApp number, e.g. 08123456789",
                CopyKey::Saving => "Saving...",
                CopyKey::AddContact => "Add contact",
                CopyKey::AccountPrivacy => "Account & privacy",
                CopyKey::Anonymous => "You stay anonymous",
                CopyKey::Protected => "Protected",
                CopyKey::Location => "Location",
                CopyKey::Nearby => "Nearby",
                CopyKey::SosContacts => "SOS contacts",
                CopyKey::SafetySettings => "Safety settings",
                CopyKey::NearbyMap => "Nearby map",
                CopyKey::Active => "Active",
                CopyKey::SosAlerts => "SOS alerts",
                CopyKey::ReadyToUse => "Ready",
                CopyKey::LocationNotice => {
                    "Location is unavailable. Enable location permission so map and SOS work accurately."
                }
                CopyKey::SosActiveTitle => "SOS is active",
                CopyKey::SosStatusTitle => "SOS status",
                CopyKey::SosActiveBody => "Alarm and vibration will stay active until stopped.",
                CopyKey::SosClosedBody => "You can close this notice.",
                CopyKey::StopAlarm => "Stop alarm",
                CopyKey::Close => "Close",
                CopyKey::SosStillActive => {
                    "SOS alarm is still active. Stop it here or from the system notification."
                }
                CopyKey::PreparingHelp => "Preparing help for you.",
                CopyKey::SosStopped => "SOS alarm has been stopped.",
                CopyKey::AppPreparing => "The app is still preparing. Try again shortly.",
                CopyKey::PreparingSosLocation => "Preparing SOS with your latest location...",
                CopyKey::SosLocationMissing => {
                    "Location is not available yet. Enable location permission, then try SOS again."
                }
                CopyKey::SosActiveSending => {
                    "SOS alarm is active. Your location and help request are being sent to emergency contacts."
                }
                CopyKey::WhatsappSent => {
                    "Help request was sent automatically via WhatsApp. Alarm stays active until stopped."
                }
                CopyKey::WhatsappFallback => {
                    "Emergency contacts were processed. WhatsApp opened as fallback because auto-send is not available for this number. Alarm stays active until stopped."
                }
                CopyKey::SosNotified => {
                    "Help request was sent to emergency contacts. Alarm stays active until stopped."
                }
                CopyKey::SosNoChannel => {
                    "Alarm stays active, but no automatic channel succeeded. Check backend/WhatsApp API or contact someone nearby."
                }
                CopyKey::SosBackendFallback => {
                    "Backend is not connected. WhatsApp opened as fallback so the message can be sent quickly. Alarm stays active until stopped."
                }
                CopyKey::SosStaySafe => {
                    "Alarm stays active. Check your internet connection and contact someone nearby."
                }
                CopyKey::DestinationMin => "Destination needs at least 3 characters.",
                CopyKey::RouteNeedsLocation => {
                    "Location is unavailable. Enter manual coordinates in the Map tab first."
                }
                CopyKey::RouteFallbackScore => {
                    "Route is still shown. Temporary score is calculated from nearby reports."
                }
                CopyKey::ReportNeedsLocation => {
                    "Location is unavailable. Enter manual coordinates in the Map tab first."
                }
                CopyKey::ContactNameMin => "Contact name needs at least 2 characters.",
                CopyKey::ContactChannelRequired => "Enter the contact email or WhatsApp number.",
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ReportCategory {
    Lighting,
    Crime,
    Accident,
    Other,
}

impl ReportCategory {
    const fn api_value(self) -> &'static str {
        match self {
            Self::Lighting => "lighting",
            Self::Crime => "crime",
            Self::Accident => "accident",
            Self::Other => "other",
        }
    }

    const fn label_for(self, language: Language) -> &'static str {
        match language {
            Language::Indonesian => match self {
                Self::Lighting => "Pencahayaan buruk",
                Self::Crime => "Rawan kriminal",
                Self::Accident => "Rawan kecelakaan",
                Self::Other => "Lainnya",
            },
            Language::English => match self {
                Self::Lighting => "Poor lighting",
                Self::Crime => "Crime risk",
                Self::Accident => "Accident risk",
                Self::Other => "Other",
            },
        }
    }

    const fn short_label_for(self, language: Language) -> &'static str {
        match language {
            Language::Indonesian => match self {
                Self::Lighting => "Gelap",
                Self::Crime => "Kriminal",
                Self::Accident => "Kecelakaan",
                Self::Other => "Lainnya",
            },
            Language::English => match self {
                Self::Lighting => "Dark",
                Self::Crime => "Crime",
                Self::Accident => "Accident",
                Self::Other => "Other",
            },
        }
    }

    const fn color(self) -> &'static str {
        match self {
            Self::Lighting => "#f59e0b",
            Self::Crime => "#ef4444",
            Self::Accident => "#f97316",
            Self::Other => "#94a3b8",
        }
    }

    fn from_api(value: &str) -> Self {
        match value {
            "lighting" => Self::Lighting,
            "crime" => Self::Crime,
            "accident" => Self::Accident,
            _ => Self::Other,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
struct GeoPoint {
    lat: f64,
    lng: f64,
}

#[derive(Clone, Debug, Deserialize)]
struct LocationEval {
    lat: Option<f64>,
    lng: Option<f64>,
    error: Option<String>,
}

#[derive(Serialize)]
struct MapReport {
    id: String,
    category: String,
    lat: f64,
    lng: f64,
    note: Option<String>,
}

const CATEGORIES: [ReportCategory; 4] = [
    ReportCategory::Lighting,
    ReportCategory::Crime,
    ReportCategory::Accident,
    ReportCategory::Other,
];

const LOGO: Asset = asset!("/assets/jalanaman-logo.png");
const TUTORIAL_VIDEO: Asset = asset!("/assets/video/videotutorial.mp4");

const APP: &str = "min-height:100dvh;background:radial-gradient(circle at 50% 108%,rgba(34,197,94,0.34),rgba(34,197,94,0) 34%),radial-gradient(circle at 14% 8%,rgba(37,99,235,0.22),rgba(37,99,235,0) 30%),linear-gradient(145deg,#27272a 0%,#17181c 46%,#0b0f14 100%);color:#f8fafc;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;";
const SCREEN: &str = "width:100vw;max-width:100vw;min-height:100dvh;background:radial-gradient(circle at 50% 92%,rgba(34,197,94,0.18),rgba(34,197,94,0) 28%),radial-gradient(circle at 86% 10%,rgba(14,165,233,0.18),rgba(14,165,233,0) 34%),linear-gradient(180deg,rgba(39,40,45,0.94) 0%,rgba(23,24,29,0.95) 48%,rgba(10,14,19,0.98) 100%);position:relative;overflow:hidden;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12),0 0 0 1px rgba(255,255,255,0.08);";
const HEADER: &str = "height:88px;padding:16px 18px;display:flex;align-items:center;justify-content:space-between;background:linear-gradient(180deg,rgba(58,59,66,0.72),rgba(30,32,39,0.58));border-bottom:1px solid rgba(255,255,255,0.13);box-sizing:border-box;box-shadow:0 18px 46px rgba(0,0,0,0.26),inset 0 1px 0 rgba(255,255,255,0.12);backdrop-filter:blur(26px) saturate(175%);-webkit-backdrop-filter:blur(26px) saturate(175%);";
const BRAND_WRAP: &str = "display:flex;align-items:center;gap:12px;min-width:0;";
const BRAND: &str = "font-size:21px;font-weight:950;color:#f8fafc;letter-spacing:0;line-height:1;text-shadow:0 1px 1px rgba(0,0,0,0.24);white-space:nowrap;overflow:hidden;text-overflow:ellipsis;";
const SUBTITLE: &str = "margin-top:3px;font-size:10px;font-weight:800;color:#93c5fd;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;";
const HEADER_LOGO: &str = "width:42px;height:42px;object-fit:contain;flex-shrink:0;filter:drop-shadow(0 12px 20px rgba(37,99,235,0.34));";
const ICON_BUTTON: &str = "width:42px;height:42px;border-radius:14px;border:1px solid rgba(255,255,255,0.18);background:rgba(47,50,58,0.76);color:#e0f2fe;font-size:16px;font-weight:900;display:flex;align-items:center;justify-content:center;box-shadow:0 14px 32px rgba(0,0,0,0.24),inset 0 1px 0 rgba(255,255,255,0.20),inset 0 -1px 0 rgba(0,0,0,0.24);backdrop-filter:blur(18px) saturate(170%);-webkit-backdrop-filter:blur(18px) saturate(170%);";
const HEADER_ACTIONS: &str = "display:flex;align-items:center;gap:6px;flex-shrink:0;";
const LANGUAGE_TOGGLE: &str = "width:56px;height:42px;border-radius:14px;border:1px solid rgba(255,255,255,0.18);background:rgba(47,50,58,0.76);color:#e0f2fe;padding:5px;display:grid;grid-template-columns:1fr 1fr;gap:3px;align-items:center;box-shadow:0 14px 32px rgba(0,0,0,0.24),inset 0 1px 0 rgba(255,255,255,0.20),inset 0 -1px 0 rgba(0,0,0,0.24);backdrop-filter:blur(18px) saturate(170%);-webkit-backdrop-filter:blur(18px) saturate(170%);";
const LANGUAGE_SEGMENT_ACTIVE: &str = "height:30px;border-radius:10px;display:flex;align-items:center;justify-content:center;background:#1d4ed8;color:#ffffff;font-size:9px;font-weight:950;box-shadow:0 8px 16px rgba(37,99,235,0.22),inset 0 1px 0 rgba(255,255,255,0.22),inset 0 -1px 0 rgba(0,0,0,0.20);";
const LANGUAGE_SEGMENT_IDLE: &str = "height:30px;border-radius:10px;display:flex;align-items:center;justify-content:center;color:#cbd5e1;font-size:9px;font-weight:900;";
const CONTENT: &str = "position:absolute;top:88px;left:0;right:0;bottom:130px;padding:14px 14px 18px;overflow-y:auto;box-sizing:border-box;";
const MAP_CARD: &str = "height:clamp(326px,46dvh,368px);position:relative;overflow:hidden;border-radius:8px;background:rgba(44,46,53,0.66);border:1px solid rgba(255,255,255,0.16);box-shadow:0 24px 54px rgba(0,0,0,0.34),inset 0 1px 0 rgba(255,255,255,0.14);backdrop-filter:blur(18px) saturate(165%);-webkit-backdrop-filter:blur(18px) saturate(165%);";
const ROUTE_MAP_CARD: &str = "height:292px;margin-top:12px;position:relative;overflow:hidden;border-radius:8px;background:rgba(44,46,53,0.66);border:1px solid rgba(255,255,255,0.16);box-shadow:0 20px 44px rgba(0,0,0,0.30),inset 0 1px 0 rgba(255,255,255,0.14);backdrop-filter:blur(18px) saturate(165%);-webkit-backdrop-filter:blur(18px) saturate(165%);";
const MAP_IFRAME: &str =
    "position:absolute;inset:0;width:100%;height:100%;border:0;background:#111827;";
const MAP_LABEL: &str = "position:absolute;left:12px;top:12px;max-width:min(250px,calc(100% - 112px));padding:10px 12px;border-radius:8px;background:linear-gradient(180deg,rgba(45,47,55,0.78),rgba(23,25,31,0.62));border:1px solid rgba(255,255,255,0.18);color:#f8fafc;font-size:12px;font-weight:900;box-shadow:0 14px 30px rgba(0,0,0,0.26),inset 0 1px 0 rgba(255,255,255,0.16);backdrop-filter:blur(18px) saturate(170%);-webkit-backdrop-filter:blur(18px) saturate(170%);z-index:2;";
const REPORT_FAB: &str = "position:absolute;right:16px;bottom:16px;width:60px;height:60px;border-radius:18px;border:1px solid rgba(255,255,255,0.56);background:#1d4ed8;color:#ffffff;font-size:31px;line-height:1;font-weight:950;box-shadow:0 18px 34px rgba(37,99,235,0.34),inset 0 1px 0 rgba(255,255,255,0.34),inset 0 -2px 0 rgba(0,0,0,0.22);display:flex;align-items:center;justify-content:center;z-index:2;";
const MAP_PROVIDER: &str = "position:absolute;left:12px;bottom:14px;padding:8px 10px;border-radius:8px;background:rgba(10,38,91,0.78);border:1px solid rgba(255,255,255,0.25);color:#ffffff;font-size:10px;font-weight:850;box-shadow:0 12px 24px rgba(15,47,109,0.18);backdrop-filter:blur(14px) saturate(150%);-webkit-backdrop-filter:blur(14px) saturate(150%);z-index:2;";
const CARD: &str = "margin-top:12px;background:linear-gradient(180deg,rgba(52,54,62,0.64),rgba(22,24,31,0.54));border:1px solid rgba(255,255,255,0.14);border-radius:8px;padding:14px;box-shadow:0 18px 42px rgba(0,0,0,0.28),inset 0 1px 0 rgba(255,255,255,0.14);box-sizing:border-box;backdrop-filter:blur(24px) saturate(175%);-webkit-backdrop-filter:blur(24px) saturate(175%);";
const CARD_TIGHT: &str = "background:linear-gradient(180deg,rgba(52,54,62,0.66),rgba(22,24,31,0.56));border:1px solid rgba(255,255,255,0.14);border-radius:8px;padding:13px;box-shadow:0 18px 42px rgba(0,0,0,0.28),inset 0 1px 0 rgba(255,255,255,0.14);box-sizing:border-box;backdrop-filter:blur(24px) saturate(175%);-webkit-backdrop-filter:blur(24px) saturate(175%);";
const ROW: &str = "display:flex;align-items:center;justify-content:space-between;gap:12px;";
const EYEBROW: &str = "font-size:11px;color:#9ca3af;font-weight:800;margin-bottom:3px;";
const TITLE: &str = "font-size:14px;color:#f8fafc;font-weight:900;line-height:1.25;";
const BODY: &str = "font-size:12px;color:#cbd5e1;font-weight:700;line-height:1.48;";
const META_GRID: &str =
    "margin-top:12px;display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:8px;";
const META_CELL: &str = "border-radius:8px;background:rgba(255,255,255,0.07);border:1px solid rgba(255,255,255,0.12);padding:10px 8px;min-height:58px;box-sizing:border-box;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);";
const META_VALUE: &str = "font-size:14px;font-weight:950;color:#93c5fd;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;";
const META_LABEL: &str = "margin-top:2px;font-size:10px;font-weight:800;color:#9ca3af;";
const FIELD_GRID: &str = "display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-top:10px;";
const INPUT: &str = "width:100%;box-sizing:border-box;border:1px solid rgba(255,255,255,0.13);background:rgba(255,255,255,0.08);border-radius:8px;padding:12px 14px;color:#f8fafc;font-size:14px;font-weight:700;outline:none;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12),0 8px 18px rgba(0,0,0,0.14);backdrop-filter:blur(16px) saturate(150%);-webkit-backdrop-filter:blur(16px) saturate(150%);";
const TEXTAREA: &str = "width:100%;min-height:88px;box-sizing:border-box;border:1px solid rgba(255,255,255,0.13);background:rgba(255,255,255,0.08);border-radius:8px;padding:12px 14px;color:#f8fafc;font-size:14px;font-weight:700;outline:none;resize:none;font-family:inherit;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12),0 8px 18px rgba(0,0,0,0.14);backdrop-filter:blur(16px) saturate(150%);-webkit-backdrop-filter:blur(16px) saturate(150%);";
const PRIMARY_BUTTON: &str = "width:100%;margin-top:10px;border:1px solid rgba(255,255,255,0.38);border-radius:8px;background:#1d4ed8;color:#ffffff;padding:12px 14px;font-size:14px;font-weight:950;box-shadow:0 16px 30px rgba(37,99,235,0.24),inset 0 1px 0 rgba(255,255,255,0.28),inset 0 -1px 0 rgba(0,0,0,0.20);";
const SECONDARY_BUTTON: &str = "width:100%;margin-top:10px;border:1px solid rgba(147,197,253,0.32);border-radius:8px;background:rgba(255,255,255,0.07);color:#bfdbfe;padding:12px 14px;font-size:14px;font-weight:900;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);backdrop-filter:blur(16px) saturate(150%);-webkit-backdrop-filter:blur(16px) saturate(150%);";
const CATEGORY_GRID: &str = "display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-top:10px;";
const CATEGORY_BUTTON: &str = "height:54px;border:1px solid rgba(255,255,255,0.12);border-radius:8px;background:rgba(255,255,255,0.07);color:#d1d5db;display:flex;align-items:center;gap:8px;padding:0 11px;font-size:12px;font-weight:850;text-align:left;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);";
const CATEGORY_BUTTON_ACTIVE: &str = "height:54px;border:1px solid rgba(56,189,248,0.52);border-radius:8px;background:rgba(37,99,235,0.20);color:#f8fafc;display:flex;align-items:center;gap:8px;padding:0 11px;font-size:12px;font-weight:950;text-align:left;box-shadow:0 10px 22px rgba(14,165,233,0.16),inset 0 1px 0 rgba(255,255,255,0.16);";
const BOTTOM_BAR: &str = "position:absolute;left:18px;right:18px;bottom:16px;height:76px;border-radius:38px;background:rgba(31,34,40,0.78);border:1px solid rgba(255,255,255,0.18);display:grid;grid-template-columns:1fr 1fr 72px 1fr 1fr;align-items:center;padding:7px 9px;box-shadow:0 22px 54px rgba(2,6,23,0.34),inset 0 1px 0 rgba(255,255,255,0.20),inset 0 -1px 0 rgba(0,0,0,0.30);box-sizing:border-box;backdrop-filter:blur(26px) saturate(170%);-webkit-backdrop-filter:blur(26px) saturate(170%);overflow:visible;z-index:30;";
const NAV_BUTTON: &str = "height:60px;border:0;border-radius:30px;background:transparent;color:rgba(255,255,255,0.82);display:flex;flex-direction:column;align-items:center;justify-content:center;gap:4px;font-size:10px;font-weight:850;text-shadow:0 1px 1px rgba(0,0,0,0.20);";
const NAV_BUTTON_ACTIVE: &str = "height:60px;border:1px solid rgba(255,255,255,0.20);background:rgba(255,255,255,0.13);color:#ffffff;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:4px;font-size:10px;font-weight:950;border-radius:30px;box-shadow:0 12px 26px rgba(0,0,0,0.26),inset 0 1px 0 rgba(255,255,255,0.22),inset 0 -1px 0 rgba(0,0,0,0.20);backdrop-filter:blur(18px) saturate(160%);-webkit-backdrop-filter:blur(18px) saturate(160%);";
const NAV_ICON: &str = "width:20px;height:20px;display:block;stroke:currentColor;";
const SOS_BUTTON: &str = "position:absolute;left:50%;bottom:40px;transform:translateX(-50%);width:64px;height:64px;border-radius:50%;border:1px solid rgba(255,255,255,0.30);background:#ef4444;color:#ffffff;font-size:19px;font-weight:900;letter-spacing:0;box-shadow:0 0 0 7px rgba(239,68,68,0.16),0 20px 34px rgba(239,68,68,0.34),0 14px 34px rgba(0,0,0,0.32),inset 0 1px 0 rgba(255,255,255,0.34),inset 0 -12px 22px rgba(127,29,29,0.30);display:flex;align-items:center;justify-content:center;backdrop-filter:blur(20px) saturate(170%);-webkit-backdrop-filter:blur(20px) saturate(170%);z-index:31;";
const SOS_BUTTON_ACTIVE: &str = "position:absolute;left:50%;bottom:39px;transform:translateX(-50%);width:66px;height:66px;border-radius:50%;border:1px solid rgba(255,255,255,0.30);background:#dc2626;color:#ffffff;font-size:14px;font-weight:950;letter-spacing:0;box-shadow:0 0 0 9px rgba(239,68,68,0.20),0 20px 38px rgba(220,38,38,0.44),0 14px 34px rgba(0,0,0,0.34),inset 0 1px 0 rgba(255,255,255,0.30),inset 0 -12px 22px rgba(127,29,29,0.30);display:flex;align-items:center;justify-content:center;backdrop-filter:blur(20px) saturate(170%);-webkit-backdrop-filter:blur(20px) saturate(170%);z-index:31;";
const DASHBOARD_WRAP: &str = "min-height:100dvh;background:radial-gradient(circle at 50% 108%,rgba(34,197,94,0.30),rgba(34,197,94,0) 36%),linear-gradient(145deg,#27272a 0%,#17181c 48%,#0b0f14 100%);padding:18px;box-sizing:border-box;color:#f8fafc;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;";
const BACK_LINK: &str = "display:inline-flex;align-items:center;gap:6px;color:#93c5fd;text-decoration:none;font-size:13px;font-weight:850;margin-bottom:18px;";
const DASH_TITLE: &str =
    "font-size:24px;line-height:1.1;font-weight:900;color:#f8fafc;margin:0 0 14px;";
const MOTION_CSS: &str = r#"
    @keyframes ja-splash-in { from { opacity:0; transform:scale(.94); } to { opacity:1; transform:scale(1); } }
    @keyframes ja-logo-float { 0%,100% { transform:translateY(0); } 50% { transform:translateY(-7px); } }
    @keyframes ja-panel-in { from { opacity:0; transform:translateY(10px); } to { opacity:1; transform:translateY(0); } }
    @keyframes ja-sos-pulse {
      0%,100% { box-shadow:0 0 0 7px rgba(239,68,68,.16),0 20px 38px rgba(127,29,29,.38),0 14px 34px rgba(0,0,0,.34),inset 0 1px 0 rgba(255,255,255,.30),inset 0 -12px 22px rgba(0,0,0,.24); }
      60% { box-shadow:0 0 0 15px rgba(239,68,68,0),0 20px 38px rgba(127,29,29,.38),0 14px 34px rgba(0,0,0,.34),inset 0 1px 0 rgba(255,255,255,.30),inset 0 -12px 22px rgba(0,0,0,.24); }
    }
    html, body, #main, #root, #app { width:100%; min-height:100%; margin:0; padding:0; background:#0b0f14; overflow:hidden; }
    body > div { min-height:100dvh; margin:0; padding:0; background:#0b0f14; }
    * { -webkit-tap-highlight-color: transparent; }
    *, *::before, *::after { box-sizing: border-box; }
    button, input, textarea { font-family: inherit; }
    button { cursor: pointer; transition: transform 160ms ease, filter 160ms ease, box-shadow 160ms ease; }
    button:active { transform: scale(.98); filter: brightness(.98); }
    .ja-sos-orb { overflow:hidden; isolation:isolate; }
    .ja-sos-orb:active { transform:translateX(-50%) scale(.98); }
    .ja-sos-orb::before {
      content:"";
      position:absolute;
      left:11px;
      right:11px;
      top:7px;
      height:19px;
      border-radius:999px;
      background:rgba(255,255,255,.22);
      opacity:.64;
      pointer-events:none;
      z-index:0;
    }
    .ja-sos-orb::after {
      content:"";
      position:absolute;
      inset:1px;
      border-radius:inherit;
      border-top:1px solid rgba(255,255,255,.24);
      border-bottom:1px solid rgba(0,0,0,.22);
      pointer-events:none;
      z-index:0;
    }
    .ja-sos-orb > span { position:relative; z-index:1; text-shadow:0 2px 5px rgba(0,0,0,.35); }
    input::placeholder, textarea::placeholder { color: rgba(203,213,225,.64); }
    .ja-content { animation:ja-panel-in 320ms ease-out both; }
    .ja-splash { animation:ja-splash-in 420ms ease-out both; }
    .ja-splash-logo { animation:ja-logo-float 2.4s ease-in-out infinite; }
    .ja-dock::before {
      content:"";
      position:absolute;
      left:18%;
      right:18%;
      bottom:-54px;
      height:74px;
      border-radius:999px;
      background:radial-gradient(circle at 50% 30%, rgba(34,197,94,.58), rgba(14,165,233,.22) 42%, rgba(14,165,233,0) 72%);
      filter:blur(22px);
      opacity:.74;
      pointer-events:none;
      z-index:-1;
    }
    .ja-dock::after {
      content:"";
      position:absolute;
      inset:1px;
      border-radius:inherit;
      border-top:1px solid rgba(255,255,255,.20);
      pointer-events:none;
    }
    .ja-sos-active { animation:ja-sos-pulse 1.25s ease-out infinite; }
"#;

#[component]
fn Home() -> Element {
    let mut active_tab = use_signal(|| MobileTab::Map);
    let mut map_presentation = use_signal(|| MapPresentation::TwoDimensional);
    let mut language = use_signal(|| Language::Indonesian);
    let mut show_splash = use_signal(|| true);
    let mut device_hash = use_signal(String::new);
    let mut location = use_signal(|| Option::<GeoPoint>::None);
    let mut location_loading = use_signal(|| true);
    let mut location_error = use_signal(|| Option::<String>::None);
    let mut reports = use_signal(Vec::<Report>::new);
    let mut reports_loading = use_signal(|| false);
    let mut reports_error = use_signal(|| Option::<String>::None);
    let mut report_category = use_signal(|| ReportCategory::Lighting);
    let mut report_note = use_signal(String::new);
    let mut report_loading = use_signal(|| false);
    let mut report_error = use_signal(|| Option::<String>::None);
    let mut destination = use_signal(String::new);
    let mut directions = use_signal(|| Option::<DirectionsResponse>::None);
    let mut route_score = use_signal(|| Option::<RouteScoreResponse>::None);
    let mut route_loading = use_signal(|| false);
    let mut route_error = use_signal(|| Option::<String>::None);
    let mut place_suggestions = use_signal(Vec::<PlaceSuggestion>::new);
    let mut place_suggestions_loading = use_signal(|| false);
    let mut place_suggestions_error = use_signal(|| Option::<String>::None);
    let mut selected_place = use_signal(|| Option::<PlaceSuggestion>::None);
    let mut place_search_revision = use_signal(|| 0_u64);
    let mut contacts = use_signal(Vec::<EmergencyContact>::new);
    let mut contact_name = use_signal(String::new);
    let mut contact_email = use_signal(String::new);
    let mut contact_phone = use_signal(String::new);
    let mut contacts_loading = use_signal(|| false);
    let mut contacts_error = use_signal(|| Option::<String>::None);
    let mut sos_active = use_signal(|| false);
    let mut sos_msg = use_signal(|| Option::<String>::None);
    let mut sos_modal_open = use_signal(|| false);
    let mut manual_lat = use_signal(String::new);
    let mut manual_lng = use_signal(String::new);
    let mut manual_location_error = use_signal(|| Option::<String>::None);

    use_future(move || async move {
        tokio::time::sleep(Duration::from_millis(1450)).await;
        show_splash.set(false);
    });

    use_future(move || async move {
        if is_sos_alarm_active().await {
            let current_language = *language.peek();
            sos_active.set(true);
            sos_msg.set(Some(
                current_language.text(CopyKey::SosStillActive).to_string(),
            ));
            sos_modal_open.set(true);
        }
    });

    use_effect(move || {
        spawn(async move {
            request_app_permissions().await;
            tokio::time::sleep(Duration::from_millis(900)).await;
            let hash = read_device_hash().await;
            device_hash.set(hash.clone());
            contacts_loading.set(true);
            match get_contacts(&hash).await {
                Ok(items) => {
                    contacts.set(items);
                    contacts_error.set(None);
                }
                Err(err) => contacts_error.set(Some(err)),
            }
            contacts_loading.set(false);
        });
    });

    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;

            if !matches!(*active_tab.peek(), MobileTab::Map | MobileTab::Route) {
                continue;
            }

            let previous = *location.peek();
            let Ok(point) = read_location().await else {
                continue;
            };
            let moved_m = previous
                .map(|last| haversine_m(last.lat, last.lng, point.lat, point.lng))
                .unwrap_or(f64::INFINITY);

            // Avoid rebuilding the map for GPS noise while still following a walking user.
            if moved_m < 8.0 {
                continue;
            }

            location.set(Some(point));
            manual_lat.set(format!("{:.6}", point.lat));
            manual_lng.set(format!("{:.6}", point.lng));
            location_error.set(None);

            if moved_m >= 35.0 {
                match get_reports(point).await {
                    Ok(items) => {
                        reports.set(items);
                        reports_error.set(None);
                    }
                    Err(err) => reports_error.set(Some(err)),
                }
            }
        }
    });

    use_effect(move || {
        spawn(async move {
            location_loading.set(true);
            match read_location().await {
                Ok(point) => {
                    location.set(Some(point));
                    manual_lat.set(format!("{:.6}", point.lat));
                    manual_lng.set(format!("{:.6}", point.lng));
                    location_error.set(None);
                    reports_loading.set(true);
                    match get_reports(point).await {
                        Ok(items) => {
                            reports.set(items);
                            reports_error.set(None);
                        }
                        Err(err) => reports_error.set(Some(err)),
                    }
                    reports_loading.set(false);
                }
                Err(err) => location_error.set(Some(err)),
            }
            location_loading.set(false);
        });
    });

    let active_tab_value = *active_tab.read();
    let map_presentation_value = *map_presentation.read();
    let language_value = *language.read();
    let location_value = *location.read();
    let report_category_value = *report_category.read();
    let report_note_value = report_note.read().clone();
    let destination_value = destination.read().clone();
    let reports_value = reports.read().clone();
    let contacts_value = contacts.read().clone();
    let directions_value = directions.read().clone();
    let route_score_value = route_score.read().clone();
    let map_html = map_srcdoc(
        location_value,
        &reports_value,
        directions_value.as_ref().map(|d| d.polyline.as_slice()),
        route_score_value.as_ref().map(|s| s.level.as_str()),
        map_presentation_value == MapPresentation::ThreeDimensional,
        language_value,
    );
    let next_language = language_value.toggled();

    rsx! {
        main { style: APP,
            style { {MOTION_CSS} }
            div { style: SCREEN,
                header { style: HEADER,
                    div { style: BRAND_WRAP,
                        img { src: LOGO, alt: "Logo JalanAman", style: HEADER_LOGO }
                        div { style: "min-width:0;",
                            div { style: BRAND, "JalanAman" }
                            div { style: SUBTITLE, "{language_value.text(CopyKey::HeaderSubtitle)}" }
                        }
                    }
                    div { style: HEADER_ACTIONS,
                        button {
                            style: LANGUAGE_TOGGLE,
                            title: "{language_value.text(CopyKey::LanguageToggleTitle)}",
                            onclick: move |_| language.set(next_language),
                            span { style: if language_value.is_indonesian() { LANGUAGE_SEGMENT_ACTIVE } else { LANGUAGE_SEGMENT_IDLE }, "ID" }
                            span { style: if language_value.is_indonesian() { LANGUAGE_SEGMENT_IDLE } else { LANGUAGE_SEGMENT_ACTIVE }, "EN" }
                        }
                        button {
                            style: ICON_BUTTON,
                            title: "{language_value.text(CopyKey::HeaderHelpTitle)}",
                            onclick: move |_| active_tab.set(MobileTab::Help),
                            span { style: "font-size:22px;line-height:1;font-weight:950;color:#e0f2fe;text-shadow:0 1px 1px rgba(0,0,0,0.26);", "?" }
                        }
                    }
                }

                section { style: CONTENT, class: "ja-content",
                    if active_tab_value == MobileTab::Map {
                        MapView {
                            map_html,
                            reports: reports_value,
                            location: location_value,
                            loading: *reports_loading.read() || *location_loading.read(),
                            error: reports_error.read().clone().or(location_error.read().clone()),
                            route_score: route_score_value,
                            language: language_value,
                            manual_lat: manual_lat.read().clone(),
                            manual_lng: manual_lng.read().clone(),
                            manual_error: manual_location_error.read().clone(),
                            presentation: map_presentation_value,
                            on_presentation: move |presentation| map_presentation.set(presentation),
                            on_report: move |_| active_tab.set(MobileTab::Report),
                            on_manual_lat: move |value| manual_lat.set(limit_text(value, 24)),
                            on_manual_lng: move |value| manual_lng.set(limit_text(value, 24)),
                            on_manual_apply: move |_| {
                                let lat_text = manual_lat.read().clone();
                                let lng_text = manual_lng.read().clone();

                                match parse_manual_location(&lat_text, &lng_text) {
                                    Ok(point) => {
                                        location.set(Some(point));
                                        location_error.set(None);
                                        manual_location_error.set(None);
                                        reports_loading.set(true);
                                        spawn(async move {
                                            match get_reports(point).await {
                                                Ok(items) => {
                                                    reports.set(items);
                                                    reports_error.set(None);
                                                }
                                                Err(err) => reports_error.set(Some(err)),
                                            }
                                            reports_loading.set(false);
                                        });
                                    }
                                    Err(err) => manual_location_error.set(Some(err)),
                                }
                            },
                            on_refresh: move |_| {
                                location_loading.set(true);
                                reports_loading.set(true);
                                spawn(async move {
                                    match read_location().await {
                                        Ok(point) => {
                                            location.set(Some(point));
                                            manual_lat.set(format!("{:.6}", point.lat));
                                            manual_lng.set(format!("{:.6}", point.lng));
                                            location_error.set(None);
                                            match get_reports(point).await {
                                                Ok(items) => {
                                                    reports.set(items);
                                                    reports_error.set(None);
                                                }
                                                Err(err) => reports_error.set(Some(err)),
                                            }
                                        }
                                        Err(err) => location_error.set(Some(err)),
                                    }
                                    reports_loading.set(false);
                                    location_loading.set(false);
                                });
                            },
                        }
                    } else if active_tab_value == MobileTab::Route {
                        RouteView {
                            destination: destination_value,
                            map_html: map_html.clone(),
                            directions: directions_value,
                            score: route_score_value,
                            language: language_value,
                            loading: *route_loading.read(),
                            error: route_error.read().clone(),
                            suggestions: place_suggestions.read().clone(),
                            suggestions_loading: *place_suggestions_loading.read(),
                            suggestions_error: place_suggestions_error.read().clone(),
                            selected_place: selected_place.read().clone(),
                            on_destination: move |value| {
                                let value = limit_text(value, 80);
                                let query = value.trim().to_string();
                                destination.set(value);
                                selected_place.set(None);
                                directions.set(None);
                                route_score.set(None);
                                route_error.set(None);

                                let revision = (*place_search_revision.peek()).wrapping_add(1);
                                place_search_revision.set(revision);
                                if query.len() < 2 {
                                    place_suggestions.set(Vec::new());
                                    place_suggestions_loading.set(false);
                                    place_suggestions_error.set(None);
                                    return;
                                }

                                let origin = *location.peek();
                                place_suggestions_loading.set(true);
                                place_suggestions_error.set(None);
                                spawn(async move {
                                    tokio::time::sleep(Duration::from_millis(350)).await;
                                    if *place_search_revision.peek() != revision {
                                        return;
                                    }

                                    match search_places(&query, origin).await {
                                        Ok(items) => {
                                            if *place_search_revision.peek() == revision {
                                                place_suggestions.set(items);
                                                place_suggestions_error.set(None);
                                                place_suggestions_loading.set(false);
                                            }
                                        }
                                        Err(err) => {
                                            if *place_search_revision.peek() == revision {
                                                place_suggestions.set(Vec::new());
                                                place_suggestions_error.set(Some(err));
                                                place_suggestions_loading.set(false);
                                            }
                                        }
                                    }
                                });
                            },
                            on_select_place: move |place: PlaceSuggestion| {
                                let revision = (*place_search_revision.peek()).wrapping_add(1);
                                place_search_revision.set(revision);
                                destination.set(place.name.clone());
                                selected_place.set(Some(place));
                                place_suggestions.set(Vec::new());
                                place_suggestions_loading.set(false);
                                place_suggestions_error.set(None);
                                directions.set(None);
                                route_score.set(None);
                                route_error.set(None);
                            },
                            on_search: move |_| {
                                let dest = destination.read().trim().to_string();
                                let selected = selected_place.read().clone();
                                let point = *location.read();
                                if dest.len() < 3 {
                                    route_error.set(Some(language_value.text(CopyKey::DestinationMin).to_string()));
                                    return;
                                }
                                let Some(origin) = point else {
                                    route_error.set(Some(language_value.text(CopyKey::RouteNeedsLocation).to_string()));
                                    return;
                                };

                                route_loading.set(true);
                                route_error.set(None);
                                directions.set(None);
                                route_score.set(None);
                                let fallback_reports = reports.read().clone();
                                let route_target = selected
                                    .map(|place| format!("{:.7},{:.7}", place.lat, place.lng))
                                    .unwrap_or(dest);

                                spawn(async move {
                                    match get_directions(origin, &route_target).await {
                                        Err(err) => route_error.set(Some(err)),
                                        Ok(dirs) => {
                                            match calculate_route_score(dirs.polyline.clone()).await {
                                                Ok(score) => {
                                                    directions.set(Some(dirs));
                                                    route_score.set(Some(score));
                                                }
                                                Err(_) => {
                                                    let fallback_score = local_route_score(&dirs.polyline, &fallback_reports);
                                                    directions.set(Some(dirs));
                                                    route_score.set(Some(fallback_score));
                                                    route_error.set(Some(language_value.text(CopyKey::RouteFallbackScore).to_string()));
                                                }
                                            }
                                        }
                                    }
                                    route_loading.set(false);
                                });
                            },
                        }
                    } else if active_tab_value == MobileTab::Report {
                        ReportView {
                            category: report_category_value,
                            language: language_value,
                            note: report_note_value,
                            location: location_value,
                            loading: *report_loading.read(),
                            error: report_error.read().clone(),
                            on_category: move |category| report_category.set(category),
                            on_note: move |value| report_note.set(limit_text(value, 100)),
                            on_submit: move |_| {
                                let Some(point) = *location.read() else {
                                    report_error.set(Some(language_value.text(CopyKey::ReportNeedsLocation).to_string()));
                                    return;
                                };
                                let hash = device_hash.read().clone();
                                if hash.is_empty() {
                                    report_error.set(Some(language_value.text(CopyKey::AppPreparing).to_string()));
                                    return;
                                }

                                let note = report_note.read().trim().to_string();
                                let payload = CreateReportPayload {
                                    category: report_category.read().api_value().to_string(),
                                    lat: point.lat,
                                    lng: point.lng,
                                    note: if note.is_empty() { None } else { Some(note) },
                                    device_hash: hash,
                                };

                                report_loading.set(true);
                                report_error.set(None);
                                spawn(async move {
                                    match create_report(&payload).await {
                                        Ok(report) => {
                                            reports.write().insert(0, report);
                                            report_note.set(String::new());
                                            active_tab.set(MobileTab::Map);
                                        }
                                        Err(err) => report_error.set(Some(err)),
                                    }
                                    report_loading.set(false);
                                });
                            },
                        }
                    } else if active_tab_value == MobileTab::Contacts {
                        ContactsView {
                            contacts: contacts_value,
                            language: language_value,
                            name: contact_name.read().clone(),
                            email: contact_email.read().clone(),
                            phone: contact_phone.read().clone(),
                            loading: *contacts_loading.read(),
                            error: contacts_error.read().clone(),
                            on_name: move |value| contact_name.set(limit_text(value, 48)),
                            on_email: move |value| contact_email.set(limit_text(value, 90)),
                            on_phone: move |value| contact_phone.set(limit_text(value, 24)),
                            on_add: move |_| {
                                let hash = device_hash.read().clone();
                                let name = contact_name.read().trim().to_string();
                                let email_text = contact_email.read().trim().to_string();
                                let phone_text = contact_phone.read().trim().to_string();
                                if hash.is_empty() {
                                    contacts_error.set(Some(language_value.text(CopyKey::AppPreparing).to_string()));
                                    return;
                                }
                                if name.len() < 2 {
                                    contacts_error.set(Some(language_value.text(CopyKey::ContactNameMin).to_string()));
                                    return;
                                }
                                if email_text.is_empty() && phone_text.is_empty() {
                                    contacts_error.set(Some(language_value.text(CopyKey::ContactChannelRequired).to_string()));
                                    return;
                                }

                                contacts_loading.set(true);
                                contacts_error.set(None);
                                spawn(async move {
                                    let phone = normalize_whatsapp_phone(&phone_text);
                                    match add_contact(
                                        &hash,
                                        &name,
                                        if email_text.is_empty() { None } else { Some(email_text) },
                                        phone,
                                    ).await {
                                        Ok(contact) => {
                                            contacts.write().insert(0, contact);
                                            contact_name.set(String::new());
                                            contact_email.set(String::new());
                                            contact_phone.set(String::new());
                                        }
                                        Err(err) => contacts_error.set(Some(err)),
                                    }
                                    contacts_loading.set(false);
                                });
                            },
                            on_refresh: move |_| {
                                let hash = device_hash.read().clone();
                                if hash.is_empty() {
                                    contacts_error.set(Some(language_value.text(CopyKey::AppPreparing).to_string()));
                                    return;
                                }
                                contacts_loading.set(true);
                                spawn(async move {
                                    match get_contacts(&hash).await {
                                        Ok(items) => {
                                            contacts.set(items);
                                            contacts_error.set(None);
                                        }
                                        Err(err) => contacts_error.set(Some(err)),
                                    }
                                    contacts_loading.set(false);
                                });
                            },
                        }
                    } else if active_tab_value == MobileTab::Profile {
                        ProfileView {
                            location: location_value,
                            language: language_value,
                            report_count: reports.read().len(),
                            contact_count: contacts.read().len(),
                            location_error: location_error.read().clone(),
                        }
                    } else {
                        HelpView { language: language_value }
                    }
                }

                if *sos_modal_open.read() {
                    SosOverlay {
                        active: *sos_active.read(),
                        language: language_value,
                        message: sos_msg.read().clone().unwrap_or_else(|| language_value.text(CopyKey::PreparingHelp).to_string()),
                        on_close: move |_| sos_modal_open.set(false),
                        on_stop: move |_| {
                            stop_sos_alarm();
                            sos_active.set(false);
                            sos_msg.set(Some(language_value.text(CopyKey::SosStopped).to_string()));
                            sos_modal_open.set(true);
                        },
                    }
                }

                nav { style: BOTTOM_BAR, class: "ja-dock",
                    NavButton {
                        active: active_tab_value == MobileTab::Map,
                        icon: NavIconKind::Map,
                        label: language_value.text(CopyKey::MapNav),
                        onclick: move |_| active_tab.set(MobileTab::Map),
                    }
                    NavButton {
                        active: active_tab_value == MobileTab::Route,
                        icon: NavIconKind::Route,
                        label: language_value.text(CopyKey::RouteNav),
                        onclick: move |_| active_tab.set(MobileTab::Route),
                    }
                    div {}
                    NavButton {
                        active: active_tab_value == MobileTab::Contacts,
                        icon: NavIconKind::Contacts,
                        label: language_value.text(CopyKey::ContactsNav),
                        onclick: move |_| active_tab.set(MobileTab::Contacts),
                    }
                    NavButton {
                        active: active_tab_value == MobileTab::Profile,
                        icon: NavIconKind::Profile,
                        label: language_value.text(CopyKey::AccountNav),
                        onclick: move |_| active_tab.set(MobileTab::Profile),
                    }
                }

                button {
                    style: if *sos_active.read() { SOS_BUTTON_ACTIVE } else { SOS_BUTTON },
                    class: if *sos_active.read() { "ja-sos-orb ja-sos-active" } else { "ja-sos-orb" },
                    onclick: move |_| {
                        if *sos_active.read() {
                            stop_sos_alarm();
                            sos_active.set(false);
                            sos_msg.set(Some(language_value.text(CopyKey::SosStopped).to_string()));
                            sos_modal_open.set(true);
                            return;
                        }

                        let point = *location.read();
                        let hash = device_hash.read().clone();
                        if hash.is_empty() {
                            sos_msg.set(Some(language_value.text(CopyKey::AppPreparing).to_string()));
                            sos_modal_open.set(true);
                            return;
                        }
                        let whatsapp_contacts = contacts.read().clone();

                        sos_msg.set(Some(language_value.text(CopyKey::PreparingSosLocation).to_string()));
                        sos_modal_open.set(true);
                        spawn(async move {
                            let point = match point {
                                Some(point) => point,
                                None => match read_location().await {
                                    Ok(point) => {
                                        location.set(Some(point));
                                        manual_lat.set(format!("{:.6}", point.lat));
                                        manual_lng.set(format!("{:.6}", point.lng));
                                        location_error.set(None);
                                        point
                                    }
                                    Err(err) => {
                                        location_error.set(Some(err.clone()));
                                        sos_msg.set(Some(language_value.text(CopyKey::SosLocationMissing).to_string()));
                                        return;
                                    }
                                },
                            };

                        if let Err(err) = start_sos_alarm().await {
                            sos_active.set(false);
                            sos_msg.set(Some(err));
                            return;
                        }
                        sos_active.set(true);
                        sos_msg.set(Some(language_value.text(CopyKey::SosActiveSending).to_string()));

                            match trigger_sos(&hash, point).await {
                                Ok(response) => {
                                    let whatsapp_auto_sent = response
                                        .results
                                        .iter()
                                        .any(|result| result.whatsapp_sent);
                                    if whatsapp_auto_sent {
                                        sos_msg.set(Some(language_value.text(CopyKey::WhatsappSent).to_string()));
                                    } else {
                                        let whatsapp_opened = open_whatsapp_sos(&whatsapp_contacts, point)
                                            .await
                                            .unwrap_or(false);

                                        if whatsapp_opened {
                                            sos_msg.set(Some(language_value.text(CopyKey::WhatsappFallback).to_string()));
                                        } else if response.notified_count > 0 {
                                            sos_msg.set(Some(language_value.text(CopyKey::SosNotified).to_string()));
                                        } else {
                                            sos_msg.set(Some(language_value.text(CopyKey::SosNoChannel).to_string()));
                                        }
                                    }
                                }
                                Err(_) => {
                                    let whatsapp_opened = open_whatsapp_sos(&whatsapp_contacts, point)
                                        .await
                                        .unwrap_or(false);
                                    if whatsapp_opened {
                                        sos_msg.set(Some(language_value.text(CopyKey::SosBackendFallback).to_string()));
                                    } else {
                                        sos_msg.set(Some(language_value.text(CopyKey::SosStaySafe).to_string()));
                                    }
                                }
                            }
                        });
                    },
                    span {
                        if *sos_active.read() { "STOP" } else { "SOS" }
                    }
                }

                if *show_splash.read() {
                    LaunchSplash { language: language_value }
                }
            }
        }
    }
}

#[component]
fn LaunchSplash(language: Language) -> Element {
    rsx! {
        div { style: "position:absolute;inset:0;z-index:50;display:flex;align-items:center;justify-content:center;padding:28px;background:radial-gradient(circle at 50% 78%,rgba(34,197,94,0.26),rgba(34,197,94,0) 34%),linear-gradient(145deg,#27272a 0%,#17181c 52%,#0b0f14 100%);box-sizing:border-box;",
            div { class: "ja-splash", style: "width:100%;max-width:330px;text-align:center;",
                div { style: "width:142px;height:142px;margin:0 auto 24px;display:flex;align-items:center;justify-content:center;border-radius:34px;background:linear-gradient(180deg,rgba(255,255,255,0.14),rgba(255,255,255,0.06));border:1px solid rgba(255,255,255,0.17);box-shadow:0 28px 64px rgba(0,0,0,0.34),inset 0 1px 0 rgba(255,255,255,0.18);backdrop-filter:blur(24px) saturate(175%);-webkit-backdrop-filter:blur(24px) saturate(175%);",
                    img { class: "ja-splash-logo", src: LOGO, alt: "Logo JalanAman", style: "width:116px;height:116px;object-fit:contain;" }
                }
                div { style: "font-size:29px;font-weight:950;color:#f8fafc;letter-spacing:0;", "JalanAman" }
                div { style: "max-width:250px;margin:8px auto 0;color:#cbd5e1;font-size:14px;font-weight:750;line-height:1.45;", "{language.text(CopyKey::SplashSubtitle)}" }
                div { style: "width:54px;height:5px;margin:28px auto 0;border-radius:99px;background:linear-gradient(90deg,#2563eb,#0ea5e9,#14b8a6);box-shadow:0 10px 22px rgba(37,99,235,0.22);" }
            }
        }
    }
}

#[component]
fn SosOverlay(
    active: bool,
    language: Language,
    message: String,
    on_close: EventHandler<MouseEvent>,
    on_stop: EventHandler<MouseEvent>,
) -> Element {
    let status_title = if active {
        language.text(CopyKey::SosActiveTitle)
    } else {
        language.text(CopyKey::SosStatusTitle)
    };
    let status_body = if active {
        language.text(CopyKey::SosActiveBody)
    } else {
        language.text(CopyKey::SosClosedBody)
    };

    rsx! {
        div { style: "position:absolute;inset:0;z-index:45;display:flex;align-items:flex-end;padding:18px 16px 116px;background:rgba(2,6,23,0.56);box-sizing:border-box;backdrop-filter:blur(10px);-webkit-backdrop-filter:blur(10px);",
            div { class: "ja-splash", style: "width:100%;background:linear-gradient(180deg,rgba(52,54,62,0.86),rgba(16,18,24,0.78));border:1px solid rgba(255,255,255,0.14);border-radius:8px;box-shadow:0 28px 64px rgba(0,0,0,0.42),inset 0 1px 0 rgba(255,255,255,0.16);overflow:hidden;backdrop-filter:blur(24px) saturate(175%);-webkit-backdrop-filter:blur(24px) saturate(175%);",
                div { style: "height:6px;background:linear-gradient(90deg,#fb7185,#ef4444,#be123c);" }
                div { style: "padding:18px;",
                    div { style: "display:flex;align-items:flex-start;gap:12px;",
                        div { style: "width:44px;height:44px;border-radius:14px;background:rgba(127,29,29,0.30);color:#fecdd3;display:flex;align-items:center;justify-content:center;font-size:22px;font-weight:950;flex-shrink:0;box-shadow:inset 0 1px 0 rgba(255,255,255,0.14),0 10px 22px rgba(225,29,72,0.16);", "!" }
                        div { style: "min-width:0;flex:1;",
                            div { style: "font-size:17px;color:#f8fafc;font-weight:950;", "{status_title}" }
                            div { style: "margin-top:2px;font-size:11px;color:#fda4af;font-weight:850;", "{status_body}" }
                        }
                        button {
                            style: "width:36px;height:36px;border:1px solid rgba(255,255,255,0.14);border-radius:10px;background:rgba(255,255,255,0.08);color:#cbd5e1;font-size:21px;line-height:1;display:flex;align-items:center;justify-content:center;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);",
                            onclick: move |event| on_close.call(event),
                            "×"
                        }
                    }
                    div { style: "margin:16px 0 0;color:#cbd5e1;font-size:13px;font-weight:700;line-height:1.55;", "{message}" }
                    if active {
                        button {
                            style: "width:100%;margin-top:18px;border:1px solid rgba(255,255,255,0.38);border-radius:8px;background:#b91c1c;color:#ffffff;padding:13px 14px;font-size:14px;font-weight:950;box-shadow:0 14px 28px rgba(220,38,38,0.24),inset 0 1px 0 rgba(255,255,255,0.28),inset 0 -1px 0 rgba(0,0,0,0.22);",
                            onclick: move |event| on_stop.call(event),
                            "{language.text(CopyKey::StopAlarm)}"
                        }
                    } else {
                        button {
                            style: "width:100%;margin-top:18px;border:1px solid rgba(147,197,253,0.30);border-radius:8px;background:rgba(255,255,255,0.07);color:#bfdbfe;padding:13px 14px;font-size:14px;font-weight:900;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);",
                            onclick: move |event| on_close.call(event),
                            "{language.text(CopyKey::Close)}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MapView(
    map_html: String,
    reports: Vec<Report>,
    location: Option<GeoPoint>,
    loading: bool,
    error: Option<String>,
    route_score: Option<RouteScoreResponse>,
    language: Language,
    manual_lat: String,
    manual_lng: String,
    manual_error: Option<String>,
    presentation: MapPresentation,
    on_presentation: EventHandler<MapPresentation>,
    on_report: EventHandler<MouseEvent>,
    on_manual_lat: EventHandler<String>,
    on_manual_lng: EventHandler<String>,
    on_manual_apply: EventHandler<MouseEvent>,
    on_refresh: EventHandler<MouseEvent>,
) -> Element {
    let title = if loading {
        language.text(CopyKey::MapLoading).to_string()
    } else {
        format!(
            "{} {}",
            reports.len(),
            language.text(CopyKey::ReportsActiveRadius)
        )
    };
    let gps_label = location
        .map(|p| format!("{:.5}, {:.5}", p.lat, p.lng))
        .unwrap_or_else(|| language.text(CopyKey::GpsUnavailable).to_string());
    let live_badge = if loading {
        language.text(CopyKey::Loading).to_string()
    } else {
        language.text(CopyKey::Live).to_string()
    };

    rsx! {
        div {
            div { style: MAP_CARD,
                iframe {
                    style: MAP_IFRAME,
                    srcdoc: "{map_html}",
                }
                div { style: MAP_LABEL,
                    div { "{title}" }
                    div { style: "margin-top:3px;font-size:10px;color:#cbd5e1;font-weight:800;", "{gps_label}" }
                }
                div { style: MAP_PROVIDER, "{language.text(CopyKey::LiveMap)}" }
                div { style: "position:absolute;right:12px;top:12px;z-index:3;display:flex;gap:5px;padding:4px;border:1px solid rgba(255,255,255,0.18);border-radius:8px;background:rgba(38,41,48,0.78);box-shadow:0 12px 28px rgba(0,0,0,0.26),inset 0 1px 0 rgba(255,255,255,0.16),inset 0 -1px 0 rgba(0,0,0,0.24);backdrop-filter:blur(18px) saturate(170%);-webkit-backdrop-filter:blur(18px) saturate(170%);",
                    button {
                        style: if presentation == MapPresentation::TwoDimensional { "height:31px;min-width:40px;border:0;border-radius:7px;background:#1d4ed8;color:#ffffff;font-size:10px;font-weight:950;box-shadow:0 8px 16px rgba(37,99,235,0.24),inset 0 1px 0 rgba(255,255,255,0.24),inset 0 -1px 0 rgba(0,0,0,0.20);" } else { "height:31px;min-width:40px;border:0;border-radius:7px;background:transparent;color:#cbd5e1;font-size:10px;font-weight:900;" },
                        onclick: move |_| on_presentation.call(MapPresentation::TwoDimensional),
                        "2D"
                    }
                    button {
                        style: if presentation == MapPresentation::ThreeDimensional { "height:31px;min-width:40px;border:0;border-radius:7px;background:#1d4ed8;color:#ffffff;font-size:10px;font-weight:950;box-shadow:0 8px 16px rgba(37,99,235,0.24),inset 0 1px 0 rgba(255,255,255,0.24),inset 0 -1px 0 rgba(0,0,0,0.20);" } else { "height:31px;min-width:40px;border:0;border-radius:7px;background:transparent;color:#cbd5e1;font-size:10px;font-weight:900;" },
                        onclick: move |_| on_presentation.call(MapPresentation::ThreeDimensional),
                        "3D"
                    }
                }
                button {
                    style: REPORT_FAB,
                    title: "{language.text(CopyKey::QuickReportTitle)}",
                    onclick: move |event| on_report.call(event),
                    "+"
                }
            }

            if let Some(err) = error {
                Notice { message: err, danger: true }
            }

            if location.is_none() {
                div { style: CARD,
                    div { style: ROW,
                        div {
                            div { style: EYEBROW, "{language.text(CopyKey::ManualLocation)}" }
                            div { style: TITLE, "{language.text(CopyKey::UsePhoneCoordinates)}" }
                        }
                        Badge { label: language.text(CopyKey::Fallback).to_string(), bg: "#fef3c7", color: "#92400e" }
                    }
                    div { style: "margin-top:8px;",
                        div { style: BODY, "{language.text(CopyKey::ManualLocationBody)}" }
                    }
                    div { style: FIELD_GRID,
                        input {
                            style: INPUT,
                            value: "{manual_lat}",
                            placeholder: "Latitude",
                            oninput: move |event| on_manual_lat.call(event.value()),
                        }
                        input {
                            style: INPUT,
                            value: "{manual_lng}",
                            placeholder: "Longitude",
                            oninput: move |event| on_manual_lng.call(event.value()),
                        }
                    }
                    if let Some(err) = manual_error {
                        Notice { message: err, danger: true }
                    }
                    button {
                        style: PRIMARY_BUTTON,
                        onclick: move |event| on_manual_apply.call(event),
                        "{language.text(CopyKey::UseThisLocation)}"
                    }
                }
            }

            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "{language.text(CopyKey::NearbyReports)}" }
                        div { style: TITLE, "{title}" }
                    }
                    Badge { label: live_badge, bg: "rgba(37,99,235,0.24)", color: "#bfdbfe" }
                }
                div { style: "display:flex;flex-direction:column;gap:8px;margin-top:12px;",
                    if reports.is_empty() {
                        div { style: BODY, "{language.text(CopyKey::NoNearbyReports)}" }
                    } else {
                        for report in reports.iter().take(4) {
                            ReportRow { report: report.clone(), location, language }
                        }
                    }
                }
                button {
                    style: SECONDARY_BUTTON,
                    onclick: move |event| on_refresh.call(event),
                    "{language.text(CopyKey::RefreshGpsReports)}"
                }
            }

            if let Some(score) = route_score {
                div { style: CARD,
                    div { style: ROW,
                        div {
                            div { style: EYEBROW, "{language.text(CopyKey::LastRouteOverlay)}" }
                            div { style: TITLE, "{route_overlay_title(&score, language)}" }
                        }
                        Badge { label: score.level.clone(), bg: level_bg(&score.level), color: level_color(&score.level) }
                    }
                }
            }
        }
    }
}

#[component]
fn RouteView(
    destination: String,
    map_html: String,
    directions: Option<DirectionsResponse>,
    score: Option<RouteScoreResponse>,
    language: Language,
    loading: bool,
    error: Option<String>,
    suggestions: Vec<PlaceSuggestion>,
    suggestions_loading: bool,
    suggestions_error: Option<String>,
    selected_place: Option<PlaceSuggestion>,
    on_destination: EventHandler<String>,
    on_select_place: EventHandler<PlaceSuggestion>,
    on_search: EventHandler<MouseEvent>,
) -> Element {
    let has_directions = directions.is_some();
    let search_button_label = if loading {
        language.text(CopyKey::CheckingRoute)
    } else {
        language.text(CopyKey::CheckSafeRoute)
    };

    rsx! {
        div {
            div { style: CARD_TIGHT,
                div { style: EYEBROW, "{language.text(CopyKey::SearchDestination)}" }
                input {
                    style: INPUT,
                    value: "{destination}",
                    placeholder: "{language.text(CopyKey::SearchDestinationPlaceholder)}",
                    oninput: move |event| on_destination.call(event.value()),
                }
                if let Some(place) = selected_place {
                    div { style: "margin-top:9px;display:flex;align-items:center;gap:10px;padding:10px 11px;border:1px solid rgba(147,197,253,0.30);border-radius:8px;background:rgba(37,99,235,0.14);box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);",
                        span { style: "width:26px;height:26px;display:flex;align-items:center;justify-content:center;flex-shrink:0;border-radius:50%;background:#1d4ed8;color:#ffffff;font-size:13px;font-weight:900;box-shadow:0 8px 16px rgba(37,99,235,0.20),inset 0 1px 0 rgba(255,255,255,0.22);", "✓" }
                        div { style: "min-width:0;",
                            div { style: "font-size:12px;color:#f8fafc;font-weight:900;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{place.name}" }
                            div { style: "margin-top:2px;font-size:10px;color:#cbd5e1;font-weight:800;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{place.subtitle}" }
                        }
                    }
                } else if suggestions_loading {
                    div { style: "margin-top:10px;padding:10px 2px;color:#93c5fd;font-size:12px;font-weight:850;", "{language.text(CopyKey::SearchingPlaces)}" }
                } else if !suggestions.is_empty() {
                    div { style: "margin-top:9px;overflow:hidden;border:1px solid rgba(255,255,255,0.13);border-radius:8px;background:rgba(255,255,255,0.07);box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);backdrop-filter:blur(14px) saturate(150%);-webkit-backdrop-filter:blur(14px) saturate(150%);",
                        for place in suggestions {
                            {
                                let place_for_click = place.clone();
                                rsx! {
                                    button {
                                        style: "width:100%;min-height:58px;padding:10px 11px;border:0;border-bottom:1px solid rgba(255,255,255,0.10);background:transparent;color:#f8fafc;display:flex;align-items:center;gap:10px;text-align:left;",
                                        onclick: move |_| on_select_place.call(place_for_click.clone()),
                                        span { style: "width:26px;height:26px;border-radius:50%;display:flex;align-items:center;justify-content:center;flex-shrink:0;background:rgba(255,255,255,0.09);color:#93c5fd;font-size:14px;font-weight:900;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);", "●" }
                                        div { style: "min-width:0;flex:1;",
                                            div { style: "font-size:12px;font-weight:900;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{place.name}" }
                                            div { style: "margin-top:2px;color:#cbd5e1;font-size:10px;font-weight:800;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{place.subtitle}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Some(search_error) = suggestions_error {
                    div { style: "margin-top:10px;color:#fbbf24;font-size:11px;font-weight:800;line-height:1.4;", "{search_error}" }
                }
                button {
                    style: PRIMARY_BUTTON,
                    disabled: loading,
                    onclick: move |event| on_search.call(event),
                    "{search_button_label}"
                }
            }

            if let Some(err) = error {
                Notice { message: err, danger: !has_directions }
            }

            if has_directions {
                div { style: ROUTE_MAP_CARD,
                    iframe {
                        style: MAP_IFRAME,
                        srcdoc: "{map_html}",
                        title: "{language.text(CopyKey::SafeRouteMapTitle)}",
                    }
                    div { style: MAP_PROVIDER, "{language.text(CopyKey::LiveRoute)}" }
                }
            }

            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "{language.text(CopyKey::RouteScore)}" }
                        div { style: TITLE, "{language.text(CopyKey::RouteSafetyStatus)}" }
                    }
                    if let Some(score) = score.clone() {
                        Badge { label: localized_level(&score.level, language).to_string(), bg: level_bg(&score.level), color: level_color(&score.level) }
                    } else {
                        Badge { label: language.text(CopyKey::NotChecked).to_string(), bg: "rgba(37,99,235,0.18)", color: "#bfdbfe" }
                    }
                }

                if let Some(score) = score {
                    div { style: META_GRID,
                        Metric { value: format!("{:.1}", score.score), label: language.text(CopyKey::Weight) }
                        Metric { value: localized_level(&score.level, language).to_string(), label: language.text(CopyKey::Status) }
                        Metric { value: score.report_count.to_string(), label: language.text(CopyKey::Reports) }
                    }
                } else {
                    div { style: "margin-top:12px;", div { style: BODY, "{language.text(CopyKey::EnterDestinationHint)}" } }
                }
            }

            if let Some(dirs) = directions {
                div { style: CARD,
                    div { style: EYEBROW, "{language.text(CopyKey::RouteDetails)}" }
                    div { style: META_GRID,
                        Metric { value: distance_label(dirs.distance_m), label: language.text(CopyKey::Distance) }
                        Metric { value: duration_label(dirs.duration_s, language), label: language.text(CopyKey::Estimate) }
                        Metric { value: language.text(CopyKey::Walking).to_string(), label: language.text(CopyKey::Mode) }
                    }
                }
            }
        }
    }
}

#[component]
fn ReportView(
    category: ReportCategory,
    language: Language,
    note: String,
    location: Option<GeoPoint>,
    loading: bool,
    error: Option<String>,
    on_category: EventHandler<ReportCategory>,
    on_note: EventHandler<String>,
    on_submit: EventHandler<MouseEvent>,
) -> Element {
    let note_count = note.chars().count().to_string();
    let gps_status = if location.is_some() {
        language.text(CopyKey::GpsReady)
    } else {
        language.text(CopyKey::GpsNotReady)
    };
    let category_color = category.color();
    let category_label = category.label_for(language);
    let gps_badge_bg = if location.is_some() {
        "rgba(37,99,235,0.24)"
    } else {
        "rgba(127,29,29,0.30)"
    };
    let gps_badge_color = if location.is_some() {
        "#bfdbfe"
    } else {
        "#fecdd3"
    };
    let submit_label = if loading {
        language.text(CopyKey::Sending)
    } else {
        language.text(CopyKey::SubmitReport)
    };

    rsx! {
        div {
            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "{language.text(CopyKey::QuickReport)}" }
                        div { style: TITLE, "{language.text(CopyKey::ReportCategory)}" }
                    }
                    Badge { label: gps_status.to_string(), bg: gps_badge_bg, color: gps_badge_color }
                }

                div { style: CATEGORY_GRID,
                    for item in CATEGORIES {
                        CategoryButton {
                            category: item,
                            language,
                            selected: category == item,
                            onclick: move |_| on_category.call(item),
                        }
                    }
                }

                div { style: "margin-top:12px;",
                    div { style: EYEBROW, "{language.text(CopyKey::OptionalNote)}" }
                    textarea {
                        style: TEXTAREA,
                        value: "{note}",
                        maxlength: "100",
                        placeholder: "{language.text(CopyKey::Max100)}",
                        oninput: move |event| on_note.call(event.value()),
                    }
                    div { style: "margin-top:6px;text-align:right;font-size:10px;font-weight:750;color:#9ca3af;",
                        "{note_count}/100"
                    }
                }

                if let Some(err) = error {
                    Notice { message: err, danger: true }
                }

                button {
                    style: PRIMARY_BUTTON,
                    disabled: loading,
                    onclick: move |event| on_submit.call(event),
                    "{submit_label}"
                }
            }

            div { style: CARD,
                div { style: EYEBROW, "{language.text(CopyKey::ReportPreview)}" }
                div { style: "display:flex;align-items:center;gap:10px;border:1px solid rgba(255,255,255,0.12);background:rgba(255,255,255,0.07);border-radius:8px;padding:10px 11px;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);",
                    span { style: "width:11px;height:11px;border-radius:50%;background:{category_color};box-shadow:0 0 0 4px rgba(255,255,255,0.12),0 0 18px {category_color};flex-shrink:0;" }
                    div { style: "min-width:0;flex:1;",
                        div { style: "font-size:12px;font-weight:900;color:#f8fafc;", "{category_label}" }
                        div { style: "margin-top:2px;font-size:10px;font-weight:800;color:#cbd5e1;", "{gps_status}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ContactsView(
    contacts: Vec<EmergencyContact>,
    language: Language,
    name: String,
    email: String,
    phone: String,
    loading: bool,
    error: Option<String>,
    on_name: EventHandler<String>,
    on_email: EventHandler<String>,
    on_phone: EventHandler<String>,
    on_add: EventHandler<MouseEvent>,
    on_refresh: EventHandler<MouseEvent>,
) -> Element {
    let count = contacts.len().to_string();
    let refresh_label = if loading {
        language.text(CopyKey::Loading)
    } else {
        language.text(CopyKey::RefreshContacts)
    };
    let add_label = if loading {
        language.text(CopyKey::Saving)
    } else {
        language.text(CopyKey::AddContact)
    };

    rsx! {
        div {
            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "{language.text(CopyKey::EmergencyContacts)}" }
                        div { style: TITLE, "{count} {language.text(CopyKey::ContactsSaved)}" }
                    }
                    Badge { label: "Email/WA/SOS".to_string(), bg: "#fee2e2", color: "#991b1b" }
                }

                div { style: "display:flex;flex-direction:column;gap:9px;margin-top:12px;",
                    if contacts.is_empty() {
                        div { style: BODY, "{language.text(CopyKey::NoContacts)}" }
                    } else {
                        for contact in contacts {
                            ContactRow { contact, language }
                        }
                    }
                }

                if let Some(err) = error {
                    Notice { message: err, danger: true }
                }
                button {
                    style: SECONDARY_BUTTON,
                    onclick: move |event| on_refresh.call(event),
                    "{refresh_label}"
                }
            }

            div { style: CARD,
                div { style: EYEBROW, "{language.text(CopyKey::AddSosContact)}" }
                input {
                    style: INPUT,
                    value: "{name}",
                    placeholder: "{language.text(CopyKey::ContactName)}",
                    oninput: move |event| on_name.call(event.value()),
                }
                div { style: "height:8px;" }
                input {
                    style: INPUT,
                    value: "{email}",
                    placeholder: "{language.text(CopyKey::ContactEmail)}",
                    oninput: move |event| on_email.call(event.value()),
                }
                div { style: "height:8px;" }
                input {
                    style: INPUT,
                    value: "{phone}",
                    placeholder: "{language.text(CopyKey::ContactPhone)}",
                    inputmode: "tel",
                    oninput: move |event| on_phone.call(event.value()),
                }
                button {
                    style: PRIMARY_BUTTON,
                    disabled: loading,
                    onclick: move |event| on_add.call(event),
                    "{add_label}"
                }
            }
        }
    }
}

#[component]
fn ProfileView(
    location: Option<GeoPoint>,
    language: Language,
    report_count: usize,
    contact_count: usize,
    location_error: Option<String>,
) -> Element {
    let location_status = if location.is_some() {
        language.text(CopyKey::Active).to_string()
    } else {
        language.text(CopyKey::GpsNotReady).to_string()
    };

    rsx! {
        div {
            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "{language.text(CopyKey::AccountPrivacy)}" }
                        div { style: TITLE, "{language.text(CopyKey::Anonymous)}" }
                    }
                    Badge { label: language.text(CopyKey::Protected).to_string(), bg: "rgba(37,99,235,0.24)", color: "#bfdbfe" }
                }
                div { style: META_GRID,
                    Metric { value: location_status, label: language.text(CopyKey::Location) }
                    Metric { value: report_count.to_string(), label: language.text(CopyKey::Nearby) }
                    Metric { value: contact_count.to_string(), label: language.text(CopyKey::SosContacts) }
                }
            }

            div { style: CARD,
                div { style: EYEBROW, "{language.text(CopyKey::SafetySettings)}" }
                StatusRow { label: language.text(CopyKey::NearbyMap), value: language.text(CopyKey::Active).to_string() }
                StatusRow { label: language.text(CopyKey::SosAlerts), value: language.text(CopyKey::ReadyToUse).to_string() }
                if location_error.is_some() {
                    Notice { message: language.text(CopyKey::LocationNotice).to_string(), danger: true }
                }
            }
        }
    }
}

#[component]
fn HelpView(language: Language) -> Element {
    rsx! {
        div {
            div { style: CARD,
                div { style: EYEBROW, "{language.text(CopyKey::HeaderHelpTitle)}" }
                div { style: TITLE, "{language.text(CopyKey::HelpTitle)}" }
                div { style: "margin-top:8px;", div { style: BODY, "{language.text(CopyKey::HelpSubtitle)}" } }
            }

            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "{language.text(CopyKey::TutorialVideo)}" }
                        div { style: TITLE, "JalanAman" }
                    }
                    Badge { label: "MP4".to_string(), bg: "rgba(37,99,235,0.24)", color: "#bfdbfe" }
                }
                div { style: "margin-top:12px;overflow:hidden;border-radius:8px;border:1px solid rgba(255,255,255,0.14);background:#05070b;box-shadow:0 18px 36px rgba(0,0,0,0.28),inset 0 1px 0 rgba(255,255,255,0.10);",
                    video {
                        style: "width:100%;max-height:55dvh;display:block;background:#05070b;object-fit:contain;",
                        controls: true,
                        preload: "metadata",
                        source {
                            src: TUTORIAL_VIDEO,
                            r#type: "video/mp4",
                        }
                        "{language.text(CopyKey::VideoUnavailable)}"
                    }
                }
            }
        }
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! {
        main { style: APP,
            section { style: DASHBOARD_WRAP,
                Link { to: Route::Home {}, style: BACK_LINK, "< Kembali" }
                h1 { style: DASH_TITLE, "Dashboard mobile" }
                div { style: CARD,
                    div { style: EYEBROW, "Catatan" }
                    div { style: TITLE, "Dashboard stakeholder ada di frontend/web" }
                    div { style: BODY, "Mobile fokus untuk user lapangan: peta, rute, laporan cepat, kontak, dan SOS." }
                }
            }
        }
    }
}

#[component]
fn Fallback(segments: Vec<String>) -> Element {
    let _ = segments;

    rsx! { Home {} }
}

#[component]
fn CategoryButton(
    category: ReportCategory,
    language: Language,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let color = category.color();

    rsx! {
        button {
            style: if selected { CATEGORY_BUTTON_ACTIVE } else { CATEGORY_BUTTON },
            onclick: move |event| onclick.call(event),
            span {
                style: "width:11px;height:11px;border-radius:50%;background:{color};display:inline-block;flex-shrink:0;",
            }
            span { "{category.short_label_for(language)}" }
        }
    }
}

#[component]
fn ReportRow(report: Report, location: Option<GeoPoint>, language: Language) -> Element {
    let category = ReportCategory::from_api(&report.category);
    let category_color = category.color();
    let category_label = category.short_label_for(language);
    let distance = location
        .map(|point| distance_label(haversine_m(point.lat, point.lng, report.lat, report.lng)))
        .unwrap_or_else(|| {
            if language.is_indonesian() {
                "jarak belum ada".to_string()
            } else {
                "distance unavailable".to_string()
            }
        });
    let note = report
        .note
        .clone()
        .unwrap_or_else(|| category.label_for(language).to_string());

    rsx! {
        div {
            style: "display:flex;align-items:center;gap:10px;border:1px solid rgba(255,255,255,0.12);background:rgba(255,255,255,0.07);border-radius:8px;padding:10px 11px;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);",
            span {
                style: "width:11px;height:11px;border-radius:50%;background:{category_color};box-shadow:0 0 0 4px rgba(255,255,255,0.12),0 0 18px {category_color};flex-shrink:0;",
            }
            div { style: "min-width:0;flex:1;",
                div { style: "font-size:12px;font-weight:900;color:#f8fafc;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{note}" }
                div { style: "margin-top:2px;font-size:10px;font-weight:800;color:#cbd5e1;", "{category_label} | {distance}" }
            }
        }
    }
}

#[component]
fn ContactRow(contact: EmergencyContact, language: Language) -> Element {
    let status = if contact.push_endpoint.is_some() {
        if language.is_indonesian() {
            "Push siap"
        } else {
            "Push ready"
        }
    } else if contact.email.is_some() && contact.phone.is_some() {
        "Email + WA"
    } else if contact.email.is_some() {
        if language.is_indonesian() {
            "Email siap"
        } else {
            "Email ready"
        }
    } else if contact.phone.is_some() {
        if language.is_indonesian() {
            "WA siap"
        } else {
            "WA ready"
        }
    } else {
        if language.is_indonesian() {
            "Menunggu"
        } else {
            "Pending"
        }
    };
    let mut details = Vec::new();
    if let Some(email) = contact.email.clone().filter(|value| !value.is_empty()) {
        details.push(email);
    }
    if let Some(phone) = contact.phone.clone().filter(|value| !value.is_empty()) {
        details.push(format!("WA {phone}"));
    }
    if details.is_empty() {
        if let Some(device) = contact.contact_device_hash.clone() {
            details.push(device);
        }
    }
    let detail = if details.is_empty() {
        if language.is_indonesian() {
            "Belum tersambung".to_string()
        } else {
            "Not connected".to_string()
        }
    } else {
        details.join(" | ")
    };
    let waiting = status == "Menunggu" || status == "Pending";
    let status_bg = if waiting {
        "rgba(146,64,14,0.28)"
    } else {
        "rgba(37,99,235,0.24)"
    };
    let status_color = if waiting { "#fde68a" } else { "#bfdbfe" };

    rsx! {
        div { style: "display:flex;align-items:center;justify-content:space-between;gap:12px;padding:11px;border:1px solid rgba(255,255,255,0.12);border-radius:8px;background:rgba(255,255,255,0.07);box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);",
            div { style: "min-width:0;",
                div { style: TITLE, "{contact.name}" }
                div { style: "font-size:11px;color:#cbd5e1;font-weight:800;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{detail}" }
            }
            Badge { label: status.to_string(), bg: status_bg, color: status_color }
        }
    }
}

#[component]
fn Metric(value: String, label: &'static str) -> Element {
    rsx! {
        div { style: META_CELL,
            div { style: META_VALUE, "{value}" }
            div { style: META_LABEL, "{label}" }
        }
    }
}

#[component]
fn Badge(label: String, bg: &'static str, color: &'static str) -> Element {
    rsx! {
        span {
            style: "flex-shrink:0;border-radius:999px;background:{bg};color:{color};padding:7px 11px;font-size:11px;font-weight:950;white-space:nowrap;border:1px solid rgba(255,255,255,0.18);box-shadow:inset 0 1px 0 rgba(255,255,255,0.16),0 8px 16px rgba(0,0,0,0.12);",
            "{label}"
        }
    }
}

#[component]
fn Notice(message: String, danger: bool) -> Element {
    let style = if danger {
        "margin-top:10px;border-radius:8px;border:1px solid rgba(254,202,202,0.22);background:rgba(127,29,29,0.24);color:#fecdd3;padding:10px 11px;font-size:12px;font-weight:850;line-height:1.4;box-shadow:inset 0 1px 0 rgba(255,255,255,0.10);backdrop-filter:blur(14px) saturate(145%);-webkit-backdrop-filter:blur(14px) saturate(145%);"
    } else {
        "margin-top:10px;border-radius:8px;border:1px solid rgba(147,197,253,0.26);background:rgba(37,99,235,0.16);color:#bfdbfe;padding:10px 11px;font-size:12px;font-weight:850;line-height:1.4;box-shadow:inset 0 1px 0 rgba(255,255,255,0.10);backdrop-filter:blur(14px) saturate(145%);-webkit-backdrop-filter:blur(14px) saturate(145%);"
    };

    rsx! { div { style: "{style}", "{message}" } }
}

#[component]
fn StatusRow(label: &'static str, value: String) -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;justify-content:space-between;gap:12px;padding:10px 0;border-bottom:1px solid rgba(255,255,255,0.10);",
            span { style: "font-size:12px;font-weight:850;color:#cbd5e1;", "{label}" }
            span { style: "font-size:12px;font-weight:950;color:#93c5fd;text-align:right;min-width:0;overflow:hidden;text-overflow:ellipsis;", "{value}" }
        }
    }
}

#[component]
fn NavButton(
    active: bool,
    icon: NavIconKind,
    label: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            style: if active { NAV_BUTTON_ACTIVE } else { NAV_BUTTON },
            onclick: move |event| onclick.call(event),
            NavIcon { icon }
            span { "{label}" }
        }
    }
}

#[component]
fn NavIcon(icon: NavIconKind) -> Element {
    match icon {
        NavIconKind::Map => rsx! {
            svg {
                style: NAV_ICON,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M9 18 3 21V6l6-3 6 3 6-3v15l-6 3-6-3Z" }
                path { d: "M9 3v15" }
                path { d: "M15 6v15" }
            }
        },
        NavIconKind::Route => rsx! {
            svg {
                style: NAV_ICON,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "6", cy: "19", r: "2.2" }
                circle { cx: "18", cy: "5", r: "2.2" }
                path { d: "M8.2 19H17a4 4 0 0 0 0-8H7a4 4 0 0 1 0-8h8.8" }
            }
        },
        NavIconKind::Contacts => rsx! {
            svg {
                style: NAV_ICON,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M16 21v-2a4 4 0 0 0-4-4H7a4 4 0 0 0-4 4v2" }
                circle { cx: "9.5", cy: "7", r: "4" }
                path { d: "M22 21v-2a4 4 0 0 0-3-3.87" }
                path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
            }
        },
        NavIconKind::Profile => rsx! {
            svg {
                style: NAV_ICON,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "12", cy: "8", r: "4" }
                path { d: "M4 21a8 8 0 0 1 16 0" }
            }
        },
    }
}

async fn read_device_hash() -> String {
    let eval = document::eval(
        r#"
        const key = 'ja_device_hash';
        let hash = localStorage.getItem(key);
        if (!hash) {
            hash = (crypto && crypto.randomUUID)
                ? crypto.randomUUID()
                : `mobile-${Date.now()}-${Math.random().toString(36).slice(2)}`;
            localStorage.setItem(key, hash);
        }
        return hash;
        "#,
    );

    eval.await
        .ok()
        .and_then(|value| String::deserialize(&value).ok())
        .unwrap_or_else(|| format!("mobile-{}", uuid::Uuid::new_v4()))
}

async fn read_location() -> Result<GeoPoint, String> {
    let eval = document::eval(
        r#"
        return await new Promise((resolve) => {
            let nativeError = '';
            try {
                if (window.JalanAmanNative && (window.JalanAmanNative.getCurrentLocationJson || window.JalanAmanNative.getLastLocationJson)) {
                    const getter = window.JalanAmanNative.getCurrentLocationJson || window.JalanAmanNative.getLastLocationJson;
                    const native = JSON.parse(getter.call(window.JalanAmanNative));
                    if (Number.isFinite(native.lat) && Number.isFinite(native.lng)) {
                        resolve({ lat: native.lat, lng: native.lng });
                        return;
                    }
                    nativeError = native.error || '';
                    if (/izin|permission|layanan lokasi|location service|gps/i.test(nativeError)) {
                        resolve({ error: nativeError });
                        return;
                    }
                }
            } catch (err) {
                nativeError = err && err.message ? err.message : 'Lokasi native gagal dibaca.';
            }

            if (!navigator.geolocation) {
                resolve({ error: nativeError || 'Geolocation tidak tersedia di perangkat ini.' });
                return;
            }
            navigator.geolocation.getCurrentPosition(
                (pos) => resolve({
                    lat: pos.coords.latitude,
                    lng: pos.coords.longitude,
                }),
                (err) => resolve({ error: nativeError ? `${nativeError}. ${err.message || 'Izin lokasi ditolak.'}` : (err.message || 'Izin lokasi ditolak.') }),
                { enableHighAccuracy: true, timeout: 12000, maximumAge: 15000 }
            );
        });
        "#,
    );

    let value = match eval.await {
        Ok(value) => value,
        Err(err) => {
            return Err(normalize_location_error(format!(
                "Gagal membaca GPS: {err}"
            )))
        }
    };
    let result = LocationEval::deserialize(&value).map_err(|err| err.to_string())?;

    match (result.lat, result.lng) {
        (Some(lat), Some(lng)) if lat.is_finite() && lng.is_finite() => Ok(GeoPoint { lat, lng }),
        _ => {
            Err(normalize_location_error(result.error.unwrap_or_else(
                || "GPS belum memberi koordinat.".to_string(),
            )))
        }
    }
}

fn normalize_location_error(message: String) -> String {
    let lower = message.to_lowercase();

    if lower.contains("permission")
        || lower.contains("denied")
        || lower.contains("ditolak")
        || lower.contains("izin")
    {
        return "Izin lokasi belum aktif. Izinkan lokasi untuk JalanAman, lalu coba lagi."
            .to_string();
    }

    if lower.contains("gps")
        || lower.contains("layanan lokasi")
        || lower.contains("location service")
    {
        return "GPS belum aktif. Nyalakan Lokasi pada HP, lalu coba lagi.".to_string();
    }

    "Lokasi belum tersedia. Pastikan GPS dan koneksi internet aktif, lalu coba lagi.".to_string()
}

async fn request_app_permissions() {
    let _ = document::eval(
        r#"
        try {
            if (window.JalanAmanNative && window.JalanAmanNative.requestAppPermissionsJson) {
                window.JalanAmanNative.requestAppPermissionsJson();
            }
        } catch (_) {}
        return true;
        "#,
    )
    .await;
}

fn normalize_whatsapp_phone(value: &str) -> Option<String> {
    let mut digits = value
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .collect::<String>();

    if digits.is_empty() {
        return None;
    }

    if digits.starts_with("00") {
        digits = digits.trim_start_matches("00").to_string();
    } else if digits.starts_with('0') {
        digits = format!("62{}", digits.trim_start_matches('0'));
    } else if digits.starts_with('8') {
        digits = format!("62{digits}");
    }

    if digits.len() < 8 {
        None
    } else {
        Some(digits)
    }
}

async fn open_whatsapp_sos(contacts: &[EmergencyContact], point: GeoPoint) -> Result<bool, String> {
    let phone = contacts
        .iter()
        .filter_map(|contact| contact.phone.as_deref())
        .find_map(normalize_whatsapp_phone);

    let Some(phone) = phone else {
        return Ok(false);
    };

    let message = format!(
        "SOS JalanAman: saya butuh bantuan sekarang.\nLokasi saya: https://maps.google.com/?q={},{}",
        point.lat, point.lng
    );
    let phone_json = serde_json::to_string(&phone).map_err(|err| err.to_string())?;
    let message_json = serde_json::to_string(&message).map_err(|err| err.to_string())?;
    let script = format!(
        r#"
        const phone = {phone_json};
        const message = {message_json};
        try {{
            if (window.JalanAmanNative && window.JalanAmanNative.openWhatsAppJson) {{
                const result = JSON.parse(window.JalanAmanNative.openWhatsAppJson(phone, message));
                return !!result.ok;
            }}
            const url = `https://wa.me/${{phone}}?text=${{encodeURIComponent(message)}}`;
            window.location.href = url;
            return true;
        }} catch (err) {{
            return false;
        }}
        "#
    );

    let eval = document::eval(&script);
    let value = eval.await.map_err(|err| err.to_string())?;
    bool::deserialize(&value).map_err(|err| err.to_string())
}

async fn start_sos_alarm() -> Result<(), String> {
    let eval = document::eval(
        r#"
        try {
            if (window.JalanAmanNative && window.JalanAmanNative.startSosAlarmJson) {
                const result = JSON.parse(window.JalanAmanNative.startSosAlarmJson());
                return !!result.ok;
            }
        } catch (_) {}
        return false;
            "#,
    );

    let started = eval
        .await
        .ok()
        .and_then(|value| bool::deserialize(&value).ok())
        .unwrap_or(false);

    if started {
        Ok(())
    } else {
        Err("Alarm SOS belum dapat dimulai. Pastikan izin notifikasi dan suara untuk JalanAman sudah diizinkan, lalu coba lagi.".to_string())
    }
}

async fn is_sos_alarm_active() -> bool {
    let eval = document::eval(
        r#"
        try {
            if (window.JalanAmanNative && window.JalanAmanNative.isSosAlarmActiveJson) {
                const result = JSON.parse(window.JalanAmanNative.isSosAlarmActiveJson());
                return !!result.active;
            }
        } catch (_) {}
        return false;
        "#,
    );

    eval.await
        .ok()
        .and_then(|value| bool::deserialize(&value).ok())
        .unwrap_or(false)
}

fn stop_sos_alarm() {
    spawn(async {
        let _ = document::eval(
            r#"
            try {
                if (window.JalanAmanNative && window.JalanAmanNative.stopSosAlarmJson) {
                    window.JalanAmanNative.stopSosAlarmJson();
                }
            } catch (_) {}
            if (navigator.vibrate) navigator.vibrate(0);
            return true;
            "#,
        )
        .await;
    });
}

async fn get_reports(point: GeoPoint) -> Result<Vec<Report>, String> {
    let client = reqwest::Client::new();
    request_json(client.get(api_url("/reports")).query(&[
        ("lat", point.lat.to_string()),
        ("lng", point.lng.to_string()),
        ("radius", "800".to_string()),
    ]))
    .await
}

async fn create_report(payload: &CreateReportPayload) -> Result<Report, String> {
    let client = reqwest::Client::new();
    request_json(client.post(api_url("/reports")).json(payload)).await
}

async fn get_directions(point: GeoPoint, destination: &str) -> Result<DirectionsResponse, String> {
    let client = reqwest::Client::new();
    request_json(client.get(api_url("/directions")).query(&[
        ("origin_lat", point.lat.to_string()),
        ("origin_lng", point.lng.to_string()),
        ("destination", destination.to_string()),
        ("mode", "walking".to_string()),
    ]))
    .await
}

async fn search_places(
    query: &str,
    origin: Option<GeoPoint>,
) -> Result<Vec<PlaceSuggestion>, String> {
    let client = reqwest::Client::new();
    let mut parameters = vec![("q", query.to_string())];
    if let Some(origin) = origin {
        parameters.push(("lat", origin.lat.to_string()));
        parameters.push(("lng", origin.lng.to_string()));
    }

    request_json(client.get(api_url("/places")).query(&parameters)).await
}

async fn calculate_route_score(waypoints: Vec<Waypoint>) -> Result<RouteScoreResponse, String> {
    let client = reqwest::Client::new();
    request_json(
        client
            .post(api_url("/route-score"))
            .json(&RouteScorePayload { waypoints }),
    )
    .await
}

async fn get_contacts(device_hash: &str) -> Result<Vec<EmergencyContact>, String> {
    let client = reqwest::Client::new();
    request_json(
        client
            .get(api_url("/sos/contacts"))
            .query(&[("device_hash", device_hash.to_string())]),
    )
    .await
}

async fn add_contact(
    device_hash: &str,
    name: &str,
    email: Option<String>,
    phone: Option<String>,
) -> Result<EmergencyContact, String> {
    let client = reqwest::Client::new();
    request_json(
        client
            .post(api_url("/sos/contacts"))
            .json(&AddContactPayload {
                device_hash: device_hash.to_string(),
                name: name.to_string(),
                email,
                phone,
            }),
    )
    .await
}

async fn trigger_sos(device_hash: &str, point: GeoPoint) -> Result<SosTriggerResponse, String> {
    let client = reqwest::Client::new();
    request_json(
        client
            .post(api_url("/sos/trigger"))
            .json(&SosTriggerPayload {
                device_hash: device_hash.to_string(),
                lat: point.lat,
                lng: point.lng,
                message: Some("SOS JalanAman: saya butuh bantuan sekarang.".to_string()),
            }),
    )
    .await
}

async fn request_json<T>(request: reqwest::RequestBuilder) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let response = request.send().await.map_err(|_| {
        "Tidak dapat terhubung. Periksa koneksi internet lalu coba lagi.".to_string()
    })?;
    let status = response.status();

    if status.is_success() {
        return response
            .json::<T>()
            .await
            .map_err(|_| "Data belum dapat dimuat. Coba lagi sebentar.".to_string());
    }

    let _ = response.text().await;
    let _ = status;
    Err("Permintaan belum dapat diproses. Coba lagi sebentar.".to_string())
}

fn api_url(path: &str) -> String {
    format!("{}{}", api_base().trim_end_matches('/'), path)
}

fn api_base() -> &'static str {
    option_env!("JALANAMAN_API_BASE_URL").unwrap_or(DEFAULT_API_BASE)
}

fn map_srcdoc(
    location: Option<GeoPoint>,
    reports: &[Report],
    route: Option<&[Waypoint]>,
    route_level: Option<&str>,
    three_dimensional: bool,
    language: Language,
) -> String {
    let location_json = serde_json::to_string(&location).unwrap_or_else(|_| "null".to_string());
    let reports_json = serde_json::to_string(
        &reports
            .iter()
            .map(|report| MapReport {
                id: report.id.clone(),
                category: report.category.clone(),
                lat: report.lat,
                lng: report.lng,
                note: report.note.clone(),
            })
            .collect::<Vec<_>>(),
    )
    .unwrap_or_else(|_| "[]".to_string());
    let route_json =
        serde_json::to_string(&route.unwrap_or(&[])).unwrap_or_else(|_| "[]".to_string());
    let route_level_json = serde_json::to_string(&route_level.unwrap_or("Aman"))
        .unwrap_or_else(|_| "\"Aman\"".to_string());
    let three_dimensional_json = if three_dimensional { "true" } else { "false" };
    let loading_title = if language.is_indonesian() {
        "Memuat peta"
    } else {
        "Loading map"
    };
    let loading_body = if language.is_indonesian() {
        "Menyiapkan jalan dan laporan di sekitarmu."
    } else {
        "Preparing roads and reports around you."
    };
    let map_failed_title = if language.is_indonesian() {
        "Peta belum termuat"
    } else {
        "Map did not load"
    };
    let map_failed_body = if language.is_indonesian() {
        "Cek koneksi internet lalu coba refresh peta."
    } else {
        "Check your internet connection, then refresh the map."
    };
    let three_d_failed_title = if language.is_indonesian() {
        "Tampilan 3D belum termuat"
    } else {
        "3D view did not load"
    };
    let three_d_failed_body = if language.is_indonesian() {
        "Periksa koneksi internet atau kembali ke tampilan 2D."
    } else {
        "Check your internet connection or switch back to 2D."
    };

    r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <link rel="stylesheet" href="https://unpkg.com/maplibre-gl@5.24.0/dist/maplibre-gl.css" />
  <style>
    html, body, #map { margin:0; width:100%; height:100%; overflow:hidden; background:linear-gradient(145deg,#27272a,#10141a); }
    #map { position:relative; touch-action:none; cursor:grab; font-family:system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif; }
    #map.dragging { cursor:grabbing; }
    #viewport { position:absolute; inset:0; }
    #tiles, #overlay, #points { position:absolute; inset:0; }
    #tiles img { position:absolute; width:256px; height:256px; image-rendering:auto; user-select:none; -webkit-user-drag:none; pointer-events:none; }
    #overlay { pointer-events:none; z-index:3; }
    #points { z-index:4; pointer-events:none; }
    #fallback { position:absolute; inset:0; z-index:6; display:flex; flex-direction:column; align-items:center; justify-content:center; gap:6px; padding:24px; box-sizing:border-box; background:linear-gradient(145deg,rgba(39,40,45,.92),rgba(13,18,24,.86)); color:#f8fafc; font-family:system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif; text-align:center; backdrop-filter:blur(14px) saturate(150%); -webkit-backdrop-filter:blur(14px) saturate(150%); }
    #fallback strong { font-size:14px; font-weight:900; }
    #fallback span { max-width:220px; font-size:11px; font-weight:700; line-height:1.4; color:#cbd5e1; }
    .pin { position:absolute; width:24px; height:24px; margin:-12px 0 0 -12px; border-radius:50%; border:3px solid #fff; box-shadow:0 10px 18px rgba(15,23,42,.25); display:flex; align-items:center; justify-content:center; color:#fff; font:900 11px system-ui; }
    .me { position:absolute; width:18px; height:18px; margin:-9px 0 0 -9px; border-radius:50%; background:#1d4ed8; border:3px solid #fff; box-shadow:0 0 0 14px rgba(37,99,235,.17),0 10px 22px rgba(15,23,42,.26); }
    .route-end { position:absolute; width:28px; height:28px; margin:-28px 0 0 -14px; border-radius:9px 9px 9px 2px; transform:rotate(-45deg); background:#1d4ed8; border:3px solid #fff; box-shadow:0 12px 22px rgba(15,23,42,.25); display:flex; align-items:center; justify-content:center; }
    .route-end span { transform:rotate(45deg); color:#fff; font:900 12px system-ui; }
    #zoomctl { position:absolute; right:10px; top:50%; transform:translateY(-50%); z-index:5; display:flex; flex-direction:column; border-radius:12px; overflow:hidden; border:1px solid rgba(255,255,255,.18); box-shadow:0 12px 24px rgba(15,23,42,.24),inset 0 1px 0 rgba(255,255,255,.16); backdrop-filter:blur(18px) saturate(170%); -webkit-backdrop-filter:blur(18px) saturate(170%); }
    #zoomctl button { display:block; width:36px; height:36px; border:0; background:rgba(24,27,34,0.72); color:#bfdbfe; font:950 19px/36px system-ui; padding:0; }
    #zoomctl button:first-child { border-bottom:1px solid rgba(255,255,255,.12); }
    #map.is-3d #viewport, #map.is-3d #zoomctl { display:none; }
    .ja-me-3d { width:20px; height:20px; border-radius:50%; background:#1d4ed8; border:3px solid #fff; box-shadow:0 0 0 14px rgba(37,99,235,.17),0 10px 22px rgba(15,23,42,.26); box-sizing:border-box; }
    #map.is-3d .maplibregl-ctrl-top-right { top:82px; right:10px; }
    #map.is-3d .maplibregl-ctrl-top-right .maplibregl-ctrl { margin:0; }
    .maplibregl-ctrl-group { border-radius:8px; overflow:hidden; border:1px solid rgba(255,255,255,.22); box-shadow:0 12px 24px rgba(15,23,42,.24); backdrop-filter:blur(18px) saturate(170%); -webkit-backdrop-filter:blur(18px) saturate(170%); }
  </style>
</head>
<body>
<div id="map">
  <div id="viewport">
    <div id="tiles"></div>
    <svg id="overlay"></svg>
    <div id="points"></div>
  </div>
  <div id="zoomctl">
    <button id="zoomIn" type="button">+</button>
    <button id="zoomOut" type="button">&minus;</button>
  </div>
  <div id="fallback"><strong>__LOADING_TITLE__</strong><span>__LOADING_BODY__</span></div>
</div>
<script src="https://unpkg.com/maplibre-gl@5.24.0/dist/maplibre-gl.js"></script>
<script>
const locationPoint = __LOCATION__;
const reports = __REPORTS__;
const route = __ROUTE__;
const routeLevel = __ROUTE_LEVEL__;
const threeDimensional = __THREE_DIMENSIONAL__;
const mapEl = document.getElementById('map');
const viewportEl = document.getElementById('viewport');
const tilesEl = document.getElementById('tiles');
const pointsEl = document.getElementById('points');
const overlay = document.getElementById('overlay');
const fallback = document.getElementById('fallback');
const MIN_ZOOM = 3;
const MAX_ZOOM = 18;
let activeCenter = null;
let activeZoom = null;
function showFallback(title, body) {
  fallback.style.display = 'flex';
  fallback.innerHTML = `<strong>${title}</strong><span>${body}</span>`;
}
const colors = { lighting:'#f59e0b', crime:'#ef4444', accident:'#f97316', other:'#94a3b8' };
const levelColors = { Aman:'#3b82f6', Waspada:'#f59e0b', Hindari:'#ef4444' };
const firstReport = reports[0];
const tileSize = 256;

function renderThreeDimensionalMap() {
  mapEl.classList.add('is-3d');
  if (!window.maplibregl) {
    showFallback('__THREE_D_FAILED_TITLE__', '__THREE_D_FAILED_BODY__');
    return;
  }

  const map = new maplibregl.Map({
    container: 'map',
    style: 'https://tiles.openfreemap.org/styles/liberty',
    center: baseCenter(),
    zoom: route.length > 1 ? 13.5 : (locationPoint || firstReport ? 16 : 11),
    pitch: 54,
    bearing: -24,
    maxPitch: 70,
    attributionControl: false,
    canvasContextAttributes: { antialias: true },
  });

  let styleReady = false;
  map.addControl(new maplibregl.NavigationControl({ showCompass: true }), 'top-right');
  map.on('load', () => {
    styleReady = true;
    fallback.style.display = 'none';

    const layers = map.getStyle().layers || [];
    const labelLayer = layers.find(layer => layer.type === 'symbol' && layer.layout && layer.layout['text-field']);
    map.addSource('jalanaman-buildings', { type: 'vector', url: 'https://tiles.openfreemap.org/planet' });
    map.addLayer({
      id: 'jalanaman-3d-buildings',
      source: 'jalanaman-buildings',
      'source-layer': 'building',
      type: 'fill-extrusion',
      minzoom: 15,
      filter: ['!=', ['get', 'hide_3d'], true],
      paint: {
        'fill-extrusion-color': ['interpolate', ['linear'], ['get', 'render_height'], 0, '#cbd5e1', 80, '#60a5fa', 220, '#1d4ed8'],
        'fill-extrusion-height': ['interpolate', ['linear'], ['zoom'], 15, 0, 16, ['get', 'render_height']],
        'fill-extrusion-base': ['case', ['>=', ['zoom'], 16], ['get', 'render_min_height'], 0],
        'fill-extrusion-opacity': 0.88,
      },
    }, labelLayer && labelLayer.id);

    if (route.length > 1) {
      map.addSource('jalanaman-route', {
        type: 'geojson',
        data: { type: 'Feature', properties: {}, geometry: { type: 'LineString', coordinates: route.map(point => [point.lng, point.lat]) } },
      });
      map.addLayer({
        id: 'jalanaman-route-line', type: 'line', source: 'jalanaman-route',
        paint: { 'line-color': levelColors[routeLevel] || '#1d4ed8', 'line-width': 6, 'line-opacity': 0.94 },
      });
      const destination = route[route.length - 1];
      new maplibregl.Marker({ color: '#1d4ed8' }).setLngLat([destination.lng, destination.lat]).addTo(map);
      const bounds = route.reduce((value, point) => value.extend([point.lng, point.lat]), new maplibregl.LngLatBounds(route[0], route[0]));
      map.fitBounds(bounds, { padding: { top: 64, right: 52, bottom: 92, left: 52 }, maxZoom: 16, pitch: 54, bearing: -24, duration: 0 });
    }

    if (locationPoint) {
      const marker = document.createElement('div');
      marker.className = 'ja-me-3d';
      new maplibregl.Marker({ element: marker, anchor: 'center' }).setLngLat([locationPoint.lng, locationPoint.lat]).addTo(map);
    }

    if (reports.length) {
      map.addSource('jalanaman-reports', {
        type: 'geojson',
        data: { type: 'FeatureCollection', features: reports.map(report => ({ type: 'Feature', properties: { category: report.category }, geometry: { type: 'Point', coordinates: [report.lng, report.lat] } })) },
      });
      map.addLayer({
        id: 'jalanaman-report-points', type: 'circle', source: 'jalanaman-reports',
        paint: {
          'circle-radius': 8,
          'circle-color': ['match', ['get', 'category'], 'lighting', '#f59e0b', 'crime', '#ef4444', 'accident', '#f97316', '#94a3b8'],
          'circle-stroke-color': '#ffffff', 'circle-stroke-width': 2,
        },
      });
    }
  });
  setTimeout(() => {
    if (!styleReady) showFallback('__THREE_D_FAILED_TITLE__', '__THREE_D_FAILED_BODY__');
  }, 10000);
}

function baseCenter() {
  if (locationPoint) return [locationPoint.lng, locationPoint.lat];
  if (firstReport) return [firstReport.lng, firstReport.lat];
  return [106.8456, -6.2088];
}

function project(lng, lat, zoomLevel) {
  const sin = Math.sin(lat * Math.PI / 180);
  const scale = tileSize * (2 ** zoomLevel);
  return {
    x: (lng + 180) / 360 * scale,
    y: (0.5 - Math.log((1 + sin) / (1 - sin)) / (4 * Math.PI)) * scale,
  };
}

function unproject(x, y, zoomLevel) {
  const scale = tileSize * (2 ** zoomLevel);
  const lng = x / scale * 360 - 180;
  const n = Math.PI - (2 * Math.PI * y) / scale;
  const lat = (180 / Math.PI) * Math.atan(0.5 * (Math.exp(n) - Math.exp(-n)));
  return [lng, lat];
}

function chooseViewport(width, height) {
  if (route.length <= 1) {
    return { center: baseCenter(), zoom: locationPoint || firstReport ? 15 : 11 };
  }

  const routePoints = route.concat(locationPoint ? [locationPoint] : []);
  const lngs = routePoints.map(p => p.lng);
  const lats = routePoints.map(p => p.lat);
  const minLng = Math.min(...lngs);
  const maxLng = Math.max(...lngs);
  const minLat = Math.min(...lats);
  const maxLat = Math.max(...lats);
  const center = [(minLng + maxLng) / 2, (minLat + maxLat) / 2];
  const padX = 52;
  const padY = 52;

  for (let z = 16; z >= 9; z -= 1) {
    const a = project(minLng, maxLat, z);
    const b = project(maxLng, minLat, z);
    if (Math.abs(b.x - a.x) <= width - padX && Math.abs(b.y - a.y) <= height - padY) {
      return { center, zoom: z };
    }
  }

  return { center, zoom: 9 };
}

function screenPoint(lng, lat, centerPx, width, height, zoomLevel) {
  const point = project(lng, lat, zoomLevel);
  return {
    x: point.x - centerPx.x + width / 2,
    y: point.y - centerPx.y + height / 2,
  };
}

function addPoint(className, lng, lat, bg, text, centerPx, width, height, zoomLevel) {
  const point = screenPoint(lng, lat, centerPx, width, height, zoomLevel);
  const el = document.createElement('div');
  el.className = className;
  if (bg) el.style.background = bg;
  if (text) {
    if (className === 'route-end') {
      const span = document.createElement('span');
      span.textContent = text;
      el.appendChild(span);
    } else {
      el.textContent = text;
    }
  }
  el.style.left = `${point.x}px`;
  el.style.top = `${point.y}px`;
  pointsEl.appendChild(el);
}

function drawRoute(centerPx, width, height, zoomLevel) {
  overlay.setAttribute('viewBox', `0 0 ${width} ${height}`);
  overlay.setAttribute('width', String(width));
  overlay.setAttribute('height', String(height));
  overlay.innerHTML = '';

  if (route.length > 1) {
    const points = route
      .map(p => screenPoint(p.lng, p.lat, centerPx, width, height, zoomLevel))
      .map(p => `${p.x.toFixed(1)},${p.y.toFixed(1)}`)
      .join(' ');
    const polyline = document.createElementNS('http://www.w3.org/2000/svg', 'polyline');
    polyline.setAttribute('points', points);
    polyline.setAttribute('fill', 'none');
    polyline.setAttribute('stroke', levelColors[routeLevel] || '#1d4ed8');
    polyline.setAttribute('stroke-width', '7');
    polyline.setAttribute('stroke-linecap', 'round');
    polyline.setAttribute('stroke-linejoin', 'round');
    polyline.setAttribute('opacity', '0.92');
    overlay.appendChild(polyline);
  }
}

function renderMap(overrideCenter, overrideZoom) {
  const width = mapEl.clientWidth || 360;
  const height = mapEl.clientHeight || 360;
  let center;
  let zoom;
  if (overrideCenter && Number.isFinite(overrideZoom)) {
    center = overrideCenter;
    zoom = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, overrideZoom));
  } else {
    const viewport = chooseViewport(width, height);
    center = viewport.center;
    zoom = viewport.zoom;
  }
  activeCenter = center;
  activeZoom = zoom;
  viewportEl.style.transform = 'translate(0px, 0px)';
  const worldTiles = 2 ** zoom;
  const centerPx = project(center[0], center[1], zoom);
  const startX = Math.floor((centerPx.x - width / 2) / tileSize);
  const endX = Math.floor((centerPx.x + width / 2) / tileSize);
  const startY = Math.floor((centerPx.y - height / 2) / tileSize);
  const endY = Math.floor((centerPx.y + height / 2) / tileSize);
  let loaded = 0;
  let failed = 0;
  let total = 0;

  tilesEl.innerHTML = '';
  pointsEl.innerHTML = '';
  drawRoute(centerPx, width, height, zoom);

  for (let x = startX; x <= endX; x++) {
    for (let y = startY; y <= endY; y++) {
      if (y < 0 || y >= worldTiles) continue;
      total += 1;
      const wrappedX = ((x % worldTiles) + worldTiles) % worldTiles;
      const img = document.createElement('img');
      img.alt = '';
      img.decoding = 'async';
      img.referrerPolicy = 'no-referrer';
      img.src = `https://tile.openstreetmap.org/${zoom}/${wrappedX}/${y}.png`;
      img.style.left = `${Math.round(x * tileSize - centerPx.x + width / 2)}px`;
      img.style.top = `${Math.round(y * tileSize - centerPx.y + height / 2)}px`;
      img.onload = () => {
        loaded += 1;
        fallback.style.display = 'none';
      };
      img.onerror = () => {
        failed += 1;
        if (failed >= total && loaded === 0) {
          showFallback('__MAP_FAILED_TITLE__', '__MAP_FAILED_BODY__');
        }
      };
      tilesEl.appendChild(img);
    }
  }

  if (locationPoint) {
    addPoint('me', locationPoint.lng, locationPoint.lat, null, '', centerPx, width, height, zoom);
  }
  if (route.length > 1) {
    const destination = route[route.length - 1];
    addPoint('route-end', destination.lng, destination.lat, null, 'T', centerPx, width, height, zoom);
  }
  reports.forEach(report => {
    addPoint('pin', report.lng, report.lat, colors[report.category] || colors.other, '!', centerPx, width, height, zoom);
  });

  setTimeout(() => {
    if (loaded === 0) showFallback('__MAP_FAILED_TITLE__', '__MAP_FAILED_BODY__');
  }, 6000);
}

function pointFromEvent(evt) {
  if (evt.changedTouches && evt.changedTouches.length) {
    return { x: evt.changedTouches[0].clientX, y: evt.changedTouches[0].clientY };
  }
  if (evt.touches && evt.touches.length) {
    return { x: evt.touches[0].clientX, y: evt.touches[0].clientY };
  }
  return { x: evt.clientX, y: evt.clientY };
}

let dragging = false;
let dragStart = null;
let dragCenterPx = null;
let dragMoved = false;

function onDragStart(evt) {
  if (evt.touches && evt.touches.length > 1) return;
  dragging = true;
  dragMoved = false;
  dragStart = pointFromEvent(evt);
  dragCenterPx = project(activeCenter[0], activeCenter[1], activeZoom);
  mapEl.classList.add('dragging');
}

function onDragMove(evt) {
  if (!dragging) return;
  const p = pointFromEvent(evt);
  const dx = p.x - dragStart.x;
  const dy = p.y - dragStart.y;
  if (Math.abs(dx) > 3 || Math.abs(dy) > 3) dragMoved = true;
  viewportEl.style.transform = `translate(${dx}px, ${dy}px)`;
  if (evt.cancelable) evt.preventDefault();
}

function onDragEnd(evt) {
  if (!dragging) return;
  dragging = false;
  mapEl.classList.remove('dragging');
  if (!dragMoved) {
    viewportEl.style.transform = 'translate(0px, 0px)';
    return;
  }
  const p = pointFromEvent(evt);
  const dx = p.x - dragStart.x;
  const dy = p.y - dragStart.y;
  const newCenter = unproject(dragCenterPx.x - dx, dragCenterPx.y - dy, activeZoom);
  renderMap(newCenter, activeZoom);
}

mapEl.addEventListener('mousedown', onDragStart);
window.addEventListener('mousemove', onDragMove);
window.addEventListener('mouseup', onDragEnd);
mapEl.addEventListener('touchstart', onDragStart, { passive: true });
mapEl.addEventListener('touchmove', onDragMove, { passive: false });
mapEl.addEventListener('touchend', onDragEnd);
mapEl.addEventListener('touchcancel', onDragEnd);
mapEl.addEventListener('wheel', (evt) => {
  evt.preventDefault();
  const delta = evt.deltaY < 0 ? 1 : -1;
  renderMap(activeCenter, activeZoom + delta);
}, { passive: false });
mapEl.addEventListener('dblclick', (evt) => {
  evt.preventDefault();
  renderMap(activeCenter, activeZoom + 1);
});

document.getElementById('zoomIn').addEventListener('click', () => renderMap(activeCenter, activeZoom + 1));
document.getElementById('zoomOut').addEventListener('click', () => renderMap(activeCenter, activeZoom - 1));

try {
  if (threeDimensional) {
    renderThreeDimensionalMap();
  } else {
    renderMap();
  }
} catch (_) {
  showFallback(threeDimensional ? '__THREE_D_FAILED_TITLE__' : '__MAP_FAILED_TITLE__', threeDimensional ? '__THREE_D_FAILED_BODY__' : '__MAP_FAILED_BODY__');
}
</script>
</body>
</html>"#
        .replace("__LOCATION__", &location_json)
        .replace("__REPORTS__", &reports_json)
        .replace("__ROUTE__", &route_json)
        .replace("__ROUTE_LEVEL__", &route_level_json)
        .replace("__THREE_DIMENSIONAL__", three_dimensional_json)
        .replace("__LOADING_TITLE__", loading_title)
        .replace("__LOADING_BODY__", loading_body)
        .replace("__MAP_FAILED_TITLE__", map_failed_title)
        .replace("__MAP_FAILED_BODY__", map_failed_body)
        .replace("__THREE_D_FAILED_TITLE__", three_d_failed_title)
        .replace("__THREE_D_FAILED_BODY__", three_d_failed_body)
}

fn local_route_score(waypoints: &[Waypoint], reports: &[Report]) -> RouteScoreResponse {
    let mut score = 0.0;
    let mut report_count = 0;

    for report in reports {
        if distance_to_route_m(report.lat, report.lng, waypoints) <= 50.0 {
            report_count += 1;
            score += match report.category.as_str() {
                "crime" => 3.0,
                "accident" => 2.0,
                "lighting" => 1.0,
                _ => 1.0,
            };
        }
    }

    let level = if score < 5.0 {
        "Aman"
    } else if score < 15.0 {
        "Waspada"
    } else {
        "Hindari"
    };

    RouteScoreResponse {
        score,
        level: level.to_string(),
        report_count,
        cache_hit: false,
    }
}

fn distance_to_route_m(lat: f64, lng: f64, waypoints: &[Waypoint]) -> f64 {
    match waypoints {
        [] => f64::INFINITY,
        [only] => haversine_m(lat, lng, only.lat, only.lng),
        points => points
            .windows(2)
            .map(|segment| distance_to_route_segment_m(lat, lng, &segment[0], &segment[1]))
            .fold(f64::INFINITY, f64::min),
    }
}

fn distance_to_route_segment_m(lat: f64, lng: f64, a: &Waypoint, b: &Waypoint) -> f64 {
    let origin_lat = ((lat + a.lat + b.lat) / 3.0).to_radians();
    let meters_per_deg_lat = 111_320.0;
    let meters_per_deg_lng = 111_320.0 * origin_lat.cos().abs().max(0.01);

    let px = (lng - a.lng) * meters_per_deg_lng;
    let py = (lat - a.lat) * meters_per_deg_lat;
    let bx = (b.lng - a.lng) * meters_per_deg_lng;
    let by = (b.lat - a.lat) * meters_per_deg_lat;
    let len_sq = bx * bx + by * by;

    if len_sq <= f64::EPSILON {
        return (px * px + py * py).sqrt();
    }

    let t = ((px * bx + py * by) / len_sq).clamp(0.0, 1.0);
    let dx = px - t * bx;
    let dy = py - t * by;

    (dx * dx + dy * dy).sqrt()
}

fn level_bg(level: &str) -> &'static str {
    match level {
        "Aman" => "rgba(37,99,235,0.24)",
        "Waspada" => "rgba(146,64,14,0.28)",
        _ => "rgba(127,29,29,0.30)",
    }
}

fn level_color(level: &str) -> &'static str {
    match level {
        "Aman" => "#bfdbfe",
        "Waspada" => "#fde68a",
        _ => "#fecdd3",
    }
}

fn distance_label(meters: f64) -> String {
    if meters < 1000.0 {
        format!("{:.0} m", meters)
    } else {
        format!("{:.1} km", meters / 1000.0)
    }
}

fn duration_label(seconds: f64, language: Language) -> String {
    let minutes = (seconds / 60.0).round().max(1.0);
    if minutes < 60.0 {
        if language.is_indonesian() {
            format!("{minutes:.0} mnt")
        } else {
            format!("{minutes:.0} min")
        }
    } else {
        if language.is_indonesian() {
            format!("{:.1} jam", minutes / 60.0)
        } else {
            format!("{:.1} h", minutes / 60.0)
        }
    }
}

fn localized_level(level: &str, language: Language) -> &'static str {
    match language {
        Language::Indonesian => match level {
            "Aman" => "Aman",
            "Waspada" => "Waspada",
            _ => "Hindari",
        },
        Language::English => match level {
            "Aman" => "Safe",
            "Waspada" => "Caution",
            _ => "Avoid",
        },
    }
}

fn route_overlay_title(score: &RouteScoreResponse, language: Language) -> String {
    if language.is_indonesian() {
        format!(
            "{} dengan {} {}",
            localized_level(&score.level, language),
            score.report_count,
            language.text(CopyKey::RouteReportsSuffix)
        )
    } else {
        format!(
            "{} with {} {}",
            localized_level(&score.level, language),
            score.report_count,
            language.text(CopyKey::RouteReportsSuffix)
        )
    }
}

fn haversine_m(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let radius = 6_371_000.0_f64;
    let d_lat = (lat2 - lat1).to_radians();
    let d_lng = (lng2 - lng1).to_radians();
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let a = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lng / 2.0).sin().powi(2);
    radius * 2.0 * a.sqrt().atan2((1.0 - a).sqrt())
}

fn parse_manual_location(lat_text: &str, lng_text: &str) -> Result<GeoPoint, String> {
    let lat = parse_coordinate(lat_text).ok_or_else(|| "Latitude belum valid.".to_string())?;
    let lng = parse_coordinate(lng_text).ok_or_else(|| "Longitude belum valid.".to_string())?;

    if !(-90.0..=90.0).contains(&lat) {
        return Err("Latitude harus antara -90 sampai 90.".to_string());
    }

    if !(-180.0..=180.0).contains(&lng) {
        return Err("Longitude harus antara -180 sampai 180.".to_string());
    }

    Ok(GeoPoint { lat, lng })
}

fn parse_coordinate(value: &str) -> Option<f64> {
    value
        .trim()
        .replace(',', ".")
        .parse::<f64>()
        .ok()
        .filter(|coord| coord.is_finite())
}

fn limit_text(value: String, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}
