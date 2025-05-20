// use clickhouse::Row;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, Eq, PartialEq, Serialize)]
// #[cfg_attr(feature = "server", derive(Row))]
#[sea_orm(table_name = "branches")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub branch_code: String,
    pub name: String,
    pub state: String,
    pub sml: Option<String>,
    pub class: Option<String>,
    pub area_manager: Option<String>,
    pub store_manager: Option<String>,
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
