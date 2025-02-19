use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn validate_email(email: &str) -> bool {
    // Implementação básica de validação de email
    email.contains('@') && email.contains('.')
}
