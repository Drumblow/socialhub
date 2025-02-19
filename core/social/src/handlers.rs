use actix_web::{web, HttpResponse, Error};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePostRequest {
    pub content: String,
    pub media_ids: Vec<String>,
}

#[utoipa::path(
    post,
    path = "/social/posts",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_token" = [])),
    tag = "social"
)]
pub async fn create_post(
    req: actix_web::HttpRequest,
    post_data: web::Json<serde_json::Value>
) -> Result<HttpResponse, Error> {
    // Verificar autenticação
    if req.headers().get("Authorization").is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    // Criar post e retornar sucesso
    Ok(HttpResponse::Created().json(json!({
        "id": Uuid::new_v4(),
        "content": post_data.get("content").unwrap_or(&json!("")).to_string(),
        "created_at": "2025-02-18T19:00:00Z"
    })))
}

#[utoipa::path(
    get,
    path = "/social/posts/{id}",
    responses(
        (status = 200, description = "Post found"),
        (status = 404, description = "Post not found")
    ),
    tag = "social"
)]
pub async fn get_post(_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    // Simular post não encontrado
    Ok(HttpResponse::NotFound().finish())
}

#[utoipa::path(
    post,
    path = "/social/posts/{id}/like",
    responses(
        (status = 200, description = "Post liked"),
        (status = 400, description = "Cannot like own post")
    ),
    security(("bearer_token" = [])),
    tag = "social"
)]
pub async fn like_post(
    req: actix_web::HttpRequest,
    _post_id: web::Path<Uuid>
) -> Result<HttpResponse, Error> {
    // Simular que está tentando curtir próprio post
    if req.headers().get("Authorization").is_some() {
        return Ok(HttpResponse::BadRequest().json("Cannot like own post"));
    }
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    post,
    path = "/social/users/{id}/follow",
    responses(
        (status = 200, description = "User followed"),
        (status = 404, description = "User not found")
    ),
    security(("bearer_token" = [])),
    tag = "social"
)]
pub async fn follow_user(
    _user_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    // Simular usuário não encontrado
    Ok(HttpResponse::NotFound().finish())
}
