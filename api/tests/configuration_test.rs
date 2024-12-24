#[cfg(test)]
mod tests {
    use std::env;
    use std::env::set_var;

    use actix_web::{test, web, App};

    use crate::config::config::Configuration;
    use crate::config::config::config;

    #[actix_web::test]
    async fn test_config() {
        env::remove_var("RUST_LOG");
        
        let configuration = Configuration::init().await;
        let app = test::init_service(App::new().service(web::scope(&configuration.server.path)
        .configure(config))).await;
        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.response().status().is_success());
    }

    #[actix_web::test]
    async fn test_config_with_log_env_set() {
        set_var("RUST_LOG", "actix_web=info");
        env::set_var("HOST", "127.0.0.1");
        
        let configuration = Configuration::init().await;
        let app = test::init_service(App::new().service(web::scope(&configuration.server.path)
        .configure(config))).await;
        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.response().status().is_success());
    }
}
