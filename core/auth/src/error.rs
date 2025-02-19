use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::AuthenticationError(_) => {
                HttpResponse::Unauthorized().json("Authentication failed")
            }
            AuthError::InternalError => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
        }
    }
}
