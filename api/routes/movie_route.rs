use actix_web::{get, post, web::{self, Data}, HttpResponse};
use crate::{controllers::movie_controller, dtos::movie_dto::MovieDto, AppState};

#[get("/movies")]
async fn get_movies_handler(app_state: Data<AppState>) -> HttpResponse {
    return movie_controller::get_movies_cr(app_state.db.clone()).await;
}

#[post("/movies")]
async fn post_movies_handler(movie_dto: web::Json<MovieDto>, app_state: Data<AppState>) -> HttpResponse  {
    return movie_controller::post_movies(app_state.db.clone(), movie_dto).await;
}