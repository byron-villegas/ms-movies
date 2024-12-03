use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::genre::{self, ActiveModel, Entity as Genre, Model};

pub async fn find_by_name(db: DatabaseConnection, name: String) -> Option<genre::Model> {
    let option_genre: Option<genre::Model> = Genre::find()
    .filter(genre::Column::Name.eq(name))
    .one(&db)
    .await
    .unwrap();

    return option_genre;
}

pub async fn save(db: DatabaseConnection, genre: ActiveModel) -> Model {
    let result: Model = genre.insert(&db).await.unwrap();
    return result;
}