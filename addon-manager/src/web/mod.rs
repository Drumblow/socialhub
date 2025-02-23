use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::error::AddonError;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct AddonConfig {
    #[schema(example = "my-addon")]
    pub id: String,
    #[schema(example = true)]
    pub enabled: bool,
    #[schema(example = "vec![\"Action\".to_string(), \"Drama\".to_string()]")]
    pub catalog_filters: Option<Vec<String>>,
    #[schema(example = 50)]
    pub max_results: Option<usize>,
    #[schema(example = "HD")]
    pub preferred_quality: Option<String>,
}

#[utoipa::path(
    post,
    path = "/addon/{id}/configure",
    request_body = AddonConfig,
    responses(
        (status = 200, description = "Addon configured successfully", body = AddonConfig),
        (status = 400, description = "Invalid configuration")
    )
)]
pub async fn configure_addon(
    config: web::Json<AddonConfig>,
) -> Result<HttpResponse, AddonError> {
    Ok(HttpResponse::Ok().json(config.0))
}

#[utoipa::path(
    get,
    path = "/addon/{id}/config",
    responses(
        (status = 200, description = "Addon configuration retrieved", body = AddonConfig),
        (status = 404, description = "Addon not found")
    )
)]
pub async fn get_addon_config(
    id: web::Path<String>,
) -> Result<HttpResponse, AddonError> {
    let config = AddonConfig {
        id: id.into_inner(),
        enabled: true,
        catalog_filters: None,
        max_results: Some(50),
        preferred_quality: Some("HD".to_string()),
    };
    
    Ok(HttpResponse::Ok().json(config))
}

#[cfg(test)]
mod tests;
