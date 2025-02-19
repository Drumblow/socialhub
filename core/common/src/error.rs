use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error")]
    InternalError,
}
