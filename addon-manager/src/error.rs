use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AddonError {
    #[error("Failed to fetch manifest: {0}")]
    ManifestFetch(reqwest::Error),

    #[error("Invalid manifest data: {0}")]
    InvalidManifest(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Operation timeout")]
    Timeout,

    #[error("Failed to fetch streams: {0}")]
    StreamFetch(reqwest::Error),
    
    #[error("Failed to fetch catalog: {0}")]
    CatalogFetch(reqwest::Error),
    
    #[error("Invalid configuration: {0}")]
    Configuration(String),
}

impl From<reqwest::Error> for AddonError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AddonError::Timeout
        } else {
            AddonError::ManifestFetch(err)
        }
    }
}

impl ResponseError for AddonError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddonError::ManifestFetch(_) => HttpResponse::InternalServerError().json("Failed to fetch manifest"),
            AddonError::InvalidManifest(_) => HttpResponse::BadRequest().json("Invalid manifest data"),
            AddonError::CacheError(_) => HttpResponse::InternalServerError().json("Cache error"),
            AddonError::NotFound(_) => HttpResponse::NotFound().json("Resource not found"),
            AddonError::Timeout => HttpResponse::RequestTimeout().json("Operation timeout"),
            AddonError::StreamFetch(_) => HttpResponse::InternalServerError().json("Failed to fetch streams"),
            AddonError::CatalogFetch(_) => HttpResponse::InternalServerError().json("Failed to fetch catalog"),
            AddonError::Configuration(_) => HttpResponse::BadRequest().json("Invalid configuration"),
        }
    }
}
