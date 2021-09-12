// External crates
use fitparser::{FitDataField, FitDataRecord, Value}; // .FIT file manipulation
use std::collections::HashMap;
use uom::si::{
    f64::{Length as Length_f64, Velocity},
    length::meter,
    u16::Length as Length_u16,
    velocity::meter_per_second,
};

// Local crates
use super::types;
use crate::types::Duration;
use crate::types::TimeStamp;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Function scaffold macro to map from a value in the FIT parser to a "real" value
macro_rules! map_value {
    ($function_name:ident, $return_type:ident, $( $pattern:pat )|+ => $mapping:expr) => {
        fn $function_name(v: &&fitparser::Value) -> Option<$return_type> {
            match v {
                $( $pattern )|+ => ::std::option::Option::Some($mapping),
                _               => ::std::option::Option::None,
            }
        }
    }
}

// Implementations
map_value!(map_uint8, u8, Value::UInt8(x) => *x);
map_value!(map_uint16, u16, Value::UInt16(x) => *x);
map_value!(map_sint32, i32, Value::SInt32(x) => *x);
map_value!(map_float64, f64, Value::Float64(x) => *x);
map_value!(map_string, String, Value::String(x) => x.to_string());
map_value!(map_timestamp, TimeStamp, Value::Timestamp(x) => TimeStamp(*x));

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Extract manufacturer and session creation time from the FIT data file header
pub fn parse_header(header: &FitDataRecord, session: &mut types::Session) {
    session.manufacturer = header.fields()[1].value().to_string();
    session.time_created = map_timestamp(&header.fields()[3].value())
        .expect("Unable to extract session creation time.");
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Parses session information into more detail.
///
/// **Parameters:**
///
///    `fields: &[FitDataField]` -- See the fitparser crate for details: <https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html><br>
///    `session: &mut Session` -- An empty session struct to be filled in. See `types.rs` for details on this stuct.
///
/// **Returns:**
///
///    Nothing. The data is put into the `session` struct.
pub fn parse_session(fields: &[FitDataField], session: &mut types::Session) {
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();

    log::debug!("Session field_map = {:?}", field_map);

    log::trace!("Getting metrics from hashmap.");
    session.cadence_avg = field_map.get("avg_cadence").and_then(map_uint8);
    log::trace!("cadence_avg = {:?}", session.cadence_avg);

    session.cadence_max = field_map.get("max_cadence").and_then(map_uint8);
    log::trace!("cadence_max = {:?}", session.cadence_max);

    session.heartrate_avg = field_map.get("avg_heart_rate").and_then(map_uint8);
    log::trace!("heartrate_avg = {:?}", session.heartrate_avg);

    session.heartrate_max = field_map.get("max_heart_rate").and_then(map_uint8);
    log::trace!("heartrate_max = {:?}", session.heartrate_max);

    session.heartrate_min = field_map.get("min_heart_rate").and_then(map_uint8);
    log::trace!("heartrate_min = {:?}", session.heartrate_min);

    session.stance_time_avg = field_map.get("avg_stance_time").and_then(map_float64);
    log::trace!("stance_time_avg = {:?}", session.stance_time_avg);

    session.vertical_oscillation_avg = field_map
        .get("avg_vertical_oscillation")
        .and_then(map_float64);
    log::trace!(
        "vertical_oscillation_avg = {:?}",
        session.vertical_oscillation_avg
    );

    session.speed_avg = field_map
        .get("enhanced_avg_speed")
        .and_then(map_float64)
        .map(Velocity::new::<meter_per_second>);
    log::trace!("speed_avg = {:?}", session.speed_avg);

    session.speed_max = field_map
        .get("enhanced_max_speed")
        .and_then(map_float64)
        .map(Velocity::new::<meter_per_second>);
    log::trace!("speed_max = {:?}", session.speed_max);

    session.power_avg = field_map.get("avg_power").and_then(map_uint16);
    log::trace!("power_avg = {:?}", session.power_avg);

    session.power_max = field_map.get("max_power").and_then(map_uint16);
    log::trace!("power_max = {:?}", session.power_max);

    session.power_threshold = field_map.get("threshold_power").and_then(map_uint16);
    log::trace!("power_threshold = {:?}", session.power_threshold);

    // GPS - NEC = North East Corner, SWC = South West Corner
    session.nec_lat = field_map
        .get("nec_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * types::MULTIPLIER);
    log::trace!("nec_lat = {:?}", session.nec_lat);

    session.nec_lon = field_map
        .get("nec_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * types::MULTIPLIER);
    log::trace!("nec_lon = {:?}", session.nec_lon);

    session.swc_lat = field_map
        .get("swc_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * types::MULTIPLIER);
    log::trace!("swc_lat = {:?}", session.swc_lat);

    session.swc_lon = field_map
        .get("swc_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * types::MULTIPLIER);
    log::trace!("swc_lon = {:?}", session.swc_lon);

    session.activity_type = field_map.get("sport").and_then(map_string);
    log::trace!("activity_type = {:?}", session.activity_type);

    session.activity_detailed = field_map.get("sub_sport").and_then(map_string);
    log::trace!("activity_detailed = {:?}", session.activity_detailed);

    session.ascent = field_map
        .get("total_ascent")
        .and_then(map_uint16)
        .map(Length_u16::new::<meter>);
    log::trace!("ascent = {:?}", session.ascent);

    session.descent = field_map
        .get("total_descent")
        .and_then(map_uint16)
        .map(Length_u16::new::<meter>);
    log::trace!("descent = {:?}", session.descent);

    session.calories = field_map.get("total_calories").and_then(map_uint16);
    log::trace!("calories = {:?}", session.calories);

    session.distance = field_map
        .get("total_distance")
        .and_then(map_float64)
        .map(Length_f64::new::<meter>);
    log::trace!("distance = {:?}", session.distance);

    session.duration = field_map
        .get("total_elapsed_time")
        .and_then(map_float64)
        .map(types::Duration::from_secs_f64)
        .unwrap_or_default();
    log::trace!("duration = {}", session.duration);

    session.duration_active = field_map
        .get("total_timer_time")
        .and_then(map_float64)
        .map(types::Duration::from_secs_f64)
        .unwrap_or_default();
    log::trace!("duration_active = {}", session.duration_active);

    session.duration_moving = field_map
        .get("total_moving_time")
        .and_then(map_float64)
        .map(types::Duration::from_secs_f64)
        .unwrap_or_default();
    log::trace!("duration_moving = {}", session.duration_moving);

    session.start_time = field_map
        .get("start_time")
        .and_then(map_timestamp)
        .unwrap_or_default();
    log::trace!("start_time = {:?}", session.start_time);

    session.finish_time = field_map
        .get("timestamp")
        .and_then(map_timestamp)
        .unwrap_or_default();
    log::trace!("finish_time = {:?}", session.finish_time);

    session.num_laps = field_map.get("num_laps").and_then(map_uint16);
    log::trace!("num_laps = {:?}", session.num_laps);

    // TODO: Figure out a better way to read the original Array
    // Turn the Array into a string and strip out everything except the numbers.
    let tihz = field_map
        .get("time_in_hr_zone")
        .unwrap()
        .to_string()
        .replace("UInt32(", "")
        .replace(")", "")
        .replace("[", "")
        .replace("]", "")
        .replace(",", "");
    // Split the numbers, turn them into u64, convert to Duration and collect to a vector
    log::trace!("tihz = {:?}", tihz);
    let t2: Vec<Duration> = tihz
        .split(' ')
        .map(|x| str::parse::<u64>(x).unwrap())
        .map(Duration::from_millis_u64)
        .collect();
    log::trace!("t2 = {:?}", t2);
    session.time_in_hr_zones = t2;
    log::debug!("time_in_hr_zones = {:?}", session.time_in_hr_zones);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Parses lap information into more detail.
///
/// **Parameters:**
///
///    `fields: &[FitDataField]` -- See the fitparser crate for details: <https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html><br>
///    `lap: &mut Lap` -- An empty record struct to be filled in. See `activity.rs` for details on this stuct.`
///
/// **Returns:**
///
///    Nothing. The data is put into the `lap` struct.
pub fn parse_lap(fields: &[FitDataField], lap: &mut types::Lap) {
    // Collect the fields into a HashMap which we can then dig details out of.
    // x.name is the key and x.value is the value
    // Note that the value is an enum and contain a number of different types
    // See the fitparser crate for details
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();

    lap.cadence_avg = field_map.get("avg_cadence").and_then(map_uint8);

    lap.cadence_max = field_map.get("max_cadence").and_then(map_uint8);

    lap.heartrate_avg = field_map.get("avg_heart_rate").and_then(map_uint8);

    lap.heartrate_max = field_map.get("max_heart_rate").and_then(map_uint8);

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
        .map(|x| f64::from(x) * types::MULTIPLIER);

    lap.lon_start = field_map
        .get("start_position_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * types::MULTIPLIER);

    lap.lat_end = field_map
        .get("end_position_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * types::MULTIPLIER);

    lap.lon_end = field_map
        .get("end_position_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * types::MULTIPLIER);

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
        .map(Duration::from_secs_f64)
        .unwrap_or_default();

    lap.duration_active = field_map
        .get("total_timer_time")
        .and_then(map_float64)
        .map(Duration::from_secs_f64)
        .unwrap_or_default();
}

/// Parses record information into more detail.
///
/// **Parameters:**
///
///    `fields: &[FitDataField]` -- See the fitparser crate for details: <https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html><br>
///    `record: &mut Record` -- An empty record struct to be filled in. See `activity.rs` for details on this stuct.
///
/// **Returns:**
///
///    Nothing. The data is put into the `record` struct.
pub fn parse_record(fields: &[FitDataField], record: &mut types::Record) {
    // Collect the fields into a HashMap which we can then dig details out of.
    // x.name is the key and x.value is the value
    // Note that the value is an enum and contain a number of different types
    // See the fitparser crate for details
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();

    record
        .cadence
        .push(field_map.get("cadence").and_then(map_uint8));

    record.distance.push(
        field_map
            .get("distance")
            .and_then(map_float64)
            .map(Length_f64::new::<meter>),
    );

    record.altitude.push(
        field_map
            .get("enhanced_altitude")
            .and_then(map_float64)
            .map(Length_f64::new::<meter>),
    );

    record.speed.push(
        field_map
            .get("enhanced_speed")
            .and_then(map_float64)
            .map(Velocity::new::<meter_per_second>),
    );

    record
        .power
        .push(field_map.get("power").and_then(map_uint16));

    record
        .heartrate
        .push(field_map.get("heart_rate").and_then(map_uint8));

    record.lat.push(
        field_map
            .get("position_lat")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * types::MULTIPLIER),
    );

    record.lon.push(
        field_map
            .get("position_long")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * types::MULTIPLIER),
    );

    let timestamp = field_map
        .get("timestamp")
        .and_then(map_timestamp)
        .unwrap_or_default();

    let duration = match record.timestamp.first() {
        Some(x) => Duration::between(&timestamp, x),
        None => Duration::default(),
    };

    record.duration.push(duration);
    record.timestamp.push(timestamp);
}
