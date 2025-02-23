use actix_web::{web, HttpResponse, Error, Responder};
use serde_json::{Value, json};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/addons")
            .route("/install", web::post().to(install))
            .route("/list", web::get().to(list))
    );
}

pub async fn install(payload: web::Json<Value>) -> Result<HttpResponse, Error> {
    // TODO: Implementar lógica de instalação do addon
    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "message": "Addon installed successfully",
        "data": payload.0
    })))
}

pub async fn list() -> impl Responder {
    // Por enquanto, retornar uma lista mock
    HttpResponse::Ok().json(json!([
        {
            "name": "test-addon",
            "version": "1.0.0",
            "status": "installed"
        }
    ]))
}