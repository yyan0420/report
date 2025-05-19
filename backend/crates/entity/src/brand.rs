#[cfg(feature = "server")]
use clickhouse::Row;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use async_graphql::{SimpleObject, Result};

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "server", derive(Row))]
#[sea_orm(table_name = "brands")]
#[derive(SimpleObject)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub url_slug: String,
    pub private_label: bool,
    pub status: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
