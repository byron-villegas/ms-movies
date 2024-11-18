#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    
    use crate::routes::health_route::health_checker_handler;

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = test::init_service(App::new().service(web::scope("/api")
        .service(health_checker_handler))).await;
        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.response().status().is_success());
    }
}
