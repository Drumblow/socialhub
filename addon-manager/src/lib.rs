use actix_web::web::{self as actix_web_config, ServiceConfig, scope, resource, get, post};

mod plugin;
mod sandbox;
mod manager;
mod error;
pub mod stremio;
pub mod web;  // Agora é público diretamente

pub use plugin::Plugin;
pub use manager::AddonManager;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/addons")
            .service(resource("/install").route(post().to(manager::install_addon)))
            .service(resource("/list").route(get().to(manager::list_addons)))
            .service(resource("/{id}/enable").route(post().to(manager::enable_addon)))
            .service(resource("/{id}/disable").route(post().to(manager::disable_addon)))
            .service(
                resource("/{id}/configure")
                    .route(post().to(web::configure_addon))
            )
            .service(
                resource("/{id}/config")
                    .route(get().to(web::get_addon_config))
            )
    );
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use actix_web::{test, App};
    use uuid::Uuid;
    use serde_json::json;

    #[tokio::test]
    async fn test_install_addon() {
        let app = test::init_service(
            App::new().service(actix_web_config::scope("/addons").route("/install", actix_web_config::post().to(manager::install_addon)))
        ).await;

        let req = test::TestRequest::post()
            .uri("/addons/install")
            .set_json(serde_json::json!({
                "name": "test-addon",
                "version": "1.0.0"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[tokio::test]
    async fn test_list_addons() {
        let app = test::init_service(
            App::new().service(actix_web_config::scope("/addons").route("/list", actix_web_config::get().to(manager::list_addons)))
        ).await;

        let req = test::TestRequest::get()
            .uri("/addons/list")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[tokio::test]
    async fn test_enable_addon() {
        // ...existing test code...
    }

    #[tokio::test]
    async fn test_disable_addon() {
        let addon_id = Uuid::new_v4();
        let app = test::init_service(
            App::new().service(actix_web_config::scope("/addons").route("/{id}/disable", actix_web_config::post().to(manager::disable_addon)))
        ).await;

        let req = test::TestRequest::post()
            .uri(&format!("/addons/{}/disable", addon_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::web::{AddonConfig, configure_addon, get_addon_config};
    use actix_web::{test, App, web};
    use uuid::Uuid;

    #[actix_web::test]
    async fn test_web_configure_addon() {
        let app = test::init_service(
            App::new().service(
                web::resource("/addon/{id}/configure")
                    .route(web::post().to(configure_addon))
            )
        ).await;

        let config = AddonConfig {
            id: "test-addon".to_string(),
            enabled: true,
            catalog_filters: Some(vec!["Action".to_string()]),
            max_results: Some(100),
            preferred_quality: Some("HD".to_string()),
        };

        let req = test::TestRequest::post()
            .uri("/addon/test-addon/configure")
            .set_json(&config)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // ...existing tests...
}
