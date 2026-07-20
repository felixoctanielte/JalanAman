mod app;
mod app_config;
mod dashboard;
mod header;
mod map;
mod models;
mod navigation;
mod platform;
mod screens;
mod services;
mod theme;
mod utils;
mod views;

fn main() {
    dioxus::launch(app::App);
}
