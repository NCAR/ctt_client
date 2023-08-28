use reqwest::blocking::Client;
use graphql_client::reqwest::post_graphql_blocking as post_graphql;
mod queries;
pub use queries::{list_issues, create_issue};


pub fn issue_open(client: &Client, srv: &str, new: create_issue::NewIssue) -> Result<i64, String> {
    
    let issue = create_issue::Variables { new_issue:new };

    let resp = post_graphql::<queries::CreateIssue, _>(&client, srv, issue).unwrap();
    if let Some(errors) = resp.errors {
        return Err(format!("{}", errors[0].message))
    }
    let resp_data = resp.data.unwrap();
    Ok(resp_data.open.id)
}

pub fn issue_list(client: &Client, srv: &str, filter: list_issues::Variables) -> Result<Vec<list_issues::ListIssuesIssues>, ()> {
    let response_body =
        post_graphql::<queries::ListIssues, _>(&client, srv, filter).unwrap();

    let response_data: list_issues::ResponseData = response_body.data.expect("missing response data");
    Ok(response_data.issues)
}
