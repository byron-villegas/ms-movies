use actix_web::{http::StatusCode, web, HttpResponse};
use mongodb::Database;

use crate::{dtos::movie_dto::MovieDto, services::movie_service};

pub async fn get_movies_cr(db: Database) -> HttpResponse {
    let movies = movie_service::get_movies(db).await;

    HttpResponse::Ok().json(movies)
}

pub async fn post_movies(db: Database, movie_dto: web::Json<MovieDto>) -> HttpResponse {
    //let movie = movie_service::post_movie(db, movie_dto.into_inner()).await;

    return HttpResponse::new(StatusCode::CREATED);
}