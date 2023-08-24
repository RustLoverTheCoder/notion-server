use async_graphql::{Context, Object, Result};
use config::contants::DB;
use entity::{user, user::Entity as User};
use sea_orm::prelude::Uuid;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<user::Model>> {
        let db = DB.get().unwrap();
        let data = User::find_by_id(id).one(db).await?;
        Ok(data)
    }

    async fn get_user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<Option<user::Model>> {
        let db = DB.get().unwrap();
        let data = User::find_by_email(&email).one(db).await?;
        Ok(data)
    }

    async fn get_user_by_phone_number(
        &self,
        ctx: &Context<'_>,
        phone_number: String,
    ) -> Result<Option<user::Model>> {
        let db = DB.get().unwrap();
        let data = User::find_by_phone_number(&phone_number).one(db).await?;
        Ok(data)
    }
}
