// use clickhouse::Row;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, PartialEq, Serialize)]
// #[cfg_attr(feature = "server", derive(Row))]
#[sea_orm(table_name = "sales_view")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub line_id: String,
    pub invoice_no: String,
    pub cust_code: String,
    pub customer_name: Option<String>,
    pub order_no: Option<String>,
    pub deliv_date: Option<String>,
    pub stock_no: String,
    pub stock_id: String,
    pub qty: u32,
    pub sell_exgst: f32,
    pub gst: Option<f32>,
    pub tag_no: Option<String>,
    pub item_description: Option<String>,
    pub condition: Option<String>,
    pub net_amount: f32,
    pub gross_amount: f32,
    pub cost_price: Option<f32>,
    pub recommend_retail: Option<f32>,
    pub action: String,
    pub branch_code: String,
    pub address_1: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub customer_email_address: Option<String>,
    pub project_no: Option<String>,
    pub oper: String,
    pub sales_grp: Option<String>,
    pub category: Option<String>,
    pub supplier_code: Option<String>,
    pub supplier_name: Option<String>,
    pub bonus_points_earned: Option<f32>,
    pub bonus_points_used: Option<f32>,
    pub created_at: DateTimeUtc,
    pub suburb: String,
    pub state: String,
    pub post_code: String,
    pub min: Option<u32>,
    pub max: Option<u32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::branch::Entity",
        from = "Column::BranchCode",
        to = "super::branch::Column::BranchCode",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Branch,
    #[sea_orm(
        belongs_to = "super::web_product::Entity",
        from = "Column::TagNo",
        to = "super::web_product::Column::TagNo",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    WebProduct,
}

impl Related<super::web_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WebProduct.def()
    }
}

impl Related<super::branch::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Branch.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
