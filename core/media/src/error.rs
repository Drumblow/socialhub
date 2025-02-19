use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MediaError {
    #[error("Media not found")]
    NotFound,
    
    #[error("Upload error: {0}")]
    UploadError(String),
    
    #[error("Invalid media format")]
    InvalidFormat,
    
    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for MediaError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MediaError::NotFound => HttpResponse::NotFound().finish(),
            MediaError::UploadError(_) => HttpResponse::UnsupportedMediaType().finish(),
            MediaError::InvalidFormat => HttpResponse::UnsupportedMediaType().finish(),
            MediaError::InternalError => HttpResponse::InternalServerError().finish(),
        }
    }
}
