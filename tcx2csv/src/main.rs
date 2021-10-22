use tcx;

use std::fs::File;
use std::io::BufReader;

use std::error::Error;

use clap::{App, Arg}; // Command line

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

/// This is where the actual processing takes place.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        // .author(clap::crate_authors!("\n"))
        .long_about("This program will read one or more .gpx file and output session information to a .json file, the lap information (if any is found) to a .laps.csv file, and the individual records to a .records.csv file. Additionally, a summary sessions.csv file will be produced.")
        .arg(
            Arg::with_name("read")
                .value_name("FILE(S)")
                .help("One or more .gpx file(s) to process. Wildcards and multiple files (e.g. 2019*.gpx 2020*.gpx) are supported.")
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
        .arg( // Don't export detail information
            Arg::with_name("detail-off")
                .short("o")
                .long("detail-off")
                .multiple(false)
                .help("Don't export detailed information from each file parsed.")
                .takes_value(false)
        )
        .arg( // Summary file name
            Arg::with_name("summary-file")
                .short("s")
                .value_name("summary output file name")
                .long("summary-file")
                .multiple(false)
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
    logbuilder.filter_module("serde_xml_rs::de", LevelFilter::Off);
    logbuilder.target(Target::Stdout).init();

    // Check if we have file arguments. Exit oout if not.
    if !cli_args.is_present("read") {
        log::error!(
            "Missing file argument. Try again with -h for assistance.\n{}",
            cli_args.usage()
        );
        std::process::exit(1);
    } else {
        log::trace!(
            "main::run() -- File argument: {:?}",
            cli_args.values_of("read").unwrap()
        );
    }

    log::trace!(
        "main::run() -- File argument: {:?}",
        cli_args.values_of("read").unwrap()
    );

    // Find the name of the session output file
    let sessionfile = cli_args
        .value_of("summary-only")
        .unwrap_or("fit-sessions.csv");
    log::trace!("main::run() -- session output file: {}", sessionfile);

    // Let the user know if we're writing
    if !cli_args.is_present("detail-off") {
        log::debug!("Writing detail files.");
    } else {
        log::debug!("Writing summary file {} only.", &sessionfile)
    }

    ///////////////////////////////////
    // Working section
    // Do the parsing
    for filename in cli_args.values_of("read").unwrap() {
        log::info!("Processing file: {}", filename);

        let tcxfile = tcx::read(&mut BufReader::new(File::open(&filename).unwrap()))?;

        log::trace!("main::run() -- tcxfile = {:?}", tcxfile);
        if let Some(activities) = tcxfile.activities {
            log::debug!(
                "main::run() -- number of activities: {}",
                activities.activities.len()
            );
            for activity in activities.activities {
                log::debug!("main::run() -- number of laps: {}", activity.laps.len());
                for lap in activity.laps {
                    log::debug!("main::run() -- number of tracks: {}", lap.tracks.len());
                    for track in lap.tracks {
                        log::debug!(
                            "main::run() -- number of trackpoints: {}",
                            track.trackpoints.len()
                        );
                        for trackpoint in track.trackpoints {
                            log::trace!("main::run() -- trackpoint info: {:?}", trackpoint);
                        }
                    }
                }
            }
        }
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
