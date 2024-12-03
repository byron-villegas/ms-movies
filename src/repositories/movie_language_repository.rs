use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::entities::movie_language::{ActiveModel, Model};

pub async fn save(db: DatabaseConnection, movie_language: ActiveModel) -> Model {
    let result: Model = movie_language.insert(&db).await.unwrap();
    return result;
}