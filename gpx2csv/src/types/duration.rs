use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::ops::{Add, AddAssign, Sub};

use crate::types::timestamp::TimeStamp;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Wrapper for std::time::Duration so we can derive Serialize and Deserialize traits
#[derive(Deserialize, PartialEq, PartialOrd, Clone, Copy, Default, Debug)]
pub struct Duration(std::time::Duration);

impl Duration {
    /// Get duration from seconds.
    pub fn from_secs_f64(secs: f64) -> Self {
        Duration(std::time::Duration::from_secs_f64(secs))
    }

    /// Get duration from milliseconds (u64).
    pub fn from_millis_u64(millis: u64) -> Self {
        Duration(std::time::Duration::from_millis(millis))
    }

    /// Get duration from milliseconds (u32).
    pub fn from_millis_u32(millis: u32) -> Self {
        Duration(std::time::Duration::from_millis(millis as u64))
    }

    /// Calculate the duration between two TimeStamps, regardless of which comes first.
    pub fn between(ts1: &TimeStamp, ts2: &TimeStamp) -> Self {
        log::trace!(
            "types::Duration::between() -- ts1: {:?} -- ts2: {:?}",
            ts1,
            ts2
        );
        Duration(if ts2 > ts1 {
            // ts2 is after ts1
            log::debug!("types::Duration::between() -- ts2 > ts1");
            chrono::Duration::to_std(&ts2.0.signed_duration_since(ts1.0))
                .expect("types::Duration::between() -- ts2 > ts1: Duration out of bounds.")
        } else {
            log::debug!("types::Duration::between() -- ts1 >= ts2");
            chrono::Duration::to_std(&ts1.0.signed_duration_since(ts2.0))
                .expect("types::Duration::between() -- ts1 >= ts2: Duration out of bounds.")
        })
    }
}

impl Add for Duration {
    type Output = Self;
    /// Implements the `+` operation for Duration.
    fn add(self, rhs: Duration) -> Self::Output {
        Duration(
            self.0
                .checked_add(rhs.0)
                .expect("overflow when adding durations."),
        )
    }
}

impl AddAssign for Duration {
    /// Implements the `+=` operation for Duration.
    fn add_assign(&mut self, rhs: Duration) {
        self.0 = self.0 + rhs.0;
    }
}

impl Sub for Duration {
    type Output = Duration;
    /// Implements the `-` operation for Duration.
    fn sub(self, rhs: Duration) -> Duration {
        Duration(
            self.0
                .checked_sub(rhs.0)
                .expect("overflow when subtracting durations"),
        )
    }
}

impl std::fmt::Display for Duration {
    /// Impements a way to format (and hence display) Duration.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.as_secs();
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);
        write!(f, "{:02}:{:02}:{:02}", h, m, s)
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Duration", 1)?;
        state.serialize_field("secs", &self.0.as_secs_f32())?;
        state.end()
    }
}
