use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmergencyContact {
    pub id: Uuid,
    pub device_hash: String,
    pub name: String,
    pub contact_device_hash: Option<String>,
    pub push_endpoint: Option<String>,
    pub push_p256dh: Option<String>,
    pub push_auth: Option<String>,
    pub invite_token: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddContactPayload {
    pub device_hash: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GetContactsParams {
    pub device_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct SubscribePushPayload {
    pub invite_token: String,
    pub contact_device_hash: String,
    pub push_endpoint: String,
    pub push_p256dh: String,
    pub push_auth: String,
}
