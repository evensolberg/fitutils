//! Defines the `Lap` struct which contains summary information per lap, and associated functions.

use crate::fit::constfunc::{map_float64, map_sint32, map_uint16, map_uint8, LATLON_MULTIPLIER};
use crate::{Duration, FITHrZones, FITSession};

use chrono::{DateTime, Local};
use fitparser::FitDataField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uom::si::{
    f64::{Length as Length_f64, Velocity},
    length::meter,
    u16::Length as Length_u16,
    velocity::meter_per_second,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Summary information per lap
#[derive(Serialize, Deserialize, Debug, Clone, Default)] // Don't need to impl anything since we derive defaults
#[serde(default)]
#[allow(clippy::module_name_repetitions)]
pub struct FITLap {
    /// The name of the .FIT file in which the lap information is found.
    pub filename: Option<String>,

    /// Lap number.
    pub lap_num: Option<u64>,

    /// Average cadence (running, walking, biking, ...).
    pub cadence_avg: Option<u8>,

    /// Maximum cadence (running, walking, biking, ...).
    pub cadence_max: Option<u8>,

    /// Minimum heart rate (Beats per Minute).
    pub heartrate_min: Option<u8>,

    /// Average heart rate (Beats per Minute).
    pub heartrate_avg: Option<u8>,

    /// Maximum heart rate (Beats per Minute).
    pub heartrate_max: Option<u8>,

    /// Average speed (Meter per Second).
    pub speed_avg: Option<Velocity>,

    /// Maximum speed (Meter per Second).
    pub speed_max: Option<Velocity>,

    /// Average power (Watt).
    pub power_avg: Option<u16>,

    /// Maxmimum power (Watt).
    pub power_max: Option<u16>,

    /// Latitude of lap start.
    pub lat_start: Option<f64>,

    /// Longitude of lap start.
    pub lon_start: Option<f64>,

    /// Latitude of lap end.
    pub lat_end: Option<f64>,

    /// Longitude of lap end.
    pub lon_end: Option<f64>,

    /// Average stance time.
    pub stance_time_avg: Option<Duration>,

    /// Vertical oscillation.
    pub vertical_oscillation_avg: Option<f64>,

    /// Total ascent (Meter).
    pub ascent: Option<Length_u16>,

    //l Total descent (Meter).
    pub descent: Option<Length_u16>,

    /// Calories burned.
    pub calories: Option<u16>,

    /// Distance covered (Meter).
    pub distance: Option<Length_f64>,

    /// Lap total duration including pauses.
    pub duration: Option<Duration>,

    /// Lap active duration without pauses.
    pub duration_active: Option<Duration>,

    /// Lap moving duration.
    pub duration_moving: Option<Duration>,

    /// Lap start time.
    pub start_time: Option<DateTime<Local>>,

    /// Lap finish time.
    pub finish_time: Option<DateTime<Local>>,

    /// Time spent in each heart rate zone.
    pub time_in_hr_zones: FITHrZones,
}

impl FITLap {
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Parses lap information from `FitDataRecord` into more detail and returns a new `Lap`.
    ///
    /// # Parameters
    ///
    /// `fields: &[FitDataField]` -- See the References for details.
    ///
    /// `session: &Session` -- Session summary information. Currently only used to get the file name.
    ///
    /// # Returns
    ///
    /// `Result<Self, Box<dyn Error>>` -- Returns `Ok(Lap)` if everything went well, `Error` if not.
    ///
    /// # Errors
    ///
    /// None.
    ///
    /// # Example
    ///
    /// - Assume `my_session` has been parsed and filled already.
    /// - Assume `data` is a `FitDataField` with `data.kind() == MesgNum::Lap`.
    ///
    /// ```
    /// let lap = FITLap::from_fit_lap(data.fields(), &my_session)?;
    /// ```
    /// # References
    ///
    /// Struct [`fitparser::FitDataField`](https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html)
    pub fn from_fit_lap(fields: &[FitDataField], session: &FITSession) -> Self {
        // Collect the fields into a HashMap which we can then dig details out of.
        // x.name is the key and x.value is the value
        // Note that the value is an enum and contain a number of different types
        // See the fitparser crate for details
        let mut lap = Self::default();

        lap.filename = session.filename.clone();

        let field_map: HashMap<&str, &fitparser::Value> =
            fields.iter().map(|x| (x.name(), x.value())).collect();

        lap.cadence_avg = field_map.get("avg_cadence").and_then(map_uint8);
        lap.cadence_max = field_map.get("max_cadence").and_then(map_uint8);

        lap.heartrate_min = field_map.get("min_heart_rate").and_then(map_uint8);
        lap.heartrate_avg = field_map.get("avg_heart_rate").and_then(map_uint8);
        lap.heartrate_max = field_map.get("max_heart_rate").and_then(map_uint8);

        lap.stance_time_avg = field_map
            .get("avg_stance_time")
            .and_then(map_float64)
            .map(Duration::from_secs_f64);
        lap.vertical_oscillation_avg = field_map
            .get("avg_vertical_oscillation")
            .and_then(map_float64);

        lap.speed_avg = field_map
            .get("enhanced_avg_speed")
            .and_then(map_float64)
            .map(Velocity::new::<meter_per_second>);
        lap.speed_max = field_map
            .get("enhanced_max_speed")
            .and_then(map_float64)
            .map(Velocity::new::<meter_per_second>);

        lap.power_avg = field_map.get("avg_power").and_then(map_uint16);
        lap.power_max = field_map.get("max_power").and_then(map_uint16);

        lap.lat_start = field_map
            .get("start_position_lat")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);
        lap.lon_start = field_map
            .get("start_position_long")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);
        lap.lat_end = field_map
            .get("end_position_lat")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);
        lap.lon_end = field_map
            .get("end_position_long")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);

        lap.ascent = field_map
            .get("total_ascent")
            .and_then(map_uint16)
            .map(Length_u16::new::<meter>);
        lap.descent = field_map
            .get("total_descent")
            .and_then(map_uint16)
            .map(Length_u16::new::<meter>);

        lap.calories = field_map.get("total_calories").and_then(map_uint16);
        lap.distance = field_map
            .get("total_distance")
            .and_then(map_float64)
            .map(Length_f64::new::<meter>);

        lap.duration = field_map
            .get("total_elapsed_time")
            .and_then(map_float64)
            .map(Duration::from_secs_f64);
        lap.duration_active = field_map
            .get("total_timer_time")
            .and_then(map_float64)
            .map(Duration::from_secs_f64);
        lap.duration_moving = field_map
            .get("total_moving_time")
            .and_then(map_float64)
            .map(Duration::from_secs_f64);

        if let Some(fitparser::Value::Timestamp(ts)) = field_map.get("start_time") {
            lap.start_time = Some(*ts);
        } else {
            lap.start_time = None;
        }

        if let Some(fitparser::Value::Timestamp(ts)) = field_map.get("timestamp") {
            lap.finish_time = Some(*ts);
        } else {
            lap.finish_time = None;
        }

        lap.time_in_hr_zones = FITHrZones::from(field_map.get("time_in_hr_zone"));

        lap
    }

    // end impl Lap
}
