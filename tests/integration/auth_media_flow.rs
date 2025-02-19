use actix_web::{test, App, http::header};
use socialhub_auth;
use socialhub_media;
use uuid::Uuid;

#[actix_rt::test]
async fn test_auth_media_upload_flow() {
    // Setup da aplicação com múltiplos módulos
    let app = test::init_service(
        App::new()
            .configure(socialhub_auth::configure)
            .configure(socialhub_media::configure)
    ).await;

    // 1. Login
    let login_resp = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(serde_json::json!({
            "username": "test_user",
            "password": "test_pass"
        }))
        .send_request(&app)
        .await;

    assert!(login_resp.status().is_success());
    
    let auth_token = login_resp
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // 2. Upload de mídia com token
    let payload = concat!(
        "--boundary\r\n",
        "Content-Disposition: form-data; name=\"file\"; filename=\"test.jpg\"\r\n",
        "Content-Type: image/jpeg\r\n\r\n",
        "binary_image_data\r\n",
        "--boundary--\r\n"
    );

    let upload_resp = test::TestRequest::post()
        .uri("/media/upload")
        .insert_header(("Authorization", auth_token.clone())) // Clone here
        .insert_header((
            header::CONTENT_TYPE, 
            "multipart/form-data; boundary=boundary"
        ))
        .set_payload(payload)
        .send_request(&app)
        .await;

    assert!(upload_resp.status().is_success());

    // 3. Verificar metadata da mídia
    let media_id = Uuid::new_v4(); // Na prática, viria da resposta do upload
    let metadata_resp = test::TestRequest::get()
        .uri(&format!("/media/{}/metadata", media_id))
        .insert_header(("Authorization", auth_token))
        .send_request(&app)
        .await;

    assert!(metadata_resp.status().is_success());
}
