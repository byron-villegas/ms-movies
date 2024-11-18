use sea_orm::*;

use crate::entities::{genre::{self, Entity as Genre}, language::{self, Entity as Language}, movie::{self, ActiveModel, Entity as Movie, Model}};

#[derive(Debug, Clone)]
pub struct MovieWithGenresAndLanguages {
    pub movie: movie::Model,
    pub genres: Vec<genre::Model>,
    pub languages: Vec<language::Model>
}

pub async fn find_all(db: DatabaseConnection) -> Vec<MovieWithGenresAndLanguages> {

    let movies_with_genre: Vec<(movie::Model, Vec<genre::Model>)> = Movie::find().find_with_related(Genre).all(&db).await.unwrap();

    let mut movies = Vec::new();

    for(movie, genres) in movies_with_genre {
        let languages = Language::find()
        .filter(language::Column::MovieId.eq(movie.id))
        .all(&db)
        .await.unwrap();

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