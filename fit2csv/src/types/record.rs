use crate::types::constfunc::*;
use crate::types::duration::Duration;
use crate::types::session::Session;
use crate::types::timestamp::TimeStamp;

use fitparser::FitDataField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use uom::si::f64::{Length as Length_f64, Velocity};
use uom::si::{length::meter, velocity::meter_per_second};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Detailed information about each record/data point in the workout session.
#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl Record {
    /// Return a new, empty Record
    pub fn new() -> Self {
        Self::default()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Parses record information into more detail.
    ///
    /// **Parameters:**
    ///
    ///    `fields: &[FitDataField]` -- See the fitparser crate for details: <https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html><br>
    ///    `record: &mut types::Record` -- An empty record struct to be filled in. See `types.rs` for details on this stuct.
    ///    `session: &types::Session` -- Session summary information. Currently used to calculate duration.
    ///
    /// **Returns:**
    ///
    ///    `Result<Record, Box<dyn Error>>` -- Returns nothing if OK, error if problematic.
    pub fn from_fit_record(
        fields: &[FitDataField],
        session: &Session,
    ) -> Result<Record, Box<dyn Error>> {
        // Collect the fields into a HashMap which we can then dig details out of.
        // x.name is the key and x.value is the value
        // Note that the value is an enum and contain a number of different types
        // See the fitparser crate for details

        let mut record = Record::new();

        let field_map: HashMap<&str, &fitparser::Value> =
            fields.iter().map(|x| (x.name(), x.value())).collect();

        record.timestamp = field_map.get("timestamp").and_then(map_timestamp);

        let duration = record
            .timestamp
            .as_ref()
            .map(|x| Duration::between(x, session.time_created.as_ref().unwrap()));

        record.duration = duration;

        record.distance = field_map
            .get("distance")
            .and_then(map_float64)
            .map(Length_f64::new::<meter>);
        record.altitude = field_map
            .get("enhanced_altitude")
            .and_then(map_float64)
            .map(Length_f64::new::<meter>);

        record.cadence = field_map.get("cadence").and_then(map_uint8);
        record.speed = field_map
            .get("enhanced_speed")
            .and_then(map_float64)
            .map(Velocity::new::<meter_per_second>);
        record.power = field_map.get("power").and_then(map_uint16);
        record.calories = field_map.get("calories").and_then(map_uint16);
        record.heartrate = field_map.get("heart_rate").and_then(map_uint8);

        record.stance_time = field_map
            .get("stance_time")
            .and_then(map_float64)
            .map(Duration::from_secs_f64);
        record.vertical_oscillation = field_map.get("vertical_oscillation").and_then(map_float64);

        record.lat = field_map
            .get("position_lat")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);

        record.lon = field_map
            .get("position_long")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);

        Ok(record)
    }
}

impl Default for Record {
    fn default() -> Self {
        Self {
            timestamp: None,
            duration: None,
            distance: None,
            altitude: None,
            stance_time: None,
            vertical_oscillation: None,
            cadence: None,
            speed: None,
            power: None,
            heartrate: None,
            calories: None,
            lat: None,
            lon: None,
        }
    }
}
