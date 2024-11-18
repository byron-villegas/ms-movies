use sea_orm::*;

use crate::entities::movie::{ActiveModel, Entity as Movie, Model};

pub async fn find_all(db: DatabaseConnection) -> Vec<Model> {

    let movies = Movie::find().all(&db).await.unwrap();
    
    return movies;
}

pub async fn save(db: DatabaseConnection, movie: ActiveModel) -> Model {
    let result: Model = movie.insert(&db).await.unwrap();
    return result;
}