use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "movie_genre")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_name = "mgn_id")]
    pub id: i64,
    #[sea_orm(column_name = "mov_id")]
    pub movie_id: i64,
    #[sea_orm(column_name = "mgn_name")]
    pub name: String
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Movie
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Movie => Entity::belongs_to(super::movie::Entity)
                .from(Column::MovieId)
                .to(super::movie::Column::Id)
                .into(),
        }
    }
}

impl Related<super::movie::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Movie.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}