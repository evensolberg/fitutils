use clap::{App, Arg}; // Command line
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use tcx;

pub mod types;
use crate::types::set_extension;

/// This is where the actual processing takes place.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        // .author(clap::crate_authors!("\n"))
        .long_about("This program will read one or more .tcx file and output session information to a .json file, the lap information (if any is found) to a .laps.csv file, and the individual records to a .records.csv file. Additionally, a summary sessions.csv file will be produced.")
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
    let summaryfile = cli_args
        .value_of("summary-file")
        .unwrap_or("tcx-activities.csv");
    log::trace!("main::run() -- session output file: {}", summaryfile);

    // Let the user know if we're writing
    if !cli_args.is_present("detail-off") {
        log::debug!("Writing detail files.");
    } else {
        log::debug!("Writing summary file {} only.", &summaryfile)
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Working section
    // Do the parsing

    let mut act_list = types::ActivitiesList::new();

    for filename in cli_args.values_of("read").unwrap() {
        log::info!("Processing file: {}", filename);

        let mut tcdb = tcx::read(&mut BufReader::new(File::open(&filename).unwrap()))?;
        tcdb.calc_heartrates();

        // If -d then export the activity to JSON
        if cli_args.is_present("debug") {
            let outfile = set_extension(&filename, "json").as_str().to_owned();
            log::trace!(
                "main::run() -- Exporting {} to {} for debugging purposes.",
                filename,
                outfile
            );
            tcdb.export_json(&outfile)?;
        }

        log::trace!("main::run() -- tcxfile = {:?}", tcdb);
        if let Some(activities) = tcdb.activities {
            let mut curr_activities = types::ActivitiesSummary::from_activities(&activities);
            curr_activities.filename = filename.to_string();

            log::trace!("main::run() -- activities summary: {:?}", curr_activities);
            if !cli_args.is_present("detail-off") {
                // Export the activity summary to JSON
                log::debug!(
                    "main::run() -- Writing activity summary for {}",
                    &curr_activities.filename
                );
                curr_activities.export_json()?;
            }

            act_list.activities.push(curr_activities);
        }
    }

    // If we're tracing, export the summary in JSON format
    if cli_args.occurrences_of("debug") > 1 {
        log::trace!("main::run() -- Exporting summary JSON file.");
        act_list.export_json(&set_extension(&summaryfile, "json"))?;
    }

    log::debug!("main::run() -- Exporting summary CSV file.");
    act_list.export_csv(&summaryfile)?;

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
