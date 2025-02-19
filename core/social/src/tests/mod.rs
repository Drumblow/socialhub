use actix_web::{test, web, App};
use crate::handlers;
use serde_json::json;
use uuid::Uuid;

#[actix_rt::test]
async fn test_create_post() {
    let app = test::init_service(
        App::new().service(web::scope("/social").route("/posts", web::post().to(handlers::create_post)))
    ).await;

    let req = test::TestRequest::post()
        .uri("/social/posts")
        .set_json(json!({
            "content": "Test post content",
            "media_ids": [],
            "tags": ["test", "first"]
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_like_post() {
    let post_id = Uuid::new_v4();
    let app = test::init_service(
        App::new().service(web::scope("/social").route("/posts/{id}/like", web::post().to(handlers::like_post)))
    ).await;

    let req = test::TestRequest::post()
        .uri(&format!("/social/posts/{}/like", post_id))
        .insert_header(("Authorization", "Bearer test-token"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
