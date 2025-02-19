use socialhub::common::cache::CacheManager;
use socialhub::config::CacheConfig;

#[tokio::test]
async fn test_cache_with_real_data() {
    let config = CacheConfig {
        max_capacity: 1000,
        time_to_live: 3600,
        time_to_idle: 1800,
    };

    let cache = CacheManager::<String, Vec<u8>>::new(&config);
    let test_data = vec![1, 2, 3, 4, 5];

    // Test basic operations with real data
    cache.insert("test_key".to_string(), test_data.clone()).await;
    
    let result = cache.get(&"test_key".to_string()).await;
    assert_eq!(result, Some(test_data));
}

#[tokio::test]
async fn test_cache_concurrent_access() {
    let config = CacheConfig {
        max_capacity: 1000,
        time_to_live: 3600,
        time_to_idle: 1800,
    };

    let cache = CacheManager::<String, String>::new(&config);
    let cache_clone = cache.clone();

    let write_task = tokio::spawn(async move {
        for i in 0..100 {
            cache_clone
                .insert(
                    format!("key{}", i),
                    format!("value{}", i)
                )
                .await;
        }
    });

    let read_task = tokio::spawn(async move {
        for i in 0..100 {
            if let Some(value) = cache.get(&format!("key{}", i)).await {
                assert_eq!(value, format!("value{}", i));
            }
        }
    });

    tokio::try_join!(write_task, read_task).unwrap();
}
