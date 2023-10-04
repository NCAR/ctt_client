use serde::{Deserialize, Serialize};
use clap;

pub struct ListIssues;
pub const OPERATION_NAME: &str = "ListIssues";
pub const QUERY : & str = "query ListIssues($status: IssueStatus, $target: String) {\n  issues(issueStatus: $status, target: $target) {\n    id,\n    title,\n    assignedTo,\n    description,\n    toOffline,\n    target{name, status},\n  }\n}\n" ;
#[derive(Clone, clap::ValueEnum)]
pub enum IssueStatus {
    OPEN,
    CLOSED,
}
impl ::serde::Serialize for IssueStatus {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            IssueStatus::OPEN => "OPEN",
            IssueStatus::CLOSED => "CLOSED",
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for IssueStatus {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "OPEN" => Ok(IssueStatus::OPEN),
            "CLOSED" => Ok(IssueStatus::CLOSED),
            _ => panic!("can't parse {}", s),
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
    pub status: Option<IssueStatus>,
    pub target: Option<String>,
}
#[derive(Deserialize)]
pub struct ResponseData {
    pub issues: Vec<ListIssuesIssues>,
}
#[derive(Deserialize)]
pub struct ListIssuesIssues {
    pub id: i32,
    pub title: String,
    #[serde(rename = "assignedTo")]
    pub assigned_to: Option<String>,
    pub description: String,
    #[serde(rename = "toOffline")]
    pub to_offline: Option<ToOffline>,
    pub target: Option<ListIssuesIssuesTarget>,
}
#[derive(Deserialize)]
pub struct ListIssuesIssuesTarget {
    pub name: String,
    pub status: TargetStatus,
}
impl graphql_client::GraphQLQuery for ListIssues {
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
