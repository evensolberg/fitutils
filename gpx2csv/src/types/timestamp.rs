use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Wrapper for chrono::DateTime so we can derive Serialize and Deserialize traits
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct TimeStamp(pub DateTime<Utc>);

impl Default for TimeStamp {
    /// Initialize TimeStamp to current time.
    fn default() -> Self {
        TimeStamp(chrono::Utc::now())
    }
}

impl std::fmt::Display for TimeStamp {
    /// Format time to `%Y-%m-%d %H:%M:%S`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M:%S"))
    }
}
