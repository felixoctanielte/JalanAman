use dioxus::prelude::*;
use jalanaman_shared::{
    CreateReportPayload, DirectionsResponse, EmergencyContact, PlaceSuggestion, Report,
    RouteScoreResponse,
};
use serde::Deserialize;
use std::time::Duration;

use crate::app_config::{
    app_spec, CopyKey, Language, MapPresentation, MobileTab, ReportCategory, ThemeMode,
};
use crate::dashboard::{Dashboard, Fallback};
use crate::header::Header;
use crate::map::map_srcdoc;
use crate::models::{GeoPoint, MapSelectionEval, VoiceCommandEval};
use crate::navigation::{BottomNavigation, SosButton};
use crate::platform::*;
use crate::screens::*;
use crate::services::*;
use crate::theme::*;
use crate::utils::*;

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

#[derive(Clone, Copy)]
enum SosSource {
    Button,
    Voice,
}

#[allow(clippy::too_many_arguments)]
fn trigger_sos_flow(
    source: SosSource,
    language: Language,
    mut location: Signal<Option<GeoPoint>>,
    mut location_error: Signal<Option<String>>,
    mut manual_lat: Signal<String>,
    mut manual_lng: Signal<String>,
    device_hash: Signal<String>,
    contacts: Signal<Vec<EmergencyContact>>,
    mut sos_active: Signal<bool>,
    mut sos_msg: Signal<Option<String>>,
    mut sos_modal_open: Signal<bool>,
) {
    if *sos_active.read() {
        return;
    }
    stop_voice_command();

    let point = *location.read();
    let hash = device_hash.read().clone();
    if hash.is_empty() {
        sos_msg.set(Some(language.text(CopyKey::AppPreparing).to_string()));
        sos_modal_open.set(true);
        return;
    }
    let whatsapp_contacts = contacts.read().clone();

    let preparing_key = match source {
        SosSource::Button => CopyKey::PreparingSosLocation,
        SosSource::Voice => CopyKey::VoiceMatched,
    };
    sos_msg.set(Some(language.text(preparing_key).to_string()));
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
                    location_error.set(Some(err));
                    sos_msg.set(Some(language.text(CopyKey::SosLocationMissing).to_string()));
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
        sos_msg.set(Some(language.text(CopyKey::SosActiveSending).to_string()));

        match trigger_sos(&hash, point).await {
            Ok(response) => {
                let whatsapp_opened = open_whatsapp_sos(&whatsapp_contacts, point)
                    .await
                    .unwrap_or(false);

                if whatsapp_opened {
                    sos_msg.set(Some(language.text(CopyKey::WhatsappFallback).to_string()));
                } else if response.notified_count > 0 {
                    sos_msg.set(Some(language.text(CopyKey::SosNotified).to_string()));
                } else {
                    sos_msg.set(Some(language.text(CopyKey::SosNoChannel).to_string()));
                }
            }
            Err(_) => {
                let whatsapp_opened = open_whatsapp_sos(&whatsapp_contacts, point)
                    .await
                    .unwrap_or(false);
                if whatsapp_opened {
                    sos_msg.set(Some(language.text(CopyKey::SosBackendFallback).to_string()));
                } else {
                    sos_msg.set(Some(language.text(CopyKey::SosStaySafe).to_string()));
                }
            }
        }
    });
}

fn arm_voice_sos(
    voice_enabled: Signal<bool>,
    mut voice_listening: Signal<bool>,
    mut voice_error: Signal<Option<String>>,
    sos_active: Signal<bool>,
) {
    if !*voice_enabled.peek() || *sos_active.peek() || *voice_listening.peek() {
        return;
    }

    voice_listening.set(true);
    voice_error.set(None);
    spawn(async move {
        if let Err(err) = start_voice_command().await {
            voice_listening.set(false);
            voice_error.set(Some(err));
        }
    });
}

fn start_voice_sos_entry(
    language: Language,
    voice_enabled: Signal<bool>,
    mut voice_listening: Signal<bool>,
    mut voice_error: Signal<Option<String>>,
    sos_active: Signal<bool>,
    mut settings_msg: Signal<Option<String>>,
    mut sos_msg: Signal<Option<String>>,
    mut sos_modal_open: Signal<bool>,
    show_overlay: bool,
) {
    if !*voice_enabled.peek() {
        let message = language.text(CopyKey::VoiceShortcutDisabled).to_string();
        settings_msg.set(Some(message.clone()));
        if show_overlay {
            sos_msg.set(Some(message));
            sos_modal_open.set(true);
        }
        return;
    }

    settings_msg.set(Some(language.text(CopyKey::VoiceListening).to_string()));
    if show_overlay {
        sos_msg.set(Some(language.text(CopyKey::VoiceListening).to_string()));
        sos_modal_open.set(true);
    }

    spawn(async move {
        match request_microphone_permission().await {
            Ok(()) => {
                voice_listening.set(false);
                tokio::time::sleep(Duration::from_millis(250)).await;
                arm_voice_sos(voice_enabled, voice_listening, voice_error, sos_active);
            }
            Err(err) => {
                voice_listening.set(false);
                voice_error.set(Some(err.clone()));
                settings_msg.set(Some(err.clone()));
                if show_overlay {
                    sos_msg.set(Some(err));
                    sos_modal_open.set(true);
                }
            }
        }
    });
}

