//! wasm_bindgen bindings to JS helpers defined in index.html.
//! Leaflet is loaded from CDN synchronously — no async waiting needed.
use wasm_bindgen::prelude::*;

// ── Map (Leaflet) ─────────────────────────────────────────────────────────────

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = ja_initMap)]
    pub fn init_map(lat: f64, lng: f64);

    #[wasm_bindgen(js_namespace = window, js_name = ja_addReportMarker)]
    pub fn add_report_marker(id: &str, lat: f64, lng: f64, category: &str, note: &str);

    #[wasm_bindgen(js_namespace = window, js_name = ja_clearMarkers)]
    pub fn clear_markers();

    #[wasm_bindgen(js_namespace = window, js_name = ja_drawRoutePolyline)]
    pub fn draw_route_polyline(waypoints_json: &str, level: &str);

    #[wasm_bindgen(js_namespace = window, js_name = ja_initHeatmap)]
    pub fn init_heatmap(points_json: &str);

    #[wasm_bindgen(js_namespace = window, js_name = ja_panTo)]
    pub fn pan_to(lat: f64, lng: f64);
}

// ── Directions (OSRM + Nominatim – no API key) ────────────────────────────────

#[wasm_bindgen]
extern "C" {
    /// Returns a Promise<string> resolving to JSON:
    /// `{ waypoints: [{lat,lng}], polyline: [{lat,lng}] }`
    #[wasm_bindgen(js_namespace = window, js_name = ja_getDirections)]
    pub fn get_directions(origin: &str, destination: &str, mode: &str) -> js_sys::Promise;
}

// ── SOS alarm ─────────────────────────────────────────────────────────────────

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = ja_playSOSAlarm)]
    pub fn play_sos_alarm();

    #[wasm_bindgen(js_namespace = window, js_name = ja_stopSOSAlarm)]
    pub fn stop_sos_alarm();
}

// ── Geolocation ───────────────────────────────────────────────────────────────

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = ja_getCurrentPosition)]
    pub fn get_current_position(callback: &js_sys::Function);
}

// ── Clipboard ─────────────────────────────────────────────────────────────────

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = ja_copyToClipboard)]
    pub fn copy_to_clipboard(text: &str);
}
