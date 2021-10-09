use crate::types::duration::Duration;
use crate::types::timestamp::TimeStamp;

use serde::{Deserialize, Serialize};
use uom::si::f64::{Length as Length_f64, Velocity};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Detailed information about each record/data point in the workout session.
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
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
