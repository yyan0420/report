#![allow(non_snake_case)]
use crate::database::{clickhouse::ClickHouseArrayJoin, copy_from_sea_orm::join_condition};
use crate::{BrandOrigin, DateTimeRange, FixedDateTimeRanges, data_source::DataSource};
use chrono::{DateTime, Local};
use entity::{sales, web_product};
use sea_orm::RelationTrait;
use sea_query::ExprTrait;
use sea_query::{Expr, Func, query::SelectStatement};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{collections::HashMap, fmt, sync::Arc};

pub(crate) type FilterInterests = Arc<[FilterKind]>;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
pub(crate) enum FilterKind {
    Brand,
    // BrandOrigin,
    Category,
    Customer,
    DataSource,
    Operator,
    Product,
    Subcategory,
    Supplier,
    Store,
    UnlistedProduct,
}

impl FilterKind {
    pub(crate) fn apply_filter(self, query: &mut SelectStatement, value: Vec<&String>) {
        match self {
            Self::Brand => {
                query.and_where(
                    Expr::col((web_product::Entity, web_product::Column::Brand)).is_in(value),
                );
            }
            Self::Category => {
                let cats: Vec<String> = value
                    .iter()
                    .map(|s| s.replace('\'', "\\'").to_string())
                    .collect();

                query.and_where(
                    Func::cust(ClickHouseArrayJoin)
                        .arg(Expr::col(web_product::Column::Categories))
                        .is_in(cats),
                );
            }
            Self::Subcategory => {
                let subcats: Vec<String> = value
                    .iter()
                    .map(|s| s.replace('\'', "\\'").to_string())
                    .collect();

                query.and_where(
                    Func::cust(ClickHouseArrayJoin)
                        .arg(Expr::col(web_product::Column::Subcategories))
                        .is_in(subcats),
                );
            }
            Self::Customer => {
                query.and_where(Expr::col((sales::Entity, sales::Column::CustCode)).is_in(value));
            }
            Self::Supplier => {
                query.and_where(
                    Expr::col((sales::Entity, sales::Column::SupplierCode)).is_in(value),
                );
            }
            Self::UnlistedProduct => {
                query.and_where(
                    Expr::col((web_product::Entity, web_product::Column::TagNo)).is_null(),
                );
            }
            Self::Operator => {
                query.and_where(Expr::col((sales::Entity, sales::Column::Oper)).is_in(value));
            }
            Self::Store => {
                query.and_where(Expr::col((sales::Entity, sales::Column::BranchCode)).is_in(value));
            }
            Self::Product => {
                query.and_where(Expr::col((sales::Entity, sales::Column::TagNo)).is_in(value));
            }
            Self::DataSource => {}
        }
    }
}

impl fmt::Display for FilterKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let self_str = format!("{self:?}");
        let name = self_str.split('(').next().unwrap_or(&self_str);
        write!(f, "{name}")
    }
}

/// For displaying
type FilterText = String;

/// For SQL use
type FilterValue = String;

/// Due to [this issue](https://github.com/DioxusLabs/dioxus/issues/2628)
/// it is necessary to wrap the collections in an Option
#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub(crate) struct GlobalFilter {
    pub(crate) start_datetime: Option<DateTime<Local>>,
    pub(crate) end_datetime: Option<DateTime<Local>>,
    pub(crate) data_source: DataSource,
    pub(crate) brand_origin: BrandOrigin,
    #[serde_as(as = "Vec<(_, Vec<(_,_)>)>")]
    pub(crate) filters: HashMap<FilterKind, HashMap<FilterValue, FilterText>>,
}

impl GlobalFilter {
    pub(crate) async fn sanitize(&mut self) -> anyhow::Result<()> {
        // let limit = crate::data_source::get_limit()
        //     .await
        //     .map_err(|e| anyhow::anyhow!(e))?;

        // if let Some(limit) = limit {
        //     self.start_datetime = limit.to_datetime();
        // }

        Ok(())
    }

