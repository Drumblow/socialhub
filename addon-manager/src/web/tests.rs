use super::*;
use actix_web::{test, App, web::ServiceConfig};

fn test_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/addon/{id}/configure")
            .route(web::post().to(configure_addon))
    ).service(
        web::resource("/addon/{id}/config")
            .route(web::get().to(get_addon_config))
    );
}

#[actix_web::test]
async fn test_configure_addon() {
    let app = test::init_service(App::new().configure(test_config)).await;

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

    let result: AddonConfig = test::read_body_json(resp).await;
    assert_eq!(result.id, "test-addon");
    assert_eq!(result.max_results, Some(100));
}

#[actix_web::test]
async fn test_get_addon_config() {
    let app = test::init_service(App::new().configure(test_config)).await;

    let req = test::TestRequest::get()
        .uri("/addon/test-addon/config")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let config: AddonConfig = test::read_body_json(resp).await;
    assert_eq!(config.id, "test-addon");
    assert!(config.enabled);
}
