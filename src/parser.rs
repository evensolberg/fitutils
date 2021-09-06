use fitparser::{profile::field_types::MesgNum, FitDataField, Value};
use std::error::Error;
use std::fs::File;
use std::{collections::HashMap, str::FromStr};
// use std::io::ErrorKind;

// From our own code
use super::activity::{Activity, ActivityType, Lap, Record, Session};
use super::types::{Duration, TimeStamp};

// use Units of Measure crate
// https://crates.io/crates/uom
use uom::si::{
    f64::{Length as Length_f64, Velocity},
    length::meter,
    u16::Length as Length_u16,
    velocity::meter_per_second,
};

// pub mod parser {
// Function scaffold macro to map from a value in the FIT parser to a "real" value
// https://medium.com/@phoomparin/a-beginners-guide-to-rust-macros-5c75594498f1
macro_rules! map_value {
        ($name:ident, $type:ident, $( $pattern:pat )|+ => $mapping:expr) => {
            fn $name(v: &&fitparser::Value) -> Option<$type> {
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

/// Used when calculating latitudes and longitudes.
const MULTIPLIER: f64 = 180_f64 / (2_u32 << 30) as f64;

/// Reads a .FIT data file and parses it.
// TODO Look into whether we could use Option instead of Result
pub fn parse(fitfile_name: &str) -> Result<Activity, Error> {
    // placeholders - see activity.rs for definitions
    let mut session: Session = Session::default(); // Session data
    let mut record: Record = Record::default(); // Individual data points (per second, typically)
    let mut lap_vec: Vec<Lap> = Vec::new(); // Lap information vector

    // Open and read the .fit file
    let mut fit_data = File::open(fitfile_name)?;
    let file = fitparser::from_reader(&mut fit_data)?;

    // Check if the file contains session data - exit out if not.
    if !file.iter().any(|x| x.kind() == MesgNum::Session) {
        // If using Option: return None
        return Err(Box::new("Session data not found."));
    }

    // We have session data - parse it.
    for data in file {
        // for each FitDataRecord
        match data.kind() {
            // Figure out what kind it is and parse accordingly
            MesgNum::Session => parse_session(data.fields(), &mut session), // defined below
            MesgNum::Record => parse_record(data.fields(), &mut record),    // defined below
            MesgNum::Lap => {
                let mut lap = Lap::default(); // Create an empty lap instance
                parse_lap(data.fields(), &mut lap); // parse lap data
                lap_vec.push(lap); // push the lap onto the vector
            }
            _ => (),
        }
    }

    // Some fit-files do not contain corner coordinates,
    // so find them manually if missing
    if session.nec_lat.is_none()
        || session.nec_lon.is_none()
        || session.swc_lat.is_none()
        || session.swc_lon.is_none()
    {
        // Find north east corner latitude
        session.nec_lat = Some(
            record
                .lat
                .iter()
                .flatten()
                .copied()
                .fold(f64::NAN, f64::max),
        );
        // Find north east corner longitude
        session.nec_lon = Some(
            record
                .lon
                .iter()
                .flatten()
                .copied()
                .fold(f64::NAN, f64::max),
        );
        // Find south west corner latitude
        session.swc_lat = Some(
            record
                .lat
                .iter()
                .flatten()
                .copied()
                .fold(f64::NAN, f64::min),
        );
        // Find south west longitude
        session.swc_lon = Some(
            record
                .lon
                .iter()
                .flatten()
                .copied()
                .fold(f64::NAN, f64::min),
        );
    }

    // Return the activity
    Ok(Activity {
        id: session.start_time.0.format("%Y%m%d%H%M").to_string(),
        session,
        record,
        lap: lap_vec,
    })
}

/// Parses session information into more detail.
pub fn parse_session(fields: &[FitDataField], session: &mut Session) {
    // Collect the fields into a HashMap which we can then dig details out of.
    // x.name is the key and x.value is the value
    // Note that the value is an enum and contain a number of different types
    // See the fitparser crate for details
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();

    // Gather some session metrics from the HashMap into the session struct which will be returned
    session.cadence_avg = field_map.get("avg_cadence").and_then(map_uint8);

    session.cadence_max = field_map.get("max_cadence").and_then(map_uint8);

    session.heartrate_avg = field_map.get("avg_heart_rate").and_then(map_uint8);

    session.heartrate_max = field_map.get("max_heart_rate").and_then(map_uint8);

    session.speed_avg = field_map
        .get("enhanced_avg_speed")
        .and_then(map_float64)
        .map(Velocity::new::<meter_per_second>);

    session.speed_max = field_map
        .get("enhanced_max_speed")
        .and_then(map_float64)
        .map(Velocity::new::<meter_per_second>);

    session.power_avg = field_map.get("avg_power").and_then(map_uint16);

    session.power_max = field_map.get("max_power").and_then(map_uint16);

    session.nec_lat = field_map
        .get("nec_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    session.nec_lon = field_map
        .get("nec_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    session.swc_lat = field_map
        .get("swc_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    session.swc_lon = field_map
        .get("swc_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    session.laps = field_map.get("num_laps").and_then(map_uint16);

    session.activity_type = ActivityType::from_str(
        &field_map
            .get("sport")
            .and_then(map_string)
            .unwrap_or_default(),
    )
    .unwrap_or_default();
    // TODO: Compare to the definition in activity.rs
    // LINK activity.rs:119
    // https://doc.rust-lang.org/error-index.html#E0599

    session.ascent = field_map
        .get("total_ascent")
        .and_then(map_uint16)
        .map(Length_u16::new::<meter>);

    session.descent = field_map
        .get("total_descent")
        .and_then(map_uint16)
        .map(Length_u16::new::<meter>);

    session.calories = field_map.get("total_calories").and_then(map_uint16);

    session.distance = field_map
        .get("total_distance")
        .and_then(map_float64)
        .map(Length_f64::new::<meter>);

    session.duration = field_map
        .get("total_elapsed_time")
        .and_then(map_float64)
        .map(Duration::from_secs_f64)
        .unwrap_or_default();

    session.duration_active = field_map
        .get("total_timer_time")
        .and_then(map_float64)
        .map(Duration::from_secs_f64)
        .unwrap_or_default();

    session.start_time = field_map
        .get("start_time")
        .and_then(map_timestamp)
        .unwrap_or_default();
}

/// Parses record information into more detail.
fn parse_record(fields: &[FitDataField], record: &mut Record) {
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
            .map(|x| f64::from(x) * MULTIPLIER),
    );

    record.lon.push(
        field_map
            .get("position_long")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * MULTIPLIER),
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

/// Parses lap information into more detail.
fn parse_lap(fields: &[FitDataField], lap: &mut Lap) {
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
        .map(|x| f64::from(x) * MULTIPLIER);

    lap.lon_start = field_map
        .get("start_position_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    lap.lat_end = field_map
        .get("end_position_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    lap.lon_end = field_map
        .get("end_position_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

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
} // fn parse_lap
  // } // pub mod parser
