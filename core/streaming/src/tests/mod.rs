use super::*;
use actix_web::{test, web, App};
use crate::handlers::{self, StreamRequest};
use uuid::Uuid;

#[actix_rt::test]
async fn test_start_stream() {
    let app = test::init_service(
        App::new().service(web::scope("/streaming").route("/live", web::post().to(handlers::start_live)))
    ).await;

    let req = test::TestRequest::post()
        .uri("/streaming/live")
        .set_json(StreamRequest {
            title: "Test Stream".to_string(),
            stream_type: "video".to_string(),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_stream_video() {
    let app = test::init_service(
        App::new().service(
            web::scope("/streaming")
                .route("/video/{id}", web::get().to(handlers::stream_video))
        )
    ).await;

    let req = test::TestRequest::get()
        .uri("/streaming/video/123")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_stop_stream() {
    let stream_id = Uuid::new_v4();
    let app = test::init_service(
        App::new().service(web::scope("/streaming").route("/{id}/stop", web::post().to(handlers::stop_stream)))
    ).await;

    let req = test::TestRequest::post()
        .uri(&format!("/streaming/{}/stop", stream_id))
        .insert_header(("Authorization", "Bearer test-token"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_stream_info() {
    let stream_id = Uuid::new_v4();
    let app = test::init_service(
        App::new().service(web::scope("/streaming").route("/{id}", web::get().to(handlers::get_stream_info)))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/streaming/{}", stream_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
