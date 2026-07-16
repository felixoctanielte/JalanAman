//! SOS button – pure RSX. Platform triggers alarm + push via `on_sos`.
use dioxus::prelude::*;

#[component]
pub fn SosButton(
    active: bool,
    status_msg: Option<String>,
    /// Called when user taps SOS. Payload = current (lat, lng) if known.
    on_sos: EventHandler<Option<(f64, f64)>>,
    on_stop: EventHandler<()>,
    location: Option<(f64, f64)>,
) -> Element {
    rsx! {
        div { class: "absolute bottom-6 right-4 flex flex-col items-end gap-2",

            if let Some(msg) = &status_msg {
                div {
                    class: "bg-white border border-gray-200 rounded-xl shadow-lg px-4 py-2 max-w-xs text-xs text-gray-700 text-right",
                    "{msg}"
                }
            }

            button {
                class: if active {
                    "sos-btn w-20 h-20 rounded-full bg-red-600 text-white font-black text-xl shadow-2xl flex flex-col items-center justify-center gap-0.5 active:scale-95"
                } else {
                    "w-20 h-20 rounded-full bg-red-600 text-white font-black text-xl shadow-2xl flex flex-col items-center justify-center gap-0.5 active:scale-95 hover:bg-red-700 transition-colors"
                },
                onclick: move |_| {
                    if active {
                        on_stop.call(());
                    } else {
                        on_sos.call(location);
                    }
                },
                span { "SOS" }
                if active {
                    span { class: "text-xs font-normal opacity-75", "tap stop" }
                }
            }
        }
    }
}
