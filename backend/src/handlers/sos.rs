use axum::{
    extract::{Path, Query, State},
    Json,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::emergency_contact::{
        AddContactPayload, EmergencyContact, GetContactsParams, SubscribePushPayload,
    },
    AppState,
};

const SOS_RATE_LIMIT_SECS: u64 = 60;

// ── Trigger SOS ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SosTriggerPayload {
    pub device_hash: String,
    pub lat: f64,
    pub lng: f64,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SosTriggerResponse {
    pub notified_count: usize,
    pub total_contacts: usize,
    pub results: Vec<ContactNotifyResult>,
}

#[derive(Debug, Serialize)]
pub struct ContactNotifyResult {
    pub name: String,
    pub connected: bool,
    pub push_sent: bool,
    pub error: Option<String>,
}

pub async fn trigger_sos(
    State(state): State<AppState>,
    Json(payload): Json<SosTriggerPayload>,
) -> Result<Json<SosTriggerResponse>, AppError> {
    // Rate limit: 1 SOS per minute per device
    let mut redis_conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let rate_key = format!("sos_rate:{}", payload.device_hash);
    let exists: bool = redis_conn
        .exists(&rate_key)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if exists {
        return Err(AppError::TooManyRequests);
    }

    let _: () = redis_conn
        .set_ex(&rate_key, 1u8, SOS_RATE_LIMIT_SECS)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // Fetch emergency contacts
    let contacts = sqlx::query_as::<_, EmergencyContact>(
        r#"
        SELECT id, device_hash, name, contact_device_hash, push_endpoint,
               push_p256dh, push_auth, invite_token, created_at
        FROM emergency_contacts
        WHERE device_hash = $1
        ORDER BY created_at ASC
        "#,
    )
    .bind(&payload.device_hash)
    .fetch_all(&state.db)
    .await?;

    let total_contacts = contacts.len();
    let mut results: Vec<ContactNotifyResult> = Vec::new();
    let mut notified_count = 0usize;

    let body_text = format!(
        "{}\nLokasi: https://maps.google.com/?q={},{}",
        payload.message.as_deref().unwrap_or("SOS! Saya butuh bantuan!"),
        payload.lat,
        payload.lng,
    );

    for contact in &contacts {
        match (&contact.push_endpoint, &contact.push_p256dh, &contact.push_auth) {
            (Some(ep), Some(p256dh), Some(auth)) => {
                let res = send_web_push(
                    &state.config.vapid_private_key_pem,
                    ep,
                    p256dh,
                    auth,
                    &body_text,
                )
                .await;

                match res {
                    Ok(_) => {
                        notified_count += 1;
                        results.push(ContactNotifyResult {
                            name: contact.name.clone(),
                            connected: true,
                            push_sent: true,
                            error: None,
                        });
                    }
                    Err(e) => {
                        results.push(ContactNotifyResult {
                            name: contact.name.clone(),
                            connected: true,
                            push_sent: false,
                            error: Some(e.to_string()),
                        });
                    }
                }
            }
            _ => {
                results.push(ContactNotifyResult {
                    name: contact.name.clone(),
                    connected: false,
                    push_sent: false,
                    error: Some("Kontak belum menghubungkan device-nya".into()),
                });
            }
        }
    }

    Ok(Json(SosTriggerResponse {
        notified_count,
        total_contacts,
        results,
    }))
}

async fn send_web_push(
    vapid_pem: &str,
    endpoint: &str,
    p256dh: &str,
    auth: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::io::Cursor;
    use web_push::{
        ContentEncoding, IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder,
        WebPushClient, WebPushMessageBuilder,
    };

    let subscription = SubscriptionInfo::new(endpoint, p256dh, auth);

    let sig = VapidSignatureBuilder::from_pem(Cursor::new(vapid_pem.as_bytes()), &subscription)?
        .build()?;

    let mut builder = WebPushMessageBuilder::new(&subscription);
    builder.set_payload(ContentEncoding::Aes128Gcm, body.as_bytes());
    builder.set_vapid_signature(sig);
    builder.set_ttl(60);

    let client = IsahcWebPushClient::new()?;
    client.send(builder.build()?).await?;

    Ok(())
}

// ── Add emergency contact ─────────────────────────────────────────────────────

pub async fn add_contact(
    State(state): State<AppState>,
    Json(payload): Json<AddContactPayload>,
) -> Result<Json<EmergencyContact>, AppError> {
    let invite_token = Uuid::new_v4().to_string();

    let contact = sqlx::query_as::<_, EmergencyContact>(
        r#"
        INSERT INTO emergency_contacts (device_hash, name, invite_token)
        VALUES ($1, $2, $3)
        RETURNING id, device_hash, name, contact_device_hash, push_endpoint,
                  push_p256dh, push_auth, invite_token, created_at
        "#,
    )
    .bind(&payload.device_hash)
    .bind(&payload.name)
    .bind(&invite_token)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(contact))
}

// ── Get emergency contacts ────────────────────────────────────────────────────

pub async fn get_contacts(
    State(state): State<AppState>,
    Query(params): Query<GetContactsParams>,
) -> Result<Json<Vec<EmergencyContact>>, AppError> {
    let contacts = sqlx::query_as::<_, EmergencyContact>(
        r#"
        SELECT id, device_hash, name, contact_device_hash, push_endpoint,
               push_p256dh, push_auth, invite_token, created_at
        FROM emergency_contacts
        WHERE device_hash = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(&params.device_hash)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(contacts))
}

// ── Subscribe push (called from invite link page) ─────────────────────────────

pub async fn subscribe_push(
    State(state): State<AppState>,
    Json(payload): Json<SubscribePushPayload>,
) -> Result<Json<Value>, AppError> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE emergency_contacts
        SET
            contact_device_hash = $1,
            push_endpoint       = $2,
            push_p256dh         = $3,
            push_auth           = $4
        WHERE invite_token = $5
        "#,
    )
    .bind(&payload.contact_device_hash)
    .bind(&payload.push_endpoint)
    .bind(&payload.push_p256dh)
    .bind(&payload.push_auth)
    .bind(&payload.invite_token)
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(json!({ "status": "subscribed" })))
}

// ── Get invite info (public – shown on invite link landing page) ──────────────

#[derive(Serialize)]
pub struct InviteInfo {
    pub contact_name: String,
    pub already_connected: bool,
}

pub async fn get_invite_info(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<Json<InviteInfo>, AppError> {
    let contact = sqlx::query_as::<_, EmergencyContact>(
        r#"
        SELECT id, device_hash, name, contact_device_hash, push_endpoint,
               push_p256dh, push_auth, invite_token, created_at
        FROM emergency_contacts
        WHERE invite_token = $1
        "#,
    )
    .bind(&token)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(InviteInfo {
        contact_name: contact.name,
        already_connected: contact.push_endpoint.is_some(),
    }))
}
