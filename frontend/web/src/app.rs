use dioxus::prelude::*;

use crate::pages::{contacts::Contacts, dashboard::Dashboard, home::Home, invite::Invite};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/contacts")]
    Contacts {},
    #[route("/invite/:token")]
    Invite { token: String },
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}
