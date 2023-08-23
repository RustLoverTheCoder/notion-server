use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use entity::user_setting;

#[derive(Default)]
pub struct UserSettingMutation;

#[derive(InputObject, Clone, Debug)]
pub struct NewUserSettingInput {
    pub color_scheme: Option<String>,
    pub language: Option<String>,
}

#[Object]
impl UserSettingMutation {
    async fn update_user_setting(&self, ctx: &Context<'_>, new_user_setting: NewUserSettingInput) -> Result<bool> {
        todo!()
    }
}