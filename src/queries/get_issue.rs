pub struct GetIssue;
pub const OPERATION_NAME: &str = "GetIssue";
pub const QUERY : & str = "query GetIssue($id: Int!){\n  issue(issue: $id){\n    assignedTo,\n    createdAt,\n    createdBy,\n    description,\n    toOffline,\n    enforceDown,\n    id,\n    issueStatus,\n    title,\n    comments{createdBy, comment, createdAt},\n    target{name, status}\n  }\n}\n" ;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
pub enum IssueStatus {
    OPEN,
    CLOSED,
    Other(String),
}
impl ::serde::Serialize for IssueStatus {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            IssueStatus::OPEN => "OPEN",
            IssueStatus::CLOSED => "CLOSED",
            IssueStatus::Other(ref s) => s,
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for IssueStatus {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "OPEN" => Ok(IssueStatus::OPEN),
            "CLOSED" => Ok(IssueStatus::CLOSED),
            _ => Ok(IssueStatus::Other(s)),
        }
    }
}
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
pub enum ToOffline {
    TARGET,
    SIBLINGS,
    COUSINS,
    Other(String),
}
impl ::serde::Serialize for ToOffline {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            ToOffline::TARGET => "TARGET",
            ToOffline::SIBLINGS => "SIBLINGS",
            ToOffline::COUSINS => "COUSINS",
            ToOffline::Other(ref s) => s,
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for ToOffline {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "TARGET" => Ok(ToOffline::TARGET),
            "SIBLINGS" => Ok(ToOffline::SIBLINGS),
            "COUSINS" => Ok(ToOffline::COUSINS),
            _ => Ok(ToOffline::Other(s)),
        }
    }
}
#[derive(Serialize, clap::Args)]
pub struct Variables {
    pub id: i32,
}
#[derive(Deserialize)]
pub struct ResponseData {
    pub issue: Option<GetIssueIssue>,
}
#[derive(Deserialize)]
pub struct GetIssueIssue {
    #[serde(rename = "assignedTo")]
    pub assigned_to: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    pub description: String,
    #[serde(rename = "toOffline")]
    pub to_offline: Option<ToOffline>,
    #[serde(rename = "enforceDown")]
    pub enforce_down: bool,
    pub id: i32,
    #[serde(rename = "issueStatus")]
    pub issue_status: IssueStatus,
    pub title: String,
    pub comments: Vec<GetIssueIssueComments>,
    pub target: Option<GetIssueIssueTarget>,
}
#[derive(Deserialize)]
pub struct GetIssueIssueComments {
    #[serde(rename = "createdBy")]
    pub created_by: String,
    pub comment: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}
#[derive(Deserialize)]
pub struct GetIssueIssueTarget {
    pub name: String,
    pub status: TargetStatus,
}
impl graphql_client::GraphQLQuery for GetIssue {
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
