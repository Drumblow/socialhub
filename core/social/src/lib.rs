use actix_web::web;

pub mod handlers;
pub mod models;
mod service;
mod error;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/social")
            .service(web::resource("/posts").route(web::post().to(handlers::create_post)))
            .service(web::resource("/posts/{id}").route(web::get().to(handlers::get_post)))
            .service(web::resource("/posts/{id}/like").route(web::post().to(handlers::like_post)))
            .service(web::resource("/users/{id}/follow").route(web::post().to(handlers::follow_user)))
    );
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use actix_web::{test, App};
    use serde_json::json;
    use uuid::Uuid;

    #[actix_rt::test]
    async fn test_create_post() {
        let app = test::init_service(
            App::new().service(web::scope("/social").route("/posts", web::post().to(handlers::create_post)))
        ).await;

        let req = test::TestRequest::post()
            .uri("/social/posts")
            .insert_header(("Authorization", "Bearer test-token"))
            .set_json(json!({
                "content": "Test post content",
                "media_ids": [],
                "tags": ["test", "first"]
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_like_post() {
        let post_id = Uuid::new_v4();
        let app = test::init_service(
            App::new().service(web::scope("/social").route("/posts/{id}/like", web::post().to(handlers::like_post)))
        ).await;

        let req = test::TestRequest::post()
            .uri(&format!("/social/posts/{}/like", post_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_create_post_without_auth() {
        let app = test::init_service(
            App::new().service(web::scope("/social").route("/posts", web::post().to(handlers::create_post)))
        ).await;

        let req = test::TestRequest::post()
            .uri("/social/posts")
            .set_json(json!({
                "content": "Test content"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 401);
    }

    #[actix_rt::test]
    async fn test_like_own_post() {
        let post_id = Uuid::new_v4();
        let app = test::init_service(
            App::new().service(web::scope("/social").route("/posts/{id}/like", web::post().to(handlers::like_post)))
        ).await;

        let req = test::TestRequest::post()
            .uri(&format!("/social/posts/{}/like", post_id))
            .insert_header(("Authorization", "Bearer test-token"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400); // Bad Request - Can't like own post
    }

    #[actix_rt::test]
    async fn test_get_post_not_found() {
        let post_id = Uuid::new_v4();
        let app = test::init_service(
            App::new().service(web::scope("/social").route("/posts/{id}", web::get().to(handlers::get_post)))
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/social/posts/{}", post_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 404);
    }

    #[actix_rt::test]
    async fn test_follow_user_not_found() {
        let user_id = 999999;
        let app = test::init_service(
            App::new().service(web::scope("/social").route("/users/{id}/follow", web::post().to(handlers::follow_user)))
        ).await;

        let req = test::TestRequest::post()
            .uri(&format!("/social/users/{}/follow", user_id))
            .insert_header(("Authorization", "Bearer test-token"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 404);
    }
}
