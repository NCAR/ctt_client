use chrono::{NaiveDateTime, Utc};
use std::fs::File;
use std::io::Read;
use clap::{Parser, Subcommand};
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::{row, Table};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use munge_auth;

use ctt_client::{create_issue, list_issues};

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
    Open(create_issue::NewIssue),
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

#[derive(Serialize)]
struct UserLogin {
    user: String,
    timestamp: NaiveDateTime,
}

#[derive(Serialize)]
enum AuthRequest {
    Munge(String)
}


#[derive(Deserialize)]
struct Token {
    token: String,
}

fn main() {
    let mut buf = Vec::new();
    File::open("cert.pem").unwrap().read_to_end(&mut buf).unwrap();
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
    let auth = AuthRequest::Munge(munge_auth::munge(&serde_json::to_string(&login).unwrap()).unwrap());

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
            Err(()) => println!("Error listing issues"),
        },
    };
}
