pub mod error;
pub mod cache;
pub mod utils;
pub mod logging;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::thread;
    use serde_json::json;
    use crate::cache::CacheManager;

    #[test]
    fn test_structured_logging() {
        let event = logging::LogEvent {
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

    #[tokio::test]
    async fn test_cache_with_custom_ttl() {
        let config = cache::CacheConfig {
            max_capacity: 1000,
            time_to_live: 1,
            time_to_idle: 1,
        };
        let cache = CacheManager::<String, String>::new(config);
        cache.set("key".to_string(), "value".to_string()).await;
        thread::sleep(Duration::from_secs(2));
        assert!(cache.get(&"key".to_string()).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_update() {
        let cache = CacheManager::<String, String>::new(cache::CacheConfig::default());
        cache.set("key".to_string(), "value1".to_string()).await;
        cache.set("key".to_string(), "value2".to_string()).await;
        assert_eq!(cache.get(&"key".to_string()).await, Some("value2".to_string()));
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = CacheManager::<String, String>::new(cache::CacheConfig::default());
        cache.set("key1".to_string(), "value1".to_string()).await;
        cache.set("key2".to_string(), "value2".to_string()).await;
        cache.remove(&"key1".to_string()).await;
        cache.remove(&"key2".to_string()).await;
        assert_eq!(cache.cache.entry_count(), 0);
    }
}
