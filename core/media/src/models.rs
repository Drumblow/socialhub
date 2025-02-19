use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Media {
    pub id: Uuid,
    pub user_id: i32,
    pub file_type: String,
    pub url: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f64>,
    pub format: String,
}
