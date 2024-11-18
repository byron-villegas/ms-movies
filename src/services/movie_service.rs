use sea_orm::{DatabaseConnection, Set};

use crate::{
    dtos::movie_dto::MovieDto, entities::movie, repositories::movie_repository
};

pub async fn get_movies(db: DatabaseConnection) -> Vec<MovieDto> {
    let movies = movie_repository::find_all(db).await;

    let response: Vec<MovieDto> = movies
        .into_iter()
        .map(|m| MovieDto {
            id: m.id,
            title: m.title,
            originalTitle: m.original_title,
            sypnosis: m.sypnosis,
            image: m.image,
            year: m.year,
            duration: m.duration,
            durationType: m.duration_type,
            resolutionWidth: m.resolution_width,
            resolutionHeight: m.resolution_height,
            size: m.size,
            sizeType: m.size_type,
            format: m.format,
            
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