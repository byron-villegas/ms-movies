use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "movie")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub sypnosis: String,
    pub image: String,
    pub year: i16,
    pub duration: i16,
    pub duration_type: String,
    pub resolution_width: String,
    pub resolution_height: String,
    pub size: f32,
    pub size_type: String,
    pub format: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    
}

impl ActiveModelBehavior for ActiveModel {}