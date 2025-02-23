pub mod config;
pub mod routes;

pub use config::types::CacheConfig;

// Usar o cache do core
pub use socialhub_core::cache::CacheManager;

pub use socialhub_auth as auth;
pub use socialhub_media as media;
pub use socialhub_social as social;
pub use socialhub_streaming as streaming;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/addons")
            .service(web::resource("/install").route(web::post().to(routes::addons::install)))
            .service(web::resource("/list").route(web::get().to(routes::addons::list)))  // Adicionar esta linha
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_auth_flow() {
        let app = test::init_service(
            App::new()
                .configure(|cfg| {
                    socialhub_auth::configure(cfg);
                })
        ).await;

        let login_resp = test::call_service(&app, 
            test::TestRequest::post()
                .uri("/auth/login")
                .set_json(serde_json::json!({
                    "username": "testuser",
                    "password": "password123"
                }))
                .to_request()
        ).await;
        assert!(login_resp.status().is_success());
    }
}