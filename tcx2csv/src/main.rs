use env_logger::Target;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use utilities::{TCXActivitiesList, TCXActivity, TCXTrackpointList};

mod cli;

/// This is where the actual processing takes place.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build();

    // Initialize logging
    let mut logbuilder = utilities::build_log(&cli_args);
    logbuilder.target(Target::Stdout).init();

    for argument in cli_args.values_of("read").unwrap_or_default() {
        log::trace!("main::run() -- Arguments: {argument:?}");
    }
    // Find the name of the session output file
    let summaryfile = cli_args
        .value_of("summary-file")
        .unwrap_or("tcx-activities.csv");
    log::trace!("main::run() -- session output file: {summaryfile}");

    // Let the user know if we're writing
    if cli_args.is_present("detail-off") {
        log::debug!("Writing summary file {} only.", &summaryfile);
    } else {
        log::debug!("Writing summary and detail files.");
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Working section
    // Do the parsing

    let mut act_list = TCXActivitiesList::new();

    for filename in cli_args.values_of("read").unwrap_or_default() {
        log::info!("Processing file: {filename}");

        let mut tcdb = tcx::read(&mut BufReader::new(File::open(filename)?))?;
        tcdb.calc_heartrates();

        // If -d then export the activity to JSON
        if cli_args.is_present("debug") {
            let outfile = utilities::set_extension(filename, "json")
                .as_str()
                .to_owned();
            log::trace!("main::run() -- Exporting {filename} to {outfile} for debugging purposes.");
            tcdb.export_json(&outfile)?;
        }

        log::trace!("main::run() -- tcxfile = {tcdb:?}");
        if let Some(activities) = tcdb.activities {
            let mut curr_activities = TCXActivity::from_activities(&activities);
            let file_name = filename.to_string();
            curr_activities.filename = Some(file_name.clone());

            log::trace!("main::run() -- activities summary: {curr_activities:?}");
            if !cli_args.is_present("detail-off") {
                // Export the activity summary to JSON
                log::debug!("main::run() -- Writing activity summary for {file_name}");
                curr_activities.export_json()?;

                // Export the Trackpoints to CSV
                log::debug!("Parsing and exporting Trackpoint list.");
                let tp_list = TCXTrackpointList::from_activities(&activities);
                tp_list.export_csv(&utilities::set_extension(filename, "trackpoints.csv"))?;
            }

            act_list.activities.push(curr_activities);
        }
    }

    // If we're tracing, export the summary in JSON format
    if cli_args.occurrences_of("debug") > 1 {
        log::trace!("main::run() -- Exporting summary JSON file.");
        act_list.export_json(&utilities::set_extension(summaryfile, "json"))?;
    }

    log::info!("Exporting summary CSV file: {summaryfile}");
    act_list.export_csv(summaryfile)?;

    // Everything is a-okay in the end
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory
        Err(err) => {
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
