use actix_web::{test, App, http::header};
use socialhub_auth;
use socialhub_media;
use socialhub_social;
use socialhub_streaming;
use socialhub;

#[actix_rt::test]
async fn test_complete_flow() {
    let app = test::init_service(
        App::new()
            .configure(|cfg| {
                socialhub_auth::configure(cfg);
                socialhub_media::configure(cfg);
                socialhub_social::configure(cfg);
                socialhub_streaming::configure(cfg);
                socialhub::configure(cfg);
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

#[actix_rt::test]
async fn test_addon_integration() {
    let app = test::init_service(
        App::new()
            .configure(socialhub::configure)
            .configure(addon_manager::configure)  // Adicionar configuração do addon_manager
    ).await;

    // Primeiro instalar um addon para testar
    let install_req = test::TestRequest::post()
        .uri("/addons/install")
        .set_json(serde_json::json!({
            "name": "test-addon",
            "version": "1.0.0"
        }))
        .to_request();

    let install_resp = test::call_service(&app, install_req).await;
    assert!(install_resp.status().is_success(), "Failed to install addon");

    // Então tentar listar os addons
    let list_req = test::TestRequest::get()
        .uri("/addons/list")
        .to_request();

    let list_resp = test::call_service(&app, list_req).await;
    assert!(list_resp.status().is_success(), "Failed to list addons");

    // Verificar o conteúdo da resposta
    let body: serde_json::Value = test::read_body_json(list_resp).await;
    assert!(body.as_array().unwrap().len() > 0, "Addon list should not be empty");
}

#[actix_rt::test]
async fn test_addon_installation() {
    let app = test::init_service(
        App::new()
            .configure(socialhub::configure)
    ).await;

    let resp = test::call_service(&app,
        test::TestRequest::post()
            .uri("/addons/install")
            .set_json(serde_json::json!({
                "name": "test-addon",
                "version": "1.0.0"
            }))
            .to_request()
    ).await;

    assert!(resp.status().is_success());
}
