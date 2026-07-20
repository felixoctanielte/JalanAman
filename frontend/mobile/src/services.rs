use jalanaman_shared::{
    AddContactPayload, CreateReportPayload, DirectionsResponse, EmergencyContact, PlaceSuggestion,
    Report, RouteScorePayload, RouteScoreResponse, SosTriggerPayload, SosTriggerResponse, Waypoint,
};

use crate::app_config::app_spec;
use crate::models::GeoPoint;

pub(crate) async fn get_reports(point: GeoPoint) -> Result<Vec<Report>, String> {
    let client = reqwest::Client::new();
    request_json(client.get(api_url("/reports")).query(&[
        ("lat", point.lat.to_string()),
        ("lng", point.lng.to_string()),
        ("radius", app_spec().api.report_radius_m.to_string()),
    ]))
    .await
}

pub(crate) async fn create_report(payload: &CreateReportPayload) -> Result<Report, String> {
    let client = reqwest::Client::new();
    request_json(client.post(api_url("/reports")).json(payload)).await
}

pub(crate) async fn get_directions(
    point: GeoPoint,
    destination: &str,
) -> Result<DirectionsResponse, String> {
    let client = reqwest::Client::new();
    request_json(client.get(api_url("/directions")).query(&[
        ("origin_lat", point.lat.to_string()),
        ("origin_lng", point.lng.to_string()),
        ("destination", destination.to_string()),
        ("mode", "walking".to_string()),
    ]))
    .await
}

pub(crate) async fn search_places(
    query: &str,
    origin: Option<GeoPoint>,
) -> Result<Vec<PlaceSuggestion>, String> {
    let client = reqwest::Client::new();
    let mut parameters = vec![("q", query.to_string())];
    if let Some(origin) = origin {
        parameters.push(("lat", origin.lat.to_string()));
        parameters.push(("lng", origin.lng.to_string()));
    }

    request_json(client.get(api_url("/places")).query(&parameters)).await
}

pub(crate) async fn calculate_route_score(
    waypoints: Vec<Waypoint>,
) -> Result<RouteScoreResponse, String> {
    let client = reqwest::Client::new();
    request_json(
        client
            .post(api_url("/route-score"))
            .json(&RouteScorePayload { waypoints }),
    )
    .await
}

pub(crate) async fn get_contacts(device_hash: &str) -> Result<Vec<EmergencyContact>, String> {
    let client = reqwest::Client::new();
    request_json(
        client
            .get(api_url("/sos/contacts"))
            .query(&[("device_hash", device_hash.to_string())]),
    )
    .await
}

pub(crate) async fn add_contact(
    device_hash: &str,
    name: &str,
    email: Option<String>,
    phone: Option<String>,
) -> Result<EmergencyContact, String> {
    let client = reqwest::Client::new();
    request_json(
        client
            .post(api_url("/sos/contacts"))
            .json(&AddContactPayload {
                device_hash: device_hash.to_string(),
                name: name.to_string(),
                email,
                phone,
            }),
    )
    .await
}

pub(crate) async fn delete_contact(device_hash: &str, contact_id: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let _: serde_json::Value = request_json(
        client
            .delete(api_url(&format!("/sos/contacts/{contact_id}")))
            .query(&[("device_hash", device_hash.to_string())]),
    )
    .await?;
    Ok(())
}

pub(crate) async fn trigger_sos(
    device_hash: &str,
    point: GeoPoint,
) -> Result<SosTriggerResponse, String> {
    let client = reqwest::Client::new();
    request_json(
        client
            .post(api_url("/sos/trigger"))
            .json(&SosTriggerPayload {
                device_hash: device_hash.to_string(),
                lat: point.lat,
                lng: point.lng,
                message: Some("SOS JalanAman: saya butuh bantuan sekarang.".to_string()),
            }),
    )
    .await
}

async fn request_json<T>(request: reqwest::RequestBuilder) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let response = request.send().await.map_err(|_| {
        "Tidak dapat terhubung. Periksa koneksi internet lalu coba lagi.".to_string()
    })?;
    let status = response.status();

    if status.is_success() {
        return response
            .json::<T>()
            .await
            .map_err(|_| "Data belum dapat dimuat. Coba lagi sebentar.".to_string());
    }

    let _ = response.text().await;
    let _ = status;
    Err("Permintaan belum dapat diproses. Coba lagi sebentar.".to_string())
}

fn api_url(path: &str) -> String {
    format!("{}{}", api_base().trim_end_matches('/'), path)
}

fn api_base() -> &'static str {
    option_env!("JALANAMAN_API_BASE_URL").unwrap_or(app_spec().api.default_base)
}
