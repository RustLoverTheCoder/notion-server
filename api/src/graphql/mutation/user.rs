use async_graphql::{Context, Object, Result};
use entity::{async_graphql,user};


#[derive(Default)]
pub struct UserMutation;


#[derive(InputObject, Clone, Debug)]
pub struct NewProfileInput { 
    pub user_id: ID, 
    pub username: String,
}


#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, new_profile: NewProfileInput) -> Result<user::Model> {
        todo!()
    }

    async fn update_user(&self, ctx: &Context<'_>, new_profile: NewProfileInput) -> Result<user::Model> {
        todo!()
    }
}