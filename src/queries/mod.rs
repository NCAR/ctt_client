pub mod close_issue;
pub mod get_issue;
pub mod list_issues;
pub mod modify_issue;
pub mod open_issue;

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

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum TargetStatus {
    ONLINE = 1,
    DRAINING = 2,
    DOWN = 3,
    OFFLINE = 4,
}
impl ToString for TargetStatus {
    // Required method
    fn to_string(&self) -> String {
        match &self {
            TargetStatus::DOWN => "Down".to_string(),
            TargetStatus::ONLINE => "Online".to_string(),
            TargetStatus::OFFLINE => "Offline".to_string(),
            TargetStatus::DRAINING => "Draining".to_string(),
        }
    }
}
impl ::serde::Serialize for TargetStatus {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            TargetStatus::ONLINE => "ONLINE",
            TargetStatus::DRAINING => "DRAINING",
            TargetStatus::OFFLINE => "OFFLINE",
            TargetStatus::DOWN => "DOWN",
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
