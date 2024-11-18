use std::env;
use std::fs;
use std::time::Duration;

use actix_web::web;
use cargo_metadata::MetadataCommand;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;

use crate::routes::health_route::health_checker_handler;
use crate::routes::movie_route::{get_movies_handler, post_movies_handler};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(health_checker_handler)
    .service(get_movies_handler)
    .service(post_movies_handler);
}

pub struct Server {
    pub host: String,
    pub path: String,
    pub port: u16
}

pub struct Swagger {
    pub title: String,
    pub version: String
}

pub struct Configuration {
    pub server: Server,
    pub swagger: Swagger,
    pub db: DatabaseConnection
}

impl Configuration {
    pub async fn init() -> Self {

        const VERSION: &str = env!("CARGO_PKG_VERSION");

        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let meta = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .current_dir(&path)
            .exec()
            .unwrap();
        
        let root = meta.root_package().unwrap();
        let rust_version = root.rust_version.clone().unwrap().to_string();
        let actix_version = root.dependencies.iter().find(|d| d.name == "actix-web").take().unwrap().req.to_string();

        let mut host = "127.0.0.1".to_string();
    
        if env::var_os("RUST_LOG").is_none() {
            env::set_var("RUST_LOG", "actix_web=debug");
        }

        if env::var("HOST").is_ok() {
            host = env::var("HOST").unwrap();
        }

        let mut db_user = "".to_string();
        let mut db_password = "".to_string();
        let mut db_host = "".to_string();
        let mut db_name = "".to_string();

        if env::var("DB_USER").is_ok() {
            db_user = env::var("DB_USER").unwrap();
        }
    
        if env::var("DB_PASSWORD").is_ok() {
            db_password = env::var("DB_PASSWORD").unwrap();
        }
    
        if env::var("DB_HOST").is_ok() {
            db_host = env::var("DB_HOST").unwrap();
        }
    
        if env::var("DB_NAME").is_ok() {
            db_name = env::var("DB_NAME").unwrap();
        }
    
        let mut opt = ConnectOptions::new(format!("postgres://{}:{}@{}/{}", db_user, db_password, db_host, db_name));

        opt
            .max_connections(10)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(50))
            .acquire_timeout(Duration::from_secs(50))
            .idle_timeout(Duration::from_secs(50))
            .max_lifetime(Duration::from_secs(30))
            .set_schema_search_path("public");
    
        let db = Database::connect(opt).await.unwrap();
        
        let configuration = Configuration {
            server: Server {
                host: host,
                path: "/api".to_string(),
                port: 8000
            },
            swagger: Swagger {
                title: "ms-movies".to_string(),
                version: VERSION.to_owned()
            },
            db
        };

        let log_level = env::var_os("RUST_LOG").unwrap();

        let mut banner = fs::read_to_string("src/config/banner.txt").unwrap();

        banner = banner.replace("package.name", &configuration.swagger.title);
        banner = banner.replace("package.version", &configuration.swagger.version);
        banner = banner.replace("rust.version", &rust_version);
        banner = banner.replace("actix.version", &actix_version.replace("^", ""));
        banner = banner.replace("server.path", &configuration.server.path);
        banner = banner.replace("server.port", configuration.server.port.to_string().as_str());
        banner = banner.replace("log.level", &log_level.to_str().unwrap().replace("actix_web=", ""));

        println!("{banner}");

        return configuration;
    }
}