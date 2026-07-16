use axum::{extract::State, Json};
use chrono::Utc;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

    let bbox = bounding_box(&payload.waypoints, SEARCH_RADIUS_M);
    let candidates = sqlx::query_as::<_, Report>(
        r#"
        SELECT id, category::text AS category, lat, lng, note, device_hash,
               created_at, upvote_count, downvote_count, status
        FROM reports
        WHERE lat BETWEEN $1 AND $2
          AND lng BETWEEN $3 AND $4
          AND status = 'active'
          AND created_at > NOW() - INTERVAL '30 days'
        "#,
    )
    .bind(bbox.min_lat)
    .bind(bbox.max_lat)
    .bind(bbox.min_lng)
    .bind(bbox.max_lng)
    .fetch_all(&state.db)
    .await?;

    let all_reports: Vec<Report> = candidates
        .into_iter()
        .filter(|report| {
            distance_to_polyline_m(report.lat, report.lng, &payload.waypoints) <= SEARCH_RADIUS_M
        })
        .collect();

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

#[derive(Debug)]
struct BoundingBox {
    min_lat: f64,
    max_lat: f64,
    min_lng: f64,
    max_lng: f64,
}

fn bounding_box(waypoints: &[Waypoint], margin_m: f64) -> BoundingBox {
    let min_lat = waypoints
        .iter()
        .map(|wp| wp.lat)
        .fold(f64::INFINITY, f64::min);
    let max_lat = waypoints
        .iter()
        .map(|wp| wp.lat)
        .fold(f64::NEG_INFINITY, f64::max);
    let min_lng = waypoints
        .iter()
        .map(|wp| wp.lng)
        .fold(f64::INFINITY, f64::min);
    let max_lng = waypoints
        .iter()
        .map(|wp| wp.lng)
        .fold(f64::NEG_INFINITY, f64::max);
    let avg_lat = ((min_lat + max_lat) / 2.0).to_radians();
    let lat_margin = margin_m / 111_320.0;
    let lng_margin = margin_m / (111_320.0 * avg_lat.cos().abs().max(0.01));

    BoundingBox {
        min_lat: min_lat - lat_margin,
        max_lat: max_lat + lat_margin,
        min_lng: min_lng - lng_margin,
        max_lng: max_lng + lng_margin,
    }
}

fn distance_to_polyline_m(lat: f64, lng: f64, waypoints: &[Waypoint]) -> f64 {
    match waypoints {
        [] => f64::INFINITY,
        [only] => haversine_m(lat, lng, only.lat, only.lng),
        points => points
            .windows(2)
            .map(|segment| distance_to_segment_m(lat, lng, &segment[0], &segment[1]))
            .fold(f64::INFINITY, f64::min),
    }
}

fn distance_to_segment_m(lat: f64, lng: f64, a: &Waypoint, b: &Waypoint) -> f64 {
    let origin_lat = ((lat + a.lat + b.lat) / 3.0).to_radians();
    let meters_per_deg_lat = 111_320.0;
    let meters_per_deg_lng = 111_320.0 * origin_lat.cos().abs().max(0.01);

    let px = (lng - a.lng) * meters_per_deg_lng;
    let py = (lat - a.lat) * meters_per_deg_lat;
    let bx = (b.lng - a.lng) * meters_per_deg_lng;
    let by = (b.lat - a.lat) * meters_per_deg_lat;
    let len_sq = bx * bx + by * by;

    if len_sq <= f64::EPSILON {
        return (px * px + py * py).sqrt();
    }

    let t = ((px * bx + py * by) / len_sq).clamp(0.0, 1.0);
    let dx = px - t * bx;
    let dy = py - t * by;

    (dx * dx + dy * dy).sqrt()
}

fn haversine_m(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let radius = 6_371_000.0_f64;
    let d_lat = (lat2 - lat1).to_radians();
    let d_lng = (lng2 - lng1).to_radians();
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let a = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lng / 2.0).sin().powi(2);
    radius * 2.0 * a.sqrt().atan2((1.0 - a).sqrt())
}

fn make_cache_key(waypoints: &[Waypoint]) -> String {
    let mut h = Sha256::new();
    for wp in waypoints {
        h.update(format!("{:.4},{:.4};", wp.lat, wp.lng));
    }
    format!("route_score:{}", hex::encode(h.finalize()))
}
