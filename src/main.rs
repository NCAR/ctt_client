use ::reqwest::blocking::Client;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use chrono::NaiveDateTime;
use prettytable::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "issues.graphql",
    response_derives = "Debug"
    )]
struct Issues;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "newIssue.graphql",
    response_derives = "Debug"
    )]
struct NewIssue;


fn main() {
    let client = Client::new();

    let v = new_issue::Variables {
        new_issue: new_issue::NewIssue {
            assigned_to: "shanks".to_string(),
            description: "cli created".to_string(),
            down_siblings: false,
            enforce_down: false,
            target: "gu0004".to_string(),
            title: "cli test ticket".to_string()
        }
    };

    let resp = post_graphql::<NewIssue, _>(&client, "http:localhost:8000", v).unwrap();
    if let Some(errors) = resp.errors {
        println!("error:");
        for error in &errors {
            println!("{:?}", error);
        }
    }
    let resp_data = resp.data.unwrap();
    println!("{:?}", resp_data);


    let variables = issues::Variables {
        status: Some("OPEN".to_string()),
        target: None,
    };

    let response_body =
        post_graphql::<Issues, _>(&client, "http://localhost:8000", variables).unwrap();

    let response_data: issues::ResponseData = response_body.data.expect("missing response data");

    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    table.set_titles(row!(b => "id", "target", "assignee", "title"));

    for issue in response_data.issues {
        table.add_row(row!(issue.id, issue.target, issue.assigned_to, issue.title));
    }
    table.printstd();

}
