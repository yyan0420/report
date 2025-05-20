use crate::{
    components::{FilterInterests, GlobalFilter},
    database::clickhouse::{ClickHouseMultiIf, ClickHouseSumIf},
};
use entity::sales;
use sea_orm::{
    ColumnAsExpr, ColumnTrait, Condition, EntityTrait, IntoIdentity, IntoSimpleExpr, QueryFilter,
    QuerySelect, QueryTrait, Select,
};
use sea_query::{Alias, Expr, Func, SelectStatement};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct YoYQuery {
    pub(crate) global_filter: GlobalFilter,
    pub(crate) interests: FilterInterests,
}

impl YoYQuery {
    pub(crate) fn new(global_filter: GlobalFilter, interests: FilterInterests) -> Self {
        Self {
            global_filter,
            interests,
        }
    }

    // pub(crate) fn ready(&self) -> bool {
    //     match (
    //         self.global_filter.start_datetime,
    //         self.global_filter.end_datetime,
    //     ) {
    //         (Some(s), Some(e)) => s < e,
    //         _ => false,
    //     }
    // }

    // pub(crate) async fn ready_and<F, Fut, T>(self, f: F) -> Option<T>
    // where
    //     F: FnOnce(Self) -> Fut,
    //     Fut: Future<Output = T>,
    // {
    //     if self.ready() {
    //         Some(f(self).await)
    //     } else {
    //         None
    //     }
    // }

    fn add_qty_total(
        dates: &[(String, String)],
        mut query: Select<sales::Entity>,
    ) -> Select<sales::Entity> {
        for (index, (start, end)) in dates.iter().enumerate() {
            // SELECT
            //   sumIf(qty, created_at >= start AND created_at <= end),
            //   sumIf(gross_amount, created_at >= start AND created_at <= end),
            query = query
                .expr_as(
                    Func::cust(ClickHouseSumIf).args([
                        sales::Column::Qty.into_simple_expr(),
                        sales::Column::CreatedAt
                            .gte(start)
                            .and(sales::Column::CreatedAt.lte(end)),
                    ]),
                    format!("qty_{}", index + 1),
                )
                .expr_as(
                    Func::round_with_precision(
                        Func::cust(ClickHouseSumIf).args([
                            sales::Column::GrossAmount.into_simple_expr(),
                            sales::Column::CreatedAt
                                .gte(start)
                                .and(sales::Column::CreatedAt.lte(end)),
                        ]),
                        2,
                    ),
                    format!("total_{}", index + 1),
                );
        }

        query
    }

    fn add_diff(
        dates: &[(String, String)],
        mut query: Select<sales::Entity>,
    ) -> Select<sales::Entity> {
        // Rev to drop the last(oldest) daterange
        for index in 0..(dates.len() - 1) {
            let cur_idx = index + 1;
            let total_1 = Expr::col(Alias::new(format!("total_{cur_idx}")));
            let total_2 = Expr::col(Alias::new(format!("total_{}", cur_idx + 1)));

            query = query.expr_as(
                Func::round_with_precision(total_2.sub(total_1.clone()), 2),
                format!("total_diff_{cur_idx}"),
            );
        }

        query
    }

    fn add_pct(
        dates: &[(String, String)],
        mut query: Select<sales::Entity>,
    ) -> Select<sales::Entity> {
        // Rev to drop the last(oldest) daterange
        for index in 0..(dates.len() - 1) {
            let cur_idx = index + 1;
            let total_1 = Expr::col(Alias::new(format!("total_{cur_idx}")));
            let total_2 = Expr::col(Alias::new(format!("total_{}", cur_idx + 1)));

            query = query.expr_as(
                Func::round_with_precision(
                    Func::cust(ClickHouseMultiIf).args([
                        total_1.clone().eq(0.0).and(total_2.clone().eq(0.0)),
                        Expr::value(0.0),
                        total_2.clone().eq(0.0),
                        Expr::value(None::<f64>),
                        total_1.sub(total_2.clone()).div(total_2).mul(100.0),
                    ]),
                    2,
                ),
                format!("percentage_{cur_idx}"),
            );
        }

        query
    }

    pub(crate) fn add_ranged_where(
        dates: &[(String, String)],
        query: Select<sales::Entity>,
    ) -> Select<sales::Entity> {
        // WHERE (date_range_1) OR (date_range_2) .. OR (date_range_n)
        let mut cond = Condition::any();
        for (start, end) in dates {
            cond = cond.add(
                sales::Column::CreatedAt
                    .gte(start)
                    .and(sales::Column::CreatedAt.lte(end)),
            );
        }

        query.filter(cond)
    }

