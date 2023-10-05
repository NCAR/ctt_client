use chrono::{NaiveDateTime, Utc};
use clap::{Parser, Subcommand};

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, Table};
use ctt_client::queries::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::time::Duration;

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
    Open(open_issue::NewIssue),
    Close(close_issue::Variables),
    Update(modify_issue::UpdateIssue),
}

#[derive(clap::Args)]
struct Credentials {
    user: String,
}

fn print_issues(issues: Vec<list_issues::ListIssuesIssues>) {
    let mut table = Table::new();
    //table.set_format(*FORMAT_CLEAN);
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["id", "target", "assignee", "title"]);

    issues.into_iter().for_each(|issue| {
        let mut target = Cell::new(issue.target.as_ref().unwrap().name.clone());
        target = match issue.target.unwrap().status {
            TargetStatus::OFFLINE => target.fg(Color::Green),
            TargetStatus::DRAINING => target.fg(Color::Yellow),
            TargetStatus::ONLINE => target.fg(Color::Red),
            _ => target,
        };
        table.add_row(vec![
            Cell::new(issue.id.to_string()),
            target,
            Cell::new(
                issue
                    .assigned_to
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_string(),
            ),
            Cell::new(issue.title),
        ]);
    });
    println!("{table}");
}

fn print_issue(issue: get_issue::GetIssueIssue) {
    let mut table = Table::new();
    let mut target = Cell::new(issue.target.as_ref().unwrap().name.clone());
    target = match issue.target.unwrap().status {
        TargetStatus::OFFLINE => target.fg(Color::Green),
        TargetStatus::DRAINING => target.fg(Color::Yellow),
        TargetStatus::ONLINE => target.fg(Color::Red),
        _ => target,
    };
    let offline = if let Some(o) = issue.to_offline {
        Cell::new(o.to_string())
    } else {
        Cell::new("NONE".to_string()).fg(Color::Red)
    };
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            "status",
            "target",
            "assignee",
            "title",
            "description",
            "to_offline",
            "enforce",
        ]);
    table.add_row(vec![
        Cell::new(issue.issue_status.to_string()),
        target,
        Cell::new(
            issue
                .assigned_to
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
        ),
        Cell::new(issue.title),
        Cell::new(issue.description),
        offline,
        Cell::new(issue.enforce_down),
    ]);

    println!("{table}");

    let mut table = Table::new();
    //table.set_format(*FORMAT_CLEAN);
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["author", "date", "comment"]);
    issue.comments.into_iter().for_each(|c| {
        table.add_row(vec![
            c.created_by.clone(),
            c.created_at.to_string(),
            c.comment.clone(),
        ]);
    });

    println!("{table}");
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
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let args = Cli::parse();
    let srv = if let Some(s) = args.server {
        s
    } else {
        "https://127.0.0.1:8000/api".to_string()
    };

    let login = UserLogin {
        user: "shanks".to_string(),
        timestamp: Utc::now().naive_utc(),
    };
    let auth =
        AuthRequest::Munge(munge_auth::munge(&serde_json::to_string(&login).unwrap()).unwrap());

    let log_resp = client
        .post("https://127.0.0.1:8000/login")
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
        .danger_accept_invalid_certs(true)
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
            Ok(issues) => print_issues(issues),
            Err(error) => println!("Error listing issues: {}", error),
        },
        Command::Close(vars) => match ctt_client::issue_close(&client, &srv, vars) {
            Ok(status) => println!("{}", status),
            Err(error) => println!("Error opening issue: {}", error),
        },
        Command::Show(vars) => match ctt_client::issue_show(&client, &srv, vars) {
            Ok(status) => print_issue(status.expect("Issue not found")),
            Err(error) => println!("Error showing issue: {}", error),
        },
        Command::Update(vars) => match ctt_client::issue_update(&client, &srv, vars) {
            Ok(_) => (),
            Err(error) => println!("Error updating issue: {}", error),
        },
    };
}
