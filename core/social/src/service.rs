use crate::models::Post;
use uuid::Uuid;
use chrono::Utc;

pub struct SocialService {
    // Add fields later as needed
}

impl SocialService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_post(content: String, user_id: i32) -> Result<Post, String> {
        let now = Utc::now();
        Ok(Post {
            id: Uuid::new_v4(),
            content,
            user_id,
            media_ids: Vec::new(),  // Empty vector for new posts
            created_at: now,
            updated_at: now,
        })
    }
}
