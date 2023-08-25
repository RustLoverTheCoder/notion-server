use crate::{jwt::encode, util::code::rand_code};
use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject};
use config::contants::DB;
use entity::{user, user::Entity as User};
use sea_orm::{prelude::Uuid, ActiveModelTrait, Set};

#[derive(Default)]
pub struct UserMutation;

#[derive(InputObject, Clone, Debug)]
pub struct UpdateProfileInput {
    pub avatar_url: Option<String>,
    pub metadata: Option<sea_orm::entity::prelude::Json>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
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
        params: UpdateProfileInput,
    ) -> Result<user::Model> {
        let user_id = ctx.data::<Uuid>().unwrap();
        let db = DB.get().unwrap();
        let user_data = User::find_by_id(user_id.to_owned())
            .one(db)
            .await?
            .ok_or("user not found")?;

        let mut user_active: user::ActiveModel = user_data.into();

        if let Some(avatar_url) = params.avatar_url {
            user_active.avatar_url = Set(Some(avatar_url));
        }

        if let Some(metadata) = params.metadata {
            user_active.metadata = Set(Some(metadata));
        }

        if let Some(first_name) = params.first_name {
            user_active.first_name = Set(Some(first_name));
        }

        if let Some(last_name) = params.last_name {
            user_active.last_name = Set(Some(last_name));
        }

        user_active.updated_at = Set(Some(
            (chrono::Utc::now().with_timezone(
                &chrono::FixedOffset::east_opt(0).unwrap_or(chrono::FixedOffset::east(0)),
            )),
        ));

        let user_data = user_active.update(db).await?;
        return Ok(user_data);
    }

    async fn login_by_email(
        &self,
        _ctx: &Context<'_>,
        params: UserLoginEmail,
    ) -> Result<UserToken, Error> {
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

    async fn login_by_phone(
        &self,
        _ctx: &Context<'_>,
        params: UserLoginPhone,
    ) -> Result<UserToken> {
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
