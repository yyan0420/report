#![allow(clippy::type_complexity)]
// use chrono::{DateTime, Days, Local, NaiveTime};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// const START_TIME: NaiveTime = match NaiveTime::from_hms_opt(0, 0, 0) {
//     Some(nt) => nt,
//     _ => panic!("error on parsing START_TIME"),
// };

// const END_TIME: NaiveTime = match NaiveTime::from_hms_opt(23, 59, 59) {
//     Some(nt) => nt,
//     _ => panic!("error on parsing END_TIME"),
// };

// pub(crate) fn beginning_of_day(dt: NaiveDate) -> Option<DateTime<Local>> {
//     Local.from_local_datetime(&dt.and_time(START_TIME)).single()
// }

// pub(crate) fn end_of_day(dt: NaiveDate) -> Option<DateTime<Local>> {
//     Local.from_local_datetime(&dt.and_time(END_TIME)).single()
// }

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
pub(crate) enum DataSource {
    #[default]
    OnlineInStore,
    InStore,
    // Online,
}

impl FromStr for DataSource {
    type Err = String;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "OnlineInStore" => Ok(Self::OnlineInStore),
            "InStore" => Ok(Self::InStore),
            // "Online" => Ok(Self::Online),
            _ => Err(format!("uknown source {source}")),
        }
    }
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub enum Limit {
    #[default]
    Last90Days = 90,
}

impl Limit {
    // pub(crate) const ID: &str = "1";

    // pub(crate) fn to_datetime_string(self) -> Option<String> {
    //     Some(self.to_datetime()?.format("%Y-%m-%d").to_string())
    // }

    // pub(crate) fn to_datetime(self) -> Option<DateTime<Local>> {
    //     let now = Local::now();

    //     now.with_time(NaiveTime::MIN)
    //         .single()?
    //         .checked_sub_days(Days::new(self as u64))
    // }

    // pub(crate) fn to_relation(self) -> String {
    //     match self {
    //         Self::Last90Days => String::from("last_90_days"),
    //     }
    // }

    pub(crate) fn to_permission(self) -> String {
        match self {
            Self::Last90Days => String::from("limit_90_days"),
        }
    }
}

// impl spicedb::Def for Limit {
//     fn def_name(&self) -> String {
//         "datasource".to_string()
//     }
// }

impl fmt::Display for Limit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            Self::Last90Days => "Last 90 Days",
        };

        write!(f, "{display}")
    }
}

impl FromStr for Limit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for limit in Self::iter() {
            if limit.to_permission() == s {
                return Ok(limit);
            }
        }

        Err(format!(
            "cannot convert permission {s} to data source Limit"
        ))
    }
}

impl TryFrom<u32> for Limit {
    type Error = String;

    fn try_from(days: u32) -> Result<Self, Self::Error> {
        match days {
            90 => Ok(Self::Last90Days),
            _ => Err(format!("cannot convert {days} to DataSourceLimit")),
        }
    }
}

// #[server]
// pub(crate) async fn get_limit() -> Result<Option<Limit>, ServerFnError> {
//     use spicedb::{
//         authzed_api_community_neoeinstein_prost::authzed::api::v1::{
//             check_bulk_permissions_pair::Response, check_permission_response::Permissionship,
//             consistency::Requirement, CheckBulkPermissionsPair, CheckBulkPermissionsRequest,
//             CheckBulkPermissionsRequestItem, CheckBulkPermissionsResponseItem, Consistency,
//             ObjectReference, SubjectReference,
//         },
//         Def,
//     };
//     use strum::IntoEnumIterator;

//     use crate::server::db::spice_client;

//     crate::auth::server::auth_guard!()?;

//     let session: crate::auth::server::Session = extract().await?;
//     let Some(ref user) = session.current_user else {
//         return Err(ServerFnError::new("not authenticated"));
//     };

//     let mut client = spice_client().await.map_err(ServerFnError::new)?;

//     let req = CheckBulkPermissionsRequest {
//         consistency: Some(Consistency {
//             requirement: Some(Requirement::MinimizeLatency(true)),
//         }),
//         items: Limit::iter()
//             .map(|limit| CheckBulkPermissionsRequestItem {
//                 resource: Some(ObjectReference {
//                     object_type: limit.def_name().to_string(),
//                     object_id: Limit::ID.to_string(),
//                 }),
//                 permission: limit.to_permission(),
//                 subject: Some(SubjectReference {
//                     object: Some(ObjectReference {
//                         object_type: user.def_name().to_string(),
//                         object_id: user.id.to_string(),
//                     }),
//                     ..Default::default()
//                 }),
//                 ..Default::default()
//             })
//             .collect(),
//     };

//     Ok(client
//         .check_bulk_permissions(req)
//         .await?
//         .into_inner()
//         .pairs
//         .into_iter()
//         .filter_map(|CheckBulkPermissionsPair { request, response }| {
//             const PERMISSIONSHIP: i32 = Permissionship::HasPermission as i32;
//             match (request, response) {
//                 (
//                     Some(CheckBulkPermissionsRequestItem { permission, .. }),
//                     Some(Response::Item(CheckBulkPermissionsResponseItem {
//                         permissionship: PERMISSIONSHIP,
//                         ..
//                     })),
//                 ) => Some(permission),
//                 _ => None,
//             }
//         })
//         .filter_map(|chart_id| chart_id.parse().ok())
//         .max())
// }
