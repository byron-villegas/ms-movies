mod config;
mod controllers;
mod dtos;
mod entities;
mod routes;
mod services;
mod repositories;

#[cfg(test)]
mod tests;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use mongodb::Database;

use crate::config::config::config;
use crate::config::config::Configuration;

pub struct AppState {
    db: Database
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let configuration = Configuration::init().await;

    env_logger::init();

    println!("Server started successfully on http://{0}:{1}{2}", configuration.server.host, configuration.server.port, configuration.server.path);

    HttpServer::new(move || {
        let cors = Cors::default()
        .allowed_origin(&configuration.cors.allowed_origin)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![actix_web::http::header::ACCEPT, actix_web::http::header::CONTENT_TYPE])
        .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(Data::new(AppState { db: configuration.db.clone() }))
            .service(web::scope(&configuration.server.path)
            .configure(config))
    })
    .bind((configuration.server.host, configuration.server.port))?
    .run()
    .await
}
