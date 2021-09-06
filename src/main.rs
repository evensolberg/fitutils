/*
    This is very much a work in progress. I expect eventually
    a lot of code will get broken out into separate modules.
*/

// See Cargo.toml for crates versions
// Crates Usage:

// Command line
use clap::{Arg, App};

// .FIT file manipulation
use fitparser::{profile::field_types::MesgNum, FitDataField, Value};
// use fitparser;

// Files
use std::error::Error;
use std::fs::File;

// Logging
use log::LevelFilter;
use simple_logger::SimpleLogger;

// mod activity;
// mod types;

/**
 * Local structs used to contain the various bits and pieces of information extracted from the header.
 * This includes such things as device manufacturer, activity, etc.
 *
 * This will then be put into each line in the resulting CSV, so that each line essentially is self-contained.
 */

/// TODO Create a Display trait for this
struct FitHeading {
    manufacturer: String,
    time_created: String,
    num_sessions: u32,
    num_laps: u32,
    num_records: u32,
}


/// This is where the magic happens
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new("fit2csv")
                        .version("0.0.1")
                        .author("Even Solberg <even.solberg@gmail.com>")
                        .about("Provided with no guarantees or warranties whatsoever.")
                        .arg(Arg::with_name("read")
                            .short("r")
                            .long("read")
                            .value_name("FILE")
                            .help("Read a file and display the contents")
                            .takes_value(true))
                        .arg(Arg::with_name("debug")
                            .short("d")
                            .long("debug")
                            .multiple(true)
                            .help("Output debug information as we go. Supply it twice for trace-level logs")
                            .takes_value(false))
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
    log::debug!("File pointer name: {:?}", fp);

    // Read and parse the file contents
    log::trace!("Reading data");
    let file = match fitparser::from_reader(&mut fp) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };

    // log::trace!("data = {:?}", data);
    log::debug!("Number of records: {}", file.len());

    // There HAS to be a better way to do this!
    log::trace!("Data read. Extracting header.");
    let header = &file[0];
    log::trace!("Header extracted.");

    // print the data in the file header
    println!("Header kind: {:?}", header.kind());
    let header_fields = &header.fields();
    println!("Header fields:\n");

    // Try the other way to extract a header
    // let parsed = parser::parse(fitfile_name);

    // Unpack each field in the FileID header
    for n in 0..header_fields.len() {
        let field = &header_fields[n];
        println!("Name:   {}", field.name());
        println!("Number: {}", field.number());
        println!("Value:  {}", field.value());
        println!("Units:  {}\n", field.units());
    }

    log::trace!("Extract manufacturer.");
    let manufacturer = header_fields[1].value().to_string();
    log::trace!("Extract time_created.");
    let time_created = header_fields[3].value().to_string();

    log::trace!("Trying to assign the header_struct.");
    let mut header_struct = FitHeading {
        manufacturer,
        time_created,
        num_sessions: 0,
        num_laps: 0,
        num_records: 0,
    };
    log::trace!("Printing the header_struct.");
    println!("Manufacturer: {}", header_struct.manufacturer);
    println!("Time created: {}", header_struct.time_created);

    // let mut num_sessions = 0;
    // let mut num_records = 0;
    // let mut num_laps = 0;

    log::debug!("Parsing data.");
    for data in file {
        // for each FitDataRecord
        match data.kind() {
            // Figure out what kind it is and count accordingly
            MesgNum::Session => header_struct.num_sessions += 1,
            MesgNum::Record => header_struct.num_records += 1,
            MesgNum::Lap => header_struct.num_laps += 1,
            _ => (),
        }
    }

    log::trace!("Printing summary information.");
    println!("\nThis file contains {} sessions", header_struct.num_sessions);
    println!("This file contains {} laps", header_struct.num_laps);
    println!("This file contains {} records", header_struct.num_records);

    // Everything is a-okay in the end
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory
        Err(err) => {
            // Houston, This file contains a problem
            eprintln!("{}", Box::new(err)); // Say what's wrong and
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
