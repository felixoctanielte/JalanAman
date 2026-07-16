#![allow(non_snake_case)]

use dioxus::prelude::*;

// Daftarkan modul agar contacts.rs tidak error compile
pub mod app;
pub mod services;
pub mod utils;

pub mod pages {
    pub mod home; // Bisa dibiarkan kosong / tidak dipakai
    pub mod dashboard;
    pub mod contacts;
}

fn main() {
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();
    #[cfg(feature = "web")]
    console_log::init_with_level(log::Level::Debug).unwrap();

    dioxus::launch(app::App);
}