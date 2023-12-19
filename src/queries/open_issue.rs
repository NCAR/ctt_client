use super::{TargetStatus, ToOffline};
use serde::{Deserialize, Serialize};

pub struct OpenIssue;
pub const OPERATION_NAME: &str = "OpenIssue";
pub const QUERY : & str = "mutation OpenIssue($newIssue: NewIssue!) {\n  open(issue: $newIssue) {\n    id,\n    target{name,status}\n  }\n}\n" ;
#[derive(Serialize, clap::Args)]
pub struct NewIssue {
    pub target: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "toOffline")]
    #[arg(short, long, value_enum, default_value_t=ToOffline::Node)]
    pub to_offline: ToOffline,
    #[serde(rename = "assignedTo")]
    #[arg(short, long)]
    pub assigned_to: Option<String>,
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
