pub mod close_issue;
pub mod get_issue;
pub mod list_issues;
pub mod modify_issue;
pub mod open_issue;

#[derive(Clone, clap::ValueEnum, Debug)]
pub enum ToOffline {
    TARGET,
    SIBLINGS,
    COUSINS,
}
impl ToString for ToOffline {
    fn to_string(&self) -> String {
        match self {
            ToOffline::TARGET => "Target".to_string(),
            ToOffline::SIBLINGS => "Siblings".to_string(),
            ToOffline::COUSINS => "Cousins".to_string(),
        }
    }
}
impl ::serde::Serialize for ToOffline {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            ToOffline::TARGET => "TARGET",
            ToOffline::SIBLINGS => "SIBLINGS",
            ToOffline::COUSINS => "COUSINS",
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for ToOffline {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "TARGET" => Ok(ToOffline::TARGET),
            "SIBLINGS" => Ok(ToOffline::SIBLINGS),
            "COUSINS" => Ok(ToOffline::COUSINS),
            _ => panic!("Can't deserialize {}", s),
        }
    }
}

#[derive(PartialEq)]
pub enum TargetStatus {
    ONLINE,
    DRAINING,
    OFFLINE,
    DOWN,
    UNKNOWN,
}
impl ::serde::Serialize for TargetStatus {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            TargetStatus::ONLINE => "ONLINE",
            TargetStatus::DRAINING => "DRAINING",
            TargetStatus::OFFLINE => "OFFLINE",
            TargetStatus::DOWN => "DOWN",
            TargetStatus::UNKNOWN => "UNKNOWN",
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for TargetStatus {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "ONLINE" => Ok(TargetStatus::ONLINE),
            "DRAINING" => Ok(TargetStatus::DRAINING),
            "OFFLINE" => Ok(TargetStatus::OFFLINE),
            "DOWN" => Ok(TargetStatus::DOWN),
            "UNKNOWN" => Ok(TargetStatus::UNKNOWN),
            _ => panic!("Can't deserialize {}", s),
        }
    }
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
