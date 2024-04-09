pub mod close_issue;
pub mod get_issue;
pub mod list_issues;
pub mod modify_issue;
pub mod open_issue;
use core::fmt;

pub use crate::cli::ToOffline;

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum TargetStatus {
    ONLINE = 1,
    DRAINING = 2,
    DOWN = 3,
    OFFLINE = 4,
}
impl fmt::Display for TargetStatus {
    // Required method
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TargetStatus::DOWN => write!(f, "Down"),
            TargetStatus::ONLINE => write!(f, "Online"),
            TargetStatus::OFFLINE => write!(f, "Offline"),
            TargetStatus::DRAINING => write!(f, "Draining"),
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
