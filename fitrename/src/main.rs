/*
    This is very much a work in progress. I expect eventually
    a lot of code will get broken out into separate modules.
*/

// See Cargo.toml for crates versions
// Crates Usage:

use clap::{Arg, Command}; // Command line

// use std::io::Write; // needed for the log formatting
use std::{collections::HashMap, error::Error, path::Path};

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

mod fit;
mod gpxx;
mod shared;
mod tcxx;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = Command::new(clap::crate_name!())
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

    logbuilder.filter_module("serde_xml_rs::de", LevelFilter::Warn);

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
    let mut total_files: usize = 0;
    let mut prcoessed_files: usize = 0;
    let mut skipped_files: usize = 0;

    ///////////////////////////////////
    // Working section
    for filename in cli_args.values_of("read").unwrap() {
        log::debug!("Processing file: {}", filename);

        // Check if the target file exists, otherwise just continue
        if !Path::new(&filename).exists() {
            log::warn!("No such file or directory: {}", filename);
            continue;
        }

        // Read the metadata from files
        let mut value_res = Ok(HashMap::<String, String>::new());
        match utilities::get_extension(filename).as_ref() {
            "fit" => {
                value_res = fit::process_fit(filename);
                log::debug!("FIT: {:?}", value_res);
            }
            "gpx" => {
                value_res = gpxx::process_gpx(filename);
                log::debug!("GPX: {:?}", value_res);
            }
            "tcx" => {
                value_res = tcxx::process_tcx(filename);
                log::debug!("TCX {:?}", value_res);
            }
            _ => log::warn!("Unknown file type: {}.", &filename),
        }

        // Process the result of reading metadata
        match value_res {
            // Metadata read OK - try to rename
            Ok(values) => {
                let result = shared::rename_file(filename, pattern, &values, total_files, dry_run);
                match result {
                    // How did the rename go?
                    Ok(result) => {
                        log::info!("{} --> {}", filename, result);
                        prcoessed_files += 1;
                    }
                    Err(err) => {
                        log::error!("Unable to rename {} : {}", filename, err.to_string());
                        skipped_files += 1;
                    }
                }
            }
            // Problem reading metadata - let the user know.
            Err(err) => log::error!("Unable to process {} : {}", filename, err.to_string()),
        }
        total_files += 1;
    }

    if cli_args.is_present("print-summary") {
        log::info!("Total files examined:        {:6}", total_files);
        log::info!("Files processed:             {:6}", prcoessed_files);
        log::info!("Files skipped due to errors: {:6}", skipped_files);
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
            log::error!("{}", err.to_string().replace("\"", ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
