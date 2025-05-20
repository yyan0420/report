#![deny(clippy::dbg_macro)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::print_stdout)]
#![deny(clippy::unwrap_used)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::all, clippy::pedantic)]

mod brand;
mod components;
mod data_source;
mod database;
mod graphql;
mod yoy;

use async_graphql::{EmptySubscription, Schema, SimpleObject, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    http::Method,
    response::{self, Html},
    routing::get,
};
use chrono::{DateTime, Local, Months};
use clickhouse::Row;
use graphql::{Mutation, QueryRoot};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::database::clickhouse::client;

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

// ///////////////////////////////////////////////////////////
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, SimpleObject, Row)]
pub struct Summary {
    pub name: String,
    pub code: Option<String>,
    pub qty_1: u64,
    pub total_1: f64,
    pub qty_2: u64,
    pub total_2: f64,
    pub qty_3: u64,
    pub total_3: f64,
    pub qty_4: u64,
    pub total_4: f64,
    pub percentage_1: Option<f64>,
    pub percentage_2: Option<f64>,
    pub percentage_3: Option<f64>,
    pub total_diff_1: f64,
    pub total_diff_2: f64,
    pub total_diff_3: f64,
}

pub(crate) const RANGE: usize = 4;

impl Summary {
    #[must_use]
    pub const fn get_range() -> usize {
        RANGE
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum BrandOrigin {
    #[default]
    All,
    Import,
    Local,
}

impl std::str::FromStr for BrandOrigin {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "All" => Ok(Self::All),
            "Import" => Ok(Self::Import),
            "Local" => Ok(Self::Local),
            _ => Err(format!("uknown brand origin {s}")),
        }
    }
}

use std::fmt;

impl fmt::Display for BrandOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub(crate) type FixedDateTimeRanges = [DateTimeRange; Summary::get_range()];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DateTimeRange {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

impl fmt::Display for DateTimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} To {}",
            self.start.format("%Y-%m-%d"),
            self.end.format("%Y-%m-%d")
        )
    }
}

use crate::components::GlobalFilter;

impl DateTimeRange {
    pub(crate) fn new_array(global_filter: &GlobalFilter) -> Option<FixedDateTimeRanges> {
        let (Some(start_dt), Some(end_dt)) =
            (global_filter.start_datetime, global_filter.end_datetime)
        else {
            return None;
        };

        #[allow(clippy::cast_possible_truncation)]
        let fixed = std::array::from_fn(|i| Self {
            start: start_dt - Months::new(12 * (i as u32)),
            end: end_dt - Months::new(12 * (i as u32)),
        });

        Some(fixed)
    }
}
