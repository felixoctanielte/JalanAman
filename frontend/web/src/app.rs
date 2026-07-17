use dioxus::prelude::*;
use dioxus_router::prelude::*;

// Kita harus import semua komponen yang dipakai di Route
use crate::pages::dashboard::Dashboard;
use crate::pages::home::Home; // <-- INI YANG KURANG TADI
use crate::pages::contacts::Contacts;

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Dashboard {},
    
    #[route("/home")]
    Home {},
    
    #[route("/contacts")]
    Contacts {},
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}