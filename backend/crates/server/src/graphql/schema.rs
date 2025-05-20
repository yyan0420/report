use super::GlobalID;
use crate::database::clickhouse::{QueryBuilder, client};
use crate::{
    BrandOrigin, Summary,
    brand::Brand,
    components::{FilterInterests, FilterKind, GlobalFilter},
    data_source::DataSource,
    yoy::yoytable::YoYQuery,
};
use async_graphql::{
    Context, Object, OutputType,
    types::{
        ID,
        connection::{self, Connection, Edge},
    },
};
use chrono::{Local, NaiveDate, TimeZone};
use clickhouse::Client;
use entity::web_product;
use sea_query::Expr;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct QueryRoot;

async fn fetch_connection<'a, F, Fut, U, T>(
    db: &'a Client,
    after: Option<ID>,
    before: Option<ID>,
    first: Option<i32>,
    last: Option<i32>,
    count: Option<usize>,
    fetcher: F,
) -> anyhow::Result<Connection<ID, T>>
where
    F: FnOnce(&'a Client, u64, u64) -> Fut,
    Fut: Future<Output = anyhow::Result<U>>,
    U: IntoIterator<Item = T>,
    T: OutputType,
{
    let mut connection = Connection::new(false, false);

    connection::query(
        after.map(|a| a.0),
        before.map(|b| b.0),
        first,
        last,
        |after, before, first, last| async move {
            let args = super::ConnectionArgs {
                after,
                before,
                first,
                last,
                count,
            };

            let (offset, limit) = super::offset_and_limit_for_query(&args)?;
            let entities = fetcher(db, limit as u64, offset as u64).await?;

            connection
                .edges
                .extend(entities.into_iter().enumerate().map(|(idx, item)| {
                    let id: ID = GlobalID::new(super::CURSOR_PREFIX, idx).into();

                    Edge::new(id, item)
                }));

            Ok::<_, async_graphql::Error>(connection)
        },
    )
    .await
    .map_err(|err| anyhow::anyhow!(err.message))
}

#[Object]
impl QueryRoot {
    async fn brands(
        &self,
        ctx: &Context<'_>,
        after: Option<ID>,
        before: Option<ID>,
        first: Option<i32>,
        last: Option<i32>,
        count: Option<usize>,
    ) -> anyhow::Result<Connection<ID, Brand>> {
        let db = ctx
            .data::<Client>()
            .map_err(|err| anyhow::anyhow!(err.message))?;

        fetch_connection(
            db,
            after,
            before,
            first,
            last,
            count,
            async |db, _limit, _offset| Brand::fetch_all(db).await,
        )
        .await
    }

    async fn yoy_table(&self) -> async_graphql::Result<Vec<Summary>> {
        // let client = ctx
        //     .data::<Client>()
        //     .map_err(|err| anyhow::anyhow!(err.message))?;

        let start_naive = NaiveDate::from_ymd_opt(2025, 5, 16)
            .expect("Invalid start date")
            .and_hms_opt(0, 0, 0)
            .expect("Invalid start time");

        let end_naive = NaiveDate::from_ymd_opt(2025, 5, 17)
            .expect("Invalid end date")
            .and_hms_opt(0, 0, 0)
            .expect("Invalid end time");

        let start_datetime = Local
            .from_local_datetime(&start_naive)
            .single()
            .expect("Ambiguous or invalid start datetime");

        let end_datetime = Local
            .from_local_datetime(&end_naive)
            .single()
            .expect("Ambiguous or invalid end datetime");

        let filter = GlobalFilter {
            start_datetime: Some(start_datetime),
            end_datetime: Some(end_datetime),
            data_source: DataSource::InStore,
            brand_origin: BrandOrigin::All,
            filters: HashMap::new(),
        };

        let interests: FilterInterests =
            Arc::from([FilterKind::Brand, FilterKind::Store, FilterKind::Operator]);

        // You can now use `filter` as needed
        let mut test_query = YoYQuery::new(filter, interests);

        let name = (web_product::Column::Brand, String::from("name"));
        let code = (Expr::value(None::<String>), String::from("code"));
        let group_by = web_product::Column::Brand;

        let summaries: Vec<Summary> =
            QueryBuilder::new(&client()?, &test_query.build(name, code, group_by).await?)
                .fetch_all()
                .await?;

        Ok(summaries)
    }
}
