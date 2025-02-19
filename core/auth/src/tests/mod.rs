use actix_web::{test, web, App};
use crate::{handlers, models::{LoginRequest, RegisterRequest}};

#[actix_rt::test]
async fn test_login_success() {
    let app = test::init_service(
        App::new().service(web::scope("/auth").route("/login", web::post().to(handlers::login)))
    ).await;

    let req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(LoginRequest {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_login_failure() {
    let app = test::init_service(
        App::new().service(web::scope("/auth").route("/login", web::post().to(handlers::login)))
    ).await;

    let req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(LoginRequest {
            username: "invalid".to_string(),
            password: "wrong".to_string(),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_rt::test]
async fn test_register() {
    let app = test::init_service(
        App::new().service(web::scope("/auth").route("/register", web::post().to(handlers::register)))
    ).await;

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(RegisterRequest {
            username: "newuser".to_string(),
            email: "new@user.com".to_string(),
            password: "password123".to_string(),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
