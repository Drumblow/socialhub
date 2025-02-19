use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddonError {
    #[error("Addon not found")]
    NotFound,
    
    #[error("Installation error: {0}")]
    InstallationError(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Sandbox violation: {0}")]
    SandboxViolation(String),
}

impl ResponseError for AddonError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddonError::NotFound => HttpResponse::NotFound().json("Addon not found"),
            AddonError::InstallationError(_) => HttpResponse::BadRequest().json("Installation failed"),
            AddonError::PermissionDenied(_) => HttpResponse::Forbidden().json("Permission denied"),
            AddonError::SandboxViolation(_) => HttpResponse::BadRequest().json("Sandbox violation"),
        }
    }
}
