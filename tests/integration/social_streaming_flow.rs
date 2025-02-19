use actix_web::{test, App};
use socialhub_social;
use socialhub_streaming;
use uuid::Uuid;

#[actix_rt::test]
async fn test_social_streaming_integration() {
    let app = test::init_service(
        App::new()
            .configure(socialhub_social::configure)
            .configure(socialhub_streaming::configure)
    ).await;

    // 1. Iniciar uma stream
    let stream_resp = test::TestRequest::post()
        .uri("/stream/live")
        .set_json(serde_json::json!({
            "title": "Test Stream",
            "stream_type": "video"
        }))
        .insert_header(("Authorization", "Bearer test-token"))
        .send_request(&app)
        .await;

    assert!(stream_resp.status().is_success());
    
    // 2. Criar post sobre a stream
    let post_resp = test::TestRequest::post()
        .uri("/social/posts")
        .set_json(serde_json::json!({
            "content": "Check out my live stream!",
            "stream_id": Uuid::new_v4(),
            "media_type": "stream"
        }))
        .insert_header(("Authorization", "Bearer test-token"))
        .send_request(&app)
        .await;

    assert!(post_resp.status().is_success());

    // 3. Verificar informações da stream
    let stream_info_resp = test::TestRequest::get()
        .uri("/stream/info")
        .insert_header(("Authorization", "Bearer test-token"))
        .send_request(&app)
        .await;

    assert!(stream_info_resp.status().is_success());
}
