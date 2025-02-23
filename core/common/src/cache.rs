use moka::future::Cache;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use log::{info, debug, warn};
use futures::future::join_all;

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_capacity: u64,
    pub time_to_live: u64,
    pub time_to_idle: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 1000,
            time_to_live: 3600,
            time_to_idle: 1800,
        }
    }
}

#[derive(Clone)]
pub struct CacheManager<K, V>
where 
    K: Clone + Eq + std::hash::Hash + Send + Sync + std::fmt::Debug + 'static,
    V: Clone + Send + Sync + std::fmt::Debug + 'static,  // Added Debug bound for V
{
    cache: Cache<K, V>,
    metrics: Arc<RwLock<CacheMetrics>>,
}

#[derive(Default, Clone, Debug)]
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_insertions: u64,
    pub total_retrievals: u64,
    pub cache_misses_ratio: f64,
}

impl<K, V> CacheManager<K, V> 
where 
    K: Clone + Hash + Eq + Send + Sync + std::fmt::Debug + 'static,
    V: Clone + Send + Sync + std::fmt::Debug + 'static  // Added Debug bound for V
{
    pub fn new(config: CacheConfig) -> Self {
        info!("Initializing cache with capacity: {}", config.max_capacity);
        
        let cache = Cache::builder()
            .max_capacity(config.max_capacity)
            .time_to_live(std::time::Duration::from_secs(config.time_to_live))
            .time_to_idle(std::time::Duration::from_secs(config.time_to_idle))
            .build();

        let manager = Self {
            cache,
            metrics: Arc::new(std::sync::RwLock::new(CacheMetrics::default())),
        };

        info!("Cache initialized with size: {}", manager.cache.entry_count());
        manager
    }

    pub async fn set(&self, key: K, value: V) {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.total_insertions += 1;
        }

        self.cache.insert(key.clone(), value).await;
        
        let is_present = self.cache.get(&key).await.is_some();
        let size = self.real_count().await;
        
        debug!("Cache update - key: {:?}, present: {}, total items: {}", 
            key, is_present, size);
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.total_retrievals += 1;
        }

        let value = self.cache.get(key).await;
        
        if let Ok(mut metrics) = self.metrics.write() {
            if value.is_some() {
                metrics.hits += 1;
            } else {
                metrics.misses += 1;
            }
            metrics.cache_misses_ratio = metrics.misses as f64 / metrics.total_retrievals as f64;
        }
        
        value
    }

    pub fn entry_count(&self) -> u64 {
        self.cache.entry_count()
    }

    async fn real_count(&self) -> u64 {
        // Primeiro coletamos todas as chaves, desreferenciando o Arc
        let keys: Vec<K> = self.cache.iter()
            .map(|(k, _)| (*k).clone())
            .collect();
        
        // Criamos futures com as chaves clonadas
        let futures: Vec<_> = keys.iter()
            .map(|key| self.cache.get(key))
            .collect();
        
        // Executar todas as verificações em paralelo
        let results = join_all(futures).await;
        
        // Contar chaves ativas e logar
        let count = results.iter().filter(|r| r.is_some()).count() as u64;
        debug!("Cache size: {}, active keys: {:?}", count, keys);
        count
    }

    pub async fn get_size(&self) -> u64 {
        self.real_count().await
    }

    pub async fn remove(&self, key: &K) {
        warn!("Cache eviction - Removed item");
        self.cache.invalidate(key).await;
    }

    pub fn get_metrics(&self) -> CacheMetrics {
        self.metrics.read().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger::Builder;
    use log::LevelFilter;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_test_logger() {
        INIT.call_once(|| {
            std::env::set_var("RUST_LOG", "debug");
            Builder::new()
                .filter_level(LevelFilter::Debug)
                .is_test(true)
                .init();
        });
    }
    
    #[tokio::test]
    async fn test_cache_operations() {
        let cache = CacheManager::<String, i32>::new(CacheConfig::default());
        cache.set("test".to_string(), 42).await;
        let result = cache.get(&"test".to_string()).await;
        assert_eq!(result, Some(42));
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let cache = CacheManager::<String, i32>::new(CacheConfig::default());
        let mut handles = Vec::new();

        for i in 0..10 {
            let cache_ref = cache.clone();
            let handle = tokio::spawn(async move {
                let key = format!("key_{}", i);
                cache_ref.set(key.clone(), i).await;
                cache_ref.get(&key).await
            });
            handles.push(handle);
        }

        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await.unwrap();
            assert_eq!(result, Some(i as i32));
        }
    }

    #[tokio::test]
    async fn test_cache_capacity_limit() {
        init_test_logger();
        
        let cache = CacheManager::<String, i32>::new(CacheConfig {
            max_capacity: 5,
            time_to_live: 3600,
            time_to_idle: 1800,
        });

        // Inserir e verificar cada item
        for i in 0u64..5 {
            let key = format!("key_{}", i);
            cache.set(key.clone(), i as i32).await;
            
            let value = cache.get(&key).await;
            assert!(value.is_some(), "Item {} should be present", i);
            
            let size = cache.get_size().await;
            assert_eq!(size, i + 1, "Cache should have {} items", i + 1);
        }

        let final_size = cache.get_size().await;
        assert_eq!(final_size, 5, "Cache should have exactly 5 items");
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = CacheManager::<String, String>::new(CacheConfig::default());
        cache.set("key1".to_string(), "value1".to_string()).await;
        cache.set("key2".to_string(), "value2".to_string()).await;
        cache.remove(&"key1".to_string()).await;
        cache.remove(&"key2".to_string()).await;

        assert_eq!(cache.entry_count(), 0);
    }
}