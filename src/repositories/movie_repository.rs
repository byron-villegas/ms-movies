use sea_orm::*;

use crate::entities::{genre::{self, Entity as Genre}, movie::{self, ActiveModel, Entity as Movie, Model}};

pub async fn find_all(db: DatabaseConnection) -> Vec<(movie::Model, Vec<genre::Model>)> {

    let mut movies: Vec<(movie::Model, Vec<genre::Model>)> = Movie::find().find_with_related(Genre).all(&db).await.unwrap();

    if movies.len() == 0 {
        movies = vec![];
    }
    
    return movies;
}

pub async fn save(db: DatabaseConnection, movie: ActiveModel) -> Model {
    let result: Model = movie.insert(&db).await.unwrap();
    return result;
}