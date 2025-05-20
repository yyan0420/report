use crate::database::clickhouse::QueryBuilder;
use crate::graphql::GlobalID;
use anyhow::Result;
use async_graphql::{ID, OutputType, SimpleObject};
use clickhouse::Client;
use entity::brand;
use sea_query::Order;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SimpleObject)]
pub struct Brand {
    pub id: ID,
    pub name: String,
    pub url_slug: String,
    pub private_label: bool,
    pub status: bool,
}

impl Brand {
    pub(crate) async fn fetch_all(db: &Client) -> Result<Vec<Brand>> {
        debug!("fetching all brands");

        let query = sea_query::Query::select()
            .columns([
                brand::Column::Id,
                brand::Column::Name,
                brand::Column::UrlSlug,
                brand::Column::PrivateLabel,
                brand::Column::Status,
            ])
            .from(brand::Entity)
            .order_by(brand::Column::Name, Order::Asc)
            .take();

        let raw_rows: Vec<(i64, String, String, bool, bool)> =
            QueryBuilder::new(db, &query).fetch_all().await?;

        let brands = raw_rows
            .into_iter()
            .map(|(id, name, url_slug, private_label, status)| brand::Model {
                id,
                name,
                url_slug,
                private_label,
                status,
            })
            .map(Brand::from)
            .collect();

        Ok(brands)
    }
}

impl From<brand::Model> for Brand {
    fn from(model: brand::Model) -> Self {
        Self {
            id: GlobalID::new(&Self::type_name(), model.id).into(),
            name: model.name,
            url_slug: model.url_slug,
            private_label: model.private_label,
            status: model.status,
        }
    }
}
