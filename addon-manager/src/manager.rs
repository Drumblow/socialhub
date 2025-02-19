use actix_web::{web, Error, HttpResponse};
use uuid::Uuid;
use crate::plugin::Plugin;
use crate::sandbox::Sandbox;

pub struct AddonManager {
    plugins: Vec<Plugin>,
    sandbox: Sandbox,
}

impl AddonManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            sandbox: Sandbox::new(std::path::PathBuf::from("./addons")),
        }
    }
}

pub async fn install_addon(_data: web::Json<serde_json::Value>) -> Result<HttpResponse, Error> {
    // TODO: Implementar instalação de addon
    Ok(HttpResponse::Ok().finish())
}

pub async fn list_addons() -> Result<HttpResponse, Error> {
    // TODO: Implementar listagem de addons
    Ok(HttpResponse::Ok().finish())
}

pub async fn enable_addon(_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    // TODO: Implementar ativação de addon
    Ok(HttpResponse::Ok().finish())
}

pub async fn disable_addon(_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    // TODO: Implementar desativação de addon
    Ok(HttpResponse::Ok().finish())
}
