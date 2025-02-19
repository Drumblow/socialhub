pub mod common;
pub mod config;
pub mod routes;
pub mod api_docs;  // Este módulo agora deve apontar para src/api_docs.rs

pub use socialhub_auth as auth;
pub use socialhub_media as media;
pub use socialhub_social as social;
pub use socialhub_streaming as streaming;

// Re-export principais tipos
pub use common::cache::CacheManager;
pub use config::Config;

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

    #[actix_rt::test]
    async fn test_media_flow() {
        // Similar test for media module
    }

    #[actix_rt::test]
    async fn test_social_flow() {
        // Similar test for social module
    }
}
