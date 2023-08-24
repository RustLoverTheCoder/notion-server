use async_graphql::{Context, Object, Result};
use config::contants::DB;
use entity::user::Entity as User;
use entity::{user_setting, user_setting::Entity as UserSetting};
use sea_orm::prelude::Uuid;

#[derive(Default)]
pub struct UserSettingQuery;

#[Object]
impl UserSettingQuery {
    async fn get_user_setting(&self, ctx: &Context<'_>) -> Result<Option<user_setting::Model>> {
        let db = DB.get().unwrap();
        let user_id = ctx.data::<Uuid>().unwrap();
        let user_data = User::find_by_id(user_id.to_owned()).one(db).await?;
        match user_data {
            Some(user) => {
                let settings_id = user.settings_id.to_owned();
                match settings_id {
                    Some(id) => {
                        let setting = UserSetting::find_by_id(id).one(db).await?;
                        Ok(setting)
                    }
                    None => Ok(None),
                }
            }
            None => Ok(None),
        }
    }
}
