pub mod close_issue;
pub mod get_issue;
pub mod list_issues;
pub mod open_issue;
pub mod modify_issue;

#[derive(Clone, clap::ValueEnum)]
pub enum ToOffline {
    TARGET,
    SIBLINGS,
    COUSINS,
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
