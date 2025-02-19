use moka::future::Cache as MokaCache;
use std::hash::Hash;
use std::time::Duration;
use log::{debug, info, warn};
use std::sync::Arc;
use tokio::sync::RwLock;

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
            time_to_live: 3600,  // 1 hora
            time_to_idle: 1800,  // 30 minutos
        }
    }
}

#[derive(Clone)]
pub struct CacheManager<K, V> {
    pub(crate) cache: MokaCache<K, V>,
    capacity: u64,
    metrics: Arc<RwLock<CacheMetrics>>,
}

#[derive(Default, Clone, Debug)]
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
}

impl<K, V> CacheManager<K, V> 
where 
    K: Clone + Hash + Eq + Send + Sync + std::fmt::Debug + 'static,
    V: Clone + Send + Sync + 'static
{
    pub fn new(config: CacheConfig) -> Self {
        info!("Initializing cache with capacity: {}", config.max_capacity);
        Self {
            cache: MokaCache::builder()
                .max_capacity(config.max_capacity)
                .time_to_live(Duration::from_secs(config.time_to_live))
                .time_to_idle(Duration::from_secs(config.time_to_idle))
                .weigher(|_k, _v| 1)
                .eviction_listener(|k, _v, cause| {
                    debug!("Evicted key: {:?}, cause: {:?}", k, cause);
                })
                .build(),
            capacity: config.max_capacity,
            metrics: Arc::new(RwLock::new(CacheMetrics::default())),
        }
    }

    pub async fn set(&self, key: K, value: V) {
        debug!("Inserting item. Current size: {}/{}", self.cache.entry_count(), self.capacity);
        self.cache.insert(key, value).await;
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).await
    }

    pub async fn remove(&self, key: &K) {
        warn!("Cache eviction - Removed item");
        self.cache.remove(key).await;
    }

    pub async fn sync(&self) {
        self.cache.run_pending_tasks().await;
    }

    pub async fn get_metrics(&self) -> CacheMetrics {
        (*self.metrics.read().await).clone()  // Dereference before clone
    }

    pub async fn entry_count(&self) -> u64 {
        self.cache.entry_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task;
    
    #[tokio::test]
    async fn test_cache_operations() {
        let cache = CacheManager::<String, i32>::new(CacheConfig::default());
        cache.set("test".to_string(), 42).await;
        assert_eq!(cache.get(&"test".to_string()).await, Some(42));
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let cache = CacheManager::<String, i32>::new(CacheConfig::default());
        let mut handles = Vec::new();

        // Criar 10 tasks concorrentes
        for i in 0..10 {
            let cache_ref = cache.clone();
            let handle = task::spawn(async move {
                let key = format!("key_{}", i);
                cache_ref.set(key.clone(), i).await;
                cache_ref.get(&key).await
            });
            handles.push(handle);
        }

        // Aguardar todas as tasks terminarem
        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await.unwrap();
            assert_eq!(result, Some(i as i32));
        }
    }

    #[tokio::test]
    async fn test_cache_capacity_limit() {
        let config = CacheConfig {
            max_capacity: 5,
            time_to_live: 60,   // 60 segundos
            time_to_idle: 30,   // 30 segundos
        };
        
        let cache = CacheManager::<String, i32>::new(config);
        
        // Inserir itens sequencialmente
        for i in 0..5 {
            let key = format!("key_{}", i);
            cache.set(key.clone(), i).await;
            cache.sync().await;
        }

        // Esperar processamento
        tokio::time::sleep(Duration::from_millis(100)).await;
        cache.sync().await;

        // Verificar estado inicial
        let count = cache.entry_count().await;
        assert_eq!(count, 5, "Cache should be at capacity");

        // Adicionar item extra
        cache.set("key_5".to_string(), 5).await;
        cache.sync().await;

        // Verificar se mantém capacidade
        let count = cache.entry_count().await;
        assert_eq!(count, 5, "Cache should maintain capacity");
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = CacheManager::<String, String>::new(CacheConfig::default());
        cache.set("key1".to_string(), "value1".to_string()).await;
        cache.set("key2".to_string(), "value2".to_string()).await;
        cache.remove(&"key1".to_string()).await;
        cache.remove(&"key2".to_string()).await;
        assert_eq!(cache.entry_count().await, 0);
    }
}