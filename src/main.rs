use chrono::Utc;
use clap::Parser;

use comfy_table::{Cell, Color, ContentArrangement, Row, Table};
use ctt::cli::*;
use ctt::queries::*;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

fn print_issues(issues: Vec<list_issues::ListIssuesIssues>) {
    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::NOTHING)
        //.load_preset(UTF8_FULL)
        //.apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "id",
            "target",
            "ctt status",
            "Enforce",
            "assignee",
            "title",
        ]);

    issues.into_iter().for_each(|issue| {
        let target = Cell::new(issue.target.as_ref().unwrap().name.clone());
        let min_status = issue
            .related
            .iter()
            .map(|t| t.status.clone())
            .min()
            .unwrap();
        let status = Cell::new(min_status.to_string());
        let status = match min_status {
            TargetStatus::OFFLINE => status.fg(Color::Green),
            TargetStatus::DRAINING => status.fg(Color::Yellow),
            TargetStatus::ONLINE => status.fg(Color::Red),
            TargetStatus::DOWN => status,
        };
        let mut row = Row::new();
        row.add_cell(Cell::new(issue.id.to_string()));
        row.add_cell(target);
        row.add_cell(status);
        row.add_cell(if let Some(group) = issue.to_offline {
            Cell::new(group.to_string())
        } else {
            Cell::new("NONE".to_string()).fg(Color::Red)
        });
        row.add_cell(Cell::new(
            issue
                .assigned_to
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
        ));
        row.add_cell(Cell::new(issue.title));
        row.max_height(3);
        table.add_row(row);
    });
    println!("{table}");
}

fn print_issue(issue: get_issue::GetIssueIssue) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
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
    let min_status = issue
        .related
        .iter()
        .map(|t| t.status.clone())
        .min()
        .unwrap();
    let status = Cell::new(min_status.to_string());
    let status = match min_status {
        TargetStatus::OFFLINE => status.fg(Color::Green),
        TargetStatus::DRAINING => status.fg(Color::Yellow),
        TargetStatus::ONLINE => status.fg(Color::Red),
        TargetStatus::DOWN => status,
    };
    table
        .load_preset(comfy_table::presets::NOTHING)
        //.load_preset(UTF8_FULL)
        //.apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            "status",
            "target",
            "ctt status",
            "Enforce",
            "assignee",
            "title",
            "description",
        ]);
    table.add_row(vec![
        Cell::new(issue.issue_status.to_string()),
        target,
        status,
        offline,
        Cell::new(
            issue
                .assigned_to
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
        ),
        Cell::new(issue.title),
        Cell::new(issue.description),
    ]);

    println!("{table}");

    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::NOTHING)
        //.load_preset(UTF8_FULL)
        //.apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
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

fn main() {
    let mut buf = Vec::new();
    //TODO not needed after setting up server TLS properly
    File::open("/glade/work/shanks/ctt/ctt_client/cert.pem")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    let cert = reqwest::Certificate::from_pem(&buf).unwrap();
    use reqwest::header;
    let client = Client::builder()
        .add_root_certificate(cert.clone())
        //TODO FIXME get rid of this
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let args = Cli::parse();
    let srv = if let Some(s) = args.server {
        s
    } else {
        //TODO change to url after setting up dns for server
        "https://10.13.0.16:8000/api".to_string()
    };

    let login = UserLogin {
        user: users::get_current_username()
            .unwrap()
            .into_string()
            .unwrap(),
        timestamp: Utc::now().naive_utc(),
    };
    let auth =
        AuthRequest::Munge(munge_auth::munge(&serde_json::to_string(&login).unwrap()).unwrap());

    let log_resp = client
        //TODO change to url after setting up dns for server
        .post("https://10.13.0.16:8000/login")
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
        //TODO FIXME get rid of this
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .default_headers(headers)
        .build()
        .unwrap();

    match args.cmd {
        Command::Open(new_issue) => match ctt::issue_open(&client, &srv, new_issue) {
            Ok(id) => println!("Opened issue {}", &id),
            Err(error) => println!("Error opening issue: {}", error),
        },
        Command::List(filter) => match ctt::issue_list(&client, &srv, filter) {
            Ok(issues) => print_issues(issues),
            Err(error) => println!("Error listing issues: {}", error),
        },
        Command::Close(vars) => match ctt::issue_close(&client, &srv, vars) {
            Ok(status) => println!("{}", status),
            Err(error) => println!("Error opening issue: {}", error),
        },
        Command::Show(vars) => match ctt::issue_show(&client, &srv, vars) {
            Ok(Some(status)) => print_issue(status),
            Ok(None) => println!("Issue not found"),
            Err(error) => println!("Error showing issue: {}", error),
        },
        Command::Update(vars) => match ctt::issue_update(&client, &srv, vars) {
            Ok(status) => print_issue(status),
            Err(error) => println!("Error updating issue: {}", error),
        },
    };
}
