use async_graphql::{EmptySubscription, Schema};

use crate::graphql::{query::Query, mutation::Mutation};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    let schema = Schema::new(Query::default(), Mutation::default(), EmptySubscription);
    schema
}
