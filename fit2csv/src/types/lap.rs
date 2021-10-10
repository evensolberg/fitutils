use crate::types::constfunc::*;
use crate::types::duration::Duration;
use crate::types::hrzones::HrZones;
use crate::types::session::Session;
use crate::types::timestamp::TimeStamp;

use fitparser::FitDataField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use uom::si::{
    f64::{Length as Length_f64, Velocity},
    length::meter,
    u16::Length as Length_u16,
    velocity::meter_per_second,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Summary information per lap
#[derive(Serialize, Deserialize, Debug, Clone)] // Don't need to impl anything since we derive defaults
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

impl Lap {
    /// Return a new, empty Lap struct
    pub fn new() -> Self {
        Self::default()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Parses lap information into more detail.
    ///
    /// **Parameters:**
    ///
    ///    `fields: &[FitDataField]` -- See the fitparser crate for details: <https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html><br>
    ///    `lap: &mut types::Lap` -- An empty record struct to be filled in. See `types.rs` for details on this stuct.`
    ///    `session: &types::Session` -- Session summary information. Currently used to get the file name.
    ///
    /// **Returns:**
    ///
    ///    `Result<Lap, Box<dyn Error>>` -- Returns nothing if OK, error if problematic.
    pub fn from_fit_lap(fields: &[FitDataField], session: &Session) -> Result<Lap, Box<dyn Error>> {
        // Collect the fields into a HashMap which we can then dig details out of.
        // x.name is the key and x.value is the value
        // Note that the value is an enum and contain a number of different types
        // See the fitparser crate for details
        let mut lap = Lap::new();

        lap.filename = session.filename.to_owned();
        log::trace!("parsers::parse_lap() -- Lap filename = {:?}", lap.filename);

        let field_map: HashMap<&str, &fitparser::Value> =
            fields.iter().map(|x| (x.name(), x.value())).collect();
        log::trace!("parsers::parse_lap() -- Lap field_map = {:?}", field_map);

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

        lap.start_time = field_map.get("start_time").and_then(map_timestamp);
        lap.finish_time = field_map.get("timestamp").and_then(map_timestamp);

        lap.time_in_hr_zones = HrZones::from(field_map.get("time_in_hr_zone"));

        Ok(lap)
    }
    // end impl Lap
}

impl Default for Lap {
    fn default() -> Self {
        Lap {
            filename: None,
            lap_num: None,
            cadence_avg: None,
            cadence_max: None,
            heartrate_min: None,
            heartrate_avg: None,
            heartrate_max: None,
            speed_avg: None,
            speed_max: None,
            power_avg: None,
            power_max: None,
            lat_start: None,
            lon_start: None,
            lat_end: None,
            lon_end: None,
            stance_time_avg: None,
            vertical_oscillation_avg: None,
            ascent: None,
            descent: None,
            calories: None,
            distance: None,
            duration: None,
            duration_active: None,
            duration_moving: None,
            start_time: None,
            finish_time: None,
            time_in_hr_zones: HrZones::new(),
        }
    }
}
