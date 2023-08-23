use ::entity::{user, user::Entity as User};
use sea_orm::entity::prelude::*;
use sea_orm::*;

pub struct UserMutation;

impl UserMutation {
    pub async fn create_user(conn: &DbConn, form_data: user::Model) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            email: Set(form_data.email.to_owned()),
            ..Default::default()
        };
        let res = User::insert(active_model).exec(conn).await?;
        Ok(user::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }

    pub async fn update_user_by_id(
        conn: &DbConn,
        id: Uuid,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user::ActiveModel {
            id: user.id,
            email: Set(form_data.email.to_owned()),
            ..Default::default()
        }
        .update(conn)
        .await
    }
}
