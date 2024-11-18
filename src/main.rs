mod config;
mod controllers;
mod dtos;
mod entities;
mod routes;
mod services;
mod repositories;

#[cfg(test)]
mod tests;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;

use crate::config::config::config;
use crate::config::config::Configuration;

pub struct AppState {
    db: DatabaseConnection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let configuration = Configuration::init().await;

    env_logger::init();

    println!("Server started successfully on http://{0}:{1}{2}", configuration.server.host, configuration.server.port, configuration.server.path);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: configuration.db.clone() }))
            .service(web::scope(&configuration.server.path)
            .configure(config))
            .wrap(Logger::default())
    })
    .bind((configuration.server.host, configuration.server.port))?
    .run()
    .await
}
