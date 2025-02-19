//! Media handling module for SocialHub
//! 
//! # Examples
//! 
//! ```rust
//! use socialhub_media;
//! use actix_web::web;
//! 
//! async fn configure_media(cfg: &mut web::ServiceConfig) {
//!     socialhub_media::configure(cfg);
//! }
//! ```

use actix_web::web;

mod error;
pub mod models;
pub mod handlers;  // Alterado para público
mod service;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/media")
            .service(web::resource("/upload").route(web::post().to(handlers::upload)))
            .service(web::resource("/{id}")
                .route(web::get().to(handlers::get_media))
                .route(web::delete().to(handlers::delete_media)))
            .service(web::resource("/{id}/metadata")
                .route(web::get().to(handlers::get_metadata))
                .route(web::put().to(handlers::update_metadata)))
    );
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use actix_web::{test, App, http::header};
    use bytes::Bytes;
    use uuid::Uuid;
    use serde_json::json;
    use log::info;
    use std::env;

    fn init() {
        env::set_var("RUST_LOG", "debug");
        let _ = env_logger::try_init();
    }

    #[actix_rt::test]
    async fn test_upload_media() {
        init();
        info!("Running test_upload_media");

        let app = test::init_service(
            App::new().service(web::scope("/media").route("/upload", web::post().to(handlers::upload)))
        ).await;

        let payload = concat!(
            "--abbc761f78ff4d7cb7573b5a23f96ef0\r\n",
            "Content-Disposition: form-data; name=\"file\"; filename=\"test.jpg\"\r\n",
            "Content-Type: image/jpeg\r\n\r\n",
            "test file content",
            "\r\n--abbc761f78ff4d7cb7573b5a23f96ef0--\r\n"
        );

        let req = test::TestRequest::post()
            .uri("/media/upload")
            .insert_header((
                header::CONTENT_TYPE,
                "multipart/form-data; boundary=abbc761f78ff4d7cb7573b5a23f96ef0"
            ))
            .set_payload(payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        info!("Upload response status: {}", resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_get_media_not_found() {
        init();
        info!("Running test_get_media_not_found");

        let app = test::init_service(
            App::new().service(web::scope("/media").route("/{id}", web::get().to(handlers::get_media)))
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/media/{}", Uuid::new_v4()))
            .to_request();

        let resp = test::call_service(&app, req).await;
        info!("Get media not found response status: {}", resp.status());
        assert_eq!(resp.status().as_u16(), 404);
    }

    #[actix_rt::test]
    async fn test_upload_invalid_media_type() {
        init();
        info!("Running test_upload_invalid_media_type");

        let app = test::init_service(
            App::new().service(web::scope("/media").route("/upload", web::post().to(handlers::upload)))
        ).await;

        let payload = concat!(
            "--abbc761f78ff4d7cb7573b5a23f96ef0\r\n",
            "Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n",
            "Content-Type: text/plain\r\n\r\n",
            "invalid content",
            "\r\n--abbc761f78ff4d7cb7573b5a23f96ef0--\r\n"
        );

        let req = test::TestRequest::post()
            .uri("/media/upload")
            .insert_header((
                header::CONTENT_TYPE,
                "multipart/form-data; boundary=abbc761f78ff4d7cb7573b5a23f96ef0"
            ))
            .set_payload(payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        info!("Invalid media type response status: {}", resp.status());
        assert_eq!(resp.status().as_u16(), 415); // Unsupported Media Type
    }

    #[actix_rt::test]
    async fn test_get_media_metadata_not_found() {
        init();
        info!("Running test_get_media_metadata_not_found");

        let app = test::init_service(
            App::new().service(web::scope("/media").route("/{id}/metadata", web::get().to(handlers::get_metadata)))
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/media/{}/metadata", Uuid::new_v4()))
            .to_request();

        let resp = test::call_service(&app, req).await;
        info!("Get media metadata not found response status: {}", resp.status());
        assert_eq!(resp.status().as_u16(), 404);
    }

    #[actix_rt::test]
    async fn test_upload_large_file() {
        init();
        info!("Running test_upload_large_file");

        let app = test::init_service(
            App::new().service(web::scope("/media").route("/upload", web::post().to(handlers::upload)))
        ).await;

        let large_content = vec![0u8; 11 * 1024 * 1024]; // 11MB
        let payload = format!(
            "--abbc761f78ff4d7cb7573b5a23f96ef0\r\n\
            Content-Disposition: form-data; name=\"file\"; filename=\"large.jpg\"\r\n\
            Content-Type: image/jpeg\r\n\r\n\
            {}\r\n\
            --abbc761f78ff4d7cb7573b5a23f96ef0--\r\n",
            String::from_utf8_lossy(&large_content)
        );

        let req = test::TestRequest::post()
            .uri("/media/upload")
            .insert_header((
                header::CONTENT_TYPE,
                "multipart/form-data; boundary=abbc761f78ff4d7cb7573b5a23f96ef0"
            ))
            .set_payload(payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        info!("Upload large file response status: {}", resp.status());
        assert_eq!(resp.status().as_u16(), 413); // Payload Too Large
    }

    #[actix_rt::test]
    async fn test_update_metadata() {
        init();
        info!("Running test_update_metadata");

        let media_id = Uuid::new_v4();
        let app = test::init_service(
            App::new().service(web::scope("/media").route("/{id}/metadata", web::put().to(handlers::update_metadata)))
        ).await;

        let req = test::TestRequest::put()
            .uri(&format!("/media/{}/metadata", media_id))
            .set_json(json!({
                "title": "Updated Title",
                "description": "Updated description"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        info!("Update metadata response status: {}", resp.status());
        assert!(resp.status().is_success());
    }
}
