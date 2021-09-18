/*
    This is very much a work in progress. I expect eventually
    a lot of code will get broken out into separate modules.
*/

// See Cargo.toml for crates versions
// Crates Usage:

use clap::{App, Arg}; // Command line
use fitparser::profile::field_types::MesgNum; // .FIT file manipulation

// use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;

// Logging
use log::LevelFilter;
use simple_logger::SimpleLogger;

// Import our own modules and types
pub mod exporters;
pub mod parsers;
pub mod print_details;
pub mod types;
use crate::types::*;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new("fit2csv")
        .about("Parses .FIT files to .JSON and .CSV")
        .long_about("This program will read a .fit file and output session information to a .json file, the lap information (if any is found) to a .laps.csv file, and the individual records to a .csv file.")
        .arg(
            Arg::with_name("read")
                .value_name("FILE")
                .help("Read a file and display the contents")
                .takes_value(true),
        )
        .arg( // Hidden debug parameter
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .multiple(true)
                .help("Output debug information as we go. Supply it twice for trace-level logs")
                .takes_value(false)
                .hidden(true),
        )
        .arg( // Don't print any information
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .multiple(false)
                .help("Don't output any summary information about the file processed.")
                .takes_value(false)
        )
        .get_matches();

    let log_level = cli_args.occurrences_of("debug"); // Will pass this to functions in the future.

    // Set up logging according to the number of times the debug flag has been supplied
    match log_level {
        0 => SimpleLogger::new().with_level(LevelFilter::Info).init()?,
        1 => SimpleLogger::new().with_level(LevelFilter::Debug).init()?,
        _ => SimpleLogger::new().with_level(LevelFilter::Trace).init()?, // More than 1
    }

    // Get the input file name - use the dummy if nothing was supplied
    let fitfile_name = cli_args.value_of("read").unwrap_or("data/test.fit");
    log::debug!("Input file: {}", fitfile_name);
    log::debug!(
        "Parsing FIT files using Profile version: {}",
        fitparser::profile::VERSION
    );

    // open the file - return error if unable.
    let mut fp = File::open(fitfile_name)?;
    log::trace!("{} was read OK. File pointer name: {:?}", fitfile_name, fp);

    // Read and parse the file contents
    log::trace!("Reading data");
    let file = fitparser::from_reader(&mut fp)?;
    log::debug!("Data was read. Total number of records: {}", file.len());

    log::trace!("Data read. Extracting header.");
    let header = &file[0]; // There HAS to be a better way to do this!
    log::debug!("Header: {:?}", header);

    log::trace!("Creating empty session.");
    let mut my_session = types::Session::new();

    log::trace!("Extracting manufacturer and session creation time.");
    parsers::parse_header(&fitfile_name, header, &mut my_session);

    // This is the main file parsing loop. This will definitely get expanded.
    log::trace!("Initializing temporary variables.");
    let mut num_records = 0;
    let mut num_sessions = 0;
    let mut num_laps = 0;
    let mut lap_vec: Vec<Lap> = Vec::new(); // Lap information vector
    let mut records_vec: Vec<Record> = Vec::new();

    // This is where the actual parsing happens
    log::debug!("Parsing data.");
    for data in file {
        // for each FitDataRecord
        match data.kind() {
            // Figure out what kind it is and count accordingly
            MesgNum::Session => {
                parsers::parse_session(data.fields(), &mut my_session);
                log::debug!("Session: {:?}", my_session);
                num_sessions += 1;
                my_session.num_sessions = Some(num_sessions);
            }
            MesgNum::Lap => {
                let mut lap = Lap::default(); // Create an empty lap instance
                parsers::parse_lap(data.fields(), &mut lap, &my_session); // parse lap data
                num_laps += 1;
                lap.lap_num = Some(num_laps);
                log::debug!("Lap {:3}: {:?}", num_laps, lap);
                lap_vec.push(lap); // push the lap onto the vector
            }
            MesgNum::Record => {
                // FIXME: This is very inefficient since we're instantiating this for every record
                let mut record = Record::default();
                parsers::parse_record(data.fields(), &mut record, &my_session);
                log::debug!("Record: {:?}", record);
                records_vec.push(record);
                num_records += 1;
                my_session.num_records = Some(num_records);
            }
            _ => (),
        } // match
    } // for data

    log::trace!("Printing the header_struct if not quiet.");
    if !cli_args.is_present("quiet") {
        print_details::print_session(&my_session);
    }

    let serialized_session = serde_json::to_string(&my_session).unwrap();
    log::trace!("serialized_session session: {}", serialized_session);

    // Write the data
    exporters::export_session_json(&my_session, &fitfile_name)?;
    exporters::export_laps_csv(&lap_vec, &fitfile_name)?;
    exporters::export_records_csv(&records_vec, &fitfile_name)?;

    // Everything is a-okay in the end
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
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
