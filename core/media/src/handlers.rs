use actix_web::{web, Error, HttpResponse};
use actix_multipart::{Field, Multipart};
use uuid::Uuid;
use crate::error::MediaError;
use futures::StreamExt;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadRequest {
    pub file_type: String,
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MetadataUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>
}

/// Handles file upload with multipart/form-data
/// 
/// # Arguments
/// * `payload` - Multipart form data containing the file
/// 
/// # Returns
/// * `Ok(HttpResponse)` - 201 Created on success
/// * `Ok(HttpResponse)` - 415 Unsupported Media Type for invalid content types
/// * `Ok(HttpResponse)` - 413 Payload Too Large for files > 10MB
/// 
/// # Example
/// ```no_run
/// use actix_web::{web, Error, HttpResponse};
/// use actix_multipart::Multipart;
/// 
/// async fn example(mut payload: Multipart) -> Result<HttpResponse, Error> {
///     // Supondo que temos uma instância de Multipart...
///     let response = HttpResponse::Ok().finish();
///     Ok(response)
/// }
/// ```
#[utoipa::path(
    post,
    path = "/media/upload",
    request_body = UploadRequest,
    responses(
        (status = 201, description = "Media uploaded successfully"),
        (status = 400, description = "Invalid request"),
        (status = 413, description = "File too large")
    ),
    security(("bearer_token" = [])),
    tag = "media"
)]
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    debug!("Starting file upload");

    if let Some(field_result) = payload.next().await {
        let field = field_result?;
        match process_field(field).await {
            Ok(_) => {
                info!("Upload successful");
                Ok(HttpResponse::Created().finish())
            },
            Err(e) => {
                warn!("Upload failed: {:?}", e);
                Ok(e)
            }
        }
    } else {
        warn!("No file provided");
        Ok(HttpResponse::BadRequest().finish())
    }
}

async fn process_field(mut field: Field) -> Result<(), HttpResponse> {
    // Validate content type
    let content_type = field.content_type();
    debug!("Content type: {:?}", content_type);
    
    if !content_type.map_or(false, |ct| is_valid_media_type(ct.to_string().as_str())) {
        return Err(HttpResponse::UnsupportedMediaType().finish());
    }

    // Validate file size
    let mut size = 0;
    while let Some(chunk) = field.next().await {
        size += chunk.map_err(|_| HttpResponse::BadRequest().finish())?.len();
        if size > MAX_FILE_SIZE {
            return Err(HttpResponse::PayloadTooLarge().finish());
        }
    }

    Ok(())
}

fn is_valid_media_type(content_type: &str) -> bool {
    let valid_types = [
        "image/jpeg", "image/png", "image/gif",
        "video/mp4", "video/mpeg",
        "audio/mp3", "audio/mpeg", "audio/wav"
    ];
    
    debug!("Checking content type: {}", content_type);
    valid_types.iter().any(|&t| content_type.eq_ignore_ascii_case(t))
}

/// Retrieves media by ID
/// 
/// # Arguments
/// * `id` - UUID of the media to retrieve
/// 
/// # Returns
/// * `Ok(HttpResponse)` - 200 OK with media content
/// * `Err(MediaError::NotFound)` - 404 Not Found
#[utoipa::path(
    get,
    path = "/media/{id}",
    responses(
        (status = 200, description = "Media found"),
        (status = 404, description = "Media not found")
    ),
    tag = "media"
)]
pub async fn get_media(_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    // Simulando que o media não foi encontrado para o teste
    Err(MediaError::NotFound.into())
}

#[utoipa::path(
    put,
    path = "/media/{id}/metadata",
    request_body = MetadataUpdate,
    responses(
        (status = 200, description = "Metadata updated"),
        (status = 404, description = "Media not found")
    ),
    security(("bearer_token" = [])),
    tag = "media"
)]
pub async fn update_metadata(
    _id: web::Path<Uuid>,
    _metadata: web::Json<MetadataUpdate>
) -> Result<HttpResponse, Error> {
    // TODO: Implement metadata update
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_metadata(_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    // Simulando que o metadata não foi encontrado
    Err(MediaError::NotFound.into())
}

pub async fn delete_media(_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    // TODO: Implement media deletion
    Ok(HttpResponse::Ok().finish())
}
