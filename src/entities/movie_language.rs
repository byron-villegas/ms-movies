use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "movie_language")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_name = "mlg_id")]
    pub id: i64,
    #[sea_orm(column_name = "mov_id")]
    pub movie_id: i64,
    #[sea_orm(column_name = "lan_id")]
    pub language_id: i64
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Movie,
    Language
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Movie => Entity::belongs_to(super::movie::Entity)
                .from(Column::MovieId)
                .to(super::movie::Column::Id)
                .into(),
            Self::Language => Entity::belongs_to(super::language::Entity)
            .from(Column::LanguageId)
            .to(super::language::Column::Id)
            .into()
        }
    }
}

impl Related<super::movie::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Movie.def()
    }
}

impl Related<super::language::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Language.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}