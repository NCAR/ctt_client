use super::*;
pub use crate::cli::ModifyUpdateIssue as UpdateIssue;
use serde::{Deserialize, Serialize};

pub struct ModifyIssue;
pub const OPERATION_NAME: &str = "ModifyIssue";
pub const QUERY : & str = "mutation ModifyIssue($issue: UpdateIssue!) {\n  updateIssue(issue: $issue){\n    assignedTo,\n    createdAt,\n    createdBy,\n    description,\n    toOffline,\n    id,\n    status,\n    title,\n    comments{createdBy, comment, createdAt},\n    target{name, status},\n related{name, status}  }\n}" ;
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
