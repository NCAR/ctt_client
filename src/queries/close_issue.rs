pub struct CloseIssue;
pub const OPERATION_NAME: &str = "CloseIssue";
pub const QUERY : & str = "mutation CloseIssue($id: Int!, $comment: String!) {\n  close(issue: $id, comment: $comment)\n}\n" ;
use serde::{Deserialize, Serialize};
#[derive(Serialize, clap::Args)]
pub struct Variables {
    pub id: i32,
    pub comment: String,
}
#[derive(Deserialize)]
pub struct ResponseData {
    pub close: String,
}
impl graphql_client::GraphQLQuery for CloseIssue {
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
