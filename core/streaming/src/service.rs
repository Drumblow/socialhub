use uuid::Uuid;
use crate::models::{Stream, StreamType, StreamStatus};
use crate::error::StreamingError;

#[derive(Default)]
pub struct StreamingService;

impl StreamingService {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn start_stream(&self, user_id: i32, stream_type: StreamType) -> Result<Stream, StreamingError> {
        let id = Uuid::new_v4();
        Ok(Stream {
            id,
            user_id,
            stream_type,
            status: StreamStatus::Active,
            url: format!("/stream/{}", id),
        })
    }

    pub async fn stop_stream(&self, _stream_id: Uuid) -> Result<(), StreamingError> {
        // Simulated stream stopping
        Ok(())
    }

    pub async fn get_stream(&self, _id: Uuid) -> Result<Stream, StreamingError> {
        // Simulação - em produção buscaria do banco de dados
        Err(StreamingError::NotFound)
    }
}
