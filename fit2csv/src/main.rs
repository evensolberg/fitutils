/*
    This is very much a work in progress. I expect eventually
    a lot of code will get broken out into separate modules.
*/

// See Cargo.toml for crates versions
// Crates Usage:

use clap::{App, Arg}; // Command line

// use csv::WriterBuilder;
use std::error::Error;

// Logging
use log::LevelFilter;
use simple_logger::SimpleLogger;

// Import our own modules and types
pub mod exporters;
pub mod parsers;
pub mod print_details;
pub mod types;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new("fit2csv")
        .about("Parses .FIT files to .JSON and .CSV")
        .version("0.2.5")
        .long_about("This program will read a .fit file and output session information to a .json file, the lap information (if any is found) to a .laps.csv file, and the individual records to a .records.csv file. Additionally, a summary sessions.csv file will be produced.")
        .arg(
            Arg::with_name("read")
                .value_name("FILE(S)")
                .help("One or more .fit file(s) to process. Wildcards and multiple files (e.g. 2019*.fit 2020*.fit) are supported.")
                .takes_value(true)
                .multiple(true),
        )
        .arg( // Hidden debug parameter
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .multiple(true)
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .takes_value(false)
                .hidden(true),
        )
        .arg( // Don't print any information
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .multiple(false)
                .help("Don't produce any output except errors while working.")
                .takes_value(false)
        )
        .arg( // Print summary information
            Arg::with_name("print-summary")
                .short("s")
                .long("print-summary")
                .multiple(false)
                .help("Output summary detail for each session processed.")
                .takes_value(false)
        )
        .arg( // Don't print any information
            Arg::with_name("summary-only")
                .short("o")
                .long("summary-only")
                .multiple(false)
                .help("Don't output detail files for each session processed. Only create the summary file sessions.csv")
                .takes_value(false)
        )
        .get_matches();

    // Set up logging according to the number of times the debug flag has been supplied
    let log_level = cli_args.occurrences_of("debug"); // Will pass this to functions in the future.
    match log_level {
        0 => SimpleLogger::new().with_level(LevelFilter::Info).init()?,
        1 => SimpleLogger::new().with_level(LevelFilter::Debug).init()?,
        _ => SimpleLogger::new().with_level(LevelFilter::Trace).init()?, // More than 1
    }

    if !cli_args.is_present("read") {
        eprintln!("Missing file argument.\n{}", cli_args.usage());
        std::process::exit(1);
    } else {
        log::trace!("File argument: {:?}", cli_args.values_of("read").unwrap());
    }

    let fitfiles = cli_args.values_of("read").unwrap();
    log::debug!("Input files: {:?}", fitfiles);
    log::trace!(
        "Parsing FIT files using Profile version: {}",
        fitparser::profile::VERSION
    );

    ///////////////////////////////////
    // Working section

    let mut session_vec = Vec::new();

    for fitfile_name in fitfiles {
        // If not quiet, indicate which file we're processing
        if !cli_args.is_present("quiet") {
            log::info!("Processing: {}", fitfile_name);
        }

        // Parse the FIT file
        let my_activity = parsers::parse_fitfile(fitfile_name)?;

        // If requested, print the summary information for the session
        log::trace!("Printing the header_struct if requested.");
        if cli_args.is_present("print-summary") {
            print_details::print_session(&my_activity.session);
        }

        // Push the session onto the summary vector
        let my_session = my_activity.session.clone();
        session_vec.push(my_session);
        log::debug!("Session vector length: {}", session_vec.len());

        // Write the data
        if !cli_args.is_present("summary-only") {
            exporters::export_activity(&my_activity)?;
        }
        exporters::export_sessions_csv(&session_vec)?;
    }

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
