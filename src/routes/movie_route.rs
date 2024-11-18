use actix_web::{get, post, web::{self, Data}, HttpResponse};
use crate::{controllers::movie_controller::{get_movies_cr, post_movies}, dtos::movie_dto::MovieDto, AppState};

#[get("/movies")]
async fn get_movies_handler(app_state: Data<AppState>) -> HttpResponse  {
    let db = &app_state.db;

    return get_movies_cr(db.clone()).await;
}

#[post("/movies")]
async fn post_movies_handler(movie_dto: web::Json<MovieDto>, app_state: Data<AppState>) -> HttpResponse  {
    let db = &app_state.db;
    
    return post_movies(db.clone(), movie_dto).await;
}