use actix_web::{http::StatusCode, web::{self}, HttpResponse};
use sea_orm::DatabaseConnection;

use crate::{dtos::movie_dto::MovieDto, services::movie_service};

pub async fn get_movies_cr(db: DatabaseConnection) -> HttpResponse {
    let movies = movie_service::get_movies(db).await;

    HttpResponse::Ok().json(movies)
}

pub async fn post_movies(db: DatabaseConnection, movie_dto: web::Json<MovieDto>) -> HttpResponse {
    
    let result = movie_service::add_movie(db, movie_dto.0).await;

    match result {
        Ok(_) => HttpResponse::new(StatusCode::CREATED),
        Err(error) => HttpResponse::BadRequest().json(error)
    }
}