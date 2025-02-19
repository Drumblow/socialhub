use actix_web::web;

pub mod models;  // tornar público
pub mod handlers;  // tornar público
mod service;
mod error;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let streaming_service = web::Data::new(service::StreamingService::default());
    cfg.app_data(streaming_service.clone())
        .service(
            web::scope("/stream")
                .route("/video/{id}", web::get().to(handlers::stream_video))
                .route("/audio/{id}", web::get().to(handlers::stream_audio))
                .route("/live", web::post().to(handlers::start_live))
                .route("/{id}/stop", web::post().to(handlers::stop_stream))
                .route("/{id}/info", web::get().to(handlers::get_stream_info))
        );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};  // Removed unused dev::Service import
    use serde_json::json;
    use uuid::Uuid;

    #[actix_rt::test]
    async fn test_stream_video_endpoint() {
        let app = test::init_service(
            App::new()
                .configure(configure)
        ).await;
        
        let stream_id = Uuid::new_v4();
        let req = test::TestRequest::get()
            .uri(&format!("/stream/video/{}", stream_id))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_start_live_stream() {
        let app = test::init_service(
            App::new()
                .configure(configure)
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/stream/live")
            .insert_header(("Authorization", "test-token"))
            .set_json(json!({
                "title": "Test Stream",
                "stream_type": "video"
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_start_live_stream_unauthorized() {
        let app = test::init_service(
            App::new()
                .configure(configure)
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/stream/live")
            .set_json(json!({
                "title": "Test Stream",
                "stream_type": "video"
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_rt::test]
    async fn test_invalid_stream_type() {
        let app = test::init_service(
            App::new()
                .configure(configure)
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/stream/live")
            .insert_header(("Authorization", "test-token"))
            .set_json(json!({
                "title": "Test Stream",
                "stream_type": "invalid_type"
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_rt::test]
    async fn test_stop_stream_endpoint() {
        let app = test::init_service(
            App::new()
                .configure(configure)
        ).await;
        
        let stream_id = Uuid::new_v4();
        let req = test::TestRequest::post()
            .uri(&format!("/stream/{}/stop", stream_id))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_stream_audio_endpoint() {
        let app = test::init_service(
            App::new()
                .configure(configure)
        ).await;
        
        let stream_id = Uuid::new_v4();
        let req = test::TestRequest::get()
            .uri(&format!("/stream/audio/{}", stream_id))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_get_nonexistent_stream() {
        let app = test::init_service(
            App::new()
                .configure(configure)
        ).await;
        
        let stream_id = Uuid::new_v4();
        let req = test::TestRequest::get()
            .uri(&format!("/stream/{}/info", stream_id))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_rt::test]
    async fn test_malformed_stream_request() {
        let app = test::init_service(
            App::new()
                .configure(configure)
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/stream/live")
            .insert_header(("Authorization", "test-token"))
            .set_json(json!({
                "stream_type": "video"
                // missing title field
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }
}
