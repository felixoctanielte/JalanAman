//! HTTP calls to the JalanAman backend — web platform (gloo-net / fetch).
use gloo_net::http::Request;
use jalanaman_shared::*;

const BASE: &str = "/api";

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct HeatmapPoint {
    pub lat: f64,
    pub lng: f64,
    pub weight: f64,
    pub category: String,
    pub description: String,
}

pub async fn fetch_config() -> Result<PublicConfig, String> {
    req_get(&format!("{BASE}/config")).await
}

pub async fn get_reports(lat: f64, lng: f64, radius: f64) -> Result<Vec<Report>, String> {
    req_get(&format!(
        "{BASE}/reports?lat={lat}&lng={lng}&radius={radius}"
    ))
    .await
}

pub async fn get_directions(
    origin_lat: f64,
    origin_lng: f64,
    destination: &str,
    mode: &str,
) -> Result<DirectionsResponse, String> {
    let destination = urlencoding::encode(destination);
    req_get(&format!(
        "{BASE}/directions?origin_lat={origin_lat}&origin_lng={origin_lng}&destination={destination}&mode={mode}"
    ))
    .await
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

#[allow(dead_code)]
pub async fn upvote_report(id: &str) -> Result<Report, String> {
    Request::post(&format!("{BASE}/reports/{id}/upvote"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

#[allow(dead_code)]
pub async fn downvote_report(id: &str) -> Result<Report, String> {
    Request::post(&format!("{BASE}/reports/{id}/downvote"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn calculate_route_score(waypoints: Vec<Waypoint>) -> Result<RouteScoreResponse, String> {
    let resp = Request::post(&format!("{BASE}/route-score"))
        .json(&RouteScorePayload { waypoints })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn trigger_sos(
    device_hash: &str,
    lat: f64,
    lng: f64,
) -> Result<SosTriggerResponse, String> {
    let resp = Request::post(&format!("{BASE}/sos/trigger"))
        .json(&SosTriggerPayload {
            device_hash: device_hash.to_string(),
            lat,
            lng,
            message: Some("🆘 SOS! Saya butuh bantuan!".into()),
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() == 429 {
        return Err("SOS sudah dikirim. Tunggu 1 menit.".into());
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_contacts(device_hash: &str) -> Result<Vec<EmergencyContact>, String> {
    req_get(&format!("{BASE}/sos/contacts?device_hash={device_hash}")).await
}

pub async fn add_contact(
    device_hash: &str,
    name: &str,
    email: Option<String>,
) -> Result<EmergencyContact, String> {
    Request::post(&format!("{BASE}/sos/contacts"))
        .json(&AddContactPayload {
            device_hash: device_hash.to_string(),
            name: name.to_string(),
            email,
            phone: None,
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_contact(id: &str, device_hash: &str) -> Result<(), String> {
    Request::delete(&format!(
        "{BASE}/sos/contacts/{id}?device_hash={device_hash}"
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_invite_info(token: &str) -> Result<serde_json::Value, String> {
    req_get(&format!("{BASE}/sos/invite/{token}")).await
}

pub async fn subscribe_push_backend(p: &SubscribePushPayload) -> Result<(), String> {
    Request::post(&format!("{BASE}/sos/subscribe"))
        .json(p)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_heatmap_data() -> Result<Vec<HeatmapPoint>, String> {
    req_get(&format!("{BASE}/reports/heatmap")).await
}

// ── helpers ───────────────────────────────────────────────────────────────────

async fn req_get<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    Request::get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

