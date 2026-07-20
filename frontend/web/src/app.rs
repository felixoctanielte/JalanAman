use crate::pages::contacts::Contacts;
use crate::pages::dashboard::Dashboard;
use crate::pages::home::Home;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

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
