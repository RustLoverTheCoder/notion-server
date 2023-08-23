pub mod block;
pub mod user;
pub mod user_setting;

pub use block::BlockMutation;
pub use user::UserMutation;
pub use user_setting::UserSettingMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UserMutation, UserSettingMutation, BlockMutation);
