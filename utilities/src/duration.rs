//! Redefines `std::time::Duration` to allow for additional functionality.

use chrono::{DateTime, Local};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::ops::{Add, AddAssign, Sub};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Wrapper for `std::time::Duration` so we can derive Serialize and Deserialize traits
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Clone, Copy, Default, Debug)]
pub struct Duration(pub std::time::Duration);

impl Duration {
    /// Get duration from seconds.
    #[must_use]
    pub fn from_secs_f64(secs: f64) -> Self {
        Self(std::time::Duration::from_secs_f64(secs))
    }

    #[allow(dead_code)]
    /// Get duration from milliseconds (u64).
    #[must_use]
    pub const fn from_millis_u64(millis: u64) -> Self {
        Self(std::time::Duration::from_millis(millis))
    }

    #[allow(dead_code)]
    /// Get duration from milliseconds (u32).
    #[must_use]
    pub fn from_millis_u32(millis: u32) -> Self {
        Self(std::time::Duration::from_millis(u64::from(millis)))
    }

    /// Calculate the duration between two `TimeStamps`, regardless of which comes first.
    #[must_use]
    pub fn between(ts1: &DateTime<Local>, ts2: &DateTime<Local>) -> Self {
        Self(if ts2 > ts1 {
            // ts2 is after ts1
            chrono::Duration::to_std(&ts2.signed_duration_since(ts1.with_timezone(&Local)))
                .expect("types::Duration::between() -- ts2 > ts1: Duration out of bounds.")
        } else {
            chrono::Duration::to_std(&ts1.signed_duration_since(ts2.with_timezone(&Local)))
                .expect("types::Duration::between() -- ts1 >= ts2: Duration out of bounds.")
        })
    }

    /// Get the number of seconds in the duration.
    #[must_use]
    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }
}

impl Add for Duration {
    type Output = Self;
    /// Implements the `+` operation for Duration.
    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0
                .checked_add(rhs.0)
                .expect("overflow when adding durations."),
        )
    }
}

impl AddAssign for Duration {
    /// Implements the `+=` operation for Duration.
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
    }
}

impl Sub for Duration {
    type Output = Self;
    /// Implements the `-` operation for Duration.
    fn sub(self, rhs: Self) -> Self {
        Self(
            self.0
                .checked_sub(rhs.0)
                .expect("overflow when subtracting durations"),
        )
    }
}

impl std::fmt::Display for Duration {
    /// Implements a way to format (and hence display) Duration.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.as_secs();
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);
        write!(f, "{h:02}:{m:02}:{s:02}")
    }
}

impl Serialize for Duration {
    /// Serializes the Duration for output into various types of files.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Duration", 1)?;
        state.serialize_field("secs", &self.0.as_secs_f32())?;
        state.end()
    }
}

#[cfg(test)]
///
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    fn test_from_secs_f64() {
        let dur = Duration::from_secs_f64(120.100);

        assert_eq!(dur.0.as_secs(), 120);
        assert_eq!(dur.0.as_millis(), 120_100);
    }

    #[assay]
    fn test_from_millis_u64() {
        let dur = Duration::from_millis_u64(123_123);

        assert_eq!(dur.0.as_secs(), 123);
        assert_eq!(dur.0.as_secs_f32(), 123.123_f32);
        assert_eq!(dur.0.as_nanos(), 123_123_000_000);
    }

    #[assay]
    fn test_from_millis_u32() {
        let dur = Duration::from_millis_u32(123_123);

        assert_eq!(dur.0.as_secs(), 123);
        assert_eq!(dur.0.as_secs_f32(), 123.123_f32);
        assert_eq!(dur.0.as_nanos(), 123_123_000_000);
    }

    #[assay]
    fn test_between() {
        let t1 = Local::now();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let t2 = Local::now();
        let b1 = Duration::between(&t1, &t2);
        let b2 = Duration::between(&t2, &t1);
        println!("b1 = {b1}, b2 = {b2}");

        assert_eq!(b1.to_string(), "00:00:01".to_string());
        assert_eq!(b2.to_string(), "00:00:01".to_string());
        assert_eq!(b1.0.as_secs(), 1);
        assert_eq!(b2.0.as_secs(), 1);
    }

    #[assay]
    /// Tests the addition functionality to add two durations together
    fn test_add() {
        assert_eq!(
            (Duration::from_secs_f64(1.0) + Duration::from_secs_f64(1.0))
                .0
                .as_secs(),
            2
        );
        assert_eq!(
            (Duration::from_secs_f64(1.0) + Duration::from_secs_f64(1.0))
                .0
                .as_millis(),
            2000
        );

        assert_eq!(
            (Duration::from_secs_f64(1.0) + Duration::from_secs_f64(1.5))
                .0
                .as_secs(),
            2
        );
        assert_eq!(
            (Duration::from_secs_f64(1.0) + Duration::from_secs_f64(1.5))
                .0
                .as_millis(),
            2500
        );

        assert_eq!(
            (Duration::from_secs_f64(1.0) + Duration::from_secs_f64(1.6))
                .0
                .as_secs(),
            2
        );
        assert_eq!(
            (Duration::from_secs_f64(1.0) + Duration::from_secs_f64(1.6))
                .0
                .as_millis(),
            2600
        );

        assert_eq!(
            (Duration::from_secs_f64(1.6) + Duration::from_secs_f64(1.6))
                .0
                .as_secs(),
            3
        );
        assert_eq!(
            (Duration::from_secs_f64(1.6) + Duration::from_secs_f64(1.6))
                .0
                .as_millis(),
            3200
        );
    }

    #[assay]
    /// Tests the add assign (+=) functionality
    fn test_add_assign() {
        let mut d1 = Duration::from_secs_f64(1.0);
        assert_eq!(d1.0.as_secs(), 1);

        d1 += Duration::from_millis_u32(1500);
        assert_eq!(d1.0.as_millis(), 2500);
        assert_eq!(d1.0.as_secs(), 2);

        d1 += Duration::from_millis_u32(600);
        assert_eq!(d1.0.as_millis(), 3100);
        assert_eq!(d1.0.as_secs(), 3);
    }

    #[assay]
    /// Tests the subtraction functionality
    fn test_sub() {
        assert_eq!(
            (Duration::from_millis_u32(2500) - Duration::from_millis_u32(1000))
                .0
                .as_millis(),
            1500
        );
        assert_eq!(
            (Duration::from_millis_u32(2500) - Duration::from_millis_u32(1000))
                .0
                .as_secs(),
            1
        );
        assert_eq!(
            (Duration::from_millis_u32(2500) - Duration::from_millis_u32(1000))
                .0
                .as_secs_f32(),
            1.5
        );

        assert_eq!(
            (Duration::from_millis_u32(2500) - Duration::from_millis_u32(1500))
                .0
                .as_millis(),
            1000
        );
        assert_eq!(
            (Duration::from_millis_u32(2500) - Duration::from_millis_u32(1500))
                .0
                .as_secs(),
            1
        );
        assert_eq!(
            (Duration::from_millis_u32(2500) - Duration::from_millis_u32(1500))
                .0
                .as_secs_f32(),
            1.0
        );

        assert_eq!(
            (Duration::from_millis_u32(2500) - Duration::from_millis_u32(1600))
                .0
                .as_millis(),
            900
        );
        assert_eq!(
            (Duration::from_millis_u32(2500) - Duration::from_millis_u32(1600))
                .0
                .as_secs(),
            0
        );
        assert_eq!(
            (Duration::from_millis_u32(2500) - Duration::from_millis_u32(1600))
                .0
                .as_secs_f32(),
            0.9
        );
    }

    #[assay]
    ///
    fn test_display() {
        let d1 = Duration::from_secs_f64(3750.2);
        println!("d1 = {d1}");

        assert_eq!(format!("{d1}"), "01:02:30");
    }
}
