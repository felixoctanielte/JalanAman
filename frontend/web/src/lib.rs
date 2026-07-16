mod app;
mod components;
mod hooks;
mod pages;
mod services;
mod utils;

// Trunk calls this automatically via wasm-bindgen
pub fn main() {
    console_log::init_with_level(log::Level::Debug).ok();
    dioxus::launch(app::App);
}
