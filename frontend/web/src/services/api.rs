//! HTTP calls to the JalanAman backend — web platform (gloo-net / fetch).
use gloo_net::http::Request;
use jalanaman_shared::*;

const BASE: &str = "/api";

pub async fn fetch_config() -> Result<PublicConfig, String> {
    req_get(&format!("{BASE}/config")).await
}

pub async fn get_reports(lat: f64, lng: f64, radius: f64) -> Result<Vec<Report>, String> {
    req_get(&format!("{BASE}/reports?lat={lat}&lng={lng}&radius={radius}")).await
}

pub async fn create_report(p: &CreateReportPayload) -> Result<Report, String> {
    let resp = Request::post(&format!("{BASE}/reports"))
        .json(p)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() == 429 {
        return Err("Tunggu 10 menit sebelum melapor di lokasi yang sama.".into());
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn upvote_report(id: &str) -> Result<Report, String> {
    Request::post(&format!("{BASE}/reports/{id}/upvote"))
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())
}

pub async fn downvote_report(id: &str) -> Result<Report, String> {
    Request::post(&format!("{BASE}/reports/{id}/downvote"))
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())
}

pub async fn calculate_route_score(waypoints: Vec<Waypoint>) -> Result<RouteScoreResponse, String> {
    let resp = Request::post(&format!("{BASE}/route-score"))
        .json(&RouteScorePayload { waypoints })
        .map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn trigger_sos(device_hash: &str, lat: f64, lng: f64) -> Result<SosTriggerResponse, String> {
    let resp = Request::post(&format!("{BASE}/sos/trigger"))
        .json(&SosTriggerPayload {
            device_hash: device_hash.to_string(),
            lat,
            lng,
            message: Some("🆘 SOS! Saya butuh bantuan!".into()),
        })
        .map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?;
    if resp.status() == 429 {
        return Err("SOS sudah dikirim. Tunggu 1 menit.".into());
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_contacts(device_hash: &str) -> Result<Vec<EmergencyContact>, String> {
    req_get(&format!("{BASE}/sos/contacts?device_hash={device_hash}")).await
}

pub async fn add_contact(device_hash: &str, name: &str) -> Result<EmergencyContact, String> {
    Request::post(&format!("{BASE}/sos/contacts"))
        .json(&AddContactPayload {
            device_hash: device_hash.to_string(),
            name: name.to_string(),
        })
        .map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())
}

pub async fn get_invite_info(token: &str) -> Result<serde_json::Value, String> {
    req_get(&format!("{BASE}/sos/invite/{token}")).await
}

pub async fn subscribe_push_backend(p: &SubscribePushPayload) -> Result<(), String> {
    Request::post(&format!("{BASE}/sos/subscribe"))
        .json(p).map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?;
    Ok(())
}

// ── helpers ───────────────────────────────────────────────────────────────────

async fn req_get<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    Request::get(url)
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())
}
