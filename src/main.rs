use actix_web::{App, HttpServer, middleware};
use log::info;

mod config;
mod routes;
mod common;

use config::Config;
use common::cache::CacheManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let config = Config::from_env();
    let cache_manager = CacheManager::<String, Vec<u8>>::new(&config.cache);

    info!("Starting SocialHub server...");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(actix_web::web::Data::new(cache_manager.clone()))
            .configure(routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
