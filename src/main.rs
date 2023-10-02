use chrono::{NaiveDateTime, Utc};
use clap::{Parser, Subcommand};

use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::{row, Table};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use ctt_client::{get_issue, create_issue, list_issues, close_issue, update_issue};

#[derive(Parser)]
#[command(name = "ctt")]
#[command(about = "cli client for the ctt graphql api server", long_about=None)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
    #[arg(short, long)]
    server: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    List(list_issues::Variables),
    Show(get_issue::Variables),
    Open(create_issue::NewIssue),
    Close(close_issue::Variables),
    Update(update_issue::UpdateIssue),
}

#[derive(clap::Args)]
struct Credentials {
    user: String,
}

fn print_issues(issues: &Vec<list_issues::ListIssuesIssues>) {
    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);
    table.set_titles(row!(b => "id", "target", "assignee", "title"));

    for issue in issues {
        table.add_row(row!(issue.id, issue.target, issue.assigned_to, issue.title));
    }
    table.printstd();
}

fn print_issue(issue: &get_issue::GetIssueIssue) {
    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);
    table.set_titles(row!(b => "id", "target", "assignee", "title", "description", "siblings", "enforce"));

    table.add_row(row!(issue.id, issue.target, issue.assigned_to, issue.title, issue.description, issue.down_siblings, issue.enforce_down));
    table.printstd();

    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);
    table.set_titles(row!(b => "author", "date", "comment"));
    for c in &issue.comments{
        table.add_row(row!(c.author, c.date, c.comment));
    }

    table.printstd();
}
fn print_updateissue(issue: &update_issue::UpdateIssueUpdate) {
    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);
    table.set_titles(row!(b => "id", "target", "assignee", "title", "description", "siblings", "enforce"));

    table.add_row(row!(issue.id, issue.target, issue.assigned_to, issue.title, issue.description, issue.down_siblings, issue.enforce_down));
    table.printstd();

    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);
    table.set_titles(row!(b => "author", "date", "comment"));
    for c in &issue.comments{
        table.add_row(row!(c.author, c.date, c.comment));
    }

    table.printstd();
}

#[derive(Serialize)]
struct UserLogin {
    user: String,
    timestamp: NaiveDateTime,
}

#[derive(Serialize)]
enum AuthRequest {
    Munge(String),
}

#[derive(Deserialize)]
struct Token {
    token: String,
}

fn main() {
    let mut buf = Vec::new();
    File::open("cert.pem")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    let cert = reqwest::Certificate::from_pem(&buf).unwrap();
    use reqwest::header;
    let client = Client::builder()
        .add_root_certificate(cert.clone())
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let args = Cli::parse();
    let srv = if let Some(s) = args.server {
        s
    } else {
        "https://localhost:8000/api".to_string()
    };

    let login = UserLogin {
        user: "shanks".to_string(),
        timestamp: Utc::now().naive_utc(),
    };
    let auth =
        AuthRequest::Munge(munge_auth::munge(&serde_json::to_string(&login).unwrap()).unwrap());

    let log_resp = client
        .post("https://localhost:8000/login")
        .json(&auth)
        .send()
        .unwrap();
    let token: Token = log_resp.json().unwrap();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", &token.token)).unwrap(),
    );

    let client = Client::builder()
        .add_root_certificate(cert)
        .timeout(Duration::from_secs(5))
        .default_headers(headers)
        .build()
        .unwrap();

    match args.cmd {
        Command::Open(new_issue) => match ctt_client::issue_open(&client, &srv, new_issue) {
            Ok(id) => println!("Opened issue {}", &id),
            Err(error) => println!("Error opening issue: {}", error),
        },
        Command::List(filter) => match ctt_client::issue_list(&client, &srv, filter) {
            Ok(issues) => print_issues(&issues),
            Err(error) => println!("Error listing issues: {}", error),
        },
        Command::Close(vars) => match ctt_client::issue_close(&client, &srv, vars) {
            Ok(status) => println!("{}", status),
            Err(error) => println!("Error opening issue: {}", error),
        },
        Command::Show(vars) => match ctt_client::issue_show(&client, &srv, vars) {
            Ok(status) => print_issue(&status.expect("Issue not found")),
            Err(error) => println!("Error showing issue: {}", error),
        },
        Command::Update(vars) => match ctt_client::issue_update(&client, &srv, vars) {
            Ok(status) => print_updateissue(&status),
            Err(error) => println!("Error updating issue: {}", error),
        },
    };
}
