use env_logger::Target;
use std::error::Error;
use utilities::{FITActivities, FITActivity};

mod cli;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build_cli();

    // Initialize logging
    let mut logbuilder = utilities::build_log(&cli_args);
    logbuilder.target(Target::Stdout).init();

    // If tracing, output the names of the files being processed
    for argument in cli_args.values_of("read").unwrap() {
        log::trace!("main::run() -- Arguments: {:?}", argument);
    }

    // Find the name of the session output file
    let sessionfile = cli_args
        .value_of("summary-file")
        .unwrap_or("fit-sessions.csv");
    log::debug!("main::run() -- session output file: {}", sessionfile);

    // Let the user know if we're writing
    if !cli_args.is_present("detail-off") {
        log::info!("Writing detail files.");
    } else {
        log::info!("Writing summary file {} only.", &sessionfile)
    }

    ///////////////////////////////////
    // Working section

    // Create an empty placeholder for all the activities
    let mut activities = FITActivities::new();

    for filename in cli_args.values_of("read").unwrap() {
        log::info!("Processing file: {}", filename);

        // Parse the FIT file
        let activity = FITActivity::from_file(filename)?;

        // Output the files
        if cli_args.is_present("print-summary") {
            activity.session.print_summary();
        }

        // Export the data if requested
        if !cli_args.is_present("detail-off") {
            activity.export()?;
        }

        // Push the session onto the summary vector
        activities.activities_list.push(activity);
    }

    // Export the summary information
    if cli_args.is_present("summary-file") {
        log::info!("Summary information written to: {}", &sessionfile);
        activities.export_summary_csv(sessionfile)?;
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
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
