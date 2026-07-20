#![allow(non_snake_case)]

use dioxus::prelude::*;

// 1. Daftarkan semua modul utama proyek
pub mod app;
pub mod components;
pub mod services;
pub mod utils;

// 2. Deklarasikan struktur modul halaman (pages) dengan benar
pub mod pages {
    pub mod contacts;
    pub mod dashboard;
    pub mod home;
}

// 3. Import halaman agar bisa dirender oleh Router
use pages::dashboard::Dashboard;
use pages::home::Home;

// 4. Definisikan Route resmi aplikasi web JalanAman di sini
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
}

fn main() {
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();
    #[cfg(feature = "web")]
    console_log::init_with_level(log::Level::Debug).unwrap();

    dioxus::launch(app::App);
}
