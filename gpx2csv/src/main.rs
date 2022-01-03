//! The main program file.
use clap::{App, Arg};
use std::error::Error; // Command line

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

// Local modules
pub mod types;
use crate::types::{Activities, Activity};

/// This is where the actual processing takes place.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        // .author(clap::crate_authors!("\n"))
        .long_about("This program will read one or more .gpx file and output session information to a .json file, the lap information (if any is found) to a .laps.csv file, and the individual records to a .records.csv file. Additionally, a summary sessions.csv file will be produced.")
        .arg(
            Arg::new("read")
                .value_name("FILE(S)")
                .help("One or more .gpx file(s) to process. Wildcards and multiple_occurrences files (e.g. 2019*.gpx 2020*.gpx) are supported.")
                .takes_value(true)
                .required(true)
                .multiple_occurrences(true),
        )
        .arg( // Hidden debug parameter
            Arg::new("debug")
                .short('d')
                .long("debug")
                .multiple_occurrences(true)
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .takes_value(false)
                .hide(true),
        )
        .arg( // Don't print any information
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .multiple_occurrences(false)
                .help("Don't produce any output except errors while working.")
                .takes_value(false)
        )
        .arg( // Don't export detail information
            Arg::new("detail-off")
                .short('o')
                .long("detail-off")
                .multiple_occurrences(false)
                .help("Don't export detailed information from each file parsed.")
                .takes_value(false)
        )
        .arg( // Summary file name
            Arg::new("summary-file")
                .short('s')
                .value_name("summary output file name")
                .long("summary-file")
                .multiple_occurrences(false)
                .help("Summary output file name.")
                .takes_value(true)
        )
        .get_matches();

    // If the user specifies that they don't want detail and the summary detail is off, nothing gets written.
    // This kinda defeats the purpose, so we let the user know.
    if cli_args.is_present("detail-off") && !cli_args.is_present("summary-file") {
        return Err("Supplied --detail-off parameter with no --summary-file parameter means there is nothing to write. Exiting.".into());
    }

    // create a log builder
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    if cli_args.is_present("quiet") {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.occurrences_of("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    for argument in cli_args.values_of("read").unwrap() {
        log::trace!("main::run() -- Arguments: {:?}", argument);
    }

    // Find the name of the session output file
    let sessionfile = cli_args
        .value_of("summary-only")
        .unwrap_or("fit-sessions.csv");
    log::trace!("main::run() -- session output file: {}", sessionfile);

    // Let the user know if we're writing
    if !cli_args.is_present("detail-off") {
        log::info!("Writing detail files.");
    } else {
        log::info!("Writing summary file {} only.", &sessionfile)
    }

    ///////////////////////////////////
    // Working section

    // Create an empty placeholder for all the activities
    let mut activities = Activities::new();

    // Do the parsing
    for filename in cli_args.values_of("read").unwrap() {
        log::info!("Processing file: {}", filename);

        // Extract the activity from the file
        let activity = Activity::from_file(filename)?;

        // Export the data if requested
        if !cli_args.is_present("detail-off") {
            activity.export()?; // metadata, tracks, waypoints
        }

        // Add the current activity to the list of activities and destroy the activity
        activities.activities_list.push(activity);
    }

    // Export the summary list of activities
    if cli_args.is_present("summary-file") {
        log::info!("Summary information written to: {}", &sessionfile);
        activities.export_csv(sessionfile)?;
    }

    // Everything is a-okay in the end
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory
        Err(err) => {
            Builder::new()
                .filter_level(LevelFilter::Error)
                .target(Target::Stdout)
                .init();
            log::error!("{}", err.to_string().replace("\"", ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
