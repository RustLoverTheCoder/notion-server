use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::graphql::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    schema
}
