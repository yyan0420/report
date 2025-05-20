use anyhow::Context;
use clickhouse::{Client, Row, query::Query};
use percent_encoding::percent_decode_str;
use sea_query::{Iden, prepare::Write, query::SelectStatement};
use serde::Deserialize;

pub(crate) struct QueryBuilder {
    query: Query,
}

impl QueryBuilder {
    pub(crate) fn new(client: &Client, query: &SelectStatement) -> Self {
        let q = query.to_string(sea_query::MysqlQueryBuilder);

        tracing::debug!("{q}");

        Self {
            query: client.query(&q),
        }
    }

    pub(crate) async fn fetch_all<T>(self) -> clickhouse::error::Result<Vec<T>>
    where
        T: Row + for<'b> Deserialize<'b>,
    {
        self.query.fetch_all::<T>().await
    }

    // pub(crate) async fn fetch_one<T>(self) -> clickhouse::error::Result<T>
    // where
    //     T: Row + for<'b> Deserialize<'b>,
    // {
    //     self.query.fetch_one::<T>().await
    // }
}

pub(crate) fn client() -> anyhow::Result<Client> {
    let database_url: String =
        std::env::var("CLICKHOUSE_URL").context("CLICKHOUSE_URL is not set")?;

    // FIXME
    // It breaks if the original password has percent-encoding character sets.
    // [`Url::parse`] should not be used here as we don't want to encode/decoding anything fomr the
    // URL.
    let url = url::Url::parse(&database_url)
        .context(format!("cannot parase CLICKHOUSE_URL {database_url}"))?;

    let password = percent_decode_str(url.password().context("cannot get password from url")?)
        .decode_utf8()?;

    Ok(Client::default()
        .with_user(url.username())
        .with_password(password)
        .with_database(url.path().trim_start_matches('/'))
        .with_url(format!(
            "{}://{}:{:?}",
            url.scheme(),
            url.host().context("cannot get host from url")?,
            url.port().context("cannot get port from url")?
        )))
}

pub(crate) struct ClickHouseSumIf;

impl Iden for ClickHouseSumIf {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "sumIf").unwrap();
    }
}

pub(crate) struct ClickHouseMultiIf;

impl Iden for ClickHouseMultiIf {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "multiIf").unwrap();
    }
}

pub(crate) struct ClickHouseArrayJoin;

impl Iden for ClickHouseArrayJoin {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "arrayJoin").unwrap();
    }
}
