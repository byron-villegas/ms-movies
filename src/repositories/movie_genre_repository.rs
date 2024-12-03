use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::entities::movie_genre::{ActiveModel, Model};

pub async fn save(db: DatabaseConnection, movie_genre: ActiveModel) -> Model {
    let result: Model = movie_genre.insert(&db).await.unwrap();
    return result;
}