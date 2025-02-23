pub mod auth_media_flow;
pub mod social_streaming_flow;

use actix_web::{
    test,
    dev::{Service, ServiceRequest, ServiceResponse},
    Error, App
};
use crate::addon_manager;

pub async fn setup_test_app() -> impl Service<ServiceRequest, Response = ServiceResponse, Error = Error> {
    test::init_service(
        App::new()
            .configure(|cfg| {
                addon_manager::configure(cfg);
            })
    ).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_addon_endpoints() {
        let app = setup_test_app().await;
        
        let req = test::TestRequest::get()
            .uri("/addons/list")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
