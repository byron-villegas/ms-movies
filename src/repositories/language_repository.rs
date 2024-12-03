use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::language::{self, ActiveModel, Entity as Language, Model};

pub async fn find_by_name(db: DatabaseConnection, name: String) -> Option<language::Model> {
    let option_language: Option<language::Model> = Language::find()
    .filter(language::Column::Name.eq(name))
    .one(&db)
    .await
    .unwrap();

    return option_language;
}

pub async fn save(db: DatabaseConnection, language: ActiveModel) -> Model {
    let result: Model = language.insert(&db).await.unwrap();
    return result;
}