//! Redefines `Duration` as a newtype over `f64` (seconds) for type safety and native serde support.

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Duration in seconds, stored as `f64`. Provides type safety over bare `f64` while
/// supporting native `Serialize`/`Deserialize` (no custom impl needed).
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy, Default, Debug)]
pub struct Duration(pub f64);

impl Duration {
    /// Get duration from seconds, sanitizing invalid values (NaN, negative, infinite → 0.0).
    #[must_use]
    pub fn from_secs_f64(secs: f64) -> Self {
        if secs.is_nan() || secs.is_sign_negative() || secs.is_infinite() {
            return Self::default();
        }
        Self(secs)
    }

    /// Get duration from milliseconds (u64).
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn from_millis_u64(millis: u64) -> Self {
        Self(millis as f64 / 1000.0)
    }

    /// Get duration from milliseconds (u32).
    #[must_use]
    pub fn from_millis_u32(millis: u32) -> Self {
        Self(f64::from(millis) / 1000.0)
    }

    /// Calculate the duration between two `DateTime<Local>`, regardless of which comes first.
    ///
    /// # Parameters
    ///
    /// * `ts1: &DateTime<Local>` -- The first timestamp.
    /// * `ts2: &DateTime<Local>` -- The second timestamp.
    ///
    /// # Returns
    ///
    /// * `Self` -- The duration between the two timestamps.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Local;
    /// use utilities::Duration;
    ///
    /// let t1 = Local::now();
    /// let t2 = Local::now();
    /// let b1 = Duration::between(&t1, &t2);
    /// let b2 = Duration::between(&t2, &t1);
    /// assert_eq!(b1, b2);
    /// ```
    #[must_use]
    pub fn between(ts1: &DateTime<Local>, ts2: &DateTime<Local>) -> Self {
        let diff = if ts2 > ts1 {
            ts2.signed_duration_since(ts1)
        } else {
            ts1.signed_duration_since(ts2)
        };
        #[allow(clippy::cast_precision_loss)]
        Self(diff.num_milliseconds() as f64 / 1000.0)
    }

    /// Get the number of whole seconds in the duration.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub const fn as_secs(&self) -> u64 {
        self.0 as u64
    }
}

impl Add for Duration {
    type Output = Self;
    /// Implements the `+` operation for Duration.
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Duration {
    /// Implements the `+=` operation for Duration.
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Duration {
    type Output = Self;
    /// Implements the `-` operation for Duration.
    fn sub(self, rhs: Self) -> Self {
        Self((self.0 - rhs.0).max(0.0))
    }
}

impl std::fmt::Display for Duration {
    /// Implements a way to format (and hence display) Duration as HH:MM:SS.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0 as u64;
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);
        write!(f, "{h:02}:{m:02}:{s:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_secs_f64() {
        let dur = Duration::from_secs_f64(120.100);

        assert_eq!(dur.as_secs(), 120);
        assert!((dur.0 - 120.1).abs() < f64::EPSILON);
    }

    #[test]
    fn test_from_secs_f64_nan() {
        assert_eq!(Duration::from_secs_f64(f64::NAN), Duration::default());
    }

    #[test]
    fn test_from_secs_f64_negative() {
        assert_eq!(Duration::from_secs_f64(-5.0), Duration::default());
    }

    #[test]
    fn test_from_secs_f64_infinite() {
        assert_eq!(Duration::from_secs_f64(f64::INFINITY), Duration::default());
    }

    #[test]
    fn test_from_millis_u64() {
        let dur = Duration::from_millis_u64(123_123);

        assert_eq!(dur.as_secs(), 123);
        assert!((dur.0 - 123.123).abs() < 1e-10);
    }

    #[test]
    fn test_from_millis_u32() {
        let dur = Duration::from_millis_u32(123_123);

        assert_eq!(dur.as_secs(), 123);
        assert!((dur.0 - 123.123).abs() < 1e-10);
    }

    #[test]
    fn test_between() {
        let t1 = Local::now();
        std::thread::sleep(std::time::Duration::from_millis(50));
        let t2 = Local::now();
        let b1 = Duration::between(&t1, &t2);
        let b2 = Duration::between(&t2, &t1);

        assert!(b1.0 >= 0.04);
        assert!((b1.0 - b2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add() {
        assert_eq!(
            (Duration::from_secs_f64(1.0) + Duration::from_secs_f64(1.0)).as_secs(),
            2
        );
        assert!(
            ((Duration::from_secs_f64(1.0) + Duration::from_secs_f64(1.5)).0 - 2.5).abs()
                < f64::EPSILON
        );
        assert!(
            ((Duration::from_secs_f64(1.6) + Duration::from_secs_f64(1.6)).0 - 3.2).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn test_add_assign() {
        let mut d1 = Duration::from_secs_f64(1.0);
        assert_eq!(d1.as_secs(), 1);

        d1 += Duration::from_millis_u32(1500);
        assert!((d1.0 - 2.5).abs() < 1e-10);

        d1 += Duration::from_millis_u32(600);
        assert!((d1.0 - 3.1).abs() < 1e-10);
    }

    #[test]
    fn test_sub() {
        assert!(
            ((Duration::from_millis_u32(2500) - Duration::from_millis_u32(1000)).0 - 1.5).abs()
                < 1e-10
        );
        // Subtraction floors at zero
        assert_eq!(
            (Duration::from_millis_u32(1000) - Duration::from_millis_u32(2000)).0,
            0.0
        );
    }

    #[test]
    fn test_display() {
        let d1 = Duration::from_secs_f64(3750.2);
        assert_eq!(format!("{d1}"), "01:02:30");
    }
}
