// use chrono::DateTime;
// External crates
use fitparser::profile::field_types::MesgNum; // .FIT file manipulation
use fitparser::{FitDataField, Value}; // .FIT file manipulation
use std::collections::HashMap;
// use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
// use std::io::Error::ErrorKind;

use uom::si::{
    f64::{Length as Length_f64, Velocity},
    length::meter,
    u16::Length as Length_u16,
    velocity::meter_per_second,
};

// Local crates
// use super::exporters;
use super::types;
use crate::types::{Activity, TimeStamp};
use crate::types::{Duration, HrZones};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Used in calculating latitudes and longitudes.
const LATLON_MULTIPLIER: f64 = 180_f64 / (2_u32 << 30) as f64;

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
/// Parses the input file into its constituent parts.
///
/// **Parameters:***
///
///    `filename: &str` -- The filename for the FIT file to be parsed.
///
/// **Returns:**
///
///    `Result<Activity, Box<dyn Error>>` -- Ok(Activity) if succesful, otherwise an error.
///
/// **Example:**
///
///   ```rust
///    mod parsers;
///
///    let my_activity = parsers::parse_fitfile("fitfile.fit")?;
///   ```
pub fn parse_fitfile(filename: &str) -> Result<types::Activity, Box<dyn Error>> {
    // open the file - return error if unable.
    let mut fp = File::open(filename).expect("FIT file corrupt or not found.");
    log::trace!(
        "parsers::parse_fitfile() -- {} was read OK. File pointer name: {:?}",
        filename,
        fp
    );

    // Read and parse the file contents
    log::trace!("parsers::parse_fitfile() -- Reading data");
    let file = fitparser::from_reader(&mut fp).expect("Unable to read FIT file.");
    log::debug!(
        "parsers::parse_fitfile() -- Data was read. Total number of records: {}",
        file.len()
    );

    log::trace!("parsers::parse_fitfile() -- Data read. Extracting header.");
    let header = &file[0]; // There HAS to be a better way to do this!
    log::debug!("parsers::parse_fitfile() -- Header: {:?}", header);

    log::trace!("parsers::parse_fitfile() -- Creating empty session.");
    let mut my_session = types::Session::new();
    my_session.filename = Some(filename.to_string());

    // This is the main file parsing loop. This will definitely get expanded.
    log::trace!("parsers::parse_fitfile() -- Initializing temporary variables.");
    let mut num_records = 0;
    let mut num_sessions = 0;
    let mut num_laps = 0;
    let mut lap_vec: Vec<types::Lap> = Vec::new(); // Lap information vector
    let mut records_vec: Vec<types::Record> = Vec::new();

    // This is where the actual parsing happens
    log::debug!("parsers::parse_fitfile() -- Parsing data.");
    for data in file {
        // for each FitDataRecord
        match data.kind() {
            // Figure out what kind it is and parse accordingly
            MesgNum::FileId => {
                // File header
                parse_header(data.fields(), &mut my_session);
                log::debug!(
                    "parsers::parse_fitfile() -- Session after parsing header: {:?}",
                    my_session
                );
            }
            MesgNum::Session => {
                parse_session(data.fields(), &mut my_session);
                log::debug!("parsers::parse_fitfile() -- Session: {:?}", my_session);
                num_sessions += 1;
                my_session.num_sessions = Some(num_sessions);
            }
            MesgNum::Lap => {
                let mut lap = types::Lap::default(); // Create an empty lap instance
                parse_lap(data.fields(), &mut lap, &my_session); // parse lap data
                num_laps += 1;
                lap.lap_num = Some(num_laps);
                log::debug!("parsers::parse_fitfile() -- Lap {:3}: {:?}", num_laps, lap);
                lap_vec.push(lap); // push the lap onto the vector
            }
            MesgNum::Record => {
                // FIXME: This is very inefficient since we're instantiating this for every record
                let mut record = types::Record::default();
                parse_record(data.fields(), &mut record, &my_session);
                log::debug!("parsers::parse_fitfile() -- Record: {:?}", record);
                records_vec.push(record);
                num_records += 1;
                my_session.num_records = Some(num_records);
            }
            _ => (),
        } // match
    } // for data

    let serialized_session = serde_json::to_string(&my_session).unwrap();
    log::trace!(
        "parsers::parse_fitfile() -- serialized_session session: {}",
        serialized_session
    );

    // Build the activity
    let my_activity = Activity {
        session: my_session,
        laps: lap_vec,
        records: records_vec,
    };

    // Return the activity struct
    Ok(my_activity)
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Extract manufacturer and session creation time from the FIT data file header
///
/// **Parameters:**
///
///    `fields: &[FitDataField]` -- See the fitparser crate for details: <https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html><br>
///    `session: &mut types::Session` -- An empty record struct to be filled in. See `types.rs` for details on this stuct.
///
/// **Returns:**
///
///    Nothing. The data is put into the `record` struct.
fn parse_header(fields: &[FitDataField], session: &mut types::Session) {
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();
    log::trace!(
        "parsers::parse_header() -- Header field_map = {:?}",
        field_map
    );

    session.manufacturer = field_map.get("manufacturer").and_then(map_string);
    session.product = field_map.get("product").and_then(map_string);
    session.serial_number = field_map.get("serial_number").and_then(map_string);
    session.time_created = field_map.get("time_created").and_then(map_timestamp);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Parses session information into more detail.
///
/// **Parameters:**
///
///    `fields: &[FitDataField]` -- See the fitparser crate for details: <https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html><br>
///    `session: &mut types::Session` -- An empty session struct to be filled in. See `types.rs` for details on this stuct.
///
/// **Returns:**
///
///    Nothing. The data is put into the `session` struct.
fn parse_session(fields: &[FitDataField], session: &mut types::Session) {
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();
    log::trace!("Session field_map = {:?}", field_map);

    session.activity_type = field_map.get("sport").and_then(map_string);
    session.activity_detailed = field_map.get("sub_sport").and_then(map_string);

    session.cadence_avg = field_map.get("avg_cadence").and_then(map_uint8);
    session.cadence_max = field_map.get("max_cadence").and_then(map_uint8);

    session.heartrate_avg = field_map.get("avg_heart_rate").and_then(map_uint8);
    session.heartrate_max = field_map.get("max_heart_rate").and_then(map_uint8);
    session.heartrate_min = field_map.get("min_heart_rate").and_then(map_uint8);

    session.stance_time_avg = field_map.get("avg_stance_time").and_then(map_float64);
    session.vertical_oscillation_avg = field_map
        .get("avg_vertical_oscillation")
        .and_then(map_float64);

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
    session.power_threshold = field_map.get("threshold_power").and_then(map_uint16);

    // GPS - NEC = North East Corner, SWC = South West Corner
    session.nec_lat = field_map
        .get("nec_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * LATLON_MULTIPLIER);
    session.nec_lon = field_map
        .get("nec_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * LATLON_MULTIPLIER);
    session.swc_lat = field_map
        .get("swc_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * LATLON_MULTIPLIER);
    session.swc_lon = field_map
        .get("swc_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * LATLON_MULTIPLIER);

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
        .map(types::Duration::from_secs_f64);
    session.duration_active = field_map
        .get("total_timer_time")
        .and_then(map_float64)
        .map(types::Duration::from_secs_f64);
    session.duration_moving = field_map
        .get("total_moving_time")
        .and_then(map_float64)
        .map(types::Duration::from_secs_f64);

    session.start_time = field_map.get("start_time").and_then(map_timestamp);
    session.finish_time = field_map.get("timestamp").and_then(map_timestamp);

    session.num_laps = field_map.get("num_laps").and_then(map_uint16);

    let tihz = field_map.get("time_in_hr_zone");
    session.time_in_hr_zones = parse_hr_zones(tihz);
    log::trace!("session.time_in_hr_zones = {:?}", session.time_in_hr_zones);
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
///    Nothing. The data is put into the `lap` struct.
fn parse_lap(fields: &[FitDataField], lap: &mut types::Lap, session: &types::Session) {
    // Collect the fields into a HashMap which we can then dig details out of.
    // x.name is the key and x.value is the value
    // Note that the value is an enum and contain a number of different types
    // See the fitparser crate for details
    lap.filename = session.filename.to_owned();
    log::trace!("Lap filename = {:?}", lap.filename);

    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();
    log::trace!("Lap field_map = {:?}", field_map);

    lap.cadence_avg = field_map.get("avg_cadence").and_then(map_uint8);
    lap.cadence_max = field_map.get("max_cadence").and_then(map_uint8);

    lap.heartrate_min = field_map.get("min_heart_rate").and_then(map_uint8);
    lap.heartrate_avg = field_map.get("avg_heart_rate").and_then(map_uint8);
    lap.heartrate_max = field_map.get("max_heart_rate").and_then(map_uint8);

    lap.stance_time_avg = field_map
        .get("avg_stance_time")
        .and_then(map_float64)
        .map(types::Duration::from_secs_f64);
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
        .map(types::Duration::from_secs_f64);

    lap.start_time = field_map.get("start_time").and_then(map_timestamp);
    lap.finish_time = field_map.get("timestamp").and_then(map_timestamp);

    let tihz = field_map.get("time_in_hr_zone");
    lap.time_in_hr_zones = parse_hr_zones(tihz);
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
///    Nothing. The data is put into the `record` struct.
fn parse_record(fields: &[FitDataField], record: &mut types::Record, session: &types::Session) {
    // Collect the fields into a HashMap which we can then dig details out of.
    // x.name is the key and x.value is the value
    // Note that the value is an enum and contain a number of different types
    // See the fitparser crate for details

    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();
    log::trace!("Record field map = {:?}", field_map);

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
        .map(types::Duration::from_secs_f64);
    record.vertical_oscillation = field_map.get("vertical_oscillation").and_then(map_float64);

    record.lat = field_map
        .get("position_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * LATLON_MULTIPLIER);

    record.lon = field_map
        .get("position_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * LATLON_MULTIPLIER);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Parses heart rate zone information into more detail.
///
/// **Parameters:**
///
///    `time_in_hr_zone: &Value` -- A fitparser array value containing the HR Zone information: <https://docs.rs/fitparser/0.4.0/fitparser/enum.Value.html>
///
/// **Returns:**
///
///   `HrZones` -- Returns an HrZones struct, which may or may not have values in its elements.
///
/// **Example:**
///
///   ```rust
///   let tihz = field_map.get("time_in_hr_zone");
///   lap.time_in_hr_zones = parse_hr_zones(tihz);
///   ```
///
fn parse_hr_zones(time_in_hr_zone: Option<&&Value>) -> HrZones {
    let mut hr_zones = HrZones::new();
    log::debug!("time_in_hr_zone = {:?}", time_in_hr_zone);

    match time_in_hr_zone {
        Some(Value::Array(thiz_vec)) => {
            log::trace!("parse_hr_zones: Found Array(UInt32): {:?}", thiz_vec);

            // Array[UInt32(23372), UInt32(31681), UInt32(32669), UInt32(447453), UInt32(1394934)]
            let t2: Vec<Duration> = thiz_vec
                .iter()
                .map(|x| x.to_string().parse::<u64>().unwrap())
                .map(Duration::from_millis_u64)
                .collect();

            log::debug!("tihz = {:?}", t2);

            hr_zones.hr_zone_0 = Some(t2[0]);
            hr_zones.hr_zone_1 = Some(t2[1]);
            hr_zones.hr_zone_2 = Some(t2[2]);
            hr_zones.hr_zone_3 = Some(t2[3]);
            hr_zones.hr_zone_4 = Some(t2[4]);
        }
        _ => {
            log::trace!("parse_hr_zones: Empty or None. Using default.");
        }
    }

    // return it
    log::debug!("hr_zones = {:?}", hr_zones);
    hr_zones
}
