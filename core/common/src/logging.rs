use serde::Serialize;
use log::{info, warn, error};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
pub struct LogEvent<'a> {
    pub timestamp: u64,
    pub level: &'a str,
    pub module: &'a str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

pub fn log_event(level: &str, module: &str, message: &str, context: Option<serde_json::Value>) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let event = LogEvent {
        timestamp,
        level,
        module,
        message: message.to_string(),
        context,
    };

    let json = serde_json::to_string(&event).unwrap_or_default();

    match level {
        "INFO" => info!("{}", json),
        "WARN" => warn!("{}", json),
        "ERROR" => error!("{}", json),
        _ => info!("{}", json),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_log_event_format() {
        let context = Some(json!({
            "user_id": 123,
            "action": "test"
        }));
        
        log_event("INFO", "test_module", "test message", context);
    }
}
