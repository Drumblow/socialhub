use actix_web::{App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting SocialHub server...");

    HttpServer::new(|| {
        App::new()
            .configure(socialhub_streaming::configure)
            .configure(socialhub_auth::configure)
            .configure(socialhub_social::configure)
            .configure(socialhub_media::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
