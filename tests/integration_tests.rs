use actix_web::{test, web, App, http::header};
use actix_web::web::Bytes;  // Usar Bytes do actix-web
use socialhub_auth;
use socialhub_media;
use socialhub_social;
use socialhub_streaming;
use socialhub_addon_manager;

#[actix_rt::test]
async fn test_complete_flow() {
    let app = test::init_service(
        App::new()
            .configure(|cfg| {
                socialhub_auth::configure(cfg);
                socialhub_media::configure(cfg);
                socialhub_social::configure(cfg);
                socialhub_streaming::configure(cfg);
                socialhub_addon_manager::configure(cfg);
            })
    ).await;

    // TODO: Implementar testes de fluxo completo
    assert!(true);
}

#[actix_rt::test]
async fn test_auth_with_media_upload() {
    let app = test::init_service(
        App::new()
            .configure(socialhub_auth::configure)
            .configure(socialhub_media::configure)
    ).await;

    // Primeiro faz login
    let login_req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(serde_json::json!({
            "username": "testuser",
            "password": "password123"
        }))
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert!(login_resp.status().is_success());

    // Depois tenta upload usando multipart
    let payload = concat!(
        "--abbc761f78ff4d7cb7573b5a23f96ef0\r\n",
        "Content-Disposition: form-data; name=\"file\"; filename=\"test.jpg\"\r\n",
        "Content-Type: image/jpeg\r\n\r\n",
        "test file content",
        "\r\n--abbc761f78ff4d7cb7573b5a23f96ef0--\r\n"
    );

    let upload_req = test::TestRequest::post()
        .uri("/media/upload")
        .insert_header(("Authorization", "Bearer test-token"))
        .insert_header((
            header::CONTENT_TYPE,
            "multipart/form-data; boundary=abbc761f78ff4d7cb7573b5a23f96ef0"
        ))
        .set_payload(payload)
        .to_request();

    let upload_resp = test::call_service(&app, upload_req).await;
    assert!(upload_resp.status().is_success());
}
