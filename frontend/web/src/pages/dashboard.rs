use crate::{app::Route, services::api, utils::js};
use dioxus::prelude::*;
use jalanaman_shared::{category_emoji, category_label, Report};

#[component]
pub fn Dashboard() -> Element {
    let mut reports = use_signal(Vec::<Report>::new);
    let mut loading = use_signal(|| true);
    let mut map_inited = use_signal(|| false);

    // Fetch data then init map + heatmap. Leaflet is already loaded from CDN,
    // so init_map() can be called synchronously right after data arrives.
    use_effect(move || {
        spawn(async move {
            // Jakarta-wide radius (50 km) for city-level heatmap
            let data = api::get_reports(-6.2088, 106.8456, 50_000.0)
                .await
                .unwrap_or_default();

            if !*map_inited.read() {
                map_inited.set(true);
                js::init_map(-6.2088, 106.8456);
                let pts: Vec<_> = data
                    .iter()
                    .map(|r| serde_json::json!({ "lat": r.lat, "lng": r.lng }))
                    .collect();
                js::init_heatmap(&serde_json::to_string(&pts).unwrap_or_default());
            }

            reports.set(data);
            loading.set(false);
        });
    });

    let crime_ct = reports
        .read()
        .iter()
        .filter(|r| r.category == "crime")
        .count();
    let accident_ct = reports
        .read()
        .iter()
        .filter(|r| r.category == "accident")
        .count();
    let lighting_ct = reports
        .read()
        .iter()
        .filter(|r| r.category == "lighting")
        .count();
    let other_ct = reports
        .read()
        .iter()
        .filter(|r| r.category == "other")
        .count();

    rsx! {
        div { class: "min-h-screen bg-gray-50",

            div { class: "bg-blue-700 text-white px-4 py-3 flex items-center gap-3 shadow",
                Link { to: Route::Home {}, class: "text-blue-200 hover:text-white text-sm", "← Kembali" }
                h1 { class: "font-bold text-lg", "Dashboard Keamanan Wilayah" }
            }

            div { class: "p-4 max-w-4xl mx-auto",

                div { class: "grid grid-cols-2 sm:grid-cols-4 gap-3 mb-6",
                    StatCard { icon: "🔴", label: "Rawan Begal",       count: crime_ct,    color: "red"    }
                    StatCard { icon: "🟠", label: "Rawan Kecelakaan",  count: accident_ct, color: "orange" }
                    StatCard { icon: "🟡", label: "Pencahayaan Buruk", count: lighting_ct, color: "yellow" }
                    StatCard { icon: "⚪", label: "Lainnya",           count: other_ct,    color: "gray"   }
                }

                div { class: "bg-white rounded-2xl shadow overflow-hidden mb-4",
                    div { class: "px-4 py-3 border-b border-gray-100",
                        h2 { class: "font-semibold text-gray-700", "Peta Titik Rawan (Heatmap)" }
                        p  { class: "text-xs text-gray-400 mt-0.5", "Data 30 hari terakhir · OpenStreetMap" }
                    }
                    div { class: "relative",
                        div { id: "map-container", class: "w-full h-96" }
                        if *loading.read() {
                            div { class: "absolute inset-0 flex items-center justify-center bg-gray-50/80",
                                span { class: "text-gray-400 text-sm", "Memuat heatmap..." }
                            }
                        }
                    }
                }

                div { class: "bg-white rounded-2xl shadow overflow-hidden",
                    div { class: "px-4 py-3 border-b border-gray-100 flex items-center justify-between",
                        h2 { class: "font-semibold text-gray-700", "Laporan Terbaru" }
                        span { class: "text-xs text-gray-400", "{reports.read().len()} total" }
                    }
                    div { class: "divide-y divide-gray-50 max-h-80 overflow-y-auto",
                        for r in reports.read().iter().take(50) {
                            div { class: "px-4 py-3 flex items-start gap-3 hover:bg-gray-50",
                                span { class: "text-lg flex-shrink-0", "{category_emoji(&r.category)}" }
                                div { class: "flex-1 min-w-0",
                                    p { class: "text-sm font-medium text-gray-700", "{category_label(&r.category)}" }
                                    if let Some(note) = &r.note {
                                        p { class: "text-xs text-gray-400 truncate", "{note}" }
                                    }
                                }
                                div { class: "text-right flex-shrink-0",
                                    p { class: "text-xs text-gray-400", "{r.lat:.4}, {r.lng:.4}" }
                                    div { class: "flex gap-1 justify-end mt-1",
                                        span { class: "text-xs text-green-600", "▲{r.upvote_count}" }
                                        span { class: "text-xs text-red-400",   "▼{r.downvote_count}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(icon: &'static str, label: &'static str, count: usize, color: &'static str) -> Element {
    let (card_class, count_class) = match color {
        "red" => (
            "border bg-red-50 border-red-100 rounded-2xl p-4",
            "text-2xl font-black text-red-700",
        ),
        "orange" => (
            "border bg-orange-50 border-orange-100 rounded-2xl p-4",
            "text-2xl font-black text-orange-700",
        ),
        "yellow" => (
            "border bg-yellow-50 border-yellow-100 rounded-2xl p-4",
            "text-2xl font-black text-yellow-700",
        ),
        _ => (
            "border bg-gray-50 border-gray-100 rounded-2xl p-4",
            "text-2xl font-black text-gray-700",
        ),
    };
    rsx! {
        div { class: card_class,
            div { class: "text-2xl mb-1", "{icon}" }
            div { class: count_class, "{count}" }
            div { class: "text-xs text-gray-500 mt-0.5", "{label}" }
        }
    }
}
