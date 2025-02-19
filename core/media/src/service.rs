use actix_multipart::Multipart;
use chrono::Utc;
use uuid::Uuid;
use crate::models::Media;

pub struct MediaService;

impl MediaService {
    pub async fn upload_media(_payload: Multipart) -> Result<Media, String> {
        // Simulação de upload bem-sucedido
        let now = Utc::now();
        Ok(Media {
            id: Uuid::new_v4(),
            user_id: 1,
            file_type: "image/jpeg".to_string(),
            url: "https://example.com/media/test.jpg".to_string(),
            description: Some("Test image".to_string()),
            created_at: now,
            updated_at: now,
        })
    }
}
