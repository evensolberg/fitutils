use env_logger::Target;
use std::{collections::HashMap, error::Error, path::Path};

mod cli;

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
                value_res = utilities::fit_to_hashmap(filename);
                log::debug!("FIT: {:?}", value_res);
            }
            "gpx" => {
                value_res = utilities::gpx_to_hashmap(filename);
                log::debug!("GPX: {:?}", value_res);
            }
            "tcx" => {
                value_res = utilities::tcx_to_hashmap(filename);
                log::debug!("TCX {:?}", value_res);
            }
            _ => log::warn!("Unknown file type: {}.", &filename),
        }

        // Process the result of reading metadata
        match value_res {
            // Metadata read OK - try to rename
            Ok(values) => {
                let result =
                    utilities::rename_file(filename, pattern, &values, total_files, dry_run);
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
