use env_logger::Target;
use std::error::Error;
use utilities::FITActivity;

mod cli;
mod fit;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build_cli();

    // Initialize logging
    let mut logbuilder = utilities::build_log(&cli_args);
    logbuilder.target(Target::Stdout).init();

    for argument in cli_args.values_of("read").unwrap() {
        log::trace!("main::run() -- Arguments: {:?}", argument);
    }

    let mut total_files: usize = 0;
    let mut processed_files: usize = 0;
    let skipped_files: usize = 0;

    let detailed = cli_args.is_present("print-detail");

    // The good stuff goes here
    for filename in cli_args.values_of("read").unwrap() {
        log::debug!("Processing file: {}", filename);
        match utilities::get_extension(filename).as_ref() {
            "fit" => {
                log::debug!("FIT: {}", filename);
                let act = FITActivity::from_file(filename)?;
                fit::print_activity(&act, detailed);
                processed_files += 1;
            }
            "gpx" => {
                log::debug!("GPX: {}", filename);
            }
            "tcx" => {
                log::debug!("TCX: {}", filename);
            }
            _ => log::warn!("Unknown file type: {}.", &filename),
        }
        total_files += 1;
    }

    if cli_args.is_present("print-summary") {
        log::info!("Total files examined:        {:6}", total_files);
        log::info!("Files processed:             {:6}", processed_files);
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
