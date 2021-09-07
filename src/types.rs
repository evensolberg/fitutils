use chrono::{offset::Local, DateTime};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub};
/**
 * Local structs used to contain the various bits and pieces of information extracted from the header.
 * This includes such things as device manufacturer, activity, etc.
 *
 * This will then be put into each line in the resulting CSV, so that each line essentially is self-contained.
 */
use uom::si::f64::{Length as Length_f64, Velocity};
use uom::si::u16::Length as Length_u16;

/// Wrapper for chrono::DateTime so we can derive Serialize and Deserialize traits
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeStamp(pub DateTime<Local>);

impl Default for TimeStamp {
    /// Initialize TimeStamp to current time.
    fn default() -> TimeStamp {
        TimeStamp(Local::now())
    }
}

impl std::fmt::Display for TimeStamp {
    /// Format time to `%d.%m.%Y %H:%M`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%d.%m.%Y %H:%M"))
    }
}

/// Wrapper for std::time::Duration so we can derive Serialize and Deserialize traits
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy, Default, Debug)]
pub struct Duration(std::time::Duration);

impl Duration {
    /// Get duration from seconds.
    pub fn from_secs_f64(secs: f64) -> Self {
        Duration(std::time::Duration::from_secs_f64(secs))
    }

    pub fn from_millis_u64(millis: u64) -> Self {
        Duration(std::time::Duration::from_millis(millis))
    }

    /// Calculate the duration between two TimeStamps
    pub fn between(ts1: &TimeStamp, ts2: &TimeStamp) -> Self {
        Duration(
            chrono::Duration::to_std(&ts1.0.signed_duration_since(ts2.0))
                .expect("Duration out of bounds"),
        )
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

// TODO Create a Display trait for this
pub struct FitHeading {
    pub manufacturer: String,
    pub time_created: String,
    pub num_sessions: u32,
    pub num_laps: u32,
    pub num_records: u32,
}

pub enum ActivityType {
    Running,
    Cycling,
    Rowing,
    Other(String),
}

pub enum Unit {
    Metric,
    Imperial,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Session {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub heartrate_min: Option<u8>,
    pub speed_avg: Option<Velocity>,
    pub speed_max: Option<Velocity>,
    pub power_avg: Option<u16>,
    pub power_max: Option<u16>,
    pub nec_lat: Option<f64>,
    pub nec_lon: Option<f64>,
    pub swc_lat: Option<f64>,
    pub swc_lon: Option<f64>,
    pub laps: Option<u16>,
    pub activity_type: Option<String>,
    pub activity_detailed: Option<String>,
    pub ascent: Option<Length_u16>,
    pub descent: Option<Length_u16>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Duration,
    pub duration_active: Duration,
    pub duration_moving: Duration,
    pub start_time: TimeStamp,
    pub time_in_hr_zones: Option<Vec<Duration>>,
}

/// Used in calculating latitudes and longitudes.
pub const MULTIPLIER: f64 = 180_f64 / (2_u32 << 30) as f64;
