use actix_web::{App, HttpServer, middleware};
use log::info;
use dotenv::dotenv;
use socialhub_core::cache::CacheManager;  // Atualizado para usar o novo crate
use socialhub_core::CacheConfig;  // Importar CacheConfig do novo crate

mod config;
mod routes;

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    info!("Starting SocialHub server...");

    let cache_config = CacheConfig::default();
    let _cache = CacheManager::<String, String>::new(cache_config);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(socialhub_streaming::configure)
            .configure(socialhub_auth::configure)
            .configure(socialhub_social::configure)
            .configure(socialhub_media::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
