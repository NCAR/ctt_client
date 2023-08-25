use reqwest::blocking::Client;
use chrono::{Utc, NaiveDateTime};
use std::time::Duration;
use prettytable::{row, Table};
use prettytable::format::consts::FORMAT_CLEAN;
use clap::{Parser, Subcommand};
use serde::{Deserialize,Serialize};

use ctt_client::{list_issues,create_issue};

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

#[derive(Deserialize)]
struct Token {
    token: String,
}


fn main() {
    use reqwest::header;
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build().unwrap();
    let args = Cli::parse();
    let srv = if let Some(s) = args.server {
        s 
    } else {
        "http://localhost:8000/api".to_string()
    };

    let login = UserLogin{ user: "shanks".to_string(), timestamp: Utc::now().naive_utc()};
    let log_resp = client.post("http://localhost:8000/login").json(&login).send().unwrap();
    //println!("'{}'", log_resp.text().unwrap());
    let token: Token = log_resp.json().unwrap();

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("Bearer {}", &token.token)).unwrap());

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .default_headers(headers)
        .build().unwrap();

    match args.cmd {
        Command::Open(new_issue) => {
            let id = ctt_client::issue_open(&client, &srv, new_issue).unwrap();
            println!("Opened issue {}", &id);
        },
        Command::List(filter) => {
            let issues = ctt_client::issue_list(&client, &srv, filter).unwrap();
            print_issues(&issues);
        },
    };
}
