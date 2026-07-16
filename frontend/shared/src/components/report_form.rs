//! Pure RSX form – works on web and mobile.
//! The parent supplies `on_submit` which handles the platform-specific API call.
use dioxus::prelude::*;
use crate::utils::types::{CreateReportPayload, category_emoji, category_label};

#[component]
pub fn ReportForm(
    lat: Option<f64>,
    lng: Option<f64>,
    device_hash: String,
    loading: bool,
    error: Option<String>,
    on_close: EventHandler<()>,
    /// Parent resolves this with an actual API call.
    on_submit: EventHandler<CreateReportPayload>,
) -> Element {
    let mut category = use_signal(|| "crime".to_string());
    let mut note = use_signal(|| String::new());

    let handle_submit = move |_evt: FormEvent| {
        if lat.is_none() || lng.is_none() { return; }
        let n = note.read().trim().to_string();
        on_submit.call(CreateReportPayload {
            category: category.read().clone(),
            lat: lat.unwrap(),
            lng: lng.unwrap(),
            note: if n.is_empty() { None } else { Some(n) },
            device_hash: device_hash.clone(),
        });
    };

    rsx! {
        div {
            class: "fixed inset-0 bg-black/50 z-30 flex items-end justify-center",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-white rounded-t-2xl w-full max-w-lg p-6 pb-10",
                onclick: move |e| e.stop_propagation(),

                h2 { class: "text-lg font-bold text-gray-800 mb-4", "Lapor Titik Bahaya" }

                form { onsubmit: handle_submit,

                    // Category picker
                    fieldset { class: "mb-4",
                        legend { class: "text-sm font-medium text-gray-600 mb-2", "Kategori" }
                        div { class: "grid grid-cols-2 gap-2",
                            for cat in ["crime", "accident", "lighting", "other"] {
                                label {
                                    class: if *category.read() == cat {
                                        "flex items-center gap-2 border-2 border-blue-600 bg-blue-50 rounded-xl p-3 cursor-pointer"
                                    } else {
                                        "flex items-center gap-2 border-2 border-gray-200 rounded-xl p-3 cursor-pointer hover:border-blue-300"
                                    },
                                    input {
                                        r#type: "radio",
                                        name: "category",
                                        value: cat,
                                        class: "hidden",
                                        checked: *category.read() == cat,
                                        onchange: move |_| category.set(cat.to_string()),
                                    }
                                    span { "{category_emoji(cat)}" }
                                    span { class: "text-sm font-medium", "{category_label(cat)}" }
                                }
                            }
                        }
                    }

                    // Note
                    div { class: "mb-4",
                        label { class: "text-sm font-medium text-gray-600 block mb-1",
                            "Catatan (opsional, maks 100 karakter)"
                        }
                        textarea {
                            class: "w-full border border-gray-200 rounded-xl p-3 text-sm resize-none",
                            rows: "2",
                            maxlength: "100",
                            placeholder: "Contoh: banyak motor kencang di malam hari",
                            value: "{note}",
                            oninput: move |e| note.set(e.value()),
                        }
                    }

                    if let Some(err) = &error {
                        p { class: "text-red-600 text-sm mb-3", "{err}" }
                    }

                    if lat.is_none() {
                        p { class: "text-yellow-600 text-xs mb-3",
                            "⚠️ Menunggu lokasi GPS..."
                        }
                    }

                    div { class: "flex gap-3",
                        button {
                            r#type: "button",
                            class: "flex-1 border border-gray-300 rounded-xl py-3 text-sm font-medium text-gray-600",
                            onclick: move |_| on_close.call(()),
                            "Batal"
                        }
                        button {
                            r#type: "submit",
                            class: "flex-1 bg-blue-700 text-white rounded-xl py-3 text-sm font-semibold disabled:opacity-50",
                            disabled: loading || lat.is_none(),
                            if loading { "Mengirim..." } else { "Kirim Laporan" }
                        }
                    }
                }
            }
        }
    }
}
