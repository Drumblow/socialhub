use actix_web::{web, Error, HttpResponse, HttpRequest};
use crate::models::{LoginRequest, RegisterRequest, AuthResponse};
use crate::error::AuthError;

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "auth"
)]
pub async fn login(credentials: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
    if credentials.username == "invalid" {
        return Ok(HttpResponse::Unauthorized().json("Invalid credentials"));
    }
    
    Ok(HttpResponse::Ok().json(AuthResponse {
        token: "dummy_token".to_string(),
        user_id: 1,
    }))
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 409, description = "Username already exists")
    ),
    tag = "auth"
)]
pub async fn register(user_data: web::Json<RegisterRequest>) -> Result<HttpResponse, Error> {
    if user_data.username == "existing_user" {
        return Ok(HttpResponse::Conflict().json("User already exists"));
    }
    
    Ok(HttpResponse::Ok().json("User registered successfully"))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Not authenticated")
    ),
    security(("bearer_token" = [])),
    tag = "auth"
)]
pub async fn logout(req: HttpRequest) -> Result<HttpResponse, Error> {
    match req.headers().get("Authorization") {
        Some(_) => Ok(HttpResponse::Ok().finish()),
        None => Ok(HttpResponse::Unauthorized().finish())
    }
}
