use sea_orm::{DatabaseConnection, Set};

use crate::{dtos::{movie_dto::MovieDto, validation_error_dto::ValidationErrorDto}, entities::{genre, language, movie, movie_genre, movie_language}, repositories::{genre_repository, language_repository, movie_genre_repository, movie_language_repository, movie_repository}};

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
            genres: m.genres.clone(),
            languages: m.languages.clone(),
            resolutionWidth: m.movie.resolution_width,
            resolutionHeight: m.movie.resolution_height,
            size: m.movie.size,
            sizeType: m.movie.size_type.clone(),
            format: m.movie.format.clone(),
        })
        .collect();

    return response;
}

pub async fn add_movie(db: DatabaseConnection, movie_dto: MovieDto) -> Result<String, Vec<ValidationErrorDto>> {

    let mut validations_errors: Vec<ValidationErrorDto> = Vec::new();

    if movie_dto.genres.is_empty() {
        validations_errors.push(ValidationErrorDto { field: "genres".to_string(), message: "Must have at least one gender".to_string() });
    }

    if movie_dto.languages.is_empty() {
        validations_errors.push(ValidationErrorDto { field: "language".to_string(), message: "Must have at least one language".to_string() });
    }

    if !validations_errors.is_empty() {
        return Err(validations_errors);
    }

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

    let movie_saved = movie_repository::save(db.clone(), movie).await;

    for genre_name in movie_dto.genres {
        let option_genre = genre_repository::find_by_name(db.clone(), genre_name.to_string()).await;

        let genre: genre::Model;

        if option_genre.is_none() {
            let genre_to_save = genre::ActiveModel {
                name: Set(genre_name.to_string()),
                ..Default::default()
            };

            genre = genre_repository::save(db.clone(), genre_to_save).await;

            println!("Genre Save {0}", genre.id);
        } else {
            genre = option_genre.unwrap().into();
        }

        let movie_genre = movie_genre::ActiveModel {
            movie_id: Set(movie_saved.id),
            genre_id: Set(genre.id),
            ..Default::default()
        };

        let movie_genre_saved: movie_genre::Model = movie_genre_repository::save(db.clone(), movie_genre).await;

        println!("Movie Genre {0}", movie_genre_saved.id);
    }

    for language_name in movie_dto.languages {
        let option_language = language_repository::find_by_name(db.clone(), language_name.to_string()).await;

        let language: language::Model;

        if option_language.is_none() {
            let language_to_save = language::ActiveModel {
                name: Set(language_name.to_string()),
                ..Default::default()
            };

            language = language_repository::save(db.clone(), language_to_save).await;

            println!("Language Save {0}", language.id);
        } else {
            language = option_language.unwrap().into();
        }

        let movie_language = movie_language::ActiveModel {
            movie_id: Set(movie_saved.id),
            language_id: Set(language.id),
            ..Default::default()
        };

        let movie_language_saved: movie_language::Model = movie_language_repository::save(db.clone(), movie_language).await;

        println!("Movie Language {0}", movie_language_saved.id);
    }

    println!("Movie Save {0}", movie_saved.id);

    return Ok(movie_saved.id.to_string())
}