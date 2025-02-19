use serde::{Serialize, Deserialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    pub id: Uuid,
    pub user_id: i32,
    pub stream_type: StreamType,
    pub status: StreamStatus,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum StreamType {
    Video,
    Audio,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamStatus {
    Active,
    Inactive,
    Paused,
}
