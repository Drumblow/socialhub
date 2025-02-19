pub struct Config {
    // ...existing code...
    pub cache: CacheConfig,
}

pub struct CacheConfig {
    pub max_capacity: u64,
    pub time_to_live: u64,
    pub time_to_idle: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            // ...existing code...
            cache: CacheConfig {
                max_capacity: std::env::var("CACHE_MAX_CAPACITY")
                    .unwrap_or_else(|_| "10000".to_string())
                    .parse()
                    .unwrap(),
                time_to_live: std::env::var("CACHE_TTL")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap(),
                time_to_idle: std::env::var("CACHE_TTI")
                    .unwrap_or_else(|_| "1800".to_string())
                    .parse()
                    .unwrap(),
            },
        }
    }
}
