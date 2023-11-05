use env_logger::Target;
use std::{error::Error, path::Path};

use clap::parser::ValueSource;

mod cli;
mod move_file;
mod rename_file;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
// This #allow is needed for Clippy to shut up. There has to be a bug in Clippy for this one.
#[allow(clippy::unnecessary_wraps)]
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build().get_matches();
    let dry_run = cli_args.value_source("dry-run") == Some(ValueSource::CommandLine);
    let print_summary = cli_args.value_source("print-summary") == Some(ValueSource::CommandLine);

    // Initialize logging
    let mut logbuilder = utilities::build_log(&cli_args);
    logbuilder.target(Target::Stdout).init();

    let filenames = cli_args
        .get_many::<String>("read")
        .unwrap_or_default()
        .map(std::string::String::as_str);
    log::trace!("main::run() -- Filenames: {filenames:?}");

    if dry_run {
        log::info!("Dry-run. Will not perform actual rename or move.");
    }

    let default_pattern = String::new();
    let pattern = cli_args
        .get_one::<String>("pattern")
        .unwrap_or(&default_pattern)
        .as_str();

    // Get the move pattern
    let move_files = cli_args.value_source("move") == Some(ValueSource::CommandLine);
    let move_pattern = if move_files {
        cli_args
            .get_one::<String>("move")
            .unwrap_or(&default_pattern)
            .as_str()
    } else {
        ""
    };

    let mut total_files: usize = 0;
    let mut processed_files: usize = 0;
    let mut skipped_files: usize = 0;

    ///////////////////////////////////
    // Working section
    for filename in filenames {
        log::debug!("Processing file: {filename}");

        // Check if the target file exists, otherwise just continue
        if !Path::new(&filename).exists() {
            log::warn!("File not found: {filename}");
            continue;
        }

        // Read the metadata from files
        let value_res;
        match utilities::get_extension(filename).to_lowercase().as_ref() {
            "fit" => {
                value_res = utilities::fit_to_hashmap(filename);
                log::debug!("FIT: {value_res:?}");
            }
            "gpx" => {
                value_res = utilities::gpx_to_hashmap(filename);
                log::debug!("GPX: {value_res:?}");
            }
            "tcx" => {
                value_res = utilities::tcx_to_hashmap(filename);
                log::debug!("TCX: {value_res:?}");
            }
            _ => {
                log::warn!("Unknown file type: {filename}.");
                value_res = Err("Unknown file type".into());
            }
        }

        // Process the result of reading metadata
        let new_filename;
        match value_res {
            // Metadata read OK - try to rename and move
            Ok(values) => {
                let result =
                    rename_file::rename_file(filename, pattern, &values, total_files, dry_run);
                match result {
                    // How did the rename go?
                    Ok(result) => {
                        new_filename = result.clone();
                        log::info!("{filename} --> {new_filename}");
                        if !move_files {
                            // If we're not moving the file, we're done with this file.
                            processed_files += 1;
                            continue;
                        }
                    }
                    Err(err) => {
                        log::error!("Unable to rename {filename} : {}", err.to_string());
                        skipped_files += 1;
                        continue;
                    }
                }

                if move_files {
                    let result = move_file::move_file(
                        &new_filename,
                        move_pattern,
                        &values,
                        total_files,
                        dry_run,
                    );
                    match result {
                        // How did the move go?
                        Ok(result) => {
                            log::info!("{new_filename} --> {result}");
                            processed_files += 1;
                        }
                        Err(err) => {
                            log::error!("Unable to move {new_filename} : {}", err.to_string());
                            skipped_files += 1;
                        }
                    }
                }
            }
            // Problem reading metadata - let the user know.
            Err(err) => log::error!("Unable to process {filename} : {}", err.to_string()),
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
        Ok(()) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
