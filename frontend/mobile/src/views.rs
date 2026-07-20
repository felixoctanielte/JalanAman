use dioxus::prelude::*;
use jalanaman_shared::{
    DirectionsResponse, EmergencyContact, PlaceSuggestion, Report, RouteScoreResponse,
};

use crate::app_config::{app_spec, CopyKey, Language, MapPresentation, ReportCategory};
use crate::models::GeoPoint;
use crate::theme::*;
use crate::utils::{
    distance_label, duration_label, haversine_m, level_bg, level_color, localized_level,
    route_overlay_title,
};

#[component]
pub(crate) fn LaunchSplash(language: Language) -> Element {
    rsx! {
        div { style: "position:absolute;inset:0;z-index:50;display:flex;align-items:center;justify-content:center;padding:28px;background:radial-gradient(circle at 50% 78%,rgba(34,197,94,0.26),rgba(34,197,94,0) 34%),linear-gradient(145deg,#27272a 0%,#17181c 52%,#0b0f14 100%);box-sizing:border-box;",
            div { class: "ja-splash", style: "width:100%;max-width:330px;text-align:center;",
                div { style: "width:142px;height:142px;margin:0 auto 24px;display:flex;align-items:center;justify-content:center;border-radius:34px;background:linear-gradient(180deg,rgba(255,255,255,0.14),rgba(255,255,255,0.06));border:1px solid rgba(255,255,255,0.17);box-shadow:0 28px 64px rgba(0,0,0,0.34),inset 0 1px 0 rgba(255,255,255,0.18);backdrop-filter:blur(24px) saturate(175%);-webkit-backdrop-filter:blur(24px) saturate(175%);",
                    img { class: "ja-splash-logo", src: app_spec().assets.logo, alt: app_spec().brand.logo_alt, style: "width:116px;height:116px;object-fit:contain;" }
                }
                div { style: "font-size:29px;font-weight:950;color:#f8fafc;letter-spacing:0;", "{app_spec().brand.name}" }
                div { style: "max-width:250px;margin:8px auto 0;color:#cbd5e1;font-size:14px;font-weight:750;line-height:1.45;", "{language.text(CopyKey::SplashSubtitle)}" }
                div { style: "width:54px;height:5px;margin:28px auto 0;border-radius:99px;background:linear-gradient(90deg,#2563eb,#0ea5e9,#14b8a6);box-shadow:0 10px 22px rgba(37,99,235,0.22);" }
            }
        }
    }
}