    pub(crate) fn apply_filters(
        &self,
        interests: &[FilterKind],
        query: &mut SelectStatement,
    ) -> &Self {
        self.join_web_products(query);
        self.apply_brand_origin(query);
        self.apply_data_source(query);

        self.filters
            .iter()
            .filter(|(k, _)| interests.contains(k))
            .for_each(|(k, map)| {
                let keys: Vec<&FilterValue> = map.keys().collect();
                k.apply_filter(query, keys);
            });

        self
    }

    // pub(crate) async fn apply_date_range(
    //     &mut self,
    //     query: &mut SelectStatement,
    // ) -> anyhow::Result<&Self> {
    //     use entity::sales;

    //     self.sanitize().await?;

    //     query
    //         .and_where(Expr::col(sales::Column::CreatedAt).gte(self.start_datetime_string()))
    //         .and_where(Expr::col(sales::Column::CreatedAt).lte(self.end_datetime_string()));

    //     Ok(self)
    // }

    fn apply_data_source(&self, query: &mut SelectStatement) -> &Self {
        use entity::sales;
        match self.data_source {
            DataSource::InStore => {
                query.and_where(Expr::col(sales::Column::OrderNo).not_like("ST%"));
            }
            DataSource::OnlineInStore => {}
        }

        self
    }

    fn join_web_products(&self, query: &mut SelectStatement) -> &Self {
        use sea_query::JoinType;

        let unlisted = self.filters.contains_key(&FilterKind::UnlistedProduct);

        let join_type = if unlisted {
            // Unlisted must use left join
            JoinType::LeftJoin
        } else {
            JoinType::InnerJoin
        };

        // join web_product table
        let relation_def = sales::Relation::WebProduct.def();
        query.join(
            join_type,
            relation_def.to_tbl.clone(),
            join_condition(relation_def),
        );

        self
    }

    fn apply_brand_origin(&self, query: &mut SelectStatement) -> &Self {
        use entity::{brand, web_product};

        if self.brand_origin == BrandOrigin::All {
            return self;
        }

        // join brand table
        query.inner_join(
            brand::Entity,
            Expr::col((brand::Entity, brand::Column::Name))
                .equals((web_product::Entity, web_product::Column::Brand)),
        );

        // It is necessary to use string value "true" and "false" here.
        // The `brands` table in ClicHouse is a Postgres table engine(`show table brands`)
        // and ClickHouse(and MySQL) treat 0 = false, 1 = true.
        // With
        // ```sql
        // SELECT * FROM brands WHERE private_label=true;
        // ```
        // ClickHouse naively sends out `where private_label=1` and clearly it is wrong.
        // To overcome this, we can rely on PG's string evaluation, i.e. true='true'
        match self.brand_origin {
            BrandOrigin::Import => {
                query.and_where(Expr::col(brand::Column::PrivateLabel).eq("true"));
            }
            BrandOrigin::Local => {
                query.and_where(Expr::col(brand::Column::PrivateLabel).eq("false"));
            }
            BrandOrigin::All => {}
        }

        self
    }

    // pub(crate) fn value(&self, interest: FilterKind) -> Option<&HashMap<FilterValue, FilterText>> {
    //     self.filters.get(&interest)
    // }

    // // pub(crate) fn keys(&self, interest: FilterKind) -> Option<Vec<FilterValue>> {
    // //     self.value(interest)
    // //         .map(|map| map.iter().map(|(k, _)| k.to_owned()).collect())
    // // }

    // pub(crate) fn pairs(&self, interest: FilterKind) -> Option<Vec<(FilterValue, FilterText)>> {
    //     self.value(interest)
    //         .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    // }

    // pub(crate) fn push(&mut self, kind: FilterKind, (value, text): (FilterValue, FilterText)) {
    //     let entry = self.filters.entry(kind).or_default();
    //     entry.insert(value, text);
    // }

    // // #[allow(clippy::trivially_copy_pass_by_ref)]
    // pub(crate) fn remove(
    //     &mut self,
    //     k: &FilterKind,
    // ) -> Option<(FilterKind, HashMap<FilterValue, FilterText>)> {
    //     self.filters.remove_entry(k)
    // }

