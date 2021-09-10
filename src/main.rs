/*
    This is very much a work in progress. I expect eventually
    a lot of code will get broken out into separate modules.
*/

// See Cargo.toml for crates versions
// Crates Usage:

use clap::{App, Arg}; // Command line
use fitparser::{profile::field_types::MesgNum, FitDataField, Value}; // .FIT file manipulation

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

// use std::time::Duration;
// use chrono::{DateTime, Local};
use uom::si::{
    f64::{Length as Length_f64, Velocity},
    // length::{foot, kilometer, meter, mile},
    length::meter,
    u16::Length as Length_u16,
    // velocity::{foot_per_second, kilometer_per_hour, meter_per_second, mile_per_hour},
    velocity::meter_per_second,
};

// Logging
use log::LevelFilter;
use simple_logger::SimpleLogger;

// Import all the types
pub mod types;
use crate::types::TimeStamp;

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

/**
 * Functions that make things work. This will no doubt get moved out to a module eventually.
 * As it should.
 */


fn parse_session(fields: &[FitDataField], session: &mut types::Session) {
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();

    log::debug!("field_map = {:?}", field_map);

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

    session.laps = field_map.get("num_laps").and_then(map_uint16);
    log::trace!("laps = {:?}", session.laps);

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

    // TODO: Decode the time in HR zones
    // REF: https://docs.rs/fitparser/0.4.2/fitparser/enum.Value.html
    // TODO: Figure out how to turn Some(Array([UInt32])) into Some(Vec<Duration>))
    // Maybe we can iterate_into and_then(matp)
    // This is actually a Option(Value(Array(Value(UInt32)))) which you can unwrap into Array(Value(UInt32))
    let time_in_hr_zones = field_map.get("time_in_hr_zone");
    log::debug!("time_in_hr_zones = {:?}", time_in_hr_zones);
}



/// This is where the magic happens
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new("fit2csv")
        .version("0.0.1")
        .author("Even Solberg <even.solberg@gmail.com>")
        .about("Provided with no guarantees or warranties whatsoever.")
        .arg(
            Arg::with_name("read")
                .value_name("FILE")
                .help("Read a file and display the contents")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .multiple(true)
                .help("Output debug information as we go. Supply it twice for trace-level logs")
                .takes_value(false)
                .hidden(true),
        )
        .get_matches();

    let log_level = cli_args.occurrences_of("debug"); // Will pass this to functions in the future.

    // Set up logging according to the number of times the debug flag has been supplied
    match log_level {
        0 => SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init()
            .unwrap(),
        1 => SimpleLogger::new()
            .with_level(LevelFilter::Debug)
            .init()
            .unwrap(),
        _ => SimpleLogger::new()
            .with_level(LevelFilter::Trace)
            .init()
            .unwrap(), // More than 1
    }

    // Get the input file name - use the dummy if nothing was supplied
    let fitfile_name = cli_args.value_of("read").unwrap_or("./data/test.fit");
    log::debug!("Input file: {}", fitfile_name);
    log::debug!(
        "Parsing FIT files using Profile version: {}",
        fitparser::profile::VERSION
    );

    // open the file - return error if unable.
    let mut fp = match File::open(fitfile_name) {
        Ok(fp) => fp,
        Err(e) => return Err(Box::new(e)),
    };
    log::debug!("{} was read OK. File pointer name: {:?}", fitfile_name, fp);

    // Read and parse the file contents
    log::trace!("Reading data");
    let file = match fitparser::from_reader(&mut fp) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };
    log::debug!("Data was read. Parsing.");

    // log::trace!("data = {:?}", data);
    log::debug!("Number of records: {}", file.len());

    // There HAS to be a better way to do this!
    log::trace!("Data read. Extracting header.");
    let header = &file[0];
    log::trace!("Header extracted.");

    // print the data in the file header
    println!("Header kind: {:?}", header.kind());

    // Try the other way to extract a header
    // let parsed = parser::parse(fitfile_name);

    log::trace!("Creating empty session.");
    let mut my_session = types::Session::new();

    log::trace!("Extract manufacturer.");
    my_session.manufacturer = header.fields()[1].value().to_string();
    log::trace!("Extract time_created.");
    my_session.time_created = map_timestamp(&&header.fields()[3].value())
        .expect("Unable to extract session creation time.");

    // This is the main file parsing loop. This will definitely get expanded.
    log::debug!("Parsing data.");
    for data in file {
        // for each FitDataRecord
        match data.kind() {
            // Figure out what kind it is and count accordingly
            MesgNum::Session => {
                parse_session(data.fields(), &mut my_session);
                my_session.num_sessions += 1;
            }
            MesgNum::Lap => my_session.num_laps += 1,
            MesgNum::Record => my_session.num_records += 1,
            _ => (),
        }
    }

    log::trace!("Printing the header_struct.");
    println!("\nFile header:");
    println!("Manufacturer: {}", my_session.manufacturer);
    println!("Time created: {}", my_session.time_created);
    println!("Sessions:     {:5}", my_session.num_sessions);
    println!("Laps:         {:5}", my_session.num_laps);
    println!("Records:      {:5}", my_session.num_records);

    // Everything is a-okay in the end
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory
        Err(err) => {
            // Houston, This file contains a problem
            log::error!("{}", Box::new(err)); // Say what's wrong and
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
