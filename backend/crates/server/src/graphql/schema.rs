use super::GlobalID;
use crate::brand::Brand;
use async_graphql::{
    Context, Object, OutputType,
    types::{
        ID,
        connection::{self, Connection, Edge},
    },
};
use clickhouse::Client;

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
    // async fn brand<'a>(
    //     &self,
    //     _ctx: &Context<'a>
    // ) -> Option<Result<brand::Model, async_graphql::Error>> {

    //     // Return a dummy brand object wrapped in Ok
    //     // Some(Ok(Brand {
    //     //     id: 1,
    //     //     name: "Dummy Brand".to_string(),
    //     //     url_slug: "dummy-brand".to_string(),
    //     //     private_label: false,
    //     //     status: true,
    //     // }))

    //     match Brand::fetch_all().await {
    //         Ok(mut models) => models.pop().map(Ok), // Take the first brand
    //         Err(e) => Some(Err(async_graphql::Error::new(e.to_string()))),
    //     }
    // }

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
}
