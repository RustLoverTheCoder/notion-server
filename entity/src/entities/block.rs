//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2
use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "block")]
#[graphql(concrete(name = "block", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub title: Option<sea_orm::entity::prelude::Json>,
    pub body: Option<sea_orm::entity::prelude::Json>,
    pub r#type: String,
    pub author_id: Option<Uuid>,
    pub disabled: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_parent_id(parent_id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::ParentId.eq(parent_id))
    }

    // 通过 author_id 和 parent_id 查找
    pub fn find_by_author_id_and_parent_id(author_id: Uuid, parent_id: Uuid) -> Select<Entity> {
        Self::find()
            .filter(Column::AuthorId.eq(author_id))
            .filter(Column::ParentId.eq(parent_id))
    }
}
