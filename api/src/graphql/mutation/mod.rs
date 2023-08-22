pub mod user;
pub mod user_setting;
pub mod block;


pub use user::UserMutation;


#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UserMutation);