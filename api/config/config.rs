use std::env;
use std::fs;

use actix_web::web;
use cargo_metadata::MetadataCommand;
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::Database;

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

pub struct Cors {
    pub allowed_origin: String
}

pub struct Configuration {
    pub server: Server,
    pub swagger: Swagger,
    pub db: Database,
    pub cors: Cors
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
        
        let root = meta
        .root_package()
        .unwrap();

        let rust_version = root.rust_version
        .clone()
        .unwrap()
        .to_string();

        let actix_version = root.dependencies
        .iter()
        .find(|dependency| dependency.name == "actix-web")
        .take()
        .unwrap()
        .req.to_string();

        let mut host = "127.0.0.1".to_string();
    
        if env::var_os("RUST_LOG").is_none() {
            env::set_var("RUST_LOG", "actix_web=debug");
        }

        if env::var("HOST").is_ok() {
            host = env::var("HOST").unwrap();
        }

        let mut mongo_user = "".to_string();
        let mut mongo_password = "".to_string();
        let mut mongo_host = "".to_string();
        let mut mongo_db = "".to_string();

        if env::var("MONGO_USER").is_ok() {
            mongo_user = env::var("MONGO_USER").unwrap();
        }
    
        if env::var("MONGO_PASSWORD").is_ok() {
            mongo_password = env::var("MONGO_PASSWORD").unwrap();
        }
    
        if env::var("MONGO_HOST").is_ok() {
            mongo_host = env::var("MONGO_HOST").unwrap();
        }
    
        if env::var("MONGO_DB").is_ok() {
            mongo_db = env::var("MONGO_DB").unwrap();
        }

        let mongo_uri = format!("mongodb+srv://{0}:{1}@{2}", mongo_user, mongo_password, mongo_host);
    
        let client_options = ClientOptions::parse(mongo_uri).await.unwrap();

        // Create the MongoDB client
        let client = Client::with_options(client_options).unwrap();
    
        // Get the database
        let database = client.database(&mongo_db);

        let mut allowed_origin = "".to_string();

        if env::var("CORS_ORIGIN").is_ok() {
            allowed_origin = env::var("CORS_ORIGIN").unwrap();
        }

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
            db: database,
            cors: Cors {
                allowed_origin: allowed_origin
            }
        };

        let log_level = env::var_os("RUST_LOG").unwrap();

        let mut banner = fs::read_to_string("api/config/banner.txt").unwrap();

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