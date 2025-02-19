use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_unique_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn validate_string_length(s: &str, min: usize, max: usize) -> bool {
    s.len() >= min && s.len() <= max
}
