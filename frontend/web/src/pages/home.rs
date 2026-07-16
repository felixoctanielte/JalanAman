use dioxus::prelude::*;
use jalanaman_shared::{
    components::{report_form::ReportForm, route_score::RouteScorePanel, sos_button::SosButton},
    Report, RouteScoreResponse, Waypoint,
};
use wasm_bindgen_futures::JsFuture;

use crate::{
    app::Route, components::map::MapView, hooks::use_geolocation::use_geolocation, services::api,
    utils::device::get_device_hash, utils::js,
};

#[derive(serde::Deserialize)]
struct DirectionsResult {
    waypoints: Vec<Waypoint>,
    polyline: Vec<Waypoint>,
}

#[component]
pub fn Home() -> Element {
    let mut reports = use_signal(Vec::<Report>::new);
    let location = use_geolocation();
    let mut show_report = use_signal(|| false);
    let mut show_route = use_signal(|| false);
    let mut sos_active = use_signal(|| false);
    let mut sos_msg = use_signal(|| Option::<String>::None);
    let mut report_loading = use_signal(|| false);
    let mut report_error = use_signal(|| Option::<String>::None);
    let mut route_result = use_signal(|| Option::<RouteScoreResponse>::None);
    let mut route_loading = use_signal(|| false);
    let mut route_error = use_signal(|| Option::<String>::None);
    // Prevent double-init when location signal fires multiple times
    let mut map_inited = use_signal(|| false);

    // Leaflet loads from CDN synchronously → init_map can be called immediately
    // once geolocation resolves.
    use_effect(move || {
        let loc = *location.read();
        let inited = *map_inited.read();
        if loc.is_some() && !inited {
            let (lat, lng) = loc.unwrap();
            map_inited.set(true);
            js::init_map(lat, lng);
            spawn(async move {
                if let Ok(r) = api::get_reports(lat, lng, 800.0).await {
                    reports.set(r);
                }
            });
        }
    });

    // Register service worker for push notifications
    use_effect(move || {
        spawn(async move {
            crate::services::push::register_service_worker().await;
        });
    });

    rsx! {
        div { class: "h-screen flex flex-col bg-gray-100 select-none",

            // ── Header ────────────────────────────────────────────────────────
            div { class: "bg-blue-700 text-white px-4 py-3 flex items-center justify-between shadow-md z-10 flex-shrink-0",
                div { class: "flex items-center gap-2",
                    span { class: "text-xl font-black tracking-tight", "JalanAman" }
                    span { class: "hidden sm:inline text-xs text-blue-200", "Rute Teraman Indonesia" }
                }
                div { class: "flex items-center gap-2",
                    button {
                        class: "text-sm bg-blue-600 hover:bg-blue-500 px-3 py-1.5 rounded-full font-medium transition",
                        onclick: move |_| { let v = *show_route.read(); show_route.set(!v); },
                        "🗺 Skor Rute"
                    }
                    Link {
                        to: Route::Contacts {},
                        class: "text-sm bg-blue-600 hover:bg-blue-500 px-3 py-1.5 rounded-full font-medium transition",
                        "🆘 Kontak"
                    }
                    Link {
                        to: Route::Dashboard {},
                        class: "text-sm bg-blue-800 hover:bg-blue-700 px-3 py-1.5 rounded-full font-medium transition",
                        "📊"
                    }
                }
            }

            // ── Map + overlays ─────────────────────────────────────────────────
            div { class: "flex-1 relative overflow-hidden",
                MapView { reports: reports.read().clone() }

                // Route score panel
                if *show_route.read() {
                    RouteScorePanel {
                        result: route_result.read().clone(),
                        loading: *route_loading.read(),
                        error: route_error.read().clone(),
                        on_search: move |dest: String| {
                            route_loading.set(true);
                            route_error.set(None);
                            route_result.set(None);

                            let loc_val = *location.read();
                            let origin = loc_val
                                .map(|(la, lo)| format!("{la},{lo}"))
                                .unwrap_or_else(|| "Jakarta Pusat".to_string());

                            spawn(async move {
                                match JsFuture::from(js::get_directions(&origin, &dest, "walking")).await {
                                    Err(e) => {
                                        route_error.set(Some(format!("Rute tidak ditemukan: {e:?}")));
                                    }
                                    Ok(val) => {
                                        let json_str = val.as_string().unwrap_or_default();
                                        match serde_json::from_str::<DirectionsResult>(&json_str) {
                                            Err(e) => route_error.set(Some(format!("Parse error: {e}"))),
                                            Ok(dirs) => {
                                                match api::calculate_route_score(dirs.waypoints).await {
                                                    Ok(score) => {
                                                        let pts = serde_json::to_string(&dirs.polyline)
                                                            .unwrap_or_default();
                                                        js::draw_route_polyline(&pts, &score.level);
                                                        route_result.set(Some(score));
                                                    }
                                                    Err(e) => route_error.set(Some(e)),
                                                }
                                            }
                                        }
                                    }
                                }
                                route_loading.set(false);
                            });
                        },
                    }
                }

                // Report count badge
                div { class: "absolute top-3 left-3 bg-white/90 backdrop-blur rounded-xl px-3 py-1.5 shadow text-xs text-gray-600 font-medium z-10",
                    "⚠️ {reports.read().len()} laporan aktif"
                }

                // Lapor Bahaya button
                button {
                    class: "absolute bottom-28 left-4 bg-white shadow-lg rounded-full px-4 py-3 flex items-center gap-2 border border-gray-100 hover:bg-gray-50 active:scale-95 transition text-sm font-semibold text-gray-700 z-10",
                    onclick: move |_| { show_report.set(true); report_error.set(None); },
                    "⚠️ Lapor Bahaya"
                }

                SosButton {
                    active:     *sos_active.read(),
                    status_msg: sos_msg.read().clone(),
                    location:   *location.read(),
                    on_sos: move |loc: Option<(f64, f64)>| {
                        js::play_sos_alarm();
                        sos_active.set(true);
                        sos_msg.set(Some("Alarm berbunyi! Mengirim notifikasi...".into()));
                        let (lat, lng) = loc.unwrap_or((-6.2088, 106.8456));
                        let dh = get_device_hash();
                        spawn(async move {
                            match api::trigger_sos(&dh, lat, lng).await {
                                Ok(r) => sos_msg.set(Some(format!(
                                    "Alarm aktif. {}/{} kontak diberitahu.",
                                    r.notified_count, r.total_contacts
                                ))),
                                Err(e) => sos_msg.set(Some(format!("Alarm aktif. Gagal: {e}"))),
                            }
                        });
                    },
                    on_stop: move |_| {
                        js::stop_sos_alarm();
                        sos_active.set(false);
                        sos_msg.set(None);
                    },
                }
            }

            // ── Quick Report modal ─────────────────────────────────────────────
            if *show_report.read() {
                ReportForm {
                    lat: (*location.read()).map(|(la, _)| la),
                    lng: (*location.read()).map(|(_, lo)| lo),
                    device_hash: get_device_hash(),
                    loading: *report_loading.read(),
                    error: report_error.read().clone(),
                    on_close: move |_| show_report.set(false),
                    on_submit: move |payload| {
                        report_loading.set(true);
                        report_error.set(None);
                        spawn(async move {
                            match api::create_report(&payload).await {
                                Ok(r) => {
                                    js::add_report_marker(
                                        &r.id, r.lat, r.lng,
                                        &r.category,
                                        r.note.as_deref().unwrap_or(""),
                                    );
                                    reports.write().insert(0, r);
                                    show_report.set(false);
                                }
                                Err(e) => report_error.set(Some(e)),
                            }
                            report_loading.set(false);
                        });
                    },
                }
            }
        }
    }
}
