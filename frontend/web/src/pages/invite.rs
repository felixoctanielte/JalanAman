use crate::services::{api, push};
use dioxus::prelude::*;

#[component]
pub fn Invite(token: String) -> Element {
    let mut contact_name = use_signal(|| Option::<String>::None);
    let mut already_connected = use_signal(|| false);
    let mut step = use_signal(|| 0u8); // 0=loading 1=info 2=done 3=error
    let mut error_msg = use_signal(String::new);

    let token_clone = token.clone();
    use_effect(move || {
        let t = token_clone.clone();
        spawn(async move {
            match api::get_invite_info(&t).await {
                Ok(info) => {
                    contact_name.set(info["contact_name"].as_str().map(str::to_string));
                    already_connected.set(info["already_connected"].as_bool().unwrap_or(false));
                    step.set(1);
                }
                Err(_) => {
                    error_msg.set("Link tidak valid atau sudah kadaluarsa.".into());
                    step.set(3);
                }
            }
        });
    });

    let token_for_sub = token.clone();
    let handle_subscribe = move |_| {
        let t = token_for_sub.clone();
        spawn(async move {
            match push::onboard_push(&t).await {
                Ok(_) => step.set(2),
                Err(e) => {
                    error_msg.set(e);
                    step.set(3);
                }
            }
        });
    };

    rsx! {
        div { class: "min-h-screen bg-blue-700 flex items-center justify-center p-4",
            div { class: "bg-white rounded-3xl p-8 max-w-sm w-full text-center shadow-2xl",

                div { class: "text-5xl mb-4", "🆘" }
                h1 { class: "text-2xl font-black text-gray-800 mb-1", "JalanAman" }
                p  { class: "text-gray-400 text-sm mb-6", "Undangan Kontak Darurat" }

                match *step.read() {
                    0 => rsx! { p { class: "text-gray-400 animate-pulse", "Memuat..." } },

                    1 => rsx! {
                        if let Some(name) = contact_name.read().as_ref() {
                            p { class: "text-gray-600 text-sm mb-1", "Anda diundang sebagai" }
                            p { class: "text-blue-700 font-bold text-xl mb-5", "Kontak Darurat \"{name}\"" }
                        }

                        if *already_connected.read() {
                            div { class: "bg-green-50 border border-green-200 rounded-xl p-3 text-green-700 text-sm",
                                "✅ Device ini sudah terhubung."
                            }
                        } else {
                            div { class: "bg-blue-50 border border-blue-100 rounded-xl p-3 mb-5 text-left text-sm text-gray-600",
                                p { class: "font-semibold text-gray-700 mb-1", "Yang akan terjadi:" }
                                ul { class: "list-disc list-inside space-y-0.5 text-xs",
                                    li { "Browser meminta izin notifikasi" }
                                    li { "Saat SOS ditekan, Anda langsung dapat notifikasi" }
                                    li { "Notifikasi berisi link lokasi Google Maps" }
                                }
                            }
                            button {
                                class: "w-full bg-blue-700 text-white rounded-2xl py-4 font-bold text-base hover:bg-blue-800 active:scale-95 transition",
                                onclick: handle_subscribe,
                                "Hubungkan & Aktifkan Notifikasi"
                            }
                            p { class: "text-xs text-gray-400 mt-3",
                                "Notifikasi hanya dikirim saat SOS ditekan. Privasi Anda terjaga."
                            }
                        }
                    },

                    2 => rsx! {
                        div { class: "text-5xl mb-3", "✅" }
                        h2 { class: "text-xl font-bold text-green-700 mb-2", "Berhasil Terhubung!" }
                        p  { class: "text-gray-500 text-sm",
                            "Anda akan menerima notifikasi jika kontak menekan tombol SOS."
                        }
                    },

                    _ => rsx! {
                        div { class: "text-4xl mb-3", "❌" }
                        p { class: "text-red-600 font-medium", "{error_msg}" }
                    },
                }
            }
        }
    }
}
