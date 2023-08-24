use async_graphql::{Context, InputObject, Object, Result};
use config::contants::DB;
use entity::user::Entity as User;
use entity::{user_setting::Entity as UserSetting,user_setting};
use sea_orm::{Set, ActiveModelTrait};
use sea_orm::prelude::Uuid;

#[derive(Default)]
pub struct UserSettingMutation;

#[derive(InputObject, Clone, Debug)]
pub struct NewUserSettingInput {
    pub color_scheme: Option<String>,
    pub language: Option<String>,
}

#[Object]
impl UserSettingMutation {
    async fn update_user_setting(
        &self,
        ctx: &Context<'_>,
        new_user_setting: NewUserSettingInput,
    ) -> Result<bool> {
        let db = DB.get().unwrap();
        let user_id = ctx.data::<Uuid>().unwrap();
        let user_data = User::find_by_id(user_id.to_owned()).one(db).await?;
        match user_data {
            Some(user) => {
                let settings_id = user.settings_id.to_owned();
                match settings_id {
                    Some(id) => {
                        let setting = UserSetting::find_by_id(id).one(db).await?;
                        match setting {
                            Some(setting) => {
                                let mut setting_active: user_setting::ActiveModel = setting.into();
                                if let Some(color_scheme) = new_user_setting.color_scheme {
                                    setting_active.color_scheme = Set(color_scheme.to_owned());
                                }
                                if let Some(language) = new_user_setting.language {
                                    setting_active.language = Set(language.to_owned());
                                }
                                setting_active.update(db).await?;
                                Ok(true)
                            },
                            None => Ok(false),
                        }
                    }
                    None => Ok(false),
                }
            }
            None => Ok(false),
        }
    }
}
