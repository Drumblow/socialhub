use moka::future::Cache;
use std::time::Duration;
use crate::config::CacheConfig;

pub struct CacheManager<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    cache: Cache<K, V>,
}

impl<K, V> CacheManager<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(config: &CacheConfig) -> Self {
        let cache = Cache::builder()
            .max_capacity(config.max_capacity)
            .time_to_live(Duration::from_secs(config.time_to_live))
            .time_to_idle(Duration::from_secs(config.time_to_idle))
            .build();

        Self { cache }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).await
    }

    pub async fn insert(&self, key: K, value: V) {
        self.cache.insert(key, value).await;
    }

    pub async fn remove(&self, key: &K) {
        self.cache.remove(key).await;
    }

    pub async fn invalidate_all(&self) {
        self.cache.invalidate_all();
    }
}
