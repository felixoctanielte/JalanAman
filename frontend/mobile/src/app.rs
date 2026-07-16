//! Mobile app entry – shares UI components from jalanaman-shared,
//! uses native platform APIs for location, notifications, and audio.
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
}

pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}

// ── Pages (stubs – implement with native APIs during hackathon) ───────────────

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col h-screen",
            // TODO: replace with native map component (MapLibre / platform maps)
            div { class: "flex-1 bg-gray-200 flex items-center justify-center",
                p { class: "text-gray-500", "Native map renders here" }
            }
            // Shared SOS button + report form wired to native location / audio APIs
        }
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! {
        div { "Dashboard (native)" }
    }
}