    pub(crate) async fn build<N, A, C, G>(
        &mut self,
        name: (N, A),
        code: (C, A),
        group_by: G,
    ) -> anyhow::Result<SelectStatement>
    where
        N: ColumnAsExpr + Clone,
        A: IntoIdentity + Clone,
        C: ColumnAsExpr + Clone,
        G: ColumnAsExpr + Clone,
    {
        self.global_filter.sanitize().await?;

        let mut query = sales::Entity::find()
            .select_only()
            .column_as(name.0, name.1)
            .column_as(code.0, code.1);

        let dates: Vec<(String, String)> = self
            .global_filter
            .date_range()
            .ok_or(anyhow::anyhow!("cannot get datetime ranges"))?
            .iter()
            .map(|range| {
                let start = range.start.to_utc().naive_local().to_string();
                let end = range.end.to_utc().naive_local().to_string();

                (start, end)
            })
            .collect();

        query = Self::add_qty_total(&dates, query);
        query = Self::add_pct(&dates, query);
        query = Self::add_diff(&dates, query);
        query = Self::add_ranged_where(&dates, query);

        let mut query = query
            .filter(sales::Column::TagNo.is_not_null())
            .filter(sales::Column::GrossAmount.gt(0.0))
            .filter(sales::Column::StockNo.is_not_in([".F", ".BPOINTS"]))
            .group_by(group_by)
            .into_query();

        // let limit = self.pagination.items_per_page;
        // let offset = self.pagination.offset();

        // query.limit(limit).offset(offset);

        self.global_filter
            .apply_filters(&self.interests, &mut query);

        // Self::apply_sorting(&self.sorting, &mut query);

        Ok(query)
    }

    // fn apply_sorting(sorting: &Sorting, query: &mut SelectStatement) {
    //     let index = sorting.col_idx;
    //     let total = Alias::new(format!("total_{}", index + 1));
    //     let pct = Alias::new(format!("percentage_{}", index + 1));

    //     match (sorting.sort_order, sorting.sort_by) {
    //         (SortOrder::Ascending, SortBy::Total) => {
    //             query.order_by(total, Order::Asc);
    //         }
    //         (SortOrder::Ascending, SortBy::Percentage) => {
    //             query.order_by_with_nulls(pct, Order::Asc, NullOrdering::Last);
    //         }
    //         (SortOrder::Descending, SortBy::Total) => {
    //             query.order_by(total, Order::Desc);
    //         }
    //         (SortOrder::Descending, SortBy::Percentage) => {
    //             query.order_by_with_nulls(pct, Order::Desc, NullOrdering::First);
    //         }
    //         (SortOrder::None, SortBy::Percentage | SortBy::Total) => {}
    //     }

    //     if sorting.sort_order != SortOrder::None {
    //         // No second sort for last column
    //         let second_sort = index + 1 < crate::Summary::get_range();

    //         if second_sort {
    //             let total_diff = Alias::new(format!("total_diff_{}", index + 1));
    //             query.order_by(total_diff, Order::Desc);
    //         }
    //     }
    // }

    // pub(crate) async fn reset_for_total(
    //     mut subquery: SelectStatement,
    // ) -> Result<u64, Box<dyn Error + Send + Sync + 'static>> {
    //     // We are only interested in the total count and thus resetting limit, offset and
    //     // order_by
    //     subquery.reset_limit().reset_offset().clear_order_by();

    //     let mut query = sea_query::Query::select();

    //     query
    //         .expr(Func::count("*"))
    //         .from_subquery(subquery, Alias::new("dataset"));

    //     Ok(QueryBuilder::new(&client()?, &query).fetch_one().await?)
    // }

    // pub(crate) async fn fetch_total<N, A, C, G>(
    //     mut self,
    //     name: (N, A),
    //     code: (C, A),
    //     group_by: G,
    // ) -> Result<u64, Box<dyn Error + Send + Sync + 'static>>
    // where
    //     N: IntoSimpleExpr + Clone + ColumnTrait,
    //     A: IntoIdentity + Clone,
    //     C: IntoSimpleExpr + Clone + ColumnTrait,
    //     G: IntoSimpleExpr + Clone + ColumnTrait,
    // {
    //     Self::reset_for_total(self.build(name, code, group_by).await?).await
    // }
}
