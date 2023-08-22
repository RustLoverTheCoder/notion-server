pub mod block;
pub mod user;
pub mod user_setting;


pub use user::UserQuery;


#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery);