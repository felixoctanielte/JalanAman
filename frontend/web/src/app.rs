use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::{dashboard::Dashboard, home::Home, invite::Invite};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/invite/:token")]
    Invite { token: String },
}

pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}
