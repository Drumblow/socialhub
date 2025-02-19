use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Post {
    pub id: Uuid,
    pub user_id: i32,
    pub content: String,
    pub media_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Like {
    pub id: Uuid,
    pub user_id: i32,
    pub post_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Follow {
    pub id: Uuid,
    pub follower_id: i32,
    pub following_id: i32,
    pub created_at: DateTime<Utc>,
}
