use crate::{jwt::encode, util::code::rand_code};
use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject, ID};
use config::contants::DB;
use entity::{user, user::Entity as User};
use sea_orm::prelude::Uuid;

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
        _ctx: &Context<'_>,
        params: UserLoginEmail,
    ) -> Result<UserToken, Error> {
        tracing::debug!("sign_up_by_email: {:?}", params);
        let db = DB.get().unwrap();
        let data = User::find_by_email(&params.email.to_string())
            .one(db)
            .await?
            .ok_or("user not found")?;
        let (access_token, refresh_token, expires) = encode(&data.id).unwrap();

        let users_token = UserToken {
            access_token,
            refresh_token,
            expires: expires.num_seconds(),
        };
        Ok(users_token)
    }

    async fn login_by_phone(&self, _ctx: &Context<'_>, params: UserLoginPhone) -> Result<UserToken> {
        let db = DB.get().unwrap();
        let data = User::find_by_phone_number(&params.phone_number.to_owned())
            .one(db)
            .await?
            .ok_or("user not found")?;
        let (access_token, refresh_token, expires) = encode(&data.id).unwrap();

        let users_token = UserToken {
            access_token,
            refresh_token,
            expires: expires.num_seconds(),
        };
        Ok(users_token)
    }

    async fn send_code_to_phone(&self, _ctx: &Context<'_>, phone_number: String) -> Result<bool> {
        let code = rand_code();
        tracing::debug!("send_code_to_phone: {:?}", code);
        Ok(true)
    }
}
