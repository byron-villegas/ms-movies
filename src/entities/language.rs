use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait};
   
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "language")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_name = "lan_id")]
    pub id: i64,
    #[sea_orm(column_name = "lan_name")]
    pub name: String
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    MovieLanguage
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::MovieLanguage => Entity::has_many(super::movie_language::Entity).into()
        }
    }
}

impl Related<super::movie_genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MovieLanguage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}