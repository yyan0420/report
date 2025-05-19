use anyhow::Context;
use clickhouse::{query::Query, Client, Row};
use percent_encoding::percent_decode_str;
use sea_query::{
    func::FunctionCall, prepare::Write, query::SelectStatement, Func, Iden, SimpleExpr,
};
use serde::Deserialize;

pub(crate) struct ClickHouseIfNull;

impl Iden for ClickHouseIfNull {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "ifNull").unwrap();
    }
}

pub(crate) struct ClickHouseToNullable;

impl Iden for ClickHouseToNullable {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "toNullable").unwrap();
    }
}

pub(crate) struct ClickHouseToU64;

impl Iden for ClickHouseToU64 {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "toUInt64").unwrap();
    }
}

pub(crate) struct ClickHouseAssumeNotNull;

impl Iden for ClickHouseAssumeNotNull {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "assumeNotNull").unwrap();
    }
}

pub(crate) struct ClickHouseAny;

impl Iden for ClickHouseAny {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "any").unwrap();
    }
}

pub(crate) fn any<T: Into<SimpleExpr>>(col: T) -> FunctionCall {
    Func::cust(ClickHouseAny).arg(col)
}

pub(crate) struct ClickHouseArrayJoin;

impl Iden for ClickHouseArrayJoin {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "arrayJoin").unwrap();
    }
}

pub(crate) struct ClickHouseIf;

impl Iden for ClickHouseIf {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "if").unwrap();
    }
}

pub(crate) struct ClickHouseMultiIf;

impl Iden for ClickHouseMultiIf {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "multiIf").unwrap();
    }
}

// pub(crate) struct ClickHouseEmpty;

// impl Iden for ClickHouseEmpty {
//     fn unquoted(&self, s: &mut dyn Write) {
//         write!(s, "empty").unwrap();
//     }
// }

pub(crate) struct ClickHouseToHour;

impl Iden for ClickHouseToHour {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "toHour").unwrap();
    }
}

pub(crate) struct ClickHousetoDate;

impl Iden for ClickHousetoDate {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "toDate").unwrap();
    }
}

pub(crate) struct ClickHouseAddYear;

impl Iden for ClickHouseAddYear {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "addYears").unwrap();
    }
}

pub(crate) struct ClickHousetoString;

impl Iden for ClickHousetoString {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "toString").unwrap();
    }
}

pub(crate) struct ClickHouseConcat;

impl Iden for ClickHouseConcat {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "concat").unwrap();
    }
}

pub(crate) struct ClickHouseSumIf;

impl Iden for ClickHouseSumIf {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "sumIf").unwrap();
    }
}

pub(crate) struct ClickHouseCountIf;

impl Iden for ClickHouseCountIf {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "countIf").unwrap();
    }
}

pub(crate) struct ClickHouseDistinct;

impl Iden for ClickHouseDistinct {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "distinct").unwrap();
    }
}

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

    pub(crate) async fn fetch_one<T>(self) -> clickhouse::error::Result<T>
    where
        T: Row + for<'b> Deserialize<'b>,
    {
        self.query.fetch_one::<T>().await
    }
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
