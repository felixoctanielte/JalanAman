//! Web Push subscription helpers — wraps JS helpers defined in index.html.
use super::api;
use crate::utils::device::get_device_hash;
use jalanaman_shared::SubscribePushPayload;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = ja_registerServiceWorker)]
    fn js_register_sw() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = window, js_name = ja_requestNotificationPermission)]
    fn js_request_permission() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = window, js_name = ja_subscribePush)]
    fn js_subscribe_push(vapid_key: &str) -> js_sys::Promise;
}

pub async fn register_service_worker() {
    let _ = JsFuture::from(js_register_sw()).await;
}

/// Full push onboarding: ask permission → subscribe → send sub to backend.
pub async fn onboard_push(invite_token: &str) -> Result<(), String> {
    // 1. Permission
    let perm = JsFuture::from(js_request_permission())
        .await
        .map(|v| v.as_string().unwrap_or_default())
        .unwrap_or_default();
    if perm != "granted" {
        return Err("Izin notifikasi diperlukan.".into());
    }

    // 2. VAPID key from backend
    let cfg = api::fetch_config().await?;

    // 3. Subscribe
    let sub_json = JsFuture::from(js_subscribe_push(&cfg.vapid_public_key))
        .await
        .map(|v| v.as_string().unwrap_or_default())
        .map_err(|e| format!("Push subscription gagal: {e:?}"))?;

    let sub: serde_json::Value = serde_json::from_str(&sub_json)
        .map_err(|_| "Format subscription tidak valid.".to_string())?;

    let payload = SubscribePushPayload {
        invite_token: invite_token.to_string(),
        contact_device_hash: get_device_hash(),
        push_endpoint: sub["endpoint"].as_str().unwrap_or("").to_string(),
        push_p256dh: sub["keys"]["p256dh"].as_str().unwrap_or("").to_string(),
        push_auth: sub["keys"]["auth"].as_str().unwrap_or("").to_string(),
    };

    api::subscribe_push_backend(&payload).await
}
