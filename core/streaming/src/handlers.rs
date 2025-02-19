use actix_web::{web, HttpResponse, Error as ActixError};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use log::warn;
use crate::service::StreamingService;
use crate::models::StreamType;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StreamRequest {
    pub title: String,
    pub stream_type: String,
}

#[utoipa::path(
    get,
    path = "/stream/video/{id}",
    params(("id" = Uuid, Path, description = "Stream ID")),
    responses(
        (status = 200, description = "Stream found"),
        (status = 404, description = "Stream not found")
    ),
    tag = "streaming"
)]
pub async fn stream_video(_id: web::Path<Uuid>) -> Result<HttpResponse, ActixError> {
    // TODO: Implement video streaming
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    get,
    path = "/stream/audio/{id}",
    params(("id" = Uuid, Path, description = "Stream ID")),
    responses(
        (status = 200, description = "Stream found"),
        (status = 404, description = "Stream not found")
    ),
    tag = "streaming"
)]
pub async fn stream_audio(_id: web::Path<Uuid>) -> Result<HttpResponse, ActixError> {
    // TODO: Implement audio streaming
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    post,
    path = "/stream/live",
    request_body = StreamRequest,
    responses(
        (status = 200, description = "Stream started successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid request")
    ),
    security(("bearer_token" = [])),
    tag = "streaming"
)]
pub async fn start_live(
    service: web::Data<StreamingService>,
    req: actix_web::HttpRequest,
    stream_req: web::Json<StreamRequest>
) -> Result<HttpResponse, ActixError> {
    if req.headers().get("Authorization").is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let stream_type = match stream_req.stream_type.as_str() {
        "video" => StreamType::Video,
        "audio" => StreamType::Audio,
        _ => {
            warn!("Invalid stream type received: {}", stream_req.stream_type);
            return Ok(HttpResponse::BadRequest().json("Invalid stream type"));
        }
    };

    service.start_stream(1, stream_type)
        .await
        .map(|stream| HttpResponse::Ok().json(stream))
        .map_err(ActixError::from)
}

#[utoipa::path(
    post,
    path = "/stream/{id}/stop",
    params(("id" = Uuid, Path, description = "Stream ID to stop")),
    responses(
        (status = 200, description = "Stream stopped successfully"),
        (status = 404, description = "Stream not found")
    ),
    tag = "streaming"
)]
pub async fn stop_stream(
    service: web::Data<StreamingService>,
    stream_id: web::Path<Uuid>
) -> Result<HttpResponse, ActixError> {
    service.stop_stream(stream_id.into_inner())
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| e.into())
}

pub async fn get_stream_info(
    service: web::Data<StreamingService>,
    stream_id: web::Path<Uuid>
) -> Result<HttpResponse, ActixError> {
    match service.get_stream(stream_id.into_inner()).await {
        Ok(stream) => Ok(HttpResponse::Ok().json(stream)),
        Err(e) => Err(e.into())
    }
}
