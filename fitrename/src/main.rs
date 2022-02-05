/*
    This is very much a work in progress. I expect eventually
    a lot of code will get broken out into separate modules.
*/

// See Cargo.toml for crates versions
// Crates Usage:

use clap::{App, Arg}; // Command line

// use std::io::Write; // needed for the log formatting
use std::{collections::HashMap, error::Error};

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

mod fit;
mod shared;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        // .author(clap::crate_authors!("\n"))
        .long_about("This program will rename FIT, GPX and TCX files based on metadata about the activity in the file and a pattern provided for the file name.")
        .arg(
            Arg::new("read")
                .value_name("FILE(S)")
                .help("One or more .fit, .gpx or .tcx file(s) to process. Wildcards and multiple_occurrences files (e.g. 2019*.fit 2020*.gpx) are supported.")
                .takes_value(true)
                .required(true)
                .multiple_occurrences(true),
        )
        .arg( // Rename pattern}
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .help("The pattern for new file names.")
                .multiple_occurrences(false)
                .takes_value(true)
                .required(true)
                .hide(false),
        )
        .arg( // Hidden debug parameter
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .multiple_occurrences(true)
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
        .arg( // Print summary information
            Arg::new("print-summary")
                .short('s')
                .long("print-summary")
                .help("Print a summary of the number of files processed, errors, etc.")
                .multiple_occurrences(false)
                .takes_value(false)
        )
        .arg( // Dry-run
            Arg::new("dry-run")
                .short('r')
                .long("dry-run")
                .help("Perform a dry-run. This will output what the result will be without performing the actual rename operation.")
                .multiple_occurrences(false)
                .takes_value(false)
        )
        .get_matches();

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

    // TODO: Expand on this to improve the log format.
    // logbuilder.format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()));

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    for argument in cli_args.values_of("read").unwrap() {
        log::trace!("main::run() -- Arguments: {:?}", argument);
    }

    let dry_run = cli_args.is_present("dry-run");
    if dry_run {
        log::info!("Dry-run. Will not perform actual rename.");
    }

    let pattern = cli_args.value_of("pattern").unwrap();

    ///////////////////////////////////
    // Working section
    for filename in cli_args.values_of("read").unwrap() {
        log::debug!("Processing file: {}", filename);
        let ext = shared::get_extension(&filename);

        let mut values = HashMap::new();

        match ext.as_ref() {
            "fit" => {
                values = fit::process_fit(&filename)?;
                log::debug!("FIT: {:?}", values);
            }
            "gpx" => log::debug!("GPX"),
            "tcx" => log::debug!("TCX"),
            _ => log::warn!("Unknown file type: {}.", &ext),
        }

        if let Ok(res) = shared::rename_file(filename, &pattern, &values, dry_run) {
            log::info!("{} --> {}", filename, res);
        } else {
            log::warn!("Unable to rename {} using the {}", filename, pattern);
        }
    }

    // Everything is a-okay in the end
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory - exit with code 0 (success)
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
