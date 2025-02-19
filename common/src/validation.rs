use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Field {field} is required")]
    Required { field: String },
    
    #[error("Field {field} has invalid format: {message}")]
    InvalidFormat { field: String, message: String },
    
    #[error("Value {value} for field {field} is not allowed")]
    InvalidValue { field: String, value: String },
}

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}

#[derive(Debug, Serialize)]
pub struct ValidationResponse {
    pub errors: Vec<String>,
}
