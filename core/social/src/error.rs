use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SocialError {
    #[error("Post not found")]
    PostNotFound,

    #[error("User not found")]
    UserNotFound,

    #[error("Operation not permitted")]
    NotPermitted,

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for SocialError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SocialError::PostNotFound => HttpResponse::NotFound().json("Post not found"),
            SocialError::UserNotFound => HttpResponse::NotFound().json("User not found"),
            SocialError::NotPermitted => HttpResponse::Forbidden().json("Not permitted"),
            SocialError::InternalError => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}
