use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait};
   
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "movie")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_name = "mov_id")]
    pub id: i64,
    #[sea_orm(column_name = "mov_title")]
    pub title: String,
    #[sea_orm(column_name = "mov_original_title")]
    pub original_title: String,
    #[sea_orm(column_name = "mov_sypnosis")]
    pub sypnosis: String,
    #[sea_orm(column_name = "mov_image")]
    pub image: String,
    #[sea_orm(column_name = "mov_year")]
    pub year: i16,
    #[sea_orm(column_name = "mov_duration")]
    pub duration: i16,
    #[sea_orm(column_name = "mov_duration_type")]
    pub duration_type: String,
    #[sea_orm(column_name = "mov_resolution_width")]
    pub resolution_width: i16,
    #[sea_orm(column_name = "mov_resolution_height")]
    pub resolution_height: i16,
    #[sea_orm(column_name = "mov_size")]
    pub size: f32,
    #[sea_orm(column_name = "mov_size_type")]
    pub size_type: String,
    #[sea_orm(column_name = "mov_format")]
    pub format: String
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Genre,
    Language
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Genre => Entity::has_many(super::genre::Entity).into(),
            Self::Language => Entity::has_many(super::language::Entity).into()
        }
    }
}

impl Related<super::genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genre.def()
    }
}

impl Related<super::language::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Language.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}