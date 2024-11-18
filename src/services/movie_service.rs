use sea_orm::{DatabaseConnection, Set};

use crate::{
    dtos::movie_dto::MovieDto, entities::movie, repositories::movie_repository
};

pub async fn get_movies(db: DatabaseConnection) -> Vec<MovieDto> {
    let movies = movie_repository::find_all(db).await;

    let response: Vec<MovieDto> = movies
        .iter()
        .map(|m| MovieDto {
            id: m.0.id,
            title: m.0.title.clone(),
            originalTitle: m.0.original_title.clone(),
            sypnosis: m.0.sypnosis.clone(),
            image: m.0.image.clone(),
            year: m.0.year,
            duration: m.0.duration,
            durationType: m.0.duration_type.clone(),
            genres: m.1.iter().map(|g| g.name.clone()).collect(),
            resolutionWidth: m.0.resolution_width,
            resolutionHeight: m.0.resolution_height,
            size: m.0.size,
            sizeType: m.0.size_type.clone(),
            format: m.0.format.clone(),
            
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