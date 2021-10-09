use crate::types::duration::Duration;
use crate::types::hrzones::HrZones;
use crate::types::timestamp::TimeStamp;

use serde::{Deserialize, Serialize};
use uom::si::{
    f64::{Length as Length_f64, Velocity},
    u16::Length as Length_u16,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Summary information per lap
#[derive(Default, Serialize, Deserialize, Debug, Clone)] // Don't need to impl anything since we derive defaults
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
