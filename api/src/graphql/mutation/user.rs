use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject, ID};
use config::contants::DB;
use entity::{user, user::Entity as User};
use crate::jwt::encode;

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
    pub password: String,
}

#[derive(InputObject, Clone, Debug)]
pub struct UserLoginPhone {
    pub phone_number: String,
    #[graphql(validator(chars_min_length = "6"))]
    pub code: String,
}

#[derive(SimpleObject)]
pub struct UserToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires: i64,
}

#[Object]
impl UserMutation {
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        new_profile: NewProfileInput,
    ) -> Result<user::Model> {
        todo!()
    }

    async fn login_by_email(
        &self,
        ctx: &Context<'_>,
        params: UserLoginEmail,
    ) -> Result<Option<user::Model>, Error> {
        tracing::debug!("sign_up_by_email: {:?}", params);
        let db = DB.get().unwrap();
        let data = User::find_by_email(&params.email.to_string())
            .one(db)
            .await?;
        Ok(data)
    }

    async fn login_by_phone(&self, ctx: &Context<'_>, params: UserLoginPhone) -> Result<UserToken> {
      let user_id = "5a242296-74fb-41fe-b7aa-09580ea6429b".to_string();
        let (access_token, refresh_token, expires) =
                   encode(user_id).unwrap();

               let users_token = UserToken {
                   access_token,
                   refresh_token,
                   expires: expires.num_seconds(),
               };
               Ok(users_token)
    }

    async fn send_code_to_phone(&self, ctx: &Context<'_>, phone_number: String) -> Result<bool> {
        todo!()
    }
}
