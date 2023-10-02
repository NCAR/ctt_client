use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
mod queries;
pub use queries::{get_issue, close_issue, create_issue, list_issues, update_issue};

pub fn issue_open(client: &Client, srv: &str, new: create_issue::NewIssue) -> Result<i64, String> {
    let issue = create_issue::Variables { new_issue: new };

    let resp = post_graphql::<queries::CreateIssue, _>(client, srv, issue).unwrap();
    if let Some(errors) = resp.errors {
        return Err(errors[0].message.to_string());
    }
    let resp_data = resp.data.unwrap();
    Ok(resp_data.open.id)
}

pub fn issue_close(client: &Client, srv: &str, vars: close_issue::Variables) -> Result<String, String> {
    let resp_body = post_graphql::<queries::CloseIssue, _>(client, srv, vars).unwrap();
    if let Some(errors) = resp_body.errors {
        return Err(errors[0].message.to_string());
    }
    let data: close_issue::ResponseData =
        resp_body.data.unwrap();
    Ok(data.close)
}

pub fn issue_update(client: &Client, srv: &str, vars: update_issue::UpdateIssue) -> Result<update_issue::UpdateIssueUpdate, String> {
    let issue = update_issue::Variables { issue: vars };
    let resp_body = post_graphql::<queries::UpdateIssue, _>(client, srv, issue).unwrap();
    if let Some(errors) = resp_body.errors {
        return Err(errors[0].message.to_string());
    }
    let data: update_issue::ResponseData = resp_body.data.unwrap();
    Ok(data.update)
}

pub fn issue_list(
    client: &Client,
    srv: &str,
    filter: list_issues::Variables,
) -> Result<Vec<list_issues::ListIssuesIssues>, String> {
    let response_body = post_graphql::<queries::ListIssues, _>(client, srv, filter).unwrap();
    if let Some(errors) = response_body.errors {
        return Err(errors[0].message.to_string());
    }

    let response_data: list_issues::ResponseData =
        response_body.data.unwrap();
    Ok(response_data.issues)
}

pub fn issue_show(client: &Client, srv: &str, vars: get_issue::Variables) -> Result<Option<get_issue::GetIssueIssue>, String> {
    let resp_body = post_graphql::<queries::GetIssue, _>(client, srv, vars).unwrap();
    if let Some(errors) = resp_body.errors {
        return Err(errors[0].message.to_string());
    }
    let data: get_issue::ResponseData = 
        resp_body.data.unwrap();
    Ok(data.issue)
}
