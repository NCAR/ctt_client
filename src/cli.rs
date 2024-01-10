use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};

use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "ctt")]
#[command(about = "cli client for the ctt graphql api server", long_about=None)]
#[command(
    infer_long_args = true,
    infer_subcommands = true,
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
    #[arg(short, long)]
    pub server: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    List(ListVariables),
    #[command(arg_required_else_help = true)]
    Show(GetVariables),
    #[command(arg_required_else_help = true)]
    Open(OpenNewIssue),
    #[command(arg_required_else_help = true)]
    Close(CloseVariables),
    #[command(arg_required_else_help = true)]
    Update(ModifyUpdateIssue),
}

#[derive(clap::Args)]
pub struct Credentials {
    pub user: String,
}

#[derive(Serialize)]
pub struct UserLogin {
    pub user: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Serialize)]
pub enum AuthRequest {
    Munge(String),
}

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, clap::Args)]
pub struct ListVariables {
    #[arg(short, long, value_enum, default_value_t=IssueStatus::OPEN)]
    pub status: IssueStatus,
    #[arg(short, long)]
    pub target: Option<String>,
}

#[derive(Serialize, clap::Args)]
pub struct GetVariables {
    pub id: i32,
}

#[derive(Serialize, clap::Args)]
pub struct CloseVariables {
    pub id: i32,
    pub comment: String,
}

#[derive(Serialize, clap::Args)]
pub struct OpenNewIssue {
    pub target: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "toOffline")]
    #[arg(short, long, value_enum, default_value_t=ToOffline::Node)]
    pub to_offline: ToOffline,
    #[serde(rename = "assignedTo")]
    #[arg(short, long)]
    pub assigned_to: Option<String>,
}

#[derive(Serialize, clap::Args, Debug)]
pub struct ModifyUpdateIssue {
    pub id: i32,
    #[serde(rename = "assignedTo")]
    #[arg(short, long)]
    pub assigned_to: Option<String>,
    #[arg(short, long)]
    pub description: Option<String>,
    #[serde(rename = "toOffline")]
    #[arg(short, long)]
    pub to_offline: Option<ToOffline>,
    #[arg(long)]
    pub title: Option<String>,
}

#[derive(Clone, clap::ValueEnum)]
pub enum IssueStatus {
    OPEN,
    CLOSED,
}

impl ToString for IssueStatus {
    fn to_string(&self) -> String {
        match self {
            IssueStatus::OPEN => "Open".to_string(),
            IssueStatus::CLOSED => "Closed".to_string(),
        }
    }
}
impl ::serde::Serialize for IssueStatus {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            IssueStatus::OPEN => "OPEN",
            IssueStatus::CLOSED => "CLOSED",
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for IssueStatus {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "OPEN" => Ok(IssueStatus::OPEN),
            "CLOSED" => Ok(IssueStatus::CLOSED),
            _ => panic!("can't parse {}", s),
        }
    }
}

#[derive(Clone, clap::ValueEnum, Debug)]
pub enum ToOffline {
    Node,
    Card,
    Blade,
}
impl ToString for ToOffline {
    fn to_string(&self) -> String {
        match self {
            ToOffline::Node => "Node".to_string(),
            ToOffline::Card => "Card".to_string(),
            ToOffline::Blade => "Blade".to_string(),
        }
    }
}
impl ::serde::Serialize for ToOffline {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            ToOffline::Node => "NODE",
            ToOffline::Card => "CARD",
            ToOffline::Blade => "BLADE",
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for ToOffline {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "NODE" => Ok(ToOffline::Node),
            "CARD" => Ok(ToOffline::Card),
            "BLADE" => Ok(ToOffline::Blade),
            _ => panic!("Can't deserialize {}", s),
        }
    }
}
