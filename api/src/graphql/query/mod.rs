pub mod block;
pub mod user;
pub mod user_setting;

pub use block::BlockQuery;
pub use user::UserQuery;
pub use user_setting::UserSettingQuery;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery, BlockQuery, UserSettingQuery);
