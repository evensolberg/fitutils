use env_logger::Target;
use std::{error::Error, path::Path};

use clap::parser::ValueSource;

mod cli;
mod rename_file;

/// Prints all variable codes available for use in rename/move patterns.
fn print_codes() {
    println!("Variable codes available for use in patterns:\n");

    println!("Date/Time");
    println!("  {{%year}}    / {{%yr}}   Year (4 digits, e.g. 2024)");
    println!("  {{%month}}   / {{%mo}}   Month (2 digits, e.g. 07)");
    println!("  {{%day}}     / {{%dy}}   Day of month (2 digits, e.g. 04)");
    println!("  {{%weekday}} / {{%wd}}   Weekday name (e.g. Mon)");
    println!("  {{%24hour}}  / {{%24}}   Hour in 24-hour format (2 digits, e.g. 14)");
    println!("  {{%hour}}    / {{%hr}}   Same as {{%24hour}}");
    println!("  {{%12hour}}  / {{%12}}   Hour in 12-hour format (2 digits, e.g. 02)");
    println!("  {{%ampm}}    / {{%ap}}   AM/PM indicator (lowercase, e.g. pm)");
    println!("  {{%minute}}  / {{%mt}}   Minute (2 digits, e.g. 30)");
    println!("  {{%second}}  / {{%sc}}   Second (2 digits, e.g. 05)");
    println!("  {{%duration}}/ {{%du}}   Duration of activity in seconds");
    println!();
    println!("Device & Activity");
    println!("  {{%manufacturer}} / {{%mf}}   Device manufacturer (e.g. Garmin)  [FIT/GPX]");
    println!("  {{%product}}      / {{%pr}}   Device product name                [FIT/GPX/TCX]");
    println!("  {{%serial_number}}/ {{%sn}}   Device serial number               [FIT; partial GPX]");
    println!("  {{%activity}}     / {{%at}}   Activity type (e.g. Running)       [FIT/GPX/TCX]");
    println!("  {{%activity_detailed}} / {{%ad}}  Detailed subtype               [FIT/GPX/TCX]");
    println!("  Note: GPX and TCX populate these fields; device fields default to 'unknown'.");
    println!();
    println!("File");
    println!("  {{%type}} / {{%ty}}   File type extension (fit/gpx/tcx)");
    println!("                   Case controlled by --type-case / -t");
    println!();
    println!("Both braced ({{%code}}) and bare (%code) syntax are supported in patterns.");
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
// This #allow is needed for Clippy to shut up. There has to be a bug in Clippy for this one.
#[allow(clippy::unnecessary_wraps, clippy::too_many_lines)]
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build().get_matches();
    let dry_run = cli_args.value_source("dry-run") == Some(ValueSource::CommandLine);
    let print_summary = cli_args.value_source("print-summary") == Some(ValueSource::CommandLine);

    // Initialize logging
    let mut logbuilder = utilities::build_log(&cli_args);
    logbuilder.target(Target::Stdout).init();

    // Print variable codes and exit early if requested
    if cli_args.value_source("print-codes") == Some(ValueSource::CommandLine) {
        print_codes();
        return Ok(());
    }

    let raw_inputs: Vec<String> = cli_args
        .get_many::<String>("read")
        .unwrap_or_default()
        .cloned()
        .collect();
    let filenames = utilities::expand_globs(&raw_inputs);
    log::trace!("main::run() -- Filenames: {filenames:?}");

    if dry_run {
        log::info!("Dry-run. Will not perform actual rename or move.");
    }

    let pattern = cli_args
        .get_one::<String>("pattern")
        .expect("clap requires --pattern unless --print-codes is given")
        .as_str();

    // Get the move pattern
    let move_files = cli_args.value_source("move") == Some(ValueSource::CommandLine);
    let move_pattern = if move_files {
        cli_args
            .get_one::<String>("move")
            .expect("clap provides a value for --move when it is supplied")
            .as_str()
    } else {
        ""
    };

    let type_case_upper = matches!(
        cli_args.get_one::<String>("type-case").map(String::as_str),
        Some("upper" | "u" | "U")
    );

    let mut total_files: usize = 0;
    let mut processed_files: usize = 0;
    let mut skipped_files: usize = 0;

    ///////////////////////////////////
    // Working section
    for filename in filenames.iter().map(String::as_str) {
        total_files += 1;
        log::debug!("Processing file: {filename}");

        // Check if the target file exists, otherwise just continue
        if !Path::new(&filename).exists() {
            log::warn!("File not found: {filename}");
            continue;
        }

        // Read the metadata from files
        let value_res;
        match utilities::get_extension(filename).as_ref() {
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
        match value_res {
            Ok(mut values) => {
                // Inject the file-type template variables ({%type}, {%ty}).
                // get_extension() always returns a non-empty string ("unknown"
                // for missing or trailing-dot extensions), so no guard needed.
                let ext = utilities::get_extension(filename);
                let type_val = if type_case_upper {
                    ext.to_uppercase()
                } else {
                    ext
                };
                values.insert("%type".to_string(), type_val.clone());
                values.insert("%ty".to_string(), type_val);

                let move_pat = if move_files { Some(move_pattern) } else { None };
                match rename_file::rename_and_move(
                    filename,
                    pattern,
                    move_pat,
                    &values,
                    total_files,
                    dry_run,
                ) {
                    Ok(new_path) => {
                        log::info!("{filename} --> {new_path}");
                        processed_files += 1;
                    }
                    Err(err) => {
                        log::error!("Unable to process {filename}: {err}");
                        skipped_files += 1;
                    }
                }
            }
            Err(err) => log::error!("Unable to process {filename}: {err}"),
        }
    }

    if print_summary {
        log::info!("Total files examined:        {total_files:6}");
        log::info!("Files processed:             {processed_files:6}");
        log::info!("Files skipped due to errors: {skipped_files:6}");
    }

    // Everything is a-okay in the end
    Ok(())
} // fn run()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_extension_normalises_to_lowercase() {
        // get_extension() always returns a lowercase string.
        // The {%type} injection in run() depends on this — the `lower` case
        // mode needs no transform, and `upper` always starts from lowercase.
        assert_eq!(utilities::get_extension("workout.FIT"), "fit");
        assert_eq!(utilities::get_extension("route.gpx"), "gpx");
        assert_eq!(utilities::get_extension("activity.TCX"), "tcx");
    }

    /// Ensure the `matches!` expression used in `run()` treats all upper aliases correctly.
    #[test]
    fn type_case_upper_matches_all_aliases() {
        for value in &["upper", "u", "U"] {
            assert!(
                matches!(Some(*value), Some("upper" | "u" | "U")),
                "'{value}' should be treated as upper-case",
            );
        }
        for value in &["lower", "l", "L"] {
            assert!(
                !matches!(Some(*value), Some("upper" | "u" | "U")),
                "'{value}' should not be treated as upper-case",
            );
        }
    }

    /// Smoke-test that `print_codes` does not panic.
    #[test]
    fn print_codes_does_not_panic() {
        print_codes();
    }
}

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
