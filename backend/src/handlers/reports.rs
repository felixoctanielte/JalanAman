use axum::{
    extract::{Path, Query, State},
    Json,
};
use redis::AsyncCommands;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::report::{CreateReportPayload, GetReportsParams, Report},
    AppState,
};

const VALID_CATEGORIES: [&str; 4] = ["lighting", "crime", "accident", "other"];
const COOLDOWN_SECS: u64 = 600; // 10 minutes
const AUTO_HIDE_DOWNVOTES: i32 = 3;

pub async fn create_report(
    State(state): State<AppState>,
    Json(payload): Json<CreateReportPayload>,
) -> Result<Json<Report>, AppError> {
    if !VALID_CATEGORIES.contains(&payload.category.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Kategori tidak valid. Pilih salah satu: {:?}",
            VALID_CATEGORIES
        )));
    }

    if payload.note.as_ref().map_or(false, |n| n.len() > 100) {
        return Err(AppError::BadRequest("Catatan maksimal 100 karakter".into()));
    }

    // Rate limit: 1 report per device per 10 min at same location (±0.001° ≈ 100m)
    let mut redis_conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let cooldown_key = format!(
        "cooldown:{}:{:.3}:{:.3}",
        payload.device_hash, payload.lat, payload.lng
    );

    let exists: bool = redis_conn
        .exists(&cooldown_key)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if exists {
        return Err(AppError::TooManyRequests);
    }

    let report = sqlx::query_as::<_, Report>(
        r#"
        INSERT INTO reports (category, lat, lng, note, device_hash)
        VALUES ($1::report_category, $2, $3, $4, $5)
        RETURNING id, category::text AS category, lat, lng, note, device_hash,
                  created_at, upvote_count, downvote_count, status
        "#,
    )
    .bind(&payload.category)
    .bind(payload.lat)
    .bind(payload.lng)
    .bind(&payload.note)
    .bind(&payload.device_hash)
    .fetch_one(&state.db)
    .await?;

    let _: () = redis_conn
        .set_ex(&cooldown_key, 1u8, COOLDOWN_SECS)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(report))
}

pub async fn get_reports(
    State(state): State<AppState>,
    Query(params): Query<GetReportsParams>,
) -> Result<Json<Vec<Report>>, AppError> {
    let radius = params.radius.unwrap_or(500.0);

    let reports = sqlx::query_as::<_, Report>(
        r#"
        SELECT id, category::text AS category, lat, lng, note, device_hash,
               created_at, upvote_count, downvote_count, status
        FROM reports
        WHERE earth_distance(ll_to_earth(lat, lng), ll_to_earth($1, $2)) <= $3
          AND status = 'active'
          AND created_at > NOW() - INTERVAL '30 days'
        ORDER BY created_at DESC
        LIMIT 200
        "#,
    )
    .bind(params.lat)
    .bind(params.lng)
    .bind(radius)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(reports))
}

pub async fn upvote_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Report>, AppError> {
    let report = sqlx::query_as::<_, Report>(
        r#"
        UPDATE reports
        SET upvote_count = upvote_count + 1
        WHERE id = $1 AND status = 'active'
        RETURNING id, category::text AS category, lat, lng, note, device_hash,
                  created_at, upvote_count, downvote_count, status
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(report))
}

pub async fn downvote_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Report>, AppError> {
    let report = sqlx::query_as::<_, Report>(
        r#"
        UPDATE reports
        SET
            downvote_count = downvote_count + 1,
            status = CASE
                WHEN downvote_count + 1 >= $2 THEN 'hidden'
                ELSE status
            END
        WHERE id = $1
        RETURNING id, category::text AS category, lat, lng, note, device_hash,
                  created_at, upvote_count, downvote_count, status
        "#,
    )
    .bind(id)
    .bind(AUTO_HIDE_DOWNVOTES)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(report))
}
