// use clickhouse::Row;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, Eq, PartialEq, Serialize)]
// #[cfg_attr(feature = "server", derive(Row))]
#[sea_orm(table_name = "web_products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub sku: String,
    pub tag_no: Option<String>,
    pub brand: String,
    pub categories: Vec<String>,
    pub subcategories: Vec<String>,
    pub model: String,
    pub name: String,
    pub image: Option<String>,
    pub url_slug: String,
}

#[derive(Deserialize, Serialize)]
enum ColumnSerde {
    Id,
    Sku,
    TagNo,
    Brand,
    Categories,
    Subcategories,
    Model,
    Name,
    Image,
    UrlSlug,
}

impl From<ColumnSerde> for Column {
    fn from(c: ColumnSerde) -> Self {
        match c {
            ColumnSerde::Id => Column::Id,
            ColumnSerde::Sku => Column::Sku,
            ColumnSerde::TagNo => Column::TagNo,
            ColumnSerde::Brand => Column::Brand,
            ColumnSerde::Categories => Column::Categories,
            ColumnSerde::Subcategories => Column::Subcategories,
            ColumnSerde::Model => Column::Model,
            ColumnSerde::Name => Column::Name,
            ColumnSerde::Image => Column::Image,
            ColumnSerde::UrlSlug => Column::UrlSlug,
        }
    }
}

impl From<Column> for ColumnSerde {
    fn from(c: Column) -> Self {
        match c {
            Column::Id => ColumnSerde::Id,
            Column::Sku => ColumnSerde::Sku,
            Column::TagNo => ColumnSerde::TagNo,
            Column::Brand => ColumnSerde::Brand,
            Column::Categories => ColumnSerde::Categories,
            Column::Subcategories => ColumnSerde::Subcategories,
            Column::Model => ColumnSerde::Model,
            Column::Name => ColumnSerde::Name,
            Column::Image => ColumnSerde::Image,
            Column::UrlSlug => ColumnSerde::UrlSlug,
        }
    }
}

impl Serialize for Column {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ColumnSerde::serialize(&(*self).into(), serializer)
    }
}

impl<'de> Deserialize<'de> for Column {
    fn deserialize<D>(deserializer: D) -> Result<Column, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ColumnSerde::deserialize(deserializer)?.into())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sales::Entity")]
    Sales,
}

impl Related<super::sales::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sales.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
