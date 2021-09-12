/**
 * Local structs used to contain the various bits and pieces of information extracted from the header.
 * This includes such things as device manufacturer, activity, etc.
 *
 * This will then be put into each line in the resulting CSV, so that each line essentially is self-contained.
 */
use chrono::{offset::Local, DateTime};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub};
use uom::si::{
    f64::{Length as Length_f64, Velocity},
    // length::{foot, kilometer, meter, mile},
    length::meter,
    u16::Length as Length_u16,
    // velocity::{foot_per_second, kilometer_per_hour, meter_per_second, mile_per_hour},
    velocity::meter_per_second,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Used in calculating latitudes and longitudes.
pub const MULTIPLIER: f64 = 180_f64 / (2_u32 << 30) as f64;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ActivityType {
    Running,
    Cycling,
    Rowing,
    Other(String),
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum Unit {
    Metric,
    Imperial,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Summary information about the workout session
#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub manufacturer: String,
    pub time_created: TimeStamp,
    pub activity_type: Option<String>,
    pub activity_detailed: Option<String>,
    pub num_sessions: Option<u16>,
    pub num_laps: Option<u16>,
    pub num_records: Option<u32>,
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub heartrate_min: Option<u8>,
    pub speed_avg: Option<Velocity>,
    pub speed_max: Option<Velocity>,
    pub power_avg: Option<u16>,
    pub power_max: Option<u16>,
    pub power_threshold: Option<u16>,
    pub nec_lat: Option<f64>,
    pub nec_lon: Option<f64>,
    pub swc_lat: Option<f64>,
    pub swc_lon: Option<f64>,
    pub stance_time_avg: Option<f64>,
    pub vertical_oscillation_avg: Option<f64>,
    pub ascent: Option<Length_u16>,
    pub descent: Option<Length_u16>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Duration,
    pub duration_active: Duration,
    pub duration_moving: Duration,
    pub start_time: TimeStamp,
    pub finish_time: TimeStamp,
    pub time_in_hr_zones: Vec<Duration>,
}

impl Session {
    /// Initialize Session with default empty values
    pub fn new() -> Self {
        Session::default()
    }
}

impl Default for Session {
    /// Set defaults to be either empty or zero.
    fn default() -> Self {
        Session {
            manufacturer: "".to_string(),
            time_created: TimeStamp::default(),
            activity_type: Some("".to_string()),
            activity_detailed: Some("".to_string()),
            num_sessions: Some(0),
            num_laps: Some(0),
            num_records: Some(0),
            cadence_avg: Some(0),
            cadence_max: Some(0),
            heartrate_avg: Some(0),
            heartrate_max: Some(0),
            heartrate_min: Some(0),
            speed_avg: Some(Velocity::new::<meter_per_second>(0.0)),
            speed_max: Some(Velocity::new::<meter_per_second>(0.0)),
            power_avg: Some(0),
            power_max: Some(0),
            power_threshold: Some(0),
            nec_lat: Some(0.0),
            nec_lon: Some(0.0),
            swc_lat: Some(0.0),
            swc_lon: Some(0.0),
            stance_time_avg: Some(0.0),
            vertical_oscillation_avg: Some(0.0),
            ascent: Some(Length_u16::new::<meter>(0)),
            descent: Some(Length_u16::new::<meter>(0)),
            calories: Some(0),
            distance: Some(Length_f64::new::<meter>(0.0)),
            duration: Duration::default(),
            duration_active: Duration::default(),
            duration_moving: Duration::default(),
            start_time: TimeStamp::default(),
            finish_time: TimeStamp::default(),
            time_in_hr_zones: Vec::new(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Summary information per lap
#[derive(Default, Serialize, Deserialize, Debug)] // Don't need to impl anything since we derive defaults
#[serde(default)]
pub struct Lap {
    pub lap_num: u64,
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<Velocity>,
    pub speed_max: Option<Velocity>,
    pub power_avg: Option<u16>,
    pub power_max: Option<u16>,
    pub lat_start: Option<f64>,
    pub lon_start: Option<f64>,
    pub lat_end: Option<f64>,
    pub lon_end: Option<f64>,
    pub ascent: Option<Length_u16>,
    pub descent: Option<Length_u16>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Duration,
    pub duration_active: Duration,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Detailed information about each record/data point in the workout session.
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Record {
    pub cadence: Vec<Option<u8>>,
    pub distance: Vec<Option<Length_f64>>,
    pub altitude: Vec<Option<Length_f64>>,
    pub speed: Vec<Option<Velocity>>,
    pub heartrate: Vec<Option<u8>>,
    pub power: Vec<Option<u16>>,
    pub lat: Vec<Option<f64>>,
    pub lon: Vec<Option<f64>>,
    pub timestamp: Vec<TimeStamp>,
    pub duration: Vec<Duration>,
}
