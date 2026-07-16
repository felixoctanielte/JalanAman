mod app;
mod components;
mod hooks;
mod pages;
mod services;
mod utils;

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(app::App);
}
