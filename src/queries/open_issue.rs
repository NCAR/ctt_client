use super::ToOffline;
pub struct OpenIssue;
pub const OPERATION_NAME: &str = "OpenIssue";
pub const QUERY : & str = "mutation OpenIssue($newIssue: NewIssue!) {\n  open(issue: $newIssue) {\n    id,\n    target{name,status}\n  }\n}\n" ;
use serde::{Deserialize, Serialize};
pub enum TargetStatus {
    ONLINE,
    DRAINING,
    OFFLINE,
    DOWN,
    UNKNOWN,
    Other(String),
}
impl ::serde::Serialize for TargetStatus {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            TargetStatus::ONLINE => "ONLINE",
            TargetStatus::DRAINING => "DRAINING",
            TargetStatus::OFFLINE => "OFFLINE",
            TargetStatus::DOWN => "DOWN",
            TargetStatus::UNKNOWN => "UNKNOWN",
            TargetStatus::Other(ref s) => s,
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for TargetStatus {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "ONLINE" => Ok(TargetStatus::ONLINE),
            "DRAINING" => Ok(TargetStatus::DRAINING),
            "OFFLINE" => Ok(TargetStatus::OFFLINE),
            "DOWN" => Ok(TargetStatus::DOWN),
            "UNKNOWN" => Ok(TargetStatus::UNKNOWN),
            _ => Ok(TargetStatus::Other(s)),
        }
    }
}
#[derive(Serialize, clap::Args)]
pub struct NewIssue {
    #[serde(rename = "assignedTo")]
    pub assigned_to: Option<String>,
    pub description: String,
    #[serde(rename = "toOffline")]
    pub to_offline: Option<ToOffline>,
    #[serde(rename = "enforceDown")]
    pub enforce_down: Option<bool>,
    pub target: String,
    pub title: String,
}
#[derive(Serialize)]
pub struct Variables {
    #[serde(rename = "newIssue")]
    pub new_issue: NewIssue,
}
#[derive(Deserialize)]
pub struct ResponseData {
    pub open: OpenIssueOpen,
}
#[derive(Deserialize)]
pub struct OpenIssueOpen {
    pub id: i32,
    pub target: Option<OpenIssueOpenTarget>,
}
#[derive(Deserialize)]
pub struct OpenIssueOpenTarget {
    pub name: String,
    pub status: TargetStatus,
}
impl graphql_client::GraphQLQuery for OpenIssue {
    type Variables = Variables;
    type ResponseData = ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: QUERY,
            operation_name: OPERATION_NAME,
        }
    }
}
