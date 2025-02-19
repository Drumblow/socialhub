use actix_web::{test, web, App};
use crate::manager;
use uuid::Uuid;

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
    let addon_id = Uuid::new_v4();
    let app = test::init_service(
        App::new().service(web::scope("/addons").route("/{id}/enable", web::post().to(manager::enable_addon)))
    ).await;

    let req = test::TestRequest::post()
        .uri(&format!("/addons/{}/enable", addon_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
