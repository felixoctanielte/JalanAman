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
        AddContactPayload, DeleteContactParams, EmergencyContact, GetContactsParams,
        SubscribePushPayload,
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
    pub email_sent: bool,
    pub error: Option<String>,
}

pub async fn trigger_sos(
    State(state): State<AppState>,
    Json(payload): Json<SosTriggerPayload>,
) -> Result<Json<SosTriggerResponse>, AppError> {
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

    let contacts = sqlx::query_as::<_, EmergencyContact>(
        r#"
        SELECT id, device_hash, name, email, phone, contact_device_hash, push_endpoint,
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
        payload
            .message
            .as_deref()
            .unwrap_or("SOS! Saya butuh bantuan!"),
        payload.lat,
        payload.lng,
    );

    for contact in &contacts {
        let mut push_sent = false;
        let mut email_sent = false;
        let mut error_msg: Option<String> = None;

        // Try push notification first
        if let (Some(ep), Some(p256dh), Some(auth)) = (
            &contact.push_endpoint,
            &contact.push_p256dh,
            &contact.push_auth,
        ) {
            match send_web_push(
                &state.config.vapid_private_key_pem,
                ep,
                p256dh,
                auth,
                &body_text,
            )
            .await
            {
                Ok(_) => {
                    push_sent = true;
                    notified_count += 1;
                }
                Err(e) => error_msg = Some(format!("push: {e}")),
            }
        }

        // Fall back to email if push failed or not configured
        if !push_sent {
            if let Some(email_addr) = &contact.email {
                if !email_addr.is_empty() {
                    match send_email_alert(
                        &state.config.smtp_host,
                        state.config.smtp_port,
                        &state.config.smtp_user,
                        &state.config.smtp_pass,
                        &state.config.smtp_from,
                        email_addr,
                        &contact.name,
                        &body_text,
                    )
                    .await
                    {
                        Ok(_) => {
                            email_sent = true;
                            notified_count += 1;
                        }
                        Err(e) => {
                            let msg = format!("email: {e}");
                            error_msg = Some(match error_msg {
                                Some(prev) => format!("{prev}; {msg}"),
                                None => msg,
                            });
                        }
                    }
                }
            }
        }

        results.push(ContactNotifyResult {
            name: contact.name.clone(),
            connected: contact.push_endpoint.is_some()
                || contact.email.is_some()
                || contact.phone.is_some(),
            push_sent,
            email_sent,
            error: error_msg,
        });
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

#[allow(clippy::too_many_arguments)]
async fn send_email_alert(
    smtp_host: &str,
    smtp_port: u16,
    smtp_user: &str,
    smtp_pass: &str,
    smtp_from: &str,
    to_email: &str,
    contact_name: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use lettre::{
        message::{Attachment, MultiPart, SinglePart},
        transport::smtp::authentication::Credentials,
        AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    };

    const LOGO: &[u8] = include_bytes!("../../assets/jalanaman-logo-email.png");
    let escaped_body = escape_html(body).replace('\n', "<br>");
    let maps_url = body
        .split_whitespace()
        .find(|part| part.starts_with("https://maps.google.com/"))
        .unwrap_or("https://maps.google.com/");
    let escaped_maps_url = escape_html(maps_url);
    let escaped_contact_name = escape_html(contact_name);
    let plain_body = format!(
        "JALANAMAN_SOS_TRIGGER=1\n\nPemberitahuan SOS JalanAman\n\n{body}\n\nHalo {contact_name}, Anda menerima email ini karena alamat ini didaftarkan sebagai kontak darurat di JalanAman. JalanAman tidak meminta password, OTP, atau pembayaran melalui email ini. Jika situasi darurat, segera hubungi orang terkait atau layanan darurat 112."
    );
    let html_body = format!(
        r#"<!doctype html>
<html lang="id"><body style="margin:0;background:#f1f5f9;color:#0f172a;font-family:Arial,Helvetica,sans-serif;">
<table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:#f1f5f9;padding:24px 12px;"><tr><td align="center">
<table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="max-width:600px;background:#ffffff;border:1px solid #dbeafe;border-radius:14px;overflow:hidden;">
<tr><td style="padding:24px;text-align:center;background:#eff6ff;border-bottom:1px solid #dbeafe;">
<img src="cid:jalanaman-logo" width="72" height="72" alt="Logo JalanAman" style="display:block;margin:0 auto 10px;width:72px;height:72px;">
<div style="font-size:22px;font-weight:700;color:#0f3d91;">JalanAman</div>
<div style="margin-top:5px;font-size:13px;color:#475569;">Pemberitahuan untuk kontak darurat terdaftar</div>
</td></tr>
<tr><td style="padding:24px;">
<h1 style="margin:0 0 14px;font-size:22px;line-height:1.3;color:#b91c1c;">Permintaan bantuan SOS</h1>
<p style="margin:0 0 18px;font-size:15px;line-height:1.65;color:#334155;">{escaped_body}</p>
<table role="presentation" cellspacing="0" cellpadding="0"><tr><td style="border-radius:8px;background:#1d4ed8;"><a href="{escaped_maps_url}" style="display:inline-block;padding:12px 18px;color:#ffffff;text-decoration:none;font-size:14px;font-weight:700;">Buka lokasi di Google Maps</a></td></tr></table>
<p style="margin:14px 0 0;font-size:12px;line-height:1.5;color:#64748b;word-break:break-all;">Alamat lokasi: {escaped_maps_url}</p>
<div style="margin-top:22px;padding:14px;border:1px solid #dbeafe;border-radius:8px;background:#f8fbff;font-size:13px;line-height:1.6;color:#475569;">Halo {escaped_contact_name}, email ini dikirim karena alamat Anda didaftarkan sebagai kontak darurat di JalanAman. Kami tidak meminta password, OTP, data pembayaran, atau balasan berisi informasi pribadi.</div>
<p style="margin:20px 0 0;font-size:13px;line-height:1.6;color:#334155;">Jika ini benar-benar darurat, segera hubungi orang terkait atau layanan darurat <strong>112</strong>.</p>
</td></tr>
<tr><td style="padding:16px 24px;border-top:1px solid #e2e8f0;background:#f8fafc;font-size:11px;line-height:1.5;color:#64748b;">Dikirim otomatis oleh JalanAman. Tidak ada pelacak iklan atau lampiran yang perlu dibuka.</td></tr>
</table></td></tr></table></body></html>"#
    );

    let email = Message::builder()
        .from(smtp_from.parse()?)
        .to(format!("{contact_name} <{to_email}>").parse()?)
        .subject("[JALANAMAN-SOS] Pemberitahuan darurat untuk kontak terdaftar")
        .multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(plain_body))
                .multipart(
                    MultiPart::related()
                        .singlepart(SinglePart::html(html_body))
                        .singlepart(
                            Attachment::new_inline("jalanaman-logo".to_string())
                                .body(LOGO.to_vec(), "image/png".parse()?),
                        ),
                ),
        )?;

    let creds = Credentials::new(smtp_user.to_string(), smtp_pass.to_string());
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(smtp_host)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    mailer.send(email).await?;
    Ok(())
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// ── Add emergency contact ─────────────────────────────────────────────────────

pub async fn add_contact(
    State(state): State<AppState>,
    Json(payload): Json<AddContactPayload>,
) -> Result<Json<EmergencyContact>, AppError> {
    let invite_token = Uuid::new_v4().to_string();

    let contact = sqlx::query_as::<_, EmergencyContact>(
        r#"
        INSERT INTO emergency_contacts (device_hash, name, email, phone, invite_token)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, device_hash, name, email, phone, contact_device_hash, push_endpoint,
                  push_p256dh, push_auth, invite_token, created_at
        "#,
    )
    .bind(&payload.device_hash)
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.phone)
    .bind(&invite_token)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(contact))
}

// ── Delete emergency contact ──────────────────────────────────────────────────

pub async fn delete_contact(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<DeleteContactParams>,
) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query("DELETE FROM emergency_contacts WHERE id = $1 AND device_hash = $2")
        .bind(id)
        .bind(&params.device_hash)
        .execute(&state.db)
        .await?
        .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(json!({ "deleted": true })))
}

// ── Get emergency contacts ────────────────────────────────────────────────────

pub async fn get_contacts(
    State(state): State<AppState>,
    Query(params): Query<GetContactsParams>,
) -> Result<Json<Vec<EmergencyContact>>, AppError> {
    let contacts = sqlx::query_as::<_, EmergencyContact>(
        r#"
        SELECT id, device_hash, name, email, phone, contact_device_hash, push_endpoint,
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
        SELECT id, device_hash, name, email, phone, contact_device_hash, push_endpoint,
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
