use super::*;
use serde_json::json;

#[test]
fn test_structured_logging() {
    let event = LogEvent {
        timestamp: 1708291200,
        level: "INFO",
        module: "test",
        message: "test message".to_string(),
        context: Some(json!({
            "user_id": 123,
            "action": "login"
        }))
    };

    let json_str = serde_json::to_string(&event).unwrap();
    assert!(json_str.contains("test message"));
    assert!(json_str.contains("user_id"));
}
