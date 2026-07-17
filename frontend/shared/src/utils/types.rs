use serde::{Deserialize, Serialize};

// ── Domain types (backend ↔ frontend contract) ────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Report {
    pub id: String,
    pub category: String,
    pub lat: f64,
    pub lng: f64,
    pub note: Option<String>,
    pub device_hash: String,
    pub created_at: String,
    pub upvote_count: i32,
    pub downvote_count: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReportPayload {
    pub category: String,
    pub lat: f64,
    pub lng: f64,
    pub note: Option<String>,
    pub device_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Waypoint {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteScorePayload {
    pub waypoints: Vec<Waypoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectionsResponse {
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub waypoints: Vec<Waypoint>,
    pub polyline: Vec<Waypoint>,
    pub distance_m: f64,
    pub duration_s: f64,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlaceSuggestion {
    pub name: String,
    pub subtitle: String,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouteScoreResponse {
    pub score: f64,
    pub level: String, // "Aman" | "Waspada" | "Hindari"
    pub report_count: usize,
    pub cache_hit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SosTriggerPayload {
    pub device_hash: String,
    pub lat: f64,
    pub lng: f64,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SosTriggerResponse {
    pub notified_count: usize,
    pub total_contacts: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmergencyContact {
    pub id: String,
    pub device_hash: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub contact_device_hash: Option<String>,
    pub push_endpoint: Option<String>,
    pub push_p256dh: Option<String>,
    pub push_auth: Option<String>,
    pub invite_token: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddContactPayload {
    pub device_hash: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribePushPayload {
    pub invite_token: String,
    pub contact_device_hash: String,
    pub push_endpoint: String,
    pub push_p256dh: String,
    pub push_auth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicConfig {
    pub vapid_public_key: String,
    pub google_maps_api_key: String,
}

// ── Shared scoring preview (mirrors backend algorithm) ────────────────────────

pub fn safety_level_label(level: &str) -> &'static str {
    match level {
        "Aman" => "✅ Aman",
        "Waspada" => "⚠️ Waspada",
        _ => "🚫 Hindari",
    }
}

pub fn category_label(cat: &str) -> &'static str {
    match cat {
        "crime" => "Rawan Begal/Copet",
        "accident" => "Rawan Kecelakaan",
        "lighting" => "Pencahayaan Buruk",
        _ => "Lainnya",
    }
}

pub fn category_emoji(cat: &str) -> &'static str {
    match cat {
        "crime" => "🔴",
        "accident" => "🟠",
        "lighting" => "🟡",
        _ => "⚪",
    }
}
