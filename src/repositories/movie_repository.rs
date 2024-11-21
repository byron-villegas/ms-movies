
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::{genre::Entity as Genre, language::Entity as Language, movie::{self, ActiveModel, Entity as Movie, Model}, movie_genre::{self, Entity as MovieGenre}, movie_language::{self, Entity as MovieLanguage}};

#[derive(Debug, Clone)]
pub struct MovieWithGenresAndLanguages {
    pub movie: movie::Model,
    pub genres: Vec<String>,
    pub languages: Vec<String>
}

pub async fn find_all(db: DatabaseConnection) -> Vec<MovieWithGenresAndLanguages> {

    let movies_with_genre: Vec<(movie::Model, Vec<movie_genre::Model>)> = Movie::find().find_with_related(MovieGenre).all(&db).await.unwrap();

    let mut movies = Vec::new();

    for(movie, movie_genres) in movies_with_genre {
        let mut genres: Vec<String> = Vec::new();
        let mut languages: Vec<String> = Vec::new();

        for movie_genre in movie_genres {
            let genre = Genre::find_by_id(movie_genre.genre_id).one(&db).await.unwrap().unwrap();
            
            genres.push(genre.name);
        }
        
        let movie_languages = MovieLanguage::find()
        .filter(movie_language::Column::MovieId.eq(movie.id))
        .all(&db)
        .await.unwrap();

        for movie_language in movie_languages {
            let language = Language::find_by_id(movie_language.language_id).one(&db).await.unwrap().unwrap();

            languages.push(language.name);
        }

        movies.push(MovieWithGenresAndLanguages {
            movie,
            genres,
            languages
        });
    }

    if movies.len() == 0 {
        movies = vec![];
    }
    
    return movies;
}

pub async fn save(db: DatabaseConnection, movie: ActiveModel) -> Model {
    let result: Model = movie.insert(&db).await.unwrap();
    return result;
}