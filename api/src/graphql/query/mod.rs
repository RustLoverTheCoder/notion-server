pub mod block;
pub mod user;
pub mod user_setting;

use async_graphql::{Context, Object};

pub struct QueryRoot;

pub struct Token(pub String);

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
}
