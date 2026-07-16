use axum::{extract::State, Json};
use chrono::Utc;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

use crate::{error::AppError, models::report::Report, AppState};

const SEARCH_RADIUS_M: f64 = 50.0;
const CACHE_TTL_SECS: u64 = 600;

#[derive(Debug, Deserialize)]
pub struct RouteScorePayload {
    pub waypoints: Vec<Waypoint>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Waypoint {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteScoreResponse {
    pub score: f64,
    /// "Aman" | "Waspada" | "Hindari"
    pub level: String,
    pub report_count: usize,
    pub cache_hit: bool,
}

pub async fn calculate_route_score(
    State(state): State<AppState>,
    Json(payload): Json<RouteScorePayload>,
) -> Result<Json<RouteScoreResponse>, AppError> {
    if payload.waypoints.is_empty() {
        return Err(AppError::BadRequest("Diperlukan minimal 1 waypoint".into()));
    }

    let cache_key = make_cache_key(&payload.waypoints);

    let mut redis_conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // Try cache first
    if let Ok(cached) = redis_conn.get::<_, String>(&cache_key).await {
        if let Ok(mut resp) = serde_json::from_str::<RouteScoreResponse>(&cached) {
            resp.cache_hit = true;
            return Ok(Json(resp));
        }
    }

    // Query reports near each waypoint, dedup by ID
    let mut seen_ids: HashSet<uuid::Uuid> = HashSet::new();
    let mut all_reports: Vec<Report> = Vec::new();

    for wp in &payload.waypoints {
        let nearby = sqlx::query_as::<_, Report>(
            r#"
            SELECT id, category::text AS category, lat, lng, note, device_hash,
                   created_at, upvote_count, downvote_count, status
            FROM reports
            WHERE earth_distance(ll_to_earth(lat, lng), ll_to_earth($1, $2)) <= $3
              AND status = 'active'
              AND created_at > NOW() - INTERVAL '30 days'
            "#,
        )
        .bind(wp.lat)
        .bind(wp.lng)
        .bind(SEARCH_RADIUS_M)
        .fetch_all(&state.db)
        .await?;

        for r in nearby {
            if seen_ids.insert(r.id) {
                all_reports.push(r);
            }
        }
    }

    let (score, level) = compute_score(&all_reports);

    let resp = RouteScoreResponse {
        score,
        level,
        report_count: all_reports.len(),
        cache_hit: false,
    };

    // Store in cache
    if let Ok(json) = serde_json::to_string(&resp) {
        let _: Result<(), _> = redis_conn.set_ex(&cache_key, json, CACHE_TTL_SECS).await;
    }

    Ok(Json(resp))
}

fn compute_score(reports: &[Report]) -> (f64, String) {
    let now = Utc::now();
    let mut score = 0.0f64;

    for r in reports {
        let age_days = (now - r.created_at).num_days().max(0) as f64;
        // Linear decay: weight 1.0 at day 0, 0.1 at day 30
        let recency = (1.0 - (age_days / 30.0) * 0.9).max(0.1);

        let category_weight = match r.category.as_str() {
            "crime" => 3.0,
            "accident" => 2.0,
            "lighting" => 1.0,
            _ => 1.0,
        };

        let community = 1.0 + (r.upvote_count.max(0) as f64 * 0.2);

        score += recency * category_weight * community;
    }

    let level = if score < 5.0 {
        "Aman"
    } else if score < 15.0 {
        "Waspada"
    } else {
        "Hindari"
    };

    (score, level.to_string())
}

fn make_cache_key(waypoints: &[Waypoint]) -> String {
    let mut h = Sha256::new();
    for wp in waypoints {
        h.update(format!("{:.4},{:.4};", wp.lat, wp.lng));
    }
    format!("route_score:{}", hex::encode(h.finalize()))
}
