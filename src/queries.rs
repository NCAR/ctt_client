#![allow(clippy::all, warnings)]
pub struct ListIssues;
pub mod list_issues {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ListIssues";
    pub const QUERY : & str = "query ListIssues($status: String, $target: String) {\n  issues(issueStatus: $status, target: $target) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n  }\n}\n\nquery GetIssue($id: Int) {\n  issue(issue: $id) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n\nmutation CreateIssue($newIssue: NewIssue!) {\n  open(issue: $newIssue) {\n    id\n  }\n}\n\nmutation CloseIssue($id: Int, $comment: String) {\n  close(issue: $id, comment: $comment)\n}\n\nmutation UpdateIssue($issue: UpdateIssue!) {\n  update(issue: $issue) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Clone, clap::ValueEnum)]
    pub enum IssueStatus {
        OPEN,
        CLOSED,
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
                _ => panic!("Not a valid status!"),
            }
        }
    }
    #[derive(Serialize, clap::Args)]
    pub struct Variables {
        #[arg(short, long, value_enum, default_value_t=IssueStatus::OPEN)]
        pub status: IssueStatus,
        #[arg(short, long)]
        pub target: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub issues: Vec<ListIssuesIssues>,
    }
    #[derive(Deserialize)]
    pub struct ListIssuesIssues {
        #[serde(rename = "assignedTo")]
        pub assigned_to: String,
        pub description: String,
        #[serde(rename = "downSiblings")]
        pub down_siblings: Boolean,
        #[serde(rename = "enforceDown")]
        pub enforce_down: Boolean,
        pub id: Int,
        #[serde(rename = "issueStatus")]
        pub issue_status: IssueStatus,
        pub target: String,
        pub title: String,
    }
}
impl graphql_client::GraphQLQuery for ListIssues {
    type Variables = list_issues::Variables;
    type ResponseData = list_issues::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: list_issues::QUERY,
            operation_name: list_issues::OPERATION_NAME,
        }
    }
}
pub struct GetIssue;
pub mod get_issue {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetIssue";
    pub const QUERY : & str = "query ListIssues($status: String, $target: String) {\n  issues(issueStatus: $status, target: $target) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n  }\n}\n\nquery GetIssue($id: Int) {\n  issue(issue: $id) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n\nmutation CreateIssue($newIssue: NewIssue!) {\n  open(issue: $newIssue) {\n    id\n  }\n}\n\nmutation CloseIssue($id: Int, $comment: String) {\n  close(issue: $id, comment: $comment)\n}\n\nmutation UpdateIssue($issue: UpdateIssue!) {\n  update(issue: $issue) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    use chrono::NaiveDateTime;
    #[derive()]
    pub enum IssueStatus {
        OPEN,
        CLOSED,
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
                _ => panic!("Not a valid status!"),
            }
        }
    }
    #[derive(Serialize, clap::Args)]
    pub struct Variables {
        pub id: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub issue: Option<GetIssueIssue>,
    }
    #[derive(Deserialize)]
    pub struct GetIssueIssue {
        #[serde(rename = "assignedTo")]
        pub assigned_to: String,
        pub description: String,
        #[serde(rename = "downSiblings")]
        pub down_siblings: Boolean,
        #[serde(rename = "enforceDown")]
        pub enforce_down: Boolean,
        pub id: Int,
        #[serde(rename = "issueStatus")]
        pub issue_status: IssueStatus,
        pub target: String,
        pub title: String,
        pub comments: Vec<GetIssueIssueComments>,
    }
    #[derive(Deserialize)]
    pub struct GetIssueIssueComments {
        pub author: String,
        pub date: NaiveDateTime,
        pub comment: String,
    }
}
impl graphql_client::GraphQLQuery for GetIssue {
    type Variables = get_issue::Variables;
    type ResponseData = get_issue::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_issue::QUERY,
            operation_name: get_issue::OPERATION_NAME,
        }
    }
}
pub struct CreateIssue;
pub mod create_issue {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateIssue";
    pub const QUERY : & str = "query ListIssues($status: String, $target: String) {\n  issues(issueStatus: $status, target: $target) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n  }\n}\n\nquery GetIssue($id: Int) {\n  issue(issue: $id) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n\nmutation CreateIssue($newIssue: NewIssue!) {\n  open(issue: $newIssue) {\n    id\n  }\n}\n\nmutation CloseIssue($id: Int, $comment: String) {\n  close(issue: $id, comment: $comment)\n}\n\nmutation UpdateIssue($issue: UpdateIssue!) {\n  update(issue: $issue) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize, clap::Args)]
    pub struct NewIssue {
        #[serde(rename = "assignedTo")]
        #[arg(short, long)]
        pub assigned_to: Option<String>,
        pub description: String,
        #[serde(rename = "downSiblings")]
        #[arg(short, long, default_value_t = false)]
        pub down_siblings: Boolean,
        #[serde(rename = "enforceDown")]
        #[arg(short, long, default_value_t = false)]
        pub enforce_down: Boolean,
        pub target: String,
        pub title: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "newIssue")]
        pub new_issue: NewIssue,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub open: CreateIssueOpen,
    }
    #[derive(Deserialize)]
    pub struct CreateIssueOpen {
        pub id: Int,
    }
}
impl graphql_client::GraphQLQuery for CreateIssue {
    type Variables = create_issue::Variables;
    type ResponseData = create_issue::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_issue::QUERY,
            operation_name: create_issue::OPERATION_NAME,
        }
    }
}
pub struct CloseIssue;
pub mod close_issue {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CloseIssue";
    pub const QUERY : & str = "query ListIssues($status: String, $target: String) {\n  issues(issueStatus: $status, target: $target) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n  }\n}\n\nquery GetIssue($id: Int) {\n  issue(issue: $id) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n\nmutation CreateIssue($newIssue: NewIssue!) {\n  open(issue: $newIssue) {\n    id\n  }\n}\n\nmutation CloseIssue($id: Int, $comment: String) {\n  close(issue: $id, comment: $comment)\n}\n\nmutation UpdateIssue($issue: UpdateIssue!) {\n  update(issue: $issue) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize, clap::Args)]
    pub struct Variables {
        pub id: Int,
        pub comment: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub close: String,
    }
}
impl graphql_client::GraphQLQuery for CloseIssue {
    type Variables = close_issue::Variables;
    type ResponseData = close_issue::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: close_issue::QUERY,
            operation_name: close_issue::OPERATION_NAME,
        }
    }
}
pub struct UpdateIssue;
pub mod update_issue {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateIssue";
    pub const QUERY : & str = "query ListIssues($status: String, $target: String) {\n  issues(issueStatus: $status, target: $target) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n  }\n}\n\nquery GetIssue($id: Int) {\n  issue(issue: $id) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n\nmutation CreateIssue($newIssue: NewIssue!) {\n  open(issue: $newIssue) {\n    id\n  }\n}\n\nmutation CloseIssue($id: Int, $comment: String) {\n  close(issue: $id, comment: $comment)\n}\n\nmutation UpdateIssue($issue: UpdateIssue!) {\n  update(issue: $issue) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    use chrono::NaiveDateTime;
    #[derive()]
    pub enum IssueStatus {
        OPEN,
        CLOSED,
        Other(String),
    }
    impl ::serde::Serialize for IssueStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                IssueStatus::OPEN => "OPEN",
                IssueStatus::CLOSED => "CLOSED",
                IssueStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IssueStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "OPEN" => Ok(IssueStatus::OPEN),
                "CLOSED" => Ok(IssueStatus::CLOSED),
                _ => Ok(IssueStatus::Other(s)),
            }
        }
    }
    #[derive(Serialize, clap::Args)]
    pub struct UpdateIssue {
        pub id: Int,
        #[serde(rename = "assignedTo")]
        #[arg(short, long)]
        pub assigned_to: Option<String>,
        #[arg(short, long)]
        pub description: Option<String>,
        #[serde(rename = "enforceDown")]
        #[arg(short, long)]
        pub enforce_down: Option<Boolean>,
        #[arg(short, long)]
        pub title: Option<String>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub issue: UpdateIssue,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub update: UpdateIssueUpdate,
    }
    #[derive(Deserialize)]
    pub struct UpdateIssueUpdate {
        #[serde(rename = "assignedTo")]
        pub assigned_to: String,
        pub description: String,
        #[serde(rename = "downSiblings")]
        pub down_siblings: Boolean,
        #[serde(rename = "enforceDown")]
        pub enforce_down: Boolean,
        pub id: Int,
        #[serde(rename = "issueStatus")]
        pub issue_status: IssueStatus,
        pub target: String,
        pub title: String,
        pub comments: Vec<UpdateIssueUpdateComments>,
    }
    #[derive(Deserialize)]
    pub struct UpdateIssueUpdateComments {
        pub author: String,
        pub date: NaiveDateTime,
        pub comment: String,
    }
}
impl graphql_client::GraphQLQuery for UpdateIssue {
    type Variables = update_issue::Variables;
    type ResponseData = update_issue::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_issue::QUERY,
            operation_name: update_issue::OPERATION_NAME,
        }
    }
}
