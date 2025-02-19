#[cfg(test)]
mod tests {
    use super::super::cache::CacheManager;
    use crate::config::CacheConfig;
    use tokio::time::{sleep, Duration};
    use env_logger::{Builder, Env};
    use log::LevelFilter;

    fn init_test_logger() {
        let env = Env::default().filter_or("RUST_LOG", "debug");
        Builder::from_env(env)
            .format_timestamp_micros()
            .filter_module("socialhub::common::cache", LevelFilter::Debug)
            .init();
    }

    fn create_test_config() -> CacheConfig {
        CacheConfig {
            max_capacity: 100,
            time_to_live: 2,
            time_to_idle: 1,
        }
    }

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache = CacheManager::<String, String>::new(&create_test_config());
        
        // Test insert and get
        cache.insert("key1".to_string(), "value1".to_string()).await;
        assert_eq!(
            cache.get(&"key1".to_string()).await,
            Some("value1".to_string())
        );

        // Test remove
        cache.remove(&"key1".to_string()).await;
        assert_eq!(cache.get(&"key1".to_string()).await, None);
    }

    #[tokio::test]
    async fn test_cache_ttl() {
        let cache = CacheManager::<String, String>::new(&create_test_config());
        
        cache.insert("key2".to_string(), "value2".to_string()).await;
        sleep(Duration::from_secs(3)).await;
        
        assert_eq!(cache.get(&"key2".to_string()).await, None);
    }

    #[tokio::test]
    async fn test_cache_capacity() {
        init_test_logger();
        
        let config = CacheConfig {
            max_capacity: 2,
            time_to_live: 10,
            time_to_idle: 5,
        };
        let cache = CacheManager::<String, String>::new(&config);

        // Insert first two items
        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.sync().await;
        
        cache.insert("key2".to_string(), "value2".to_string()).await;
        cache.sync().await;

        // Access key1 to make it most recently used
        assert_eq!(cache.get(&"key1".to_string()).await, Some("value1".to_string()));
        cache.sync().await;
        
        // Sleep briefly to ensure access times are different
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Insert key3, should evict key2 (least recently used)
        cache.insert("key3".to_string(), "value3".to_string()).await;
        cache.sync().await;

        // Verify LRU behavior
        let key1 = cache.get(&"key1".to_string()).await;
        let key2 = cache.get(&"key2".to_string()).await;
        let key3 = cache.get(&"key3".to_string()).await;

        assert_eq!(key1, Some("value1".to_string()), "key1 should remain (most recently used)");
        assert_eq!(key2, None, "key2 should be evicted (least recently used)");
        assert_eq!(key3, Some("value3".to_string()), "key3 should be present (newly added)");
        
        let stats = cache.get_metrics().await;
        assert_eq!(stats.size, 2, "Cache should maintain max capacity of 2");
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = CacheManager::<String, String>::new(&create_test_config());
        
        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.insert("key2".to_string(), "value2".to_string()).await;
        
        cache.invalidate_all().await;
        
        assert_eq!(cache.get(&"key1".to_string()).await, None);
        assert_eq!(cache.get(&"key2".to_string()).await, None);
    }
}
