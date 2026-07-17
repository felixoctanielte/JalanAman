use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Report {
    pub id: Uuid,
    pub category: String,
    pub lat: f64,
    pub lng: f64,
    pub note: Option<String>,
    pub device_hash: String,
    pub created_at: DateTime<Utc>,
    pub upvote_count: i32,
    pub downvote_count: i32,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReportPayload {
    pub category: String,
    pub lat: f64,
    pub lng: f64,
    pub note: Option<String>,
    pub device_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct GetReportsParams {
    pub lat: f64,
    pub lng: f64,
    pub radius: Option<f64>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct HeatmapPoint {
    pub lat: f64,
    pub lng: f64,
    pub weight: f64,
    pub category: String,
    pub note: Option<String>,
}