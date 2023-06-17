//! The main program file.
use clap::parser::ValueSource;
use env_logger::Target;
use std::error::Error; // Command line

mod cli;

/// This is where the actual processing takes place.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build();

    // Initialize logging
    let mut logbuilder = utilities::build_log(&cli_args);
    logbuilder.target(Target::Stdout).init();

    let filenames = cli_args
        .get_many::<String>("read")
        .unwrap_or_default()
        .map(std::string::String::as_str);

    log::trace!("main::run() -- Files to be read: {filenames:?}");

    // Find the name of the session output file
    let session_file_name = String::from("fit-sessions.csv");
    let sessionfile = cli_args
        .get_one::<String>("summary-file")
        .unwrap_or(&session_file_name)
        .as_str();

    // Let the user know if we're writing details
    if cli_args.value_source("detail-off") == Some(ValueSource::CommandLine) {
        log::info!("Writing summary file {sessionfile} only.");
    } else {
        log::info!("Writing summary file {sessionfile} and details.");
    }

    ///////////////////////////////////
    // Working section

    // Create an empty placeholder for all the activities
    let mut activities = utilities::GPXActivities::new();

    // Do the parsing
    for filename in filenames {
        log::info!("Processing file: {filename}");

        // Extract the activity from the file
        let activity = utilities::GPXActivity::from_file(filename)?;

        // Export the data if requested
        if cli_args.value_source("detail-off") != Some(ValueSource::CommandLine) {
            activity.export()?; // metadata, tracks, waypoints
        }

        // Add the current activity to the list of activities and destroy the activity
        activities.activities_list.push(activity);
    }

    // Export the summary list of activities
    if cli_args.value_source("summary-file") == Some(ValueSource::CommandLine) {
        log::info!("Summary information written to: {sessionfile}");
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
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
