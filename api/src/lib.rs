pub mod graphql;
pub mod token;
pub mod error;


use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::header::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use graphql::schema::{build_schema, AppSchema};

use config::init;

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

async fn graphql_handler(
    State(schema): State<AppSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    // if let Some(token) = get_token_from_headers(&headers) {
    //     req = req.data(token);
    // }
    schema.execute(req).await.into()
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    init().await?;
    let schema = build_schema();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .with_state(schema);

    println!("Playground: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
