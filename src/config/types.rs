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