//! Custom hook that wraps the browser Geolocation API.
use crate::utils::js::get_current_position;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

/// Returns a signal that resolves to `Some((lat, lng))` once the browser
/// provides a position. Stays `None` if geolocation is denied.
pub fn use_geolocation() -> Signal<Option<(f64, f64)>> {
    let mut location: Signal<Option<(f64, f64)>> = use_signal(|| None);

    use_effect(move || {
        let cb = Closure::once(move |lat: JsValue, lng: JsValue| {
            if let (Some(la), Some(lo)) = (lat.as_f64(), lng.as_f64()) {
                location.set(Some((la, lo)));
            }
        });
        get_current_position(cb.as_ref().unchecked_ref());
        cb.forget();
    });

    location
}
