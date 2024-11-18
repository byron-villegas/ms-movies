use sea_orm::{DatabaseConnection, Set};

use crate::{
    dtos::movie_dto::MovieDto, entities::movie, repositories::movie_repository
};

pub async fn get_movies(db: DatabaseConnection) -> Vec<MovieDto> {
    let movies = movie_repository::find_all(db).await;

    let response: Vec<MovieDto> = movies
        .iter()
        .map(|m| MovieDto {
            id: m.movie.id,
            title: m.movie.title.clone(),
            originalTitle: m.movie.original_title.clone(),
            sypnosis: m.movie.sypnosis.clone(),
            image: m.movie.image.clone(),
            year: m.movie.year,
            duration: m.movie.duration,
            durationType: m.movie.duration_type.clone(),
            genres: m.genres.iter().map(|g| g.name.clone()).collect(),
            languages: m.languages.iter().map(|l| l.name.clone()).collect(),
            resolutionWidth: m.movie.resolution_width,
            resolutionHeight: m.movie.resolution_height,
            size: m.movie.size,
            sizeType: m.movie.size_type.clone(),
            format: m.movie.format.clone(),
            
        })
        .collect();

    return response;
}

pub async fn add_movie(db: DatabaseConnection, movie_dto: MovieDto) {
    let movie = movie::ActiveModel {
        title: Set(movie_dto.title),
        original_title: Set(movie_dto.originalTitle),
        sypnosis: Set(movie_dto.sypnosis),
        image: Set(movie_dto.image),
        year: Set(movie_dto.year),
        duration: Set(movie_dto.duration),
        duration_type: Set(movie_dto.durationType),
        resolution_width: Set(movie_dto.resolutionWidth),
        resolution_height: Set(movie_dto.resolutionHeight),
        size: Set(movie_dto.size),
        size_type: Set(movie_dto.sizeType),
        format: Set(movie_dto.format),
        ..Default::default()
    };

    movie_repository::save(db, movie).await;
}