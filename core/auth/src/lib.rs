//! Authentication module for SocialHub
//! 
//! This module handles user authentication, registration and session management.
//! 
//! # Examples
//! 
//! ```rust
//! use socialhub_auth;
//! use actix_web::{web, App, HttpResponse};
//! 
//! async fn configure_app() {
//!     let app = App::new()
//!         .configure(socialhub_auth::configure);
//! }
//! ```

#[allow(unused_imports)]
use actix_web::{web, HttpResponse};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

pub mod handlers;
pub mod models;
mod service;
mod error;

pub use handlers::*;
pub use models::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(handlers::login))
            .route("/register", web::post().to(handlers::register))
            .route("/logout", web::post().to(handlers::logout))
    );
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use actix_web::{test, App};
    use crate::{handlers, models::{LoginRequest, RegisterRequest}};
    use serde_json::json;

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
    async fn test_register_duplicate_user() {
        let app = test::init_service(
            App::new().service(web::scope("/auth").route("/register", web::post().to(handlers::register)))
        ).await;

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(json!({
                "username": "existing_user",
                "email": "existing@test.com",
                "password": "password123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 409); // Conflict
    }

    #[actix_rt::test]
    async fn test_logout_without_token() {
        let app = test::init_service(
            App::new().service(web::scope("/auth").route("/logout", web::post().to(handlers::logout)))
        ).await;

        let req = test::TestRequest::post()
            .uri("/auth/logout")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 401); // Unauthorized
    }

    #[actix_rt::test]
    async fn test_logout_with_token() {
        let app = test::init_service(
            App::new().service(web::scope("/auth").route("/logout", web::post().to(handlers::logout)))
        ).await;

        let req = test::TestRequest::post()
            .uri("/auth/logout")
            .insert_header(("Authorization", "Bearer valid-token"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
