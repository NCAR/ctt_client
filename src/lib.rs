use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
pub mod cli;
pub mod queries;
use queries::*;

pub fn issue_open(client: &Client, srv: &str, new: open_issue::NewIssue) -> Result<i32, String> {
    let issue = open_issue::Variables { new_issue: new };

    let resp = post_graphql::<open_issue::OpenIssue, _>(client, srv, issue).unwrap();
    if let Some(errors) = resp.errors {
        return Err(errors[0].message.to_string());
    }
    let resp_data = resp.data.unwrap();
    Ok(resp_data.open.id)
}

pub fn issue_close(
    client: &Client,
    srv: &str,
    vars: close_issue::Variables,
) -> Result<String, String> {
    let resp_body = post_graphql::<close_issue::CloseIssue, _>(client, srv, vars).unwrap();
    if let Some(errors) = resp_body.errors {
        return Err(errors[0].message.to_string());
    }
    let data: close_issue::ResponseData = resp_body.data.unwrap();
    Ok(data.close)
}

pub fn issue_list(
    client: &Client,
    srv: &str,
    filter: list_issues::Variables,
) -> Result<Vec<list_issues::ListIssuesIssues>, String> {
    let response_body = post_graphql::<list_issues::ListIssues, _>(client, srv, filter).unwrap();
    if let Some(errors) = response_body.errors {
        return Err(errors[0].message.to_string());
    }

    let response_data: list_issues::ResponseData = response_body.data.unwrap();
    Ok(response_data.issues)
}

pub fn issue_show(
    client: &Client,
    srv: &str,
    vars: get_issue::Variables,
) -> Result<Option<get_issue::GetIssueIssue>, String> {
    let resp_body = post_graphql::<get_issue::GetIssue, _>(client, srv, vars).unwrap();
    if let Some(errors) = resp_body.errors {
        return Err(errors[0].message.to_string());
    }
    let data: get_issue::ResponseData = resp_body.data.unwrap();
    Ok(data.issue)
}

pub fn issue_update(
    client: &Client,
    srv: &str,
    vars: modify_issue::UpdateIssue,
) -> Result<get_issue::GetIssueIssue, String> {
    let issue = modify_issue::Variables { issue: vars };
    let resp_body = post_graphql::<modify_issue::ModifyIssue, _>(client, srv, issue).unwrap();
    if let Some(errors) = resp_body.errors {
        return Err(errors[0].message.to_string());
    }
    let data: modify_issue::ResponseData = resp_body.data.unwrap();
    Ok(data.update_issue)
}
