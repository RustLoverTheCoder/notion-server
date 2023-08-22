use async_graphql::{Context, Object, Result};
use entity::{async_graphql,user};


#[derive(Default)]
pub struct UserQuery;


#[Object]
impl UserQuery {
    async fn find_user_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<user::Model> {
        todo!()
    }
}