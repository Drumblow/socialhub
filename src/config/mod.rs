pub mod types;
pub use types::CacheConfig;

pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub media: MediaConfig,
    pub auth: AuthConfig,
}

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

pub struct MediaConfig {
    pub upload_dir: String,
    pub max_file_size: usize,
}

pub struct AuthConfig {
    pub token_expiration: u64,
    pub refresh_token_expiration: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap(),
                workers: std::env::var("SERVER_WORKERS")
                    .unwrap_or_else(|_| "4".to_string())
                    .parse()
                    .unwrap(),
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgres://localhost/socialhub".to_string()),
                max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .unwrap(),
            },
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
            media: MediaConfig {
                upload_dir: std::env::var("MEDIA_UPLOAD_DIR")
                    .unwrap_or_else(|_| "./uploads".to_string()),
                max_file_size: std::env::var("MEDIA_MAX_FILE_SIZE")
                    .unwrap_or_else(|_| "10485760".to_string()) // 10MB
                    .parse()
                    .unwrap(),
            },
            auth: AuthConfig {
                token_expiration: std::env::var("AUTH_TOKEN_EXPIRATION")
                    .unwrap_or_else(|_| "86400".to_string()) // 24 hours
                    .parse()
                    .unwrap(),
                refresh_token_expiration: std::env::var("AUTH_REFRESH_TOKEN_EXPIRATION")
                    .unwrap_or_else(|_| "604800".to_string()) // 7 days
                    .parse()
                    .unwrap(),
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.workers, 4);
        assert_eq!(config.database.max_connections, 5);
        assert_eq!(config.cache.max_capacity, 10000);
        assert_eq!(config.cache.time_to_live, 3600);
        assert_eq!(config.cache.time_to_idle, 1800);
        assert_eq!(config.media.max_file_size, 10485760);
        assert_eq!(config.auth.token_expiration, 86400);
        assert_eq!(config.auth.refresh_token_expiration, 604800);
    }
}