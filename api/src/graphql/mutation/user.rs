use async_graphql::{Context, Object, Result,InputObject,ID,SimpleObject};
use entity::user;


#[derive(Default)]
pub struct UserMutation;


#[derive(InputObject, Clone, Debug)]
pub struct NewProfileInput { 
    pub user_id: ID, 
    pub username: String,
}

#[derive(InputObject, Clone, Debug)]
pub struct UserLoginEmail { 
    /// Ensure that the value is in email format
    #[graphql(validator(email))]
    pub email: String, 
    /// Non-nullable input for password. 
    /// Min-Char-len = 8 characters
    #[graphql(validator(chars_min_length = "8"))]
    pub password: String
}

#[derive(InputObject, Clone, Debug)]
pub struct UserLoginPhone { 
    pub phone_number: String, 
    /// Non-nullable input for password. 
    /// Min-Char-len = 8 characters
    #[graphql(validator(chars_min_length = "6"))]
    pub code: String
}

#[derive(SimpleObject)]
pub struct UserToken { 
    pub access_token: String,
    pub refresh_token: String,
    pub expires: i64,
}


#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, new_profile: NewProfileInput) -> Result<user::Model> {
        todo!()
    }

    async fn update_user(&self, ctx: &Context<'_>, new_profile: NewProfileInput) -> Result<user::Model> {
        todo!()
    }

    async fn login_by_email(&self, ctx: &Context<'_>, user: UserLoginEmail) -> Result<UserToken> {
        todo!()
    }

    async fn login_by_phone(&self, ctx: &Context<'_>, user: UserLoginPhone) -> Result<UserToken> {
        todo!()
    }

    async fn send_code_to_phone(&self, ctx: &Context<'_>, phone_number: String) -> Result<bool> {
        todo!()
    }
}