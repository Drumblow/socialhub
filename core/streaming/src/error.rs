use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum StreamingError {
    #[error("Stream not found")]
    NotFound,
    
    #[error("Stream error: {0}")]
    StreamError(String),
    
    #[error("Invalid stream format")]
    InvalidFormat,
    
    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for StreamingError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::NotFound => HttpResponse::NotFound().finish(),
            Self::StreamError(msg) => HttpResponse::BadRequest().json(msg),
            Self::InvalidFormat => HttpResponse::UnsupportedMediaType().finish(),
            Self::InternalError => HttpResponse::InternalServerError().finish(),
        }
    }
}
