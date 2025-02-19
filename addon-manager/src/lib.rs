use actix_web::web;

mod plugin;
mod sandbox;
mod manager;
mod error;

pub use plugin::Plugin;
pub use manager::AddonManager;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/addons")
            .service(web::resource("/install").route(web::post().to(manager::install_addon)))
            .service(web::resource("/list").route(web::get().to(manager::list_addons)))
            .service(web::resource("/{id}/enable").route(web::post().to(manager::enable_addon)))
            .service(web::resource("/{id}/disable").route(web::post().to(manager::disable_addon)))
    );
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use actix_web::{test, App};
    use uuid::Uuid;
    use serde_json::json;

    #[actix_rt::test]
    async fn test_install_addon() {
        let app = test::init_service(
            App::new().service(web::scope("/addons").route("/install", web::post().to(manager::install_addon)))
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

    #[actix_rt::test]
    async fn test_list_addons() {
        let app = test::init_service(
            App::new().service(web::scope("/addons").route("/list", web::get().to(manager::list_addons)))
        ).await;

        let req = test::TestRequest::get()
            .uri("/addons/list")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_enable_addon() {
        // ...existing test code...
    }

    #[actix_rt::test]
    async fn test_disable_addon() {
        let addon_id = Uuid::new_v4();
        let app = test::init_service(
            App::new().service(web::scope("/addons").route("/{id}/disable", web::post().to(manager::disable_addon)))
        ).await;

        let req = test::TestRequest::post()
            .uri(&format!("/addons/{}/disable", addon_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
