use async_graphql::{EmptySubscription, Schema};

use crate::graphql::{mutation::Mutation, query::Query};
use config::contants::DB;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    let db = DB.get().unwrap();
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
