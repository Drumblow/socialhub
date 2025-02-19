pub mod auth_media_flow;
pub mod social_streaming_flow;

use actix_web::{
    dev::{Service, ServiceResponse},
    Error, web, App, test
};

pub async fn setup_test_app() -> impl Service<
    actix_web::dev::ServiceRequest,
    Response = ServiceResponse,
    Error = Error,
> {
    test::init_service(
        App::new()
            .configure(socialhub_auth::configure)
            .configure(socialhub_media::configure)
            .configure(socialhub_social::configure)
            .configure(socialhub_streaming::configure)
    ).await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[actix_rt::test]
    async fn test_full_integration() {
        let app = setup_test_app().await;
        // Aqui podemos adicionar testes que usam múltiplos módulos
        assert!(true);
    }
}