    // #[allow(clippy::trivially_copy_pass_by_ref)]
    // pub(crate) fn remove_one(
    //     &mut self,
    //     k: &FilterKind,
    //     v: &FilterValue,
    // ) -> Option<(FilterValue, FilterText)> {
    //     if let Some(filter_item) = self.filters.get_mut(k) {
    //         let removed = filter_item.remove_entry(v);

    //         if filter_item.is_empty() {
    //             self.filters.remove_entry(k);
    //         }

    //         removed
    //     } else {
    //         None
    //     }
    // }

    // pub(crate) fn start_datetime_string(&self) -> Option<String> {
    //     self.start_datetime
    //         .map(|dt| dt.to_utc().naive_local().to_string())
    // }

    // pub(crate) fn end_datetime_string(&self) -> Option<String> {
    //     self.end_datetime
    //         .map(|dt| dt.to_utc().naive_local().to_string())
    // }

    // /// true if `start_datetime` and `end_datetime` are some
    // /// AND `start_datetime` < `end_datetime`
    // pub(crate) fn ready(&self) -> bool {
    //     match (self.start_datetime, self.end_datetime) {
    //         (Some(s), Some(e)) => s < e,
    //         _ => false,
    //     }
    // }

    // pub(crate) async fn ready_and<F, Fut, T>(&self, f: F) -> Option<T>
    // where
    //     F: FnOnce(Self) -> Fut + Send,
    //     Fut: Future<Output = T> + Send,
    // {
    //     if self.ready() {
    //         Some(f(self.to_owned()).await)
    //     } else {
    //         None
    //     }
    // }

    pub(crate) fn date_range(&self) -> Option<FixedDateTimeRanges> {
        DateTimeRange::new_array(self)
    }
}

// macro_rules! handle_fetch {
//     ($result:expr) => {
//         match $result {
//             Ok(r) => Ok(r),
//             Err(e) => Err(ServerFnError::ServerError(e.to_string())),
//         }
//     };
// }

// pub(crate) use handle_fetch;

// type YoyType = (
//     HashMap<DateTimeRange, u64>,
//     HashMap<DateTimeRange, f64>,
//     HashMap<DateTimeRange, Option<f64>>,
// );

// pub(crate) fn create_qty_total_maps(ranges: &[DateTimeRange], summary: &crate::Summary) -> YoyType {
//     use std::collections::HashMap;

//     let qty = HashMap::from([
//         (ranges[0], summary.qty_1),
//         (ranges[1], summary.qty_2),
//         (ranges[2], summary.qty_3),
//         (ranges[3], summary.qty_4),
//     ]);

//     let total = HashMap::from([
//         (ranges[0], summary.total_1),
//         (ranges[1], summary.total_2),
//         (ranges[2], summary.total_3),
//         (ranges[3], summary.total_4),
//     ]);

//     let percentage = HashMap::from([
//         (ranges[0], summary.percentage_1),
//         (ranges[1], summary.percentage_2),
//         (ranges[2], summary.percentage_3),
//     ]);

//     (qty, total, percentage)
// }

// macro_rules! server_fn_helper {
//     ($entity:ident, $route:path) => {
//         #[allow(dead_code)]
//         pub(crate) struct Helper;

//         #[server]
//         #[allow(dead_code)]
//         pub(crate) async fn all() -> Result<Vec<$entity::Model>, ServerFnError> {
//             Ok(Helper::all().await.unwrap())
//         }

//         #[server]
//         pub(crate) async fn paginate(page: u64) -> Result<Vec<$entity::Model>, ServerFnError> {
//             Ok(Helper::paginate(page).await.unwrap())
//         }

//         #[server]
//         pub(crate) async fn total() -> Result<u64, ServerFnError> {
//             Ok(Helper::total().await.unwrap())
//         }

//         #[server]
//         #[allow(dead_code)]
//         pub(crate) async fn one(id: i64) -> Result<Option<$entity::Model>, ServerFnError> {
//             Ok(Helper::one(id).await.unwrap())
//         }

//         #[server]
//         #[allow(dead_code)]
//         pub(crate) async fn save(model: $entity::Model) -> Result<$entity::Model, ServerFnError> {
//             Ok(Helper::save(model).await.unwrap())
//         }
//     };
// }

// use server_fn_helper;
