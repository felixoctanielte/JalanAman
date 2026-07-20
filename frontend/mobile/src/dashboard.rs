use dioxus::prelude::*;

use crate::app::{Home, Route};
use crate::app_config::{CopyKey, Language};
use crate::theme::*;

#[component]
pub(crate) fn Dashboard() -> Element {
    rsx! {
        main { style: APP,
            section { style: DASHBOARD_WRAP,
                Link { to: Route::Home {}, style: BACK_LINK, "{Language::Indonesian.text(CopyKey::DashboardBack)}" }
                h1 { style: DASH_TITLE, "{Language::Indonesian.text(CopyKey::DashboardTitle)}" }
                div { style: CARD,
                    div { style: EYEBROW, "{Language::Indonesian.text(CopyKey::DashboardNoteEyebrow)}" }
                    div { style: TITLE, "{Language::Indonesian.text(CopyKey::DashboardNoteTitle)}" }
                    div { style: BODY, "{Language::Indonesian.text(CopyKey::DashboardNoteBody)}" }
                }
            }
        }
    }
}

#[component]
pub(crate) fn Fallback(segments: Vec<String>) -> Element {
    let _ = segments;
    rsx! { Home {} }
}
