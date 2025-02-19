use actix_web::{test, web, App};
use bytes::Bytes;
use crate::handlers;
use uuid::Uuid;

#[actix_rt::test]
async fn test_upload_media() {
    let app = test::init_service(
        App::new().service(web::scope("/media").route("/upload", web::post().to(handlers::upload)))
    ).await;

    let bytes = Bytes::from_static(b"test file content");
    let req = test::TestRequest::post()
        .uri("/media/upload")
        .set_payload(bytes)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_upload_image() {
    let app = test::init_service(
        App::new().service(web::scope("/media").route("/upload", web::post().to(handlers::upload)))
    ).await;

    let payload = actix_web::web::Bytes::from_static(b"fake image content");
    let req = test::TestRequest::post()
        .uri("/media/upload")
        .insert_header(("Content-Type", "image/jpeg"))
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_media() {
    let media_id = Uuid::new_v4();
    let app = test::init_service(
        App::new().service(web::scope("/media").route("/{id}", web::get().to(handlers::get_media)))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/media/{}", media_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_media_metadata() {
    let media_id = Uuid::new_v4();
    let app = test::init_service(
        App::new().service(web::scope("/media").route("/{id}/metadata", web::get().to(handlers::get_metadata)))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/media/{}/metadata", media_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_delete_media() {
    let media_id = Uuid::new_v4();
    let app = test::init_service(
        App::new().service(web::scope("/media").route("/{id}", web::delete().to(handlers::delete_media)))
    ).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/media/{}", media_id))
        .insert_header(("Authorization", "Bearer test-token"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_media_not_found() {
    let app = test::init_service(
        App::new().service(
            web::scope("/media")
                .route("/{id}", web::get().to(handlers::get_media))
        )
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/media/{}", Uuid::new_v4()))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

// Mais testes para metadata, delete, etc...
