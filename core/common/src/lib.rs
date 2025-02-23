pub mod error;
pub mod cache;
pub mod utils;
pub mod logging;

pub use cache::{CacheManager, CacheConfig, CacheMetrics};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use env_logger::Builder;
    use log::LevelFilter;

    fn init_test_logger() {
        Builder::new()
            .filter_level(LevelFilter::Debug)
            .is_test(true)
            .init();
    }

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
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
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
    async fn test_cache_increment() {
        let cache = CacheManager::<String, i32>::new(cache::CacheConfig::default());
        let key = "counter".to_string();
        
        // Set initial value
        cache.set(key.clone(), 0).await;
        assert_eq!(cache.get(&key).await, Some(0));
        
        // Increment value
        for i in 1..=5 {
            let current_value = cache.get(&key).await.unwrap_or(0);
            cache.set(key.clone(), current_value + 1).await;
            assert_eq!(cache.get(&key).await, Some(i));
        }
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = CacheManager::<String, String>::new(cache::CacheConfig::default());
        cache.set("key1".to_string(), "value1".to_string()).await;
        cache.set("key2".to_string(), "value2".to_string()).await;
        cache.remove(&"key1".to_string()).await;
        cache.remove(&"key2".to_string()).await;
        assert_eq!(cache.entry_count(), 0);
    }
}
