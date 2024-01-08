pub use crate::cli::CloseVariables as Variables;
use serde::Deserialize;

pub struct CloseIssue;
pub const OPERATION_NAME: &str = "CloseIssue";
pub const QUERY : & str = "mutation CloseIssue($id: Int!, $comment: String!) {\n  close(issue: $id, comment: $comment)\n}\n" ;
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
