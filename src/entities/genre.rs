use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait};
   
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "genre")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_name = "gen_id")]
    pub id: i64,
    #[sea_orm(column_name = "gen_name")]
    pub name: String
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    MovieGenre
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::MovieGenre => Entity::has_many(super::movie_genre::Entity).into()
        }
    }
}

impl Related<super::movie_genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MovieGenre.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}