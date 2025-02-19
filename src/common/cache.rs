use moka::future::Cache;
use std::{time::Duration, sync::Arc, collections::VecDeque};
use tokio::sync::RwLock;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::time::Instant;

use crate::config::CacheConfig;

#[derive(Clone)]
pub struct CacheManager<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    cache: Cache<K, V>,
    metrics: Arc<RwLock<CacheMetrics<K>>>,
    max_capacity: u64,
    lru_queue: Arc<RwLock<VecDeque<K>>>, // Track LRU order
}

#[derive(Clone)]
struct CacheMetrics<K> {
    access_times: HashMap<K, Instant>,
    hit_count: usize,
    miss_count: usize,
    eviction_count: usize,
}

impl<K, V> CacheManager<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + std::fmt::Debug + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(config: &CacheConfig) -> Self {
        info!("Initializing cache with capacity: {}", config.max_capacity);
        
        let cache = Cache::builder()
            .max_capacity(config.max_capacity)
            .time_to_live(Duration::from_secs(config.time_to_live))
            .time_to_idle(Duration::from_secs(config.time_to_idle))
            .weigher(|_k, _v| 1u32)  // Each entry counts as 1
            .eviction_listener(move |k, _v, cause| {
                debug!("Eviction occurred - Key: {:?}, Cause: {:?}", k, cause);
            })
            .build();

        Self {
            cache,
            metrics: Arc::new(RwLock::new(CacheMetrics {
                access_times: HashMap::new(),
                hit_count: 0,
                miss_count: 0,
                eviction_count: 0,
            })),
            max_capacity: config.max_capacity,
            lru_queue: Arc::new(RwLock::new(VecDeque::with_capacity(config.max_capacity as usize))),
        }
    }

    async fn update_lru(&self, key: &K) {
        let mut queue = self.lru_queue.write().await;
        if let Some(pos) = queue.iter().position(|k| k == key) {
            queue.remove(pos);
        }
        queue.push_back(key.clone());
        
        // Ensure queue doesn't exceed capacity
        while queue.len() > self.max_capacity as usize {
            if let Some(lru_key) = queue.pop_front() {
                self.cache.remove(&lru_key).await;
                debug!("LRU eviction - Key: {:?}", lru_key);
            }
        }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        let result = self.cache.get(key).await;
        if result.is_some() {
            self.update_lru(key).await;
        }
        
        // Update metrics
        let mut metrics = self.metrics.write().await;
        match &result {
            Some(_) => {
                metrics.hit_count += 1;
                metrics.access_times.insert(key.clone(), Instant::now());
                debug!("Cache hit - Key: {:?}, Hit rate: {:.2}%", 
                    key, 
                    self.calculate_hit_rate(&metrics)
                );
            }
            None => {
                metrics.miss_count += 1;
                debug!("Cache miss - Key: {:?}, Miss rate: {:.2}%",
                    key,
                    self.calculate_miss_rate(&metrics)
                );
            }
        }
        
        result
    }

    pub async fn insert(&self, key: K, value: V) {
        let current_size = self.cache.entry_count();
        debug!(
            "Inserting key {:?}. Current size: {}/{}", 
            key, 
            current_size,
            self.max_capacity
        );

        if current_size >= self.max_capacity {
            // Evict least recently used item
            let mut queue = self.lru_queue.write().await;
            if let Some(lru_key) = queue.pop_front() {
                self.cache.remove(&lru_key).await;
                let mut metrics = self.metrics.write().await;
                metrics.eviction_count += 1;
                metrics.access_times.remove(&lru_key);
                warn!(
                    "Cache eviction - Removed LRU key: {:?}", 
                    lru_key
                );
            }
        }

        self.cache.insert(key.clone(), value).await;
        self.update_lru(&key).await;
    }

    pub async fn remove(&self, key: &K) {
        self.cache.remove(key).await;
    }

    pub async fn invalidate_all(&self) {
        self.cache.invalidate_all();
    }

    pub async fn sync(&self) {
        self.cache.run_pending_tasks().await;
    }

    pub async fn get_metrics(&self) -> CacheStats {
        self.cache.run_pending_tasks().await;
        let metrics = self.metrics.read().await;
        CacheStats {
            hit_rate: self.calculate_hit_rate(&metrics),
            miss_rate: self.calculate_miss_rate(&metrics),
            eviction_count: metrics.eviction_count,
            size: self.cache.weighted_size(),
        }
    }

    fn calculate_hit_rate(&self, metrics: &CacheMetrics<K>) -> f64 {
        let total = metrics.hit_count + metrics.miss_count;
        if total == 0 { 0.0 } else { (metrics.hit_count as f64 / total as f64) * 100.0 }
    }

    fn calculate_miss_rate(&self, metrics: &CacheMetrics<K>) -> f64 {
        let total = metrics.hit_count + metrics.miss_count;
        if total == 0 { 0.0 } else { (metrics.miss_count as f64 / total as f64) * 100.0 }
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub hit_rate: f64,
    pub miss_rate: f64,
    pub eviction_count: usize,
    pub size: u64,
}
