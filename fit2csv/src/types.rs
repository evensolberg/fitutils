/**
 * Local structs used to contain the various bits and pieces of information extracted from the header.
 * This includes such things as device manufacturer, activity, etc.
 *
 * This will then be put into each line in the resulting CSV, so that each line essentially is self-contained.
 */
use chrono::{offset::Local, DateTime};
use serde::ser::{SerializeStruct, Serializer};
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
    /// Format time to `%d.%m.%Y %H:%M`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%d.%m.%Y %H:%M"))
    }
}

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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the all the information about the file and its contents
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Activity {
    pub session: Session,
    pub laps: Vec<Lap>,
    pub records: Vec<Record>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Summary information about the workout session
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub filename: Option<String>, // TODO: Switch to PathBuf
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub serial_number: Option<String>,
    pub time_created: Option<TimeStamp>,
    pub activity_type: Option<String>,
    pub activity_detailed: Option<String>,
    pub num_sessions: Option<u16>,
    pub num_laps: Option<u16>,
    pub num_records: Option<u64>,
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
    pub duration: Option<Duration>,
    pub duration_active: Option<Duration>,
    pub duration_moving: Option<Duration>,
    pub start_time: Option<TimeStamp>,
    pub finish_time: Option<TimeStamp>,
    pub time_in_hr_zones: HrZones,
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
            filename: Some("".to_string()),
            manufacturer: Some("".to_string()),
            product: Some("".to_string()),
            serial_number: Some("".to_string()),
            time_created: Some(TimeStamp::default()),
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
            duration: Some(Duration::default()),
            duration_active: Some(Duration::default()),
            duration_moving: Some(Duration::default()),
            start_time: Some(TimeStamp::default()),
            finish_time: Some(TimeStamp::default()),
            time_in_hr_zones: HrZones::default(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Summary information per lap
#[derive(Default, Serialize, Deserialize, Debug)] // Don't need to impl anything since we derive defaults
#[serde(default)]
pub struct Lap {
    pub filename: Option<String>,
    pub lap_num: Option<u64>,
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_min: Option<u8>,
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
    pub stance_time_avg: Option<Duration>,
    pub vertical_oscillation_avg: Option<f64>,
    pub ascent: Option<Length_u16>,
    pub descent: Option<Length_u16>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Option<Duration>,
    pub duration_active: Option<Duration>,
    pub duration_moving: Option<Duration>,
    pub start_time: Option<TimeStamp>,
    pub finish_time: Option<TimeStamp>,
    pub time_in_hr_zones: HrZones,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Detailed information about each record/data point in the workout session.
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Record {
    pub timestamp: Option<TimeStamp>,
    pub duration: Option<Duration>,
    pub distance: Option<Length_f64>,
    pub altitude: Option<Length_f64>,
    pub stance_time: Option<Duration>,
    pub vertical_oscillation: Option<f64>,
    pub cadence: Option<u8>,
    pub speed: Option<Velocity>,
    pub power: Option<u16>,
    pub heartrate: Option<u8>,
    pub calories: Option<u16>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Detailed information about how much time is spent in each heart rate zone.
///
/// The actual zones are defined as the training levels based on your maximum heart rate, which is usually calculated
/// as 220 - your age in years.
///
/// **HR Zones:**
///
///    **0**: Warmup<br>
///    **1**: Fat Burn<br>
///    **2**: Aerobic<br>
///    **3**: Anaerobic<br>
///    **4**: Speed/Power<br>
///
/// **Reference:**
///
///    <https://www.heart.org/en/healthy-living/fitness/fitness-basics/target-heart-rates>
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(default)]
pub struct HrZones {
    pub hr_zone_0: Option<Duration>,
    pub hr_zone_1: Option<Duration>,
    pub hr_zone_2: Option<Duration>,
    pub hr_zone_3: Option<Duration>,
    pub hr_zone_4: Option<Duration>,
}

impl HrZones {
    /// Initialize Session with default empty values
    pub fn new() -> Self {
        HrZones::default()
    }
}

impl Default for HrZones {
    /// Set defaults to be either empty or zero.
    fn default() -> Self {
        HrZones {
            hr_zone_0: None,
            hr_zone_1: None,
            hr_zone_2: None,
            hr_zone_3: None,
            hr_zone_4: None,
        }
    }
}

impl Serialize for HrZones {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("HrZones", 5)?;
        let dur_zero = Duration::from_millis_u64(0);
        state.serialize_field(
            "hr_zone_0_secs",
            &self.hr_zone_0.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.serialize_field(
            "hr_zone_1_secs",
            &self.hr_zone_1.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.serialize_field(
            "hr_zone_2_secs",
            &self.hr_zone_2.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.serialize_field(
            "hr_zone_3_secs",
            &self.hr_zone_3.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.serialize_field(
            "hr_zone_4_secs",
            &self.hr_zone_4.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.end()
    }
}
