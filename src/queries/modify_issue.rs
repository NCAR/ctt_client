use super::*;
use serde::{Deserialize, Serialize};

pub struct ModifyIssue;
pub const OPERATION_NAME: &str = "ModifyIssue";
pub const QUERY : & str = "mutation ModifyIssue($issue: UpdateIssue!) {\n  updateIssue(issue: $issue){\n    assignedTo,\n    createdAt,\n    createdBy,\n    description,\n    toOffline,\n    enforceDown,\n    id,\n    issueStatus,\n    title,\n    comments{createdBy, comment, createdAt},\n    target{name, status}\n  }\n}" ;
#[derive(Serialize, clap::Args, Debug)]
pub struct UpdateIssue {
    pub id: i32,
    #[serde(rename = "assignedTo")]
    #[arg(short, long)]
    pub assigned_to: Option<String>,
    #[arg(short, long)]
    pub description: Option<String>,
    #[serde(rename = "enforceDown")]
    #[arg(short, long)]
    pub enforce_down: Option<bool>,
    #[serde(rename = "toOffline")]
    #[arg(long)]
    pub to_offline: Option<ToOffline>,
    #[arg(short, long)]
    pub title: Option<String>,
}
#[derive(Serialize, Debug)]
pub struct Variables {
    pub issue: UpdateIssue,
}
#[derive(Deserialize)]
pub struct ResponseData {
    #[serde(rename = "updateIssue")]
    pub update_issue: super::get_issue::GetIssueIssue,
}
#[derive(Deserialize)]
pub struct ModifyIssueUpdateIssue {
    pub title: String,
    pub id: i32,
    #[serde(rename = "assignedTo")]
    pub assigned_to: Option<String>,
    pub description: String,
    #[serde(rename = "toOffline")]
    pub to_offline: Option<ToOffline>,
    #[serde(rename = "enforceDown")]
    pub enforce_down: bool,
}
impl graphql_client::GraphQLQuery for ModifyIssue {
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