fn stop_sos_flow(
    language: Language,
    mut sos_active: Signal<bool>,
    mut sos_msg: Signal<Option<String>>,
    mut sos_modal_open: Signal<bool>,
) {
    stop_voice_command();
    stop_sos_alarm();
    sos_active.set(false);
    sos_msg.set(Some(language.text(CopyKey::SosStopped).to_string()));
    sos_modal_open.set(true);
}

fn is_voice_sos_command(text: &str) -> bool {
    let normalized = text.to_lowercase();
    [
        "jalanaman sos",
        "jalan aman sos",
        "aktifkan sos",
        "nyalakan sos",
        "sos",
        "tolong",
        "bantuan",
        "minta bantuan",
        "darurat",
        "bahaya",
    ]
    .iter()
    .any(|keyword| normalized.contains(keyword))
}

#[component]
pub(crate) fn Home() -> Element {
    let mut active_tab = use_signal(|| app_spec().default_tab);
    let mut map_presentation = use_signal(|| MapPresentation::TwoDimensional);
    let mut language = use_signal(|| app_spec().default_language);
    let mut theme_mode = use_signal(|| ThemeMode::Dark);
    let mut settings_msg = use_signal(|| Option::<String>::None);
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
    let mut report_location = use_signal(|| Option::<GeoPoint>::None);
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
    let mut voice_enabled = use_signal(|| false);
    let mut voice_listening = use_signal(|| false);
    let mut voice_last_text = use_signal(|| Option::<String>::None);
    let mut voice_error = use_signal(|| Option::<String>::None);
    let mut manual_lat = use_signal(String::new);
    let mut manual_lng = use_signal(String::new);
    let mut manual_location_error = use_signal(|| Option::<String>::None);

    use_future(move || async move {
        tokio::time::sleep(Duration::from_millis(1450)).await;
        show_splash.set(false);
    });

    use_effect(move || {
        spawn(async move {
            let saved_theme = read_theme_mode().await;
            theme_mode.set(saved_theme);

            let saved_voice = read_voice_sos_enabled().await;
            voice_enabled.set(saved_voice);
        });
    });

    use_future(move || async move {
        loop {
            let eval = document::eval(
                r#"
                return await new Promise((resolve) => {
                    const receive = (event) => {
                        const data = event.data;
                        if (!data || data.type !== 'jalanaman-map-report') return;
                        window.removeEventListener('message', receive);
                        resolve({ lat: data.lat, lng: data.lng });
                    };
                    window.addEventListener('message', receive);
                });
                "#,
            );

            let Ok(value) = eval.await else {
                continue;
            };
            let Ok(selected) = MapSelectionEval::deserialize(&value) else {
                continue;
            };
            if !selected.lat.is_finite() || !selected.lng.is_finite() {
                continue;
            }

            report_location.set(Some(GeoPoint {
                lat: selected.lat,
                lng: selected.lng,
            }));
            report_error.set(None);
            active_tab.set(MobileTab::Report);
        }
    });

    use_future(move || async move {
        loop {
            let eval = document::eval(
                r#"
                return await new Promise((resolve) => {
                    const receive = (event) => {
                        const data = event.data;
                        if (!data || data.type !== 'jalanaman-voice-command') return;
                        window.removeEventListener('message', receive);
                        resolve({
                            status: data.status || null,
                            text: data.text || null,
                            error: data.error || null,
                        });
                    };
                    window.addEventListener('message', receive);
                });
                "#,
            );

            let Ok(value) = eval.await else {
                continue;
            };
            let Ok(command) = VoiceCommandEval::deserialize(&value) else {
                continue;
            };
            let current_language = *language.peek();

            if matches!(command.status.as_deref(), Some("listening")) {
                voice_listening.set(true);
                voice_error.set(None);
                continue;
            }

            voice_listening.set(false);

            if let Some(err) = command.error.filter(|err| !err.trim().is_empty()) {
                voice_error.set(Some(err.clone()));
                if *sos_modal_open.peek() && !*sos_active.peek() {
                    sos_msg.set(Some(err));
                }
                continue;
            }

            let Some(text) = command.text.filter(|text| !text.trim().is_empty()) else {
                continue;
            };
            voice_last_text.set(Some(text.clone()));

            if is_voice_sos_command(&text) {
                voice_error.set(None);
                sos_msg.set(Some(
                    current_language.text(CopyKey::VoiceMatched).to_string(),
                ));
                sos_modal_open.set(true);
                trigger_sos_flow(
                    SosSource::Voice,
                    current_language,
                    location,
                    location_error,
                    manual_lat,
                    manual_lng,
                    device_hash,
                    contacts,
                    sos_active,
                    sos_msg,
                    sos_modal_open,
                );
            } else {
                let message = format!(
                    "{} ({})",
                    current_language.text(CopyKey::VoiceNotRecognized),
                    text
                );
                voice_error.set(Some(message));
                if *sos_modal_open.peek() && !*sos_active.peek() {
                    sos_msg.set(Some(
                        current_language
                            .text(CopyKey::VoiceNotRecognized)
                            .to_string(),
                    ));
                }
            }
        }
    });

    use_future(move || async move {
        tokio::time::sleep(Duration::from_millis(900)).await;
        loop {
            if let Some(action) = consume_launch_action().await {
                let current_language = *language.peek();
                match action.as_str() {
                    "voice_sos" | "voice" | "voice_sos_widget" => {
                        start_voice_sos_entry(
                            current_language,
                            voice_enabled,
                            voice_listening,
                            voice_error,
                            sos_active,
                            settings_msg,
                            sos_msg,
                            sos_modal_open,
                            true,
                        );
                    }
                    "sos" | "sos_widget" => {
                        trigger_sos_flow(
                            SosSource::Button,
                            current_language,
                            location,
                            location_error,
                            manual_lat,
                            manual_lng,
                            device_hash,
                            contacts,
                            sos_active,
                            sos_msg,
                            sos_modal_open,
                        );
                    }
                    _ => {}
                }
            }
            tokio::time::sleep(Duration::from_millis(700)).await;
        }
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
    let theme_mode_value = *theme_mode.read();
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
        None,
        None,
        map_presentation_value == MapPresentation::ThreeDimensional,
        language_value,
        true,
    );
    let route_map_html = map_srcdoc(
        location_value,
        &reports_value,
        directions_value.as_ref().map(|d| d.polyline.as_slice()),
        route_score_value.as_ref().map(|s| s.level.as_str()),
        map_presentation_value == MapPresentation::ThreeDimensional,
        language_value,
        false,
    );
    rsx! {
        main {
            style: if theme_mode_value == ThemeMode::Light { APP_LIGHT } else { APP },
            class: if theme_mode_value == ThemeMode::Light { "ja-theme-light" } else { "ja-theme-dark" },
            style { {MOTION_CSS} }
            div { style: if theme_mode_value == ThemeMode::Light { SCREEN_LIGHT } else { SCREEN },
                Header {
                    language: language_value,
                    on_language: move |next| language.set(next),
                    on_help: move |_| active_tab.set(MobileTab::Help),
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
                            on_manual_lat: move |value| {
                                manual_lat.set(limit_text(value, app_spec().limits.manual_coordinate_max))
                            },
                            on_manual_lng: move |value| {
                                manual_lng.set(limit_text(value, app_spec().limits.manual_coordinate_max))
                            },
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
                                    Err(err) => manual_location_error
                                        .set(Some(language_value.text(err).to_string())),
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
                            map_html: route_map_html,
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
                                let value = limit_text(value, app_spec().limits.destination_max);
                                let query = value.trim().to_string();
                                destination.set(value);
                                selected_place.set(None);
                                directions.set(None);
                                route_score.set(None);
                                route_error.set(None);

                                let revision = (*place_search_revision.peek()).wrapping_add(1);
                                place_search_revision.set(revision);
                                if query.len() < app_spec().limits.place_query_min {
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
                                if dest.len() < app_spec().limits.destination_min {
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
                        {
                            let selected_report_location =
                                report_location.read().as_ref().copied().or(location_value);
                            rsx! {
                        ReportView {
                            category: report_category_value,
                            language: language_value,
                            note: report_note_value,
                            location: selected_report_location,
                            location_was_selected: report_location.read().is_some(),
                            loading: *report_loading.read(),
                            error: report_error.read().clone(),
                            on_category: move |category| report_category.set(category),
                            on_note: move |value| report_note.set(limit_text(value, app_spec().limits.report_note_max)),
                            on_submit: move |_| {
                                let point = report_location
                                    .read()
                                    .as_ref()
                                    .copied()
                                    .or(*location.read());
                                let Some(point) = point else {
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
                                            report_location.set(None);
                                            active_tab.set(MobileTab::Map);
                                        }
                                        Err(err) => report_error.set(Some(err)),
                                    }
                                    report_loading.set(false);
                                });
                            },
                        }
                            }
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
                            on_name: move |value| contact_name.set(limit_text(value, app_spec().limits.contact_name_max)),
                            on_email: move |value| contact_email.set(limit_text(value, app_spec().limits.contact_email_max)),
                            on_phone: move |value| contact_phone.set(limit_text(value, app_spec().limits.contact_phone_max)),
                            on_delete: move |contact_id: String| {
                                let hash = device_hash.read().clone();
                                if hash.is_empty() {
                                    contacts_error.set(Some(language_value.text(CopyKey::AppPreparing).to_string()));
                                    return;
                                }

                                contacts_loading.set(true);
                                contacts_error.set(None);
                                spawn(async move {
                                    match delete_contact(&hash, &contact_id).await {
                                        Ok(()) => contacts.write().retain(|contact| contact.id != contact_id),
                                        Err(err) => contacts_error.set(Some(err)),
                                    }
                                    contacts_loading.set(false);
                                });
                            },
                            on_add: move |_| {
                                let hash = device_hash.read().clone();
                                let name = contact_name.read().trim().to_string();
                                let email_text = contact_email.read().trim().to_string();
                                let phone_text = contact_phone.read().trim().to_string();
                                if hash.is_empty() {
                                    contacts_error.set(Some(language_value.text(CopyKey::AppPreparing).to_string()));
                                    return;
                                }
                                if name.len() < app_spec().limits.contact_name_min {
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
                            voice_enabled: *voice_enabled.read(),
                            voice_listening: *voice_listening.read(),
                            theme_mode: theme_mode_value,
                            settings_msg: settings_msg.read().clone().or(voice_error.read().clone()),
                            on_voice_enabled: move |enabled| {
                                settings_msg.set(None);
                                voice_enabled.set(enabled);
                                write_voice_sos_enabled(enabled);

                                if !enabled {
                                    stop_voice_command();
                                    voice_listening.set(false);
                                    voice_error.set(None);
                                    settings_msg.set(Some(language_value.text(CopyKey::ChatOnly).to_string()));
                                    return;
                                }

                                settings_msg.set(Some(language_value.text(CopyKey::MicrophoneRequested).to_string()));
                                spawn(async move {
                                    match request_microphone_permission().await {
                                        Ok(()) => {
                                            voice_listening.set(false);
                                            voice_error.set(None);
                                            settings_msg.set(Some(
                                                language_value
                                                    .text(CopyKey::VoiceShortcutReady)
                                                    .to_string(),
                                            ));
                                        }
                                        Err(err) => {
                                            voice_enabled.set(false);
                                            write_voice_sos_enabled(false);
                                            voice_listening.set(false);
                                            voice_error.set(Some(err.clone()));
                                            settings_msg.set(Some(err));
                                        }
                                    }
                                });
                            },
                            on_request_microphone: move |_| {
                                if *voice_listening.read() {
                                    stop_voice_command();
                                    voice_listening.set(false);
                                    settings_msg.set(Some(language_value.text(CopyKey::TestVoiceSos).to_string()));
                                    return;
                                }

                                start_voice_sos_entry(
                                    language_value,
                                    voice_enabled,
                                    voice_listening,
                                    voice_error,
                                    sos_active,
                                    settings_msg,
                                    sos_msg,
                                    sos_modal_open,
                                    true,
                                );
                            },
                            on_theme_mode: move |mode| {
                                theme_mode.set(mode);
                                write_theme_mode(mode);
                            },
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
                            stop_sos_flow(language_value, sos_active, sos_msg, sos_modal_open);
                        },
                    }
                }

                BottomNavigation {
                    active_tab: active_tab_value,
                    language: language_value,
                    on_tab: move |tab| active_tab.set(tab),
                }

                SosButton {
                    active: *sos_active.read(),
                    language: language_value,
                    on_click: move |_| {
                        if *sos_active.read() {
                            stop_sos_flow(language_value, sos_active, sos_msg, sos_modal_open);
                            return;
                        }

                        trigger_sos_flow(
                            SosSource::Button,
                            language_value,
                            location,
                            location_error,
                            manual_lat,
                            manual_lng,
                            device_hash,
                            contacts,
                            sos_active,
                            sos_msg,
                            sos_modal_open,
                        );
                    },
                }

                if *show_splash.read() {
                    LaunchSplash { language: language_value }
                }
            }
        }
    }
}
