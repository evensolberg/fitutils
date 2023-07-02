use env_logger::Target;
use std::error::Error;
use utilities::{FITActivity, GPXActivity, TCXActivity};

use clap::parser::ValueSource;

mod cli;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build();
    let detailed = cli_args.value_source("print-detail") == Some(ValueSource::CommandLine);
    let print_summary = cli_args.value_source("print-summary") == Some(ValueSource::CommandLine);

    // Initialize logging
    let mut logbuilder = utilities::build_log(&cli_args);
    logbuilder.target(Target::Stdout).init();

    let filenames = cli_args
        .get_many::<String>("read")
        .unwrap_or_default()
        .map(std::string::String::as_str);
    log::trace!("main::run() -- Files: {filenames:?}");

    let mut total_files: usize = 0;
    let mut processed_files: usize = 0;
    let skipped_files: usize = 0;

    // The good stuff goes here
    for filename in filenames {
        log::debug!("Processing file: {filename}");
        match utilities::get_extension(filename).as_ref() {
            "fit" => {
                let act = FITActivity::from_file(filename)?;
                act.print(detailed);
                processed_files += 1;
            }
            "gpx" => {
                let act = GPXActivity::from_file(filename)?;
                act.print(detailed);
                processed_files += 1;
            }
            "tcx" => {
                let act = TCXActivity::from_file(filename)?;
                act.print(detailed);
                processed_files += 1;
            }
            _ => log::warn!("Unknown file type: {filename}."),
        }
        total_files += 1;
    }

    if print_summary {
        log::info!("Total files examined:        {total_files:6}");
        log::info!("Files processed:             {processed_files:6}");
        log::info!("Files skipped due to errors: {skipped_files:6}");
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
