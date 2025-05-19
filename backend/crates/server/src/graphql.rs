mod mutation;
mod schema;

use anyhow::Context;
use async_graphql::ID;
use base64::{
    Engine as _, alphabet,
    engine::{
        DecodePaddingMode,
        general_purpose::{GeneralPurpose, GeneralPurposeConfig},
    },
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

const NO_PAD: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);

/// The pre-configured `NO_PAD` engines will reject inputs containing padding = characters. To encode without padding and still accept padding while decoding, create an engine with that padding mode.
const URL_SAFE_NO_PAD: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, NO_PAD);

pub(crate) use mutation::Mutation;
pub(crate) use schema::QueryRoot;

pub(crate) const CURSOR_PREFIX: &str = "arrayconnection:";

const SEPARATOR: u8 = b':';

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Default)]
pub(crate) struct GlobalID(String);

pub(crate) fn to_local_id(global_id: &str) -> anyhow::Result<i64> {
    let decoded: Vec<u8> = URL_SAFE_NO_PAD.decode(global_id)?;

    let bytes: &[u8] = decoded
        .split(|&byte| byte == SEPARATOR)
        .next_back()
        .ok_or_else(|| anyhow::anyhow!("invalid global id: {}", global_id))?;

    let id: &str = std::str::from_utf8(bytes)?;

    let id: i64 = id.parse::<i64>()?;

    Ok(id)
}

impl GlobalID {
    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn new<T: Display + num::Integer>(type_name: &str, id: T) -> Self {
        Self(URL_SAFE_NO_PAD.encode(format!("{type_name}:{id}")))
    }

    pub(crate) fn to_local_id(&self) -> anyhow::Result<i64> {
        to_local_id(&self.0)
    }
}

impl From<GlobalID> for ID {
    fn from(global_id: GlobalID) -> Self {
        ID(global_id.0)
    }
}

impl From<ID> for GlobalID {
    fn from(id: ID) -> Self {
        Self(id.0)
    }
}

enum PaginationDirection {
    Forward,
    Backward,
}

#[derive(Clone, Default)]
pub(crate) struct ConnectionArgs {
    after: Option<ID>,
    before: Option<ID>,
    first: Option<usize>,
    last: Option<usize>,
    count: Option<usize>,
}

#[derive(Error, Debug)]
enum PaginationError {
    #[error("You must either supply `first` or `last`")]
    MissingFirstOrLast,
}

fn limit(args: &ConnectionArgs) -> Result<(PaginationDirection, usize), PaginationError> {
    if let Some(first) = args.first {
        Ok((PaginationDirection::Forward, first))
    } else if let Some(last) = args.last {
        Ok((PaginationDirection::Backward, last))
    } else {
        Err(PaginationError::MissingFirstOrLast)
    }
}

fn offset(args: &ConnectionArgs) -> anyhow::Result<Option<usize>> {
    if let Some(ref after) = args.after {
        let offset: usize = to_local_id(after)
            .context("Invalid cursor provided as `after` argument")?
            .try_into()?;

        Ok(Some(offset))
    } else if let Some(ref before) = args.before {
        let offset: usize = to_local_id(before)
            .context("Invalid cursor provided as `before` argument")?
            .try_into()?;

        Ok(Some(offset.max(0)))
    } else {
        Ok(None)
    }
}

pub(crate) fn offset_and_limit_for_query(args: &ConnectionArgs) -> anyhow::Result<(usize, usize)> {
    let (direction, limit) = limit(args)?;
    let offset = offset(args)?;

    match direction {
        PaginationDirection::Forward => Ok((offset.unwrap_or(0), limit)),
        PaginationDirection::Backward => match (offset, args.count) {
            (None, None) => Err(anyhow::anyhow!(
                "You must supply a count (total number of records) option if using `last` without `before`",
            )),
            (None, Some(value)) => Ok(((value - limit).max(0), limit)),
            (Some(value), _) => {
                let start_offset = (value - limit).max(0);
                let limit = if start_offset == 0 { value } else { limit };
                Ok((start_offset, limit))
            }
        },
    }
}

/// A helper macro to make fetching entities easier without too much boilerplate
macro_rules! fetch_all {
    ($db:expr, $e: path, ($order_by: expr, $dir: expr)) => {{
        use itertools::Itertools;
        use sea_orm::{EntityTrait, QueryOrder};
        use $e as E;

        let entities = E::Entity::find().order_by($order_by, $dir);

        // println!("{}", sea_orm::debug_query!(&entities, $db));

        let entities = entities
            .all($db)
            .await?
            .into_iter()
            .map(|model| model.try_into())
            .try_collect()?;

        Ok(entities)
    }};

    ($db:expr, $e: path, $offset:expr, $limit:expr) => {{
        use $e as E;

        fetch_all!(
            $db,
            E,
            (E::Column::Id, sea_orm::query::Order::Asc),
            $limit,
            $offset
        )
    }};

    ($db:expr, $e: path, ($order_by: expr, $dir: expr), $limit:expr, $offset:expr) => {{
        use itertools::Itertools;
        use sea_orm::{EntityTrait, QueryOrder, QuerySelect};
        use $e as E;

        let entities = E::Entity::find()
            .order_by($order_by, $dir)
            .offset($offset)
            .limit($limit);

        // println!("{}", sea_orm::debug_query!(&entities, $db));

        let entities = entities
            .all($db)
            .await?
            .into_iter()
            .map(|model| model.try_into())
            .try_collect()?;

        Ok(entities)
    }};
}

pub(crate) use fetch_all;
