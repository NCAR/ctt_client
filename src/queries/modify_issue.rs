use super::ToOffline;
use serde::{Deserialize, Serialize};
pub struct ModifyIssue;
pub const OPERATION_NAME: &str = "ModifyIssue";
pub const QUERY : & str = "mutation ModifyIssue($issue: UpdateIssue!) {\n  updateIssue(issue: $issue){\n    title,\n    id,\n    assignedTo,\n    description,\n    toOffline,\n    enforceDown,\n  }\n}\n" ;
#[derive(Serialize, clap::Args)]
pub struct UpdateIssue {
    #[serde(rename = "assignedTo")]
    pub assigned_to: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "enforceDown")]
    pub enforce_down: Option<bool>,
    #[serde(rename = "toOffline")]
    pub to_offline: Option<ToOffline>,
    pub id: i32,
    pub title: Option<String>,
}
#[derive(Serialize)]
pub struct Variables {
    pub issue: UpdateIssue,
}
#[derive(Deserialize)]
pub struct ResponseData {
    #[serde(rename = "updateIssue")]
    pub update_issue: ModifyIssueUpdateIssue,
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
