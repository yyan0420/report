#![deny(clippy::dbg_macro)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::print_stdout)]
#![deny(clippy::unwrap_used)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::all, clippy::pedantic)]

mod brand;
mod clickhouse;
mod graphql;

use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    http::Method,
    response::{self, Html},
    routing::get,
};
use graphql::{Mutation, QueryRoot};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::clickhouse::client;

#[axum::debug_handler]
async fn graphiql() -> Result<Html<String>, &'static str> {
    // how ugly
    let endpoint = url::Url::parse(
        &std::env::var("PUBLIC_GRAPHQL_ENDPOINT").map_err(|_| "PUBLIC_GRAPHQL_ENDPOINT not set")?,
    )
    .map_err(|_| "cannot parse PUBLIC_GRAPHQL_ENDPOINT")?;

    let path = endpoint.path();

    Ok(response::Html(
        GraphiQLSource::build().endpoint(path).finish(),
    ))
}

/// Export Graphql Schema
pub fn export() {
    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription).finish();

    #[allow(clippy::print_stdout)]
    {
        println!("{}", &schema.sdl());
    }
}

#[allow(clippy::missing_errors_doc)]
pub async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let endpoint = url::Url::parse(&std::env::var("PUBLIC_GRAPHQL_ENDPOINT")?)?;

    let host = endpoint.host_str().unwrap_or("localhost");
    let path = endpoint.path();
    let port = endpoint.port().unwrap_or(8000);

    let db = client()?;
    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(db)
        .finish();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_private_network(true);

    let app = Router::new()
        .route(path, get(graphiql).post_service(GraphQL::new(schema)))
        .layer(ServiceBuilder::new().layer(cors));

    #[allow(clippy::print_stdout)]
    {
        println!("GraphiQL IDE: http://{host}:{port}{path}");
    }

    Ok(axum::serve(TcpListener::bind(format!("{host}:{port}")).await?, app).await?)
}