#[component]
pub(crate) fn SosOverlay(
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
pub(crate) fn MapView(
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
                div { style: "position:absolute;right:12px;top:12px;z-index:3;display:flex;gap:5px;padding:4px;border:1px solid rgba(255,255,255,0.18);border-radius:8px;background:rgba(38,41,48,0.78);box-shadow:0 12px 28px rgba(0,0,0,0.26),inset 0 1px 0 rgba(255,255,255,0.16),inset 0 -1px 0 rgba(0,0,0,0.24);backdrop-filter:blur(18px) saturate(170%);-webkit-backdrop-filter:blur(18px) saturate(170%);",
                    button {
                        style: if presentation == MapPresentation::TwoDimensional { "height:31px;min-width:40px;border:0;border-radius:7px;background:#1d4ed8;color:#ffffff;font-size:10px;font-weight:950;box-shadow:0 8px 16px rgba(37,99,235,0.24),inset 0 1px 0 rgba(255,255,255,0.24),inset 0 -1px 0 rgba(0,0,0,0.20);" } else { "height:31px;min-width:40px;border:0;border-radius:7px;background:transparent;color:#cbd5e1;font-size:10px;font-weight:900;" },
                        onclick: move |_| on_presentation.call(MapPresentation::TwoDimensional),
                        "{language.text(CopyKey::Map2D)}"
                    }
                    button {
                        style: if presentation == MapPresentation::ThreeDimensional { "height:31px;min-width:40px;border:0;border-radius:7px;background:#1d4ed8;color:#ffffff;font-size:10px;font-weight:950;box-shadow:0 8px 16px rgba(37,99,235,0.24),inset 0 1px 0 rgba(255,255,255,0.24),inset 0 -1px 0 rgba(0,0,0,0.20);" } else { "height:31px;min-width:40px;border:0;border-radius:7px;background:transparent;color:#cbd5e1;font-size:10px;font-weight:900;" },
                        onclick: move |_| on_presentation.call(MapPresentation::ThreeDimensional),
                        "{language.text(CopyKey::Map3D)}"
                    }
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
                            placeholder: "{language.text(CopyKey::Latitude)}",
                            oninput: move |event| on_manual_lat.call(event.value()),
                        }
                        input {
                            style: INPUT,
                            value: "{manual_lng}",
                            placeholder: "{language.text(CopyKey::Longitude)}",
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
pub(crate) fn RouteView(
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
pub(crate) fn ReportView(
    category: ReportCategory,
    language: Language,
    note: String,
    location: Option<GeoPoint>,
    location_was_selected: bool,
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

                if let Some(point) = location {
                    div { style: "margin-top:12px;padding:10px 11px;border:1px solid rgba(147,197,253,0.30);border-radius:8px;background:rgba(37,99,235,0.14);",
                        div { style: EYEBROW,
                            {if location_was_selected {
                                language.text(CopyKey::SelectedReportPoint)
                            } else {
                                language.text(CopyKey::CurrentGpsLocation)
                            }}
                        }
                        div { style: "font-size:13px;font-weight:950;color:#bfdbfe;",
                            "{point.lat:.6}, {point.lng:.6}"
                        }
                    }
                }

                div { style: CATEGORY_GRID,
                    for item in app_spec().report.categories.iter() {
                        CategoryButton {
                            category: item.category,
                            language,
                            selected: category == item.category,
                            onclick: move |_| on_category.call(item.category),
                        }
                    }
                }

                div { style: "margin-top:12px;",
                    div { style: EYEBROW, "{language.text(CopyKey::OptionalNote)}" }
                    textarea {
                        style: TEXTAREA,
                        value: "{note}",
                        maxlength: "{app_spec().limits.report_note_max}",
                        placeholder: "{language.text(CopyKey::Max100)}",
                        oninput: move |event| on_note.call(event.value()),
                    }
                    div { style: "margin-top:6px;text-align:right;font-size:10px;font-weight:750;color:#9ca3af;",
                        "{note_count}/{app_spec().limits.report_note_max}"
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
pub(crate) fn ContactsView(
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
    on_delete: EventHandler<String>,
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
                    Badge { label: language.text(CopyKey::ContactChannels).to_string(), bg: "#fee2e2", color: "#991b1b" }
                }

                div { style: "display:flex;flex-direction:column;gap:9px;margin-top:12px;",
                    if contacts.is_empty() {
                        div { style: BODY, "{language.text(CopyKey::NoContacts)}" }
                    } else {
                        for contact in contacts {
                            ContactRow {
                                contact,
                                language,
                                on_delete: move |contact_id| on_delete.call(contact_id),
                            }
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
pub(crate) fn ProfileView(
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
pub(crate) fn HelpView(language: Language) -> Element {
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
                        div { style: TITLE, "{app_spec().brand.name}" }
                    }
                    Badge { label: language.text(CopyKey::MediaMp4).to_string(), bg: "rgba(37,99,235,0.24)", color: "#bfdbfe" }
                }
                div { style: "margin-top:12px;overflow:hidden;border-radius:8px;border:1px solid rgba(255,255,255,0.14);background:#05070b;box-shadow:0 18px 36px rgba(0,0,0,0.28),inset 0 1px 0 rgba(255,255,255,0.10);",
                    video {
                        style: "width:100%;max-height:55dvh;display:block;background:#05070b;object-fit:contain;",
                        controls: true,
                        preload: "metadata",
                        source {
                            src: app_spec().assets.tutorial_video,
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
        .unwrap_or_else(|| language.text(CopyKey::DistanceUnavailable).to_string());
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
fn ContactRow(
    contact: EmergencyContact,
    language: Language,
    on_delete: EventHandler<String>,
) -> Element {
    let contact_id = contact.id.clone();
    let status = if contact.push_endpoint.is_some() {
        language.text(CopyKey::ContactPushReady)
    } else if contact.email.is_some() && contact.phone.is_some() {
        "Email + WA"
    } else if contact.email.is_some() {
        language.text(CopyKey::ContactEmailReady)
    } else if contact.phone.is_some() {
        language.text(CopyKey::ContactWaReady)
    } else {
        language.text(CopyKey::ContactPending)
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
        language.text(CopyKey::ContactNotConnected).to_string()
    } else {
        details.join(" | ")
    };
    let waiting = status == language.text(CopyKey::ContactPending);
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
            div { style: "display:flex;align-items:center;gap:7px;flex-shrink:0;",
                Badge { label: status.to_string(), bg: status_bg, color: status_color }
                button {
                    style: "width:34px;height:34px;border-radius:10px;border:1px solid rgba(254,202,202,0.24);background:rgba(127,29,29,0.28);color:#fecdd3;font-size:17px;font-weight:950;display:flex;align-items:center;justify-content:center;box-shadow:inset 0 1px 0 rgba(255,255,255,0.10);",
                    title: "{language.text(CopyKey::DeleteContactTitle)}",
                    onclick: move |_| on_delete.call(contact_id.clone()),
                    "×"
                }
            }
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
