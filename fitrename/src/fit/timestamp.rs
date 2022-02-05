//! Re-implements `chrono::DateTime<Local>` so it can be serialized.

use chrono::{offset::Local, DateTime};
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Wrapper for chrono::DateTime so we can derive Serialize and Deserialize traits
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct TimeStamp(pub DateTime<Local>);

impl Default for TimeStamp {
    /// Initialize TimeStamp to current time.
    fn default() -> TimeStamp {
        TimeStamp(Local::now())
    }
}

impl std::fmt::Display for TimeStamp {
    /// Format time to `%Y-%m-%d %H:%M:%S`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M:%S"))
    }
}
