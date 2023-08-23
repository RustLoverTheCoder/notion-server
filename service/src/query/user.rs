use ::entity::{user, user::Entity as User};
use sea_orm::entity::prelude::*;

pub struct UserQuery;

impl UserQuery {
    pub async fn find_user_by_id(conn: &DbConn, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(conn).await
    }
}
