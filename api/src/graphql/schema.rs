use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

pub struct Token(pub String);

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
}

pub fn build_schema() -> AppSchema {
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    schema
}
