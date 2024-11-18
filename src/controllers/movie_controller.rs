use actix_web::{http::StatusCode, web::{self}, HttpResponse};
use sea_orm::DatabaseConnection;

use crate::{dtos::movie_dto::MovieDto, services::movie_service::{add_movie, get_movies}};

pub async fn get_movies_cr(db: DatabaseConnection) -> HttpResponse {
    let movies = get_movies(db).await;

    HttpResponse::Ok().json(movies)
}

pub async fn post_movies(db: DatabaseConnection, movie_dto: web::Json<MovieDto>) -> HttpResponse {
    
    add_movie(db, movie_dto.0).await;

    HttpResponse::new (StatusCode::CREATED)
}