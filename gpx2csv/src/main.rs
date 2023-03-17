//! The main program file.
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

    for argument in cli_args.values_of("read").unwrap_or_default() {
        log::trace!("main::run() -- Arguments: {:?}", argument);
    }

    // Find the name of the session output file
    let sessionfile = cli_args
        .value_of("summary-only")
        .unwrap_or("fit-sessions.csv");
    log::trace!("main::run() -- session output file: {}", sessionfile);

    // Let the user know if we're writing
    if cli_args.is_present("detail-off") {
        log::info!("Writing summary file {} only.", &sessionfile);
    } else {
        log::info!("Writing detail files.");
    }

    ///////////////////////////////////
    // Working section

    // Create an empty placeholder for all the activities
    let mut activities = utilities::GPXActivities::new();

    // Do the parsing
    for filename in cli_args.values_of("read").unwrap_or_default() {
        log::info!("Processing file: {}", filename);

        // Extract the activity from the file
        let activity = utilities::GPXActivity::from_file(filename)?;

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
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
