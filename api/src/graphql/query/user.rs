use async_graphql::{Context, Object, Result};
use entity::{async_graphql,user};


#[derive(Default)]
pub struct UserQuery;


#[Object]
impl UserQuery {
    async fn get_user_by_id(&self, ctx: &Context<'_>, id: String) -> Result<user::Model> {
        todo!()
    }

    async fn get_user_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Vec<user::Model>> {
        todo!()
    }

    async fn get_user_by_phone_number(&self, ctx: &Context<'_>, phone_number: String) -> Result<Vec<user::Model>> {
        todo!()
    }
}