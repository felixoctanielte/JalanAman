//! Persistent device identity (localStorage UUID, not hardware-linked).
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = ja_getDeviceHash)]
    pub fn get_device_hash() -> String;
}
