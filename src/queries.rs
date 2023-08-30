#[allow(clippy::all, warnings)]
pub struct ListIssues;
pub mod list_issues {
    #![allow(dead_code)]
    use chrono::NaiveDateTime;
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ListIssues";
    pub const QUERY : & str = "query ListIssues($status: String, $target: String) {\n  issues(issueStatus: $status, target: $target) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\nmutation CreateIssue($newIssue: NewIssue!) {\n  open(issue: $newIssue) {\n    id\n  }\n}\n" ;
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
        pub comments: Vec<ListIssuesIssuesComments>,
    }
    #[derive(Deserialize)]
    pub struct ListIssuesIssuesComments {
        pub author: String,
        pub date: NaiveDateTime,
        pub comment: String,
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
pub struct CreateIssue;
pub mod create_issue {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "CreateIssue";
    pub const QUERY : & str = "query ListIssues($status: String, $target: String) {\n  issues(issueStatus: $status, target: $target) {\n    assignedTo\n    description\n    downSiblings\n    enforceDown\n    id\n    issueStatus\n    target\n    title\n    comments {\n      author\n      date\n      comment\n    }\n  }\n}\nmutation CreateIssue($newIssue: NewIssue!) {\n  open(issue: $newIssue) {\n    id\n  }\n}\n" ;
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
        #[serde(rename = "downSiblings")]
        #[arg(short, long, default_value_t = false)]
        pub down_siblings: Boolean,
        #[serde(rename = "enforceDown")]
        #[arg(short, long, default_value_t = false)]
        pub enforce_down: Boolean,
        pub target: String,
        pub title: String,
        pub description: String,
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
