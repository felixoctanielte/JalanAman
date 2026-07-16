//! Route score panel – pure RSX. Platform provides score data via props.
use crate::utils::types::RouteScoreResponse;
use dioxus::prelude::*;

#[component]
pub fn RouteScorePanel(
    result: Option<RouteScoreResponse>,
    loading: bool,
    error: Option<String>,
    /// Called with the destination string when user submits the form.
    on_search: EventHandler<String>,
) -> Element {
    let mut destination = use_signal(String::new);

    rsx! {
        div {
            class: "absolute top-16 left-3 right-3 bg-white rounded-2xl shadow-xl p-4 z-20",

            h3 { class: "font-bold text-gray-800 mb-3", "Cek Skor Keamanan Rute" }

            form {
                onsubmit: move |_e| {
                    let dest = destination.read().trim().to_string();
                    if !dest.is_empty() { on_search.call(dest); }
                },
                class: "flex gap-2",

                input {
                    class: "flex-1 border border-gray-200 rounded-xl px-3 py-2 text-sm",
                    placeholder: "Masukkan tujuan...",
                    value: "{destination}",
                    oninput: move |e| destination.set(e.value()),
                }
                button {
                    r#type: "submit",
                    class: "bg-blue-700 text-white px-4 py-2 rounded-xl text-sm font-semibold disabled:opacity-50",
                    disabled: loading,
                    if loading { "..." } else { "Cek" }
                }
            }

            if let Some(err) = &error {
                p { class: "text-red-600 text-sm mt-2", "{err}" }
            }

            if let Some(r) = &result {
                div { class: "mt-3",
                    div {
                        class: match r.level.as_str() {
                            "Aman"    => "flex items-center gap-3 bg-green-50 border border-green-200 rounded-xl p-3",
                            "Waspada" => "flex items-center gap-3 bg-yellow-50 border border-yellow-200 rounded-xl p-3",
                            _         => "flex items-center gap-3 bg-red-50 border border-red-200 rounded-xl p-3",
                        },
                        span { class: "text-3xl",
                            match r.level.as_str() {
                                "Aman"    => "✅",
                                "Waspada" => "⚠️",
                                _         => "🚫",
                            }
                        }
                        div {
                            p {
                                class: match r.level.as_str() {
                                    "Aman"    => "text-green-700 font-bold text-lg",
                                    "Waspada" => "text-yellow-700 font-bold text-lg",
                                    _         => "text-red-700 font-bold text-lg",
                                },
                                "{r.level}"
                            }
                            p { class: "text-xs text-gray-500",
                                "{r.report_count} laporan di sekitar rute"
                                if r.cache_hit { " (cached)" }
                            }
                        }
                    }
                }
            }
        }
    }
}
