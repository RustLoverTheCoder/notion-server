use async_graphql::{Context, Object, Result};
use entity::user_setting;

#[derive(Default)]
pub struct UserSettingQuery;

#[Object]
impl UserSettingQuery {
    async fn get_user_setting(&self, ctx: &Context<'_>) -> Result<user_setting::Model> {
        todo!()
    }
}
