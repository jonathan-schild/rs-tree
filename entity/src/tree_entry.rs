//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tree_entry")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub target: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tree::Entity")]
    Tree,
}

impl Related<super::tree::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tree.def()
    }
}

impl Related<super::url::Entity> for Entity {
    fn to() -> RelationDef {
        super::tree::Relation::Url.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::tree::Relation::TreeEntry.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}