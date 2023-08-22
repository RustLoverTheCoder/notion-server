use async_graphql::{Context, Object, Result};
use entity::{async_graphql,user};


#[derive(Default)]
pub struct UserMutation;


#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, id: i32) -> Result<user::Model> {
        todo!()
    }
}