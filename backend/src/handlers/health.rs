use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::AppState;

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "jalan-aman-backend",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

pub async fn get_public_config(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "vapid_public_key": state.config.vapid_public_key,
        "google_maps_api_key": state.config.google_maps_api_key,
    }))
}
