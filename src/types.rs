/**
 * Local structs used to contain the various bits and pieces of information extracted from the header.
 * This includes such things as device manufacturer, activity, etc.
 *
 * This will then be put into each line in the resulting CSV, so that each line essentially is self-contained.
 */

use uom::si::f64::{Length as Length_f64, Velocity};

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

// struct TimeStamp(pub DateTime<Local>);

pub struct Session {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
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
}
