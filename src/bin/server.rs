use actix_web::{App, HttpServer};
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use socialhub::api_docs::ApiDoc;

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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
