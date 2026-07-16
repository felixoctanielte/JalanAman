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
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum NavIconKind {
    Map,
    Route,
    Contacts,
    Profile,
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

    const fn label(self) -> &'static str {
        match self {
            Self::Lighting => "Pencahayaan buruk",
            Self::Crime => "Rawan kriminal",
            Self::Accident => "Rawan kecelakaan",
            Self::Other => "Lainnya",
        }
    }

    const fn short_label(self) -> &'static str {
        match self {
            Self::Lighting => "Gelap",
            Self::Crime => "Kriminal",
            Self::Accident => "Kecelakaan",
            Self::Other => "Lainnya",
        }
    }

    const fn color(self) -> &'static str {
        match self {
            Self::Lighting => "#f59e0b",
            Self::Crime => "#ef4444",
            Self::Accident => "#f97316",
            Self::Other => "#64748b",
        }
    }

    const fn soft_color(self) -> &'static str {
        match self {
            Self::Lighting => "#fffbeb",
            Self::Crime => "#fff1f2",
            Self::Accident => "#fff7ed",
            Self::Other => "#f8fafc",
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

const APP: &str = "min-height:100vh;background:radial-gradient(circle at 50% -12%,#5eead4 0,#0f766e 38%,#064e3b 100%);color:#111827;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;";
const SCREEN: &str = "min-height:100vh;max-width:430px;margin:0 auto;background:linear-gradient(180deg,#ecfdf5 0%,#f8fafc 42%,#ecfdf5 100%);position:relative;overflow:hidden;box-shadow:0 30px 80px rgba(6,78,59,0.35),0 0 0 1px rgba(16,185,129,0.14);";
const HEADER: &str = "height:76px;padding:16px 18px 12px;display:flex;align-items:center;justify-content:space-between;background:linear-gradient(135deg,#ffffff 0%,#f0fdfa 62%,#dcfce7 100%);border-bottom:1px solid rgba(20,184,166,0.18);box-sizing:border-box;box-shadow:0 16px 34px rgba(6,78,59,0.08);";
const BRAND_WRAP: &str = "display:flex;flex-direction:column;gap:1px;";
const BRAND: &str = "font-size:21px;font-weight:900;color:#0f172a;letter-spacing:0;";
const SUBTITLE: &str = "font-size:11px;font-weight:750;color:#0f766e;";
const ICON_BUTTON: &str = "width:38px;height:38px;border-radius:8px;border:1px solid rgba(20,184,166,0.24);background:rgba(255,255,255,0.78);color:#0f766e;font-size:16px;font-weight:900;display:flex;align-items:center;justify-content:center;box-shadow:0 10px 24px rgba(6,78,59,0.10);";
const CONTENT: &str = "position:absolute;top:76px;left:0;right:0;bottom:88px;padding:14px;overflow-y:auto;box-sizing:border-box;";
const MAP_CARD: &str = "height:378px;position:relative;overflow:hidden;border-radius:8px;background:#d1fae5;border:1px solid rgba(20,184,166,0.22);box-shadow:0 22px 42px rgba(6,78,59,0.18);";
const ROUTE_MAP_CARD: &str = "height:286px;margin-top:12px;position:relative;overflow:hidden;border-radius:8px;background:#d1fae5;border:1px solid rgba(20,184,166,0.22);box-shadow:0 18px 36px rgba(6,78,59,0.16);";
const MAP_IFRAME: &str =
    "position:absolute;inset:0;width:100%;height:100%;border:0;background:#e0f2fe;";
const MAP_LABEL: &str = "position:absolute;left:12px;top:12px;max-width:250px;padding:9px 11px;border-radius:8px;background:rgba(255,255,255,0.96);border:1px solid rgba(20,184,166,0.16);color:#0f172a;font-size:12px;font-weight:850;box-shadow:0 12px 26px rgba(6,78,59,0.14);z-index:2;";
const REPORT_FAB: &str = "position:absolute;right:16px;bottom:16px;width:56px;height:56px;border-radius:12px;border:0;background:linear-gradient(135deg,#10b981,#0f766e);color:#ffffff;font-size:30px;line-height:1;font-weight:900;box-shadow:0 18px 32px rgba(15,118,110,0.36);display:flex;align-items:center;justify-content:center;z-index:2;";
const MAP_PROVIDER: &str = "position:absolute;left:12px;bottom:14px;padding:7px 9px;border-radius:8px;background:rgba(6,78,59,0.78);color:#ffffff;font-size:10px;font-weight:800;z-index:2;";
const CARD: &str = "margin-top:12px;background:rgba(255,255,255,0.94);border:1px solid rgba(20,184,166,0.14);border-radius:8px;padding:14px;box-shadow:0 14px 30px rgba(6,78,59,0.10);box-sizing:border-box;backdrop-filter:blur(8px);";
const CARD_TIGHT: &str = "background:rgba(255,255,255,0.94);border:1px solid rgba(20,184,166,0.14);border-radius:8px;padding:13px;box-shadow:0 14px 30px rgba(6,78,59,0.10);box-sizing:border-box;backdrop-filter:blur(8px);";
const ROW: &str = "display:flex;align-items:center;justify-content:space-between;gap:12px;";
const EYEBROW: &str = "font-size:11px;color:#64748b;font-weight:750;margin-bottom:3px;";
const TITLE: &str = "font-size:14px;color:#0f172a;font-weight:850;";
const BODY: &str = "font-size:12px;color:#64748b;font-weight:650;line-height:1.45;";
const META_GRID: &str =
    "margin-top:12px;display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:8px;";
const META_CELL: &str = "border-radius:8px;background:#f0fdfa;border:1px solid rgba(20,184,166,0.16);padding:10px 8px;min-height:58px;box-sizing:border-box;";
const META_VALUE: &str = "font-size:14px;font-weight:900;color:#0f766e;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;";
const META_LABEL: &str = "margin-top:2px;font-size:10px;font-weight:750;color:#64748b;";
const FIELD_GRID: &str = "display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-top:10px;";
const INPUT: &str = "width:100%;box-sizing:border-box;border:1px solid rgba(15,118,110,0.22);background:#ffffff;border-radius:8px;padding:12px 14px;color:#0f172a;font-size:14px;font-weight:650;outline:none;box-shadow:inset 0 1px 0 rgba(255,255,255,0.80);";
const TEXTAREA: &str = "width:100%;min-height:88px;box-sizing:border-box;border:1px solid rgba(15,118,110,0.22);background:#ffffff;border-radius:8px;padding:12px 14px;color:#0f172a;font-size:14px;font-weight:650;outline:none;resize:none;font-family:inherit;";
const PRIMARY_BUTTON: &str = "width:100%;margin-top:10px;border:0;border-radius:8px;background:linear-gradient(135deg,#10b981,#0f766e);color:#ffffff;padding:12px 14px;font-size:14px;font-weight:900;box-shadow:0 14px 28px rgba(15,118,110,0.28);";
const SECONDARY_BUTTON: &str = "width:100%;margin-top:10px;border:1px solid rgba(15,118,110,0.24);border-radius:8px;background:#f0fdfa;color:#0f766e;padding:12px 14px;font-size:14px;font-weight:850;";
const CATEGORY_GRID: &str = "display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-top:10px;";
const CATEGORY_BUTTON: &str = "height:54px;border:1px solid #e5e7eb;border-radius:8px;background:#ffffff;color:#334155;display:flex;align-items:center;gap:8px;padding:0 11px;font-size:12px;font-weight:850;text-align:left;";
const CATEGORY_BUTTON_ACTIVE: &str = "height:54px;border:1px solid #14b8a6;border-radius:8px;background:#f0fdfa;color:#0f766e;display:flex;align-items:center;gap:8px;padding:0 11px;font-size:12px;font-weight:900;text-align:left;box-shadow:0 8px 18px rgba(15,118,110,0.12);";
const BOTTOM_BAR: &str = "position:absolute;left:0;right:0;bottom:0;height:88px;background:linear-gradient(180deg,#0f766e 0%,#065f46 100%);border-top:1px solid rgba(209,250,229,0.22);display:grid;grid-template-columns:1fr 1fr 78px 1fr 1fr;align-items:center;padding:8px 10px 12px;box-shadow:0 -18px 38px rgba(6,78,59,0.30);box-sizing:border-box;";
const NAV_BUTTON: &str = "height:60px;border:0;background:transparent;color:#d1fae5;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:4px;font-size:10px;font-weight:850;";
const NAV_BUTTON_ACTIVE: &str = "height:60px;border:0;background:rgba(255,255,255,0.96);color:#0f766e;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:4px;font-size:10px;font-weight:950;border-radius:8px;box-shadow:0 12px 26px rgba(6,78,59,0.24);";
const NAV_ICON: &str = "width:21px;height:21px;display:block;stroke:currentColor;";
const SOS_BUTTON: &str = "position:absolute;left:50%;bottom:45px;transform:translateX(-50%);width:66px;height:66px;border-radius:14px;border:5px solid #ecfdf5;background:linear-gradient(135deg,#fb7185,#dc2626);color:#ffffff;font-size:14px;font-weight:950;letter-spacing:0;box-shadow:0 18px 36px rgba(220,38,38,0.38),0 0 0 9px rgba(254,226,226,0.18);display:flex;align-items:center;justify-content:center;";
const SOS_BUTTON_ACTIVE: &str = "position:absolute;left:50%;bottom:45px;transform:translateX(-50%);width:66px;height:66px;border-radius:14px;border:5px solid #ecfdf5;background:#991b1b;color:#ffffff;font-size:12px;font-weight:950;letter-spacing:0;box-shadow:0 0 0 10px rgba(239,68,68,0.18),0 18px 36px rgba(127,29,29,0.42);display:flex;align-items:center;justify-content:center;";
const ALERT: &str = "position:absolute;left:18px;right:18px;bottom:100px;border-radius:8px;border:1px solid #fecaca;background:#fff1f2;color:#991b1b;padding:11px 13px;font-size:12px;font-weight:850;box-shadow:0 14px 28px rgba(127,29,29,0.14);z-index:4;";
const DASHBOARD_WRAP: &str = "min-height:100vh;max-width:430px;margin:0 auto;background:#f8fbff;padding:18px;box-sizing:border-box;color:#0f172a;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;";
const BACK_LINK: &str = "display:inline-flex;align-items:center;gap:6px;color:#0f766e;text-decoration:none;font-size:13px;font-weight:850;margin-bottom:18px;";
const DASH_TITLE: &str =
    "font-size:24px;line-height:1.1;font-weight:900;color:#0f172a;margin:0 0 14px;";

#[component]
fn Home() -> Element {
    let mut active_tab = use_signal(|| MobileTab::Map);
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
    let mut manual_lat = use_signal(String::new);
    let mut manual_lng = use_signal(String::new);
    let mut manual_location_error = use_signal(|| Option::<String>::None);

    use_effect(move || {
        spawn(async move {
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
    let location_value = *location.read();
    let is_sos_active = *sos_active.read();
    let report_category_value = *report_category.read();
    let report_note_value = report_note.read().clone();
    let destination_value = destination.read().clone();
    let reports_value = reports.read().clone();
    let contacts_value = contacts.read().clone();
    let device_hash_value = device_hash.read().clone();
    let directions_value = directions.read().clone();
    let route_score_value = route_score.read().clone();
    let map_html = map_srcdoc(
        location_value,
        &reports_value,
        directions_value.as_ref().map(|d| d.polyline.as_slice()),
        route_score_value.as_ref().map(|s| s.level.as_str()),
    );

    rsx! {
        main { style: APP,
            div { style: SCREEN,
                header { style: HEADER,
                    div { style: BRAND_WRAP,
                        span { style: BRAND, "JalanAman" }
                        span { style: SUBTITLE, "Rute aman di sekitar kamu" }
                    }
                    button {
                        style: ICON_BUTTON,
                        onclick: move |_| active_tab.set(MobileTab::Profile),
                        "i"
                    }
                }

                section { style: CONTENT,
                    if active_tab_value == MobileTab::Map {
                        MapView {
                            map_html,
                            reports: reports_value,
                            location: location_value,
                            loading: *reports_loading.read() || *location_loading.read(),
                            error: reports_error.read().clone().or(location_error.read().clone()),
                            route_score: route_score_value,
                            manual_lat: manual_lat.read().clone(),
                            manual_lng: manual_lng.read().clone(),
                            manual_error: manual_location_error.read().clone(),
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
                                    route_error.set(Some("Tujuan minimal 3 karakter.".to_string()));
                                    return;
                                }
                                let Some(origin) = point else {
                                    route_error.set(Some("Lokasi belum tersedia. Isi koordinat manual di tab Peta dulu.".to_string()));
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
                                                Err(err) => {
                                                    let fallback_score = local_route_score(&dirs.polyline, &fallback_reports);
                                                    directions.set(Some(dirs));
                                                    route_score.set(Some(fallback_score));
                                                    route_error.set(Some(format!("Skor backend belum bisa diambil ({err}). Rute tetap tampil dengan skor sementara dari laporan yang sudah dimuat.")));
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
                            note: report_note_value,
                            location: location_value,
                            loading: *report_loading.read(),
                            error: report_error.read().clone(),
                            on_category: move |category| report_category.set(category),
                            on_note: move |value| report_note.set(limit_text(value, 100)),
                            on_submit: move |_| {
                                let Some(point) = *location.read() else {
                                    report_error.set(Some("Lokasi belum tersedia. Isi koordinat manual di tab Peta dulu.".to_string()));
                                    return;
                                };
                                let hash = device_hash.read().clone();
                                if hash.is_empty() {
                                    report_error.set(Some("Device ID belum siap. Coba lagi sebentar.".to_string()));
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
                                    contacts_error.set(Some("Device ID belum siap. Coba lagi sebentar.".to_string()));
                                    return;
                                }
                                if name.len() < 2 {
                                    contacts_error.set(Some("Nama kontak minimal 2 karakter.".to_string()));
                                    return;
                                }
                                if email_text.is_empty() && phone_text.is_empty() {
                                    contacts_error.set(Some("Isi email atau nomor WhatsApp kontak.".to_string()));
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
                                    contacts_error.set(Some("Device ID belum siap. Coba lagi sebentar.".to_string()));
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
                    } else {
                        ProfileView {
                            api_base: api_base().to_string(),
                            device_hash: device_hash_value,
                            location: location_value,
                            report_count: reports.read().len(),
                            contact_count: contacts.read().len(),
                            location_error: location_error.read().clone(),
                        }
                    }
                }

                if let Some(message) = sos_msg.read().clone() {
                    div { style: ALERT,
                        div { style: "font-size:13px;font-weight:950;margin-bottom:2px;", if is_sos_active { "SOS aktif" } else { "SOS" } }
                        div { "{message}" }
                    }
                }

                nav { style: BOTTOM_BAR,
                    NavButton {
                        active: active_tab_value == MobileTab::Map,
                        icon: NavIconKind::Map,
                        label: "Peta",
                        onclick: move |_| active_tab.set(MobileTab::Map),
                    }
                    NavButton {
                        active: active_tab_value == MobileTab::Route,
                        icon: NavIconKind::Route,
                        label: "Rute",
                        onclick: move |_| active_tab.set(MobileTab::Route),
                    }
                    div {}
                    NavButton {
                        active: active_tab_value == MobileTab::Contacts,
                        icon: NavIconKind::Contacts,
                        label: "Kontak",
                        onclick: move |_| active_tab.set(MobileTab::Contacts),
                    }
                    NavButton {
                        active: active_tab_value == MobileTab::Profile,
                        icon: NavIconKind::Profile,
                        label: "Akun",
                        onclick: move |_| active_tab.set(MobileTab::Profile),
                    }
                }

                button {
                    style: if is_sos_active { SOS_BUTTON_ACTIVE } else { SOS_BUTTON },
                    onclick: move |_| {
                        if *sos_active.read() {
                            stop_sos_alarm();
                            sos_active.set(false);
                            sos_msg.set(Some("Alarm lokal dihentikan.".to_string()));
                            return;
                        }

                        let point = *location.read();
                        let hash = device_hash.read().clone();
                        if hash.is_empty() {
                            sos_msg.set(Some("Device ID belum siap. Coba lagi sebentar.".to_string()));
                            return;
                        }
                        let whatsapp_contacts = contacts.read().clone();

                        sos_msg.set(Some("Mengambil GPS terbaru untuk SOS...".to_string()));
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
                                        sos_msg.set(Some(format!("GPS belum bisa dibaca: {err}. Buka tab Peta lalu tekan Refresh GPS atau isi koordinat manual.")));
                                        return;
                                    }
                                },
                            };

                            play_sos_alarm();
                            sos_active.set(true);
                            sos_msg.set(Some("Alarm berbunyi. Mengirim alert dan membuka WhatsApp...".to_string()));

                            let whatsapp_status = match open_whatsapp_sos(&whatsapp_contacts, point).await {
                                Ok(true) => "WhatsApp dibuka dengan pesan SOS siap kirim. ",
                                Ok(false) => "Nomor WhatsApp kontak belum ada. ",
                                Err(_) => "WhatsApp belum bisa dibuka otomatis. ",
                            };

                            match trigger_sos(&hash, point).await {
                                Ok(resp) => sos_msg.set(Some(format!(
                                    "{whatsapp_status}Alert backend terkirim ke {}/{} kontak. Suara tetap aktif sampai STOP.",
                                    resp.notified_count, resp.total_contacts
                                ))),
                                Err(err) => sos_msg.set(Some(format!("{whatsapp_status}Alarm aktif, tapi alert backend gagal: {err}"))),
                            }
                        });
                    },
                    if is_sos_active { "STOP" } else { "SOS" }
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
    manual_lat: String,
    manual_lng: String,
    manual_error: Option<String>,
    on_report: EventHandler<MouseEvent>,
    on_manual_lat: EventHandler<String>,
    on_manual_lng: EventHandler<String>,
    on_manual_apply: EventHandler<MouseEvent>,
    on_refresh: EventHandler<MouseEvent>,
) -> Element {
    let title = if loading {
        "Memuat peta dan laporan".to_string()
    } else {
        format!("{} laporan aktif radius 800 m", reports.len())
    };
    let gps_label = location
        .map(|p| format!("{:.5}, {:.5}", p.lat, p.lng))
        .unwrap_or_else(|| "GPS belum tersedia".to_string());

    rsx! {
        div {
            div { style: MAP_CARD,
                iframe {
                    style: MAP_IFRAME,
                    srcdoc: "{map_html}",
                }
                div { style: MAP_LABEL,
                    div { "{title}" }
                    div { style: "margin-top:2px;font-size:10px;color:#64748b;font-weight:750;", "{gps_label}" }
                }
                div { style: MAP_PROVIDER, "OpenStreetMap real map" }
                button {
                    style: REPORT_FAB,
                    title: "Lapor cepat",
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
                            div { style: EYEBROW, "Lokasi manual" }
                            div { style: TITLE, "Pakai koordinat HP" }
                        }
                        Badge { label: "Fallback".to_string(), bg: "#fef3c7", color: "#92400e" }
                    }
                    div { style: "margin-top:8px;",
                        div { style: BODY, "GPS WebView Android sedang diblokir di build ini. Tempel lat/lng asli dari Google Maps agar peta laporan, rute, dan SOS tetap memakai lokasi nyata." }
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
                        "Pakai lokasi ini"
                    }
                }
            }

            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "Laporan terdekat" }
                        div { style: TITLE, "{title}" }
                    }
                    Badge { label: if loading { "Loading".to_string() } else { "Live".to_string() }, bg: "#dcfce7", color: "#166534" }
                }
                div { style: "display:flex;flex-direction:column;gap:8px;margin-top:12px;",
                    if reports.is_empty() {
                        div { style: BODY, "Belum ada laporan aktif dari user lain di radius ini." }
                    } else {
                        for report in reports.iter().take(4) {
                            ReportRow { report: report.clone(), location }
                        }
                    }
                }
                button {
                    style: SECONDARY_BUTTON,
                    onclick: move |event| on_refresh.call(event),
                    "Refresh GPS & laporan"
                }
            }

            if let Some(score) = route_score {
                div { style: CARD,
                    div { style: ROW,
                        div {
                            div { style: EYEBROW, "Overlay rute terakhir" }
                            div { style: TITLE, "{score.level} dengan {score.report_count} laporan di rute" }
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

    rsx! {
        div {
            div { style: CARD_TIGHT,
                div { style: EYEBROW, "Cari tujuan" }
                input {
                    style: INPUT,
                    value: "{destination}",
                    placeholder: "Cari tempat atau alamat",
                    oninput: move |event| on_destination.call(event.value()),
                }
                if let Some(place) = selected_place {
                    div { style: "margin-top:9px;display:flex;align-items:center;gap:10px;padding:10px 11px;border:1px solid #99f6e4;border-radius:8px;background:#f0fdfa;",
                        span { style: "width:24px;height:24px;display:flex;align-items:center;justify-content:center;flex-shrink:0;border-radius:50%;background:#0f766e;color:#ffffff;font-size:13px;font-weight:900;", "✓" }
                        div { style: "min-width:0;",
                            div { style: "font-size:12px;color:#0f172a;font-weight:900;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{place.name}" }
                            div { style: "margin-top:2px;font-size:10px;color:#64748b;font-weight:750;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{place.subtitle}" }
                        }
                    }
                } else if suggestions_loading {
                    div { style: "margin-top:10px;padding:10px 2px;color:#0f766e;font-size:12px;font-weight:800;", "Mencari tempat..." }
                } else if !suggestions.is_empty() {
                    div { style: "margin-top:9px;overflow:hidden;border:1px solid rgba(15,118,110,0.20);border-radius:8px;background:#ffffff;",
                        for place in suggestions {
                            {
                                let place_for_click = place.clone();
                                rsx! {
                                    button {
                                        style: "width:100%;min-height:58px;padding:10px 11px;border:0;border-bottom:1px solid #e2e8f0;background:#ffffff;color:#0f172a;display:flex;align-items:center;gap:10px;text-align:left;",
                                        onclick: move |_| on_select_place.call(place_for_click.clone()),
                                        span { style: "width:26px;height:26px;border-radius:50%;display:flex;align-items:center;justify-content:center;flex-shrink:0;background:#ecfdf5;color:#0f766e;font-size:14px;font-weight:900;", "●" }
                                        div { style: "min-width:0;flex:1;",
                                            div { style: "font-size:12px;font-weight:900;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{place.name}" }
                                            div { style: "margin-top:2px;color:#64748b;font-size:10px;font-weight:750;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{place.subtitle}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Some(search_error) = suggestions_error {
                    div { style: "margin-top:10px;color:#b45309;font-size:11px;font-weight:800;line-height:1.4;", "{search_error}" }
                }
                button {
                    style: PRIMARY_BUTTON,
                    disabled: loading,
                    onclick: move |event| on_search.call(event),
                    if loading { "Mengecek rute..." } else { "Cek rute aman" }
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
                        title: "Peta rute aman",
                    }
                    div { style: MAP_PROVIDER, "OpenStreetMap + OSRM" }
                }
            }

            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "Skor rute" }
                        div { style: TITLE, "Status keamanan rute" }
                    }
                    if let Some(score) = score.clone() {
                        Badge { label: score.level.clone(), bg: level_bg(&score.level), color: level_color(&score.level) }
                    } else {
                        Badge { label: "Belum dicek".to_string(), bg: "#f0fdfa", color: "#0f766e" }
                    }
                }

                if let Some(score) = score {
                    div { style: META_GRID,
                        Metric { value: format!("{:.1}", score.score), label: "Bobot" }
                        Metric { value: score.level.clone(), label: "Status" }
                        Metric { value: score.report_count.to_string(), label: "Laporan" }
                    }
                } else {
                    div { style: "margin-top:12px;", div { style: BODY, "Masukkan tujuan untuk melihat skor keamanan rute." } }
                }
            }

            if let Some(dirs) = directions {
                div { style: CARD,
                    div { style: EYEBROW, "Detail rute" }
                    div { style: META_GRID,
                        Metric { value: distance_label(dirs.distance_m), label: "Jarak" }
                        Metric { value: duration_label(dirs.duration_s), label: "Estimasi" }
                        Metric { value: dirs.provider, label: "Provider" }
                    }
                }
            }
        }
    }
}

#[component]
fn ReportView(
    category: ReportCategory,
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
        "GPS siap"
    } else {
        "GPS belum siap"
    };
    let category_color = category.color();
    let category_soft_color = category.soft_color();
    let category_label = category.label();
    let gps_badge_bg = if location.is_some() {
        "#dcfce7"
    } else {
        "#fee2e2"
    };
    let gps_badge_color = if location.is_some() {
        "#166534"
    } else {
        "#991b1b"
    };

    rsx! {
        div {
            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "Lapor cepat" }
                        div { style: TITLE, "Kategori laporan" }
                    }
                    Badge { label: gps_status.to_string(), bg: gps_badge_bg, color: gps_badge_color }
                }

                div { style: CATEGORY_GRID,
                    for item in CATEGORIES {
                        CategoryButton {
                            category: item,
                            selected: category == item,
                            onclick: move |_| on_category.call(item),
                        }
                    }
                }

                div { style: "margin-top:12px;",
                    div { style: EYEBROW, "Catatan opsional" }
                    textarea {
                        style: TEXTAREA,
                        value: "{note}",
                        maxlength: "100",
                        placeholder: "Maks 100 karakter",
                        oninput: move |event| on_note.call(event.value()),
                    }
                    div { style: "margin-top:6px;text-align:right;font-size:10px;font-weight:750;color:#64748b;",
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
                    if loading { "Mengirim..." } else { "Kirim laporan" }
                }
            }

            div { style: CARD,
                div { style: EYEBROW, "Preview laporan" }
                div { style: "display:flex;align-items:center;gap:10px;border:1px solid #e2e8f0;background:{category_soft_color};border-radius:8px;padding:10px 11px;",
                    span { style: "width:11px;height:11px;border-radius:50%;background:{category_color};box-shadow:0 0 0 4px rgba(255,255,255,0.78);flex-shrink:0;" }
                    div { style: "min-width:0;flex:1;",
                        div { style: "font-size:12px;font-weight:900;color:#0f172a;", "{category_label}" }
                        div { style: "margin-top:2px;font-size:10px;font-weight:750;color:#64748b;", "{gps_status}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ContactsView(
    contacts: Vec<EmergencyContact>,
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

    rsx! {
        div {
            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "Kontak darurat" }
                        div { style: TITLE, "{count} kontak tersimpan" }
                    }
                    Badge { label: "Email/WA/SOS".to_string(), bg: "#fee2e2", color: "#991b1b" }
                }

                div { style: "display:flex;flex-direction:column;gap:9px;margin-top:12px;",
                    if contacts.is_empty() {
                        div { style: BODY, "Belum ada kontak. Tambahkan email atau nomor WhatsApp agar SOS bisa mengirim alert." }
                    } else {
                        for contact in contacts {
                            ContactRow { contact }
                        }
                    }
                }

                if let Some(err) = error {
                    Notice { message: err, danger: true }
                }
                button {
                    style: SECONDARY_BUTTON,
                    onclick: move |event| on_refresh.call(event),
                    if loading { "Memuat..." } else { "Refresh kontak" }
                }
            }

            div { style: CARD,
                div { style: EYEBROW, "Tambah kontak SOS" }
                input {
                    style: INPUT,
                    value: "{name}",
                    placeholder: "Nama kontak",
                    oninput: move |event| on_name.call(event.value()),
                }
                div { style: "height:8px;" }
                input {
                    style: INPUT,
                    value: "{email}",
                    placeholder: "Email kontak",
                    oninput: move |event| on_email.call(event.value()),
                }
                div { style: "height:8px;" }
                input {
                    style: INPUT,
                    value: "{phone}",
                    placeholder: "Nomor WhatsApp, contoh 08123456789",
                    inputmode: "tel",
                    oninput: move |event| on_phone.call(event.value()),
                }
                button {
                    style: PRIMARY_BUTTON,
                    disabled: loading,
                    onclick: move |event| on_add.call(event),
                    if loading { "Menyimpan..." } else { "Tambah kontak" }
                }
            }
        }
    }
}

#[component]
fn ProfileView(
    api_base: String,
    device_hash: String,
    location: Option<GeoPoint>,
    report_count: usize,
    contact_count: usize,
    location_error: Option<String>,
) -> Element {
    let gps = location
        .map(|point| format!("{:.5}, {:.5}", point.lat, point.lng))
        .unwrap_or_else(|| "Belum tersedia".to_string());
    let device = if device_hash.is_empty() {
        "Memuat device ID".to_string()
    } else {
        short_hash(&device_hash)
    };

    rsx! {
        div {
            div { style: CARD,
                div { style: ROW,
                    div {
                        div { style: EYEBROW, "Device" }
                        div { style: TITLE, "Anonim via device ID" }
                    }
                    Badge { label: "Aktif".to_string(), bg: "#dcfce7", color: "#166534" }
                }
                div { style: META_GRID,
                    Metric { value: device, label: "Device" }
                    Metric { value: report_count.to_string(), label: "Laporan" }
                    Metric { value: contact_count.to_string(), label: "Kontak" }
                }
            }

            div { style: CARD,
                div { style: EYEBROW, "Koneksi" }
                StatusRow { label: "Backend API", value: api_base }
                StatusRow { label: "GPS", value: gps }
                if let Some(err) = location_error {
                    Notice { message: err, danger: true }
                }
            }

            div { style: CARD,
                div { style: EYEBROW, "Status layanan" }
                StatusRow { label: "Peta laporan", value: "Backend + OpenStreetMap".to_string() }
                StatusRow { label: "Lapor cepat", value: "POST /reports".to_string() }
                StatusRow { label: "Skor rute", value: "Directions + route-score".to_string() }
                StatusRow { label: "SOS", value: "Email + WhatsApp + alarm lokal".to_string() }
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
            span { "{category.short_label()}" }
        }
    }
}

#[component]
fn ReportRow(report: Report, location: Option<GeoPoint>) -> Element {
    let category = ReportCategory::from_api(&report.category);
    let category_color = category.color();
    let category_soft_color = category.soft_color();
    let category_label = category.short_label();
    let distance = location
        .map(|point| distance_label(haversine_m(point.lat, point.lng, report.lat, report.lng)))
        .unwrap_or_else(|| "jarak belum ada".to_string());
    let note = report
        .note
        .clone()
        .unwrap_or_else(|| category.label().to_string());

    rsx! {
        div {
            style: "display:flex;align-items:center;gap:10px;border:1px solid #e2e8f0;background:{category_soft_color};border-radius:8px;padding:10px 11px;",
            span {
                style: "width:11px;height:11px;border-radius:50%;background:{category_color};box-shadow:0 0 0 4px rgba(255,255,255,0.78);flex-shrink:0;",
            }
            div { style: "min-width:0;flex:1;",
                div { style: "font-size:12px;font-weight:900;color:#0f172a;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{note}" }
                div { style: "margin-top:2px;font-size:10px;font-weight:750;color:#64748b;", "{category_label} | {distance}" }
            }
        }
    }
}

#[component]
fn ContactRow(contact: EmergencyContact) -> Element {
    let status = if contact.push_endpoint.is_some() {
        "Push siap"
    } else if contact.email.is_some() && contact.phone.is_some() {
        "Email + WA"
    } else if contact.email.is_some() {
        "Email siap"
    } else if contact.phone.is_some() {
        "WA siap"
    } else {
        "Menunggu"
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
        "Belum tersambung".to_string()
    } else {
        details.join(" | ")
    };
    let status_bg = if status == "Menunggu" {
        "#fef3c7"
    } else {
        "#dcfce7"
    };
    let status_color = if status == "Menunggu" {
        "#92400e"
    } else {
        "#166534"
    };

    rsx! {
        div { style: "display:flex;align-items:center;justify-content:space-between;gap:12px;padding:11px;border:1px solid #e2e8f0;border-radius:8px;background:#f8fafc;",
            div { style: "min-width:0;",
                div { style: TITLE, "{contact.name}" }
                div { style: "font-size:11px;color:#64748b;font-weight:700;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", "{detail}" }
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
            style: "flex-shrink:0;border-radius:999px;background:{bg};color:{color};padding:7px 11px;font-size:11px;font-weight:900;white-space:nowrap;",
            "{label}"
        }
    }
}

#[component]
fn Notice(message: String, danger: bool) -> Element {
    let style = if danger {
        "margin-top:10px;border-radius:8px;border:1px solid #fecaca;background:#fff1f2;color:#991b1b;padding:10px 11px;font-size:12px;font-weight:800;line-height:1.4;"
    } else {
        "margin-top:10px;border-radius:8px;border:1px solid #99f6e4;background:#f0fdfa;color:#0f766e;padding:10px 11px;font-size:12px;font-weight:800;line-height:1.4;"
    };

    rsx! { div { style: "{style}", "{message}" } }
}

#[component]
fn StatusRow(label: &'static str, value: String) -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;justify-content:space-between;gap:12px;padding:10px 0;border-bottom:1px solid #eef2ff;",
            span { style: "font-size:12px;font-weight:800;color:#334155;", "{label}" }
            span { style: "font-size:12px;font-weight:900;color:#0f766e;text-align:right;min-width:0;overflow:hidden;text-overflow:ellipsis;", "{value}" }
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

    if lower.contains("only secure origins") {
        return "GPS Android WebView diblokir di build ini. Isi koordinat manual dulu untuk lanjut memakai laporan, rute, dan SOS.".to_string();
    }

    if lower.contains("permission") || lower.contains("denied") || lower.contains("ditolak") {
        return "Izin lokasi belum aktif. Beri permission lokasi ke aplikasi atau isi koordinat manual di tab Peta.".to_string();
    }

    message
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

fn play_sos_alarm() {
    spawn(async {
        let _ = document::eval(
            r#"
            if (navigator.vibrate) navigator.vibrate([800, 200, 800, 200, 800, 200, 800]);
            try {
                const ctx = new (window.AudioContext || window.webkitAudioContext)();
                const freqs = [880, 440, 880, 440, 880, 440, 880, 440];
                freqs.forEach((freq, i) => {
                    const osc = ctx.createOscillator();
                    const gain = ctx.createGain();
                    osc.type = 'square';
                    osc.frequency.value = freq;
                    gain.gain.value = 0.45;
                    osc.connect(gain);
                    gain.connect(ctx.destination);
                    const t = ctx.currentTime + i * 0.42;
                    osc.start(t);
                    osc.stop(t + 0.34);
                });
            } catch (_) {}
            return true;
            "#,
        )
        .await;
    });
}

fn stop_sos_alarm() {
    spawn(async {
        let _ = document::eval(
            r#"
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
    let response = request.send().await.map_err(|err| err.to_string())?;
    let status = response.status();

    if status.is_success() {
        return response.json::<T>().await.map_err(|err| err.to_string());
    }

    let body = response.text().await.unwrap_or_default();
    let message = serde_json::from_str::<serde_json::Value>(&body)
        .ok()
        .and_then(|value| {
            value
                .get("error")
                .and_then(|error| error.as_str())
                .map(ToString::to_string)
        })
        .filter(|text| !text.is_empty())
        .unwrap_or(body);

    Err(format!("HTTP {status}: {message}"))
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

    r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <style>
    html, body, #map { margin:0; width:100%; height:100%; overflow:hidden; background:#dbeafe; }
    #map { position:relative; font-family:system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif; }
    #tiles, #overlay, #points { position:absolute; inset:0; }
    #tiles img { position:absolute; width:256px; height:256px; image-rendering:auto; }
    #overlay { pointer-events:none; z-index:3; }
    #points { z-index:4; pointer-events:none; }
    #fallback { position:absolute; inset:0; z-index:6; display:flex; flex-direction:column; align-items:center; justify-content:center; gap:6px; padding:24px; box-sizing:border-box; background:#e0f2fe; color:#0f172a; font-family:system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif; text-align:center; }
    #fallback strong { font-size:14px; font-weight:900; }
    #fallback span { max-width:220px; font-size:11px; font-weight:700; line-height:1.4; color:#475569; }
    .pin { position:absolute; width:24px; height:24px; margin:-12px 0 0 -12px; border-radius:50%; border:3px solid #fff; box-shadow:0 10px 18px rgba(15,23,42,.25); display:flex; align-items:center; justify-content:center; color:#fff; font:900 11px system-ui; }
    .me { position:absolute; width:18px; height:18px; margin:-9px 0 0 -9px; border-radius:50%; background:#0f766e; border:3px solid #fff; box-shadow:0 0 0 14px rgba(15,118,110,.17),0 10px 22px rgba(15,23,42,.26); }
    .route-end { position:absolute; width:28px; height:28px; margin:-28px 0 0 -14px; border-radius:9px 9px 9px 2px; transform:rotate(-45deg); background:#0f766e; border:3px solid #fff; box-shadow:0 12px 22px rgba(15,23,42,.25); display:flex; align-items:center; justify-content:center; }
    .route-end span { transform:rotate(45deg); color:#fff; font:900 12px system-ui; }
  </style>
</head>
<body>
<div id="map">
  <div id="tiles"></div>
  <svg id="overlay"></svg>
  <div id="points"></div>
  <div id="fallback"><strong>Memuat peta real</strong><span>OpenStreetMap sedang mengambil tile jalan sekitar kamu.</span></div>
</div>
<script>
const locationPoint = __LOCATION__;
const reports = __REPORTS__;
const route = __ROUTE__;
const routeLevel = __ROUTE_LEVEL__;
const mapEl = document.getElementById('map');
const tilesEl = document.getElementById('tiles');
const pointsEl = document.getElementById('points');
const overlay = document.getElementById('overlay');
const fallback = document.getElementById('fallback');
function showFallback(title, body) {
  fallback.style.display = 'flex';
  fallback.innerHTML = `<strong>${title}</strong><span>${body}</span>`;
}
const colors = { lighting:'#f59e0b', crime:'#ef4444', accident:'#f97316', other:'#64748b' };
const levelColors = { Aman:'#22c55e', Waspada:'#f59e0b', Hindari:'#ef4444' };
const firstReport = reports[0];
const tileSize = 256;

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
    polyline.setAttribute('stroke', levelColors[routeLevel] || '#0f766e');
    polyline.setAttribute('stroke-width', '7');
    polyline.setAttribute('stroke-linecap', 'round');
    polyline.setAttribute('stroke-linejoin', 'round');
    polyline.setAttribute('opacity', '0.92');
    overlay.appendChild(polyline);
  }
}

function renderMap() {
  const width = mapEl.clientWidth || 360;
  const height = mapEl.clientHeight || 360;
  const viewport = chooseViewport(width, height);
  const center = viewport.center;
  const zoom = viewport.zoom;
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
          showFallback('Peta belum termuat', 'Tile OpenStreetMap gagal dimuat. Cek koneksi internet HP.');
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
    if (loaded === 0) showFallback('Peta belum termuat', 'Tile OpenStreetMap belum masuk. Cek koneksi internet HP.');
  }, 6000);
}

try {
  renderMap();
} catch (_) {
  showFallback('Peta belum termuat', 'Renderer tile OpenStreetMap gagal dimuat di WebView HP.');
}
</script>
</body>
</html>"#
        .replace("__LOCATION__", &location_json)
        .replace("__REPORTS__", &reports_json)
        .replace("__ROUTE__", &route_json)
        .replace("__ROUTE_LEVEL__", &route_level_json)
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
        "Aman" => "#dcfce7",
        "Waspada" => "#fef3c7",
        _ => "#fee2e2",
    }
}

fn level_color(level: &str) -> &'static str {
    match level {
        "Aman" => "#166534",
        "Waspada" => "#92400e",
        _ => "#991b1b",
    }
}

fn distance_label(meters: f64) -> String {
    if meters < 1000.0 {
        format!("{:.0} m", meters)
    } else {
        format!("{:.1} km", meters / 1000.0)
    }
}

fn duration_label(seconds: f64) -> String {
    let minutes = (seconds / 60.0).round().max(1.0);
    if minutes < 60.0 {
        format!("{minutes:.0} mnt")
    } else {
        format!("{:.1} jam", minutes / 60.0)
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

fn short_hash(value: &str) -> String {
    let len = value.chars().count();
    if len <= 12 {
        return value.to_string();
    }
    let start: String = value.chars().take(8).collect();
    let end: String = value
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    format!("{start}...{end}")
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
