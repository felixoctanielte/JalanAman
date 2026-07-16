use dioxus::prelude::*;
use jalanaman_shared::EmergencyContact;

use crate::{app::Route, services::api, utils::device::get_device_hash, utils::js};

fn app_origin() -> String {
    web_sys::window()
        .and_then(|w| w.location().origin().ok())
        .unwrap_or_else(|| "http://localhost:3000".to_string())
}

#[component]
pub fn Contacts() -> Element {
    let device_hash = get_device_hash();

    let mut contacts = use_signal(Vec::<EmergencyContact>::new);
    let mut loading = use_signal(|| true);
    let mut show_form = use_signal(|| false);
    let mut form_name = use_signal(String::new);
    let mut form_email = use_signal(String::new);
    let mut form_error = use_signal(|| Option::<String>::None);
    let mut form_saving = use_signal(|| false);
    let mut copied_id = use_signal(|| Option::<String>::None);

    let dh = device_hash.clone();
    use_effect(move || {
        let dh = dh.clone();
        spawn(async move {
            if let Ok(c) = api::get_contacts(&dh).await {
                contacts.set(c);
            }
            loading.set(false);
        });
    });

    let dh_add = device_hash.clone();
    let on_add = move |_: MouseEvent| {
        let name = form_name.read().trim().to_string();
        let email = {
            let e = form_email.read().trim().to_string();
            if e.is_empty() {
                None
            } else {
                Some(e)
            }
        };

        if name.is_empty() {
            form_error.set(Some("Nama tidak boleh kosong.".into()));
            return;
        }

        form_saving.set(true);
        form_error.set(None);
        let dh = dh_add.clone();
        spawn(async move {
            match api::add_contact(&dh, &name, email).await {
                Ok(c) => {
                    contacts.write().insert(0, c);
                    show_form.set(false);
                    form_name.set(String::new());
                    form_email.set(String::new());
                }
                Err(e) => form_error.set(Some(e)),
            }
            form_saving.set(false);
        });
    };

    let dh_del = device_hash.clone();
    let on_delete = move |id: String| {
        let dh = dh_del.clone();
        spawn(async move {
            if api::delete_contact(&id, &dh).await.is_ok() {
                contacts.write().retain(|c| c.id != id);
            }
        });
    };

    rsx! {
        div { class: "min-h-screen bg-gray-50",

            // ── Header ─────────────────────────────────────────────────────────
            div { class: "bg-blue-700 text-white px-4 py-3 flex items-center gap-3 shadow",
                Link { to: Route::Home {}, class: "text-blue-200 hover:text-white text-sm", "← Kembali" }
                h1 { class: "font-bold text-lg", "Kontak Darurat SOS" }
            }

            div { class: "p-4 max-w-lg mx-auto space-y-4",

                // Info banner
                div { class: "bg-blue-50 border border-blue-100 rounded-2xl px-4 py-3 text-sm text-blue-700",
                    p { class: "font-semibold mb-0.5", "Cara kerja" }
                    p { "Tambah kontak → kirim link undangan → kontak buka link di HP mereka → notifikasi push aktif." }
                    p { class: "mt-1 text-xs text-blue-500", "Kontak yang punya email akan tetap menerima alert via email meski belum buka link." }
                }

                // Add button / form
                if !*show_form.read() {
                    button {
                        class: "w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 rounded-2xl transition active:scale-95",
                        onclick: move |_| { show_form.set(true); form_error.set(None); },
                        "+ Tambah Kontak Darurat"
                    }
                } else {
                    div { class: "bg-white rounded-2xl shadow p-4 space-y-3",
                        h2 { class: "font-semibold text-gray-700", "Tambah Kontak Baru" }

                        div {
                            label { class: "block text-xs font-medium text-gray-500 mb-1", "Nama *" }
                            input {
                                r#type: "text",
                                placeholder: "Contoh: Ibu, Pak Budi",
                                class: "w-full border border-gray-200 rounded-xl px-3 py-2 text-sm outline-none focus:border-blue-400",
                                value: "{form_name}",
                                oninput: move |e| form_name.set(e.value()),
                            }
                        }

                        div {
                            label { class: "block text-xs font-medium text-gray-500 mb-1",
                                "Email (opsional — untuk backup jika push belum terhubung)"
                            }
                            input {
                                r#type: "email",
                                placeholder: "kontak@email.com",
                                class: "w-full border border-gray-200 rounded-xl px-3 py-2 text-sm outline-none focus:border-blue-400",
                                value: "{form_email}",
                                oninput: move |e| form_email.set(e.value()),
                            }
                        }

                        if let Some(err) = form_error.read().as_ref() {
                            p { class: "text-red-500 text-xs", "{err}" }
                        }

                        div { class: "flex gap-2",
                            button {
                                class: "flex-1 bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2.5 rounded-xl text-sm transition disabled:opacity-50",
                                disabled: *form_saving.read(),
                                onclick: on_add,
                                if *form_saving.read() { "Menyimpan..." } else { "Simpan" }
                            }
                            button {
                                class: "px-4 py-2.5 rounded-xl text-sm text-gray-500 hover:bg-gray-100 transition",
                                onclick: move |_| { show_form.set(false); form_error.set(None); },
                                "Batal"
                            }
                        }
                    }
                }

                // Loading state
                if *loading.read() {
                    div { class: "text-center text-gray-400 text-sm py-8", "Memuat kontak..." }
                } else if contacts.read().is_empty() {
                    div { class: "bg-white rounded-2xl shadow p-8 text-center",
                        div { class: "text-4xl mb-3", "📭" }
                        p { class: "text-gray-500 text-sm", "Belum ada kontak darurat." }
                        p { class: "text-gray-400 text-xs mt-1", "Tambah kontak agar bisa dihubungi saat SOS." }
                    }
                } else {
                    div { class: "space-y-3",
                        for contact in contacts.read().iter() {
                            ContactCard {
                                contact: contact.clone(),
                                copied_id: copied_id.read().clone(),
                                on_copy: move |id: String| {
                                    copied_id.set(Some(id));
                                    // Reset badge after 2s — we can't do setTimeout easily in Dioxus,
                                    // so just leave it; the badge is unobtrusive.
                                },
                                on_delete: {
                                    let on_delete = on_delete.clone();
                                    move |id: String| on_delete(id)
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ContactCard(
    contact: EmergencyContact,
    copied_id: Option<String>,
    on_copy: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    let push_ok = contact.push_endpoint.is_some();
    let has_email = contact
        .email
        .as_ref()
        .map(|e| !e.is_empty())
        .unwrap_or(false);
    let origin = app_origin();
    let invite_url = contact
        .invite_token
        .as_ref()
        .map(|t| format!("{origin}/invite/{t}"))
        .unwrap_or_default();

    let is_copied = copied_id.as_deref() == Some(contact.id.as_str());
    let contact_id_copy = contact.id.clone();
    let contact_id_delete = contact.id.clone();

    rsx! {
        div { class: "bg-white rounded-2xl shadow p-4",
            div { class: "flex items-start justify-between mb-3",
                div {
                    p { class: "font-semibold text-gray-800", "{contact.name}" }
                    if let Some(email) = &contact.email {
                        if !email.is_empty() {
                            p { class: "text-xs text-gray-400 mt-0.5", "{email}" }
                        }
                    }
                }
                button {
                    class: "text-red-400 hover:text-red-600 text-sm p-1 rounded-lg hover:bg-red-50 transition",
                    title: "Hapus kontak",
                    onclick: move |_| on_delete.call(contact_id_delete.clone()),
                    "✕"
                }
            }

            // Status badges
            div { class: "flex flex-wrap gap-2 mb-3",
                if push_ok {
                    span { class: "inline-flex items-center gap-1 bg-green-50 text-green-700 text-xs font-medium px-2.5 py-1 rounded-full",
                        "✅ Push terhubung"
                    }
                } else {
                    span { class: "inline-flex items-center gap-1 bg-amber-50 text-amber-700 text-xs font-medium px-2.5 py-1 rounded-full",
                        "⏳ Belum buka link undangan"
                    }
                }
                if has_email {
                    span { class: "inline-flex items-center gap-1 bg-blue-50 text-blue-600 text-xs font-medium px-2.5 py-1 rounded-full",
                        "📧 Email aktif"
                    }
                }
            }

            // Invite link
            if !push_ok {
                div { class: "bg-gray-50 rounded-xl p-3",
                    p { class: "text-xs text-gray-400 mb-1.5 font-medium", "Link undangan (kirim ke kontak)" }
                    div { class: "flex items-center gap-2",
                        p { class: "text-xs text-blue-600 break-all flex-1 font-mono select-all", "{invite_url}" }
                        button {
                            class: "flex-shrink-0 bg-blue-600 hover:bg-blue-700 text-white text-xs font-semibold px-3 py-1.5 rounded-lg transition",
                            onclick: move |_| {
                                js::copy_to_clipboard(&invite_url);
                                on_copy.call(contact_id_copy.clone());
                            },
                            if is_copied { "✓ Disalin" } else { "Salin" }
                        }
                    }
                }
            }
        }
    }
}
