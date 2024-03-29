//! Defines the `Record` struct which contains detailed information about each record/data point in the workout session.

use crate::fit::constfunc::{map_float64, map_sint32, map_uint16, map_uint8, LATLON_MULTIPLIER};
use crate::fit::session::FITSession;
use crate::Duration;

use chrono::{DateTime, Local, TimeZone};

use fitparser::{FitDataField, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uom::si::f64::{Length as Length_f64, Velocity};
use uom::si::{length::meter, velocity::meter_per_second};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Detailed information about each record/data point in the workout session.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
#[allow(clippy::module_name_repetitions)]
pub struct FITRecord {
    /// Record timestamp.
    pub timestamp: Option<DateTime<Local>>,

    /// How far into the current session are we (Seconds)
    pub duration: Option<Duration>,

    /// Distance covered since last record entry (Meters).
    pub distance: Option<Length_f64>,

    /// Altiude (Meters).
    pub altitude: Option<Length_f64>,

    /// Stance time (Seconds).
    pub stance_time: Option<Duration>,

    /// Vertical oscillation.
    pub vertical_oscillation: Option<f64>,

    /// Cadence in beats (or revolutions) per minute.
    pub cadence: Option<u8>,

    /// Speed (Meters per Second).
    pub speed: Option<Velocity>,

    /// Power (Watts).
    pub power: Option<u16>,

    /// Heart rate (Beats per Minute).
    pub heartrate: Option<u8>,

    /// Calories burned/
    pub calories: Option<u16>,

    /// Latitude (Degrees).
    pub lat: Option<f64>,

    /// Longitude (Degrees).
    pub lon: Option<f64>,
}

impl FITRecord {
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Parses record information into more detail.
    ///
    /// # Parameters
    ///
    /// `fields: &[FitDataField]` -- A `FitBitDataField` with `MsgNum::Record`
    ///
    /// `session: &Session` -- Session summary information. Currently used to calculate duration from start.
    ///
    /// # Returns
    ///
    /// `Result<Record, Box<dyn Error>>` -- Returns a new `Record` if OK, `Error` otherwise.
    ///
    /// # Example
    ///
    /// - Assume `my_session` has been parsed and filled already.
    /// - Assume `data` is a `FitDataField` with `data.kind() == MesgNum::Record`.
    ///
    /// ```
    /// let record = Record::from_fit_record(data.fields(), &my_session)?;
    /// ```
    ///
    /// # References
    ///
    /// Struct [`FitDataField`](https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html)
    pub fn from_fit_record(fields: &[FitDataField], session: &FITSession) -> Self {
        // Collect the fields into a HashMap which we can then dig details out of.
        // x.name is the key and x.value is the value
        // Note that the value is an enum and contain a number of different types
        // See the fitparser crate for details

        let mut record = Self::default();

        let field_map: HashMap<&str, &fitparser::Value> =
            fields.iter().map(|x| (x.name(), x.value())).collect();

        if let Some(Value::Timestamp(ts)) = field_map.get("timestamp") {
            record.timestamp = Some(*ts);
        } else {
            record.timestamp = None;
        }

        let duration = record.timestamp.as_ref().map(|start_time| {
            Duration::between(
                start_time,
                session
                    .time_created
                    .as_ref()
                    .unwrap_or(&Local.timestamp_opt(0, 0).unwrap()),
            )
        });
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

        record
    }
}
