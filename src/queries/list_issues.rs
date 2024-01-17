use super::*;
pub use crate::cli::ListVariables as Variables;
use serde::Deserialize;

pub struct ListIssues;
pub const OPERATION_NAME: &str = "ListIssues";
pub const QUERY : & str = "query ListIssues($status: IssueStatus, $target: String) {\n  issues(issueStatus: $status, target: $target) {\n    id,\n    title,\n    assignedTo,\n    description,\n    toOffline,\n updatedAt, \n    target{name, status},\n    related{name,status},\n  }\n}\n" ;
#[derive(Deserialize)]
pub struct ResponseData {
    pub issues: Vec<ListIssuesIssues>,
}
#[derive(Deserialize)]
pub struct ListIssuesIssues {
    #[serde(rename = "assignedTo")]
    pub assigned_to: Option<String>,
    pub description: String,
    pub id: i32,
    pub title: String,
    #[serde(rename = "toOffline")]
    pub to_offline: Option<ToOffline>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::NaiveDateTime,
    pub target: Option<ListIssuesIssuesTarget>,
    pub related: Vec<ListIssuesIssuesTarget>,
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
