//! Google Maps component — web-only (uses JS interop).
use crate::utils::js;
use dioxus::prelude::*;
use jalanaman_shared::Report;

#[component]
pub fn MapView(reports: Vec<Report>) -> Element {
    use_effect(move || {
        js::clear_markers();
        for r in &reports {
            js::add_report_marker(
                &r.id,
                r.lat,
                r.lng,
                &r.category,
                r.note.as_deref().unwrap_or(""),
            );
        }
    });

    rsx! {
        div {
            id: "map-container",
            class: "w-full h-full",
        }
    }
}
