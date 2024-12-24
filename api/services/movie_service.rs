use mongodb::Database;

use crate::{dtos::movie_dto::MovieDto, repositories::movie_repository};

pub async fn get_movies(db: Database) -> Vec<MovieDto> {
    let movies = movie_repository::find_all(db).await;

    let response: Vec<MovieDto> = movies
        .iter()
        .map(|movie| MovieDto {
            id: 1,
            title: movie.title.clone(),
            original_title: movie.original_title.clone(),
            sypnosis: movie.sypnosis.clone(),
            image: movie.image.clone(),
            year: movie.year,
            duration: movie.duration,
            duration_type: movie.duration_type.clone(),
            genres: movie.genres.clone(),
            languages: movie.languages.clone(),
            resolution_width: movie.resolution_width,
            resolution_height: movie.resolution_height,
            size: movie.size,
            size_type: movie.size_type.clone(),
            format: movie.format.clone(),
        })
        .collect();

    return response;
}