use clap::parser::ValueSource;
use env_logger::Target;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use utilities::{
    get_extension, set_extension, FITActivities, FITActivity, GPXActivities, GPXActivity,
    TCXActivitiesList, TCXActivity, TCXTrackpointList,
};

mod cli;

/// This is where the actual processing takes place.
#[allow(clippy::too_many_lines)]
fn run() -> Result<(), Box<dyn Error>> {
    let cli_args = cli::build().get_matches();

    // Initialize logging
    let mut logbuilder = utilities::build_log(&cli_args);
    logbuilder.target(Target::Stdout).init();

    // Trace-log the input files
    for argument in cli_args
        .get_many::<String>("read")
        .unwrap_or_default()
        .map(std::string::String::as_str)
    {
        log::trace!("run() -- Arguments: {argument:?}");
    }

    // Determine summary file settings
    let write_summary =
        cli_args.value_source("summary-file") == Some(ValueSource::CommandLine);
    let default_summary = String::from("activities.csv");
    let summary_base = cli_args
        .get_one::<String>("summary-file")
        .unwrap_or(&default_summary);
    let detail_off =
        cli_args.value_source("detail-off") == Some(ValueSource::CommandLine);
    let print_summary =
        cli_args.value_source("print-summary") == Some(ValueSource::CommandLine);

    if detail_off {
        log::info!("Writing summary file only.");
    } else {
        log::info!("Writing detail files.");
    }

    // Collectors for each format
    let mut fit_activities = FITActivities::default();
    let mut gpx_activities = GPXActivities::new();
    let mut tcx_activities = TCXActivitiesList::default();

    let mut total_files: usize = 0;
    let mut processed_files: usize = 0;
    let mut skipped_files: usize = 0;

    for filename in cli_args
        .get_many::<String>("read")
        .unwrap_or_default()
        .map(std::string::String::as_str)
    {
        total_files += 1;
        log::info!("Processing file: {filename}");

        let ext = get_extension(filename);
        match ext.as_str() {
            "fit" => {
                let activity = match FITActivity::from_file(filename) {
                    Ok(a) => a,
                    Err(e) => {
                        log::error!("Error processing {filename}: {e}");
                        skipped_files += 1;
                        continue;
                    }
                };

                if print_summary {
                    activity.print(false);
                }

                if !detail_off {
                    if let Err(e) = activity.export() {
                        log::error!("Error exporting {filename}: {e}");
                        skipped_files += 1;
                        continue;
                    }
                }

                fit_activities.activities_list.push(activity);
            }
            "gpx" => {
                let activity = match GPXActivity::from_file(filename) {
                    Ok(a) => a,
                    Err(e) => {
                        log::error!("Error processing {filename}: {e}");
                        skipped_files += 1;
                        continue;
                    }
                };

                if print_summary {
                    activity.print(false);
                }

                if !detail_off {
                    if let Err(e) = activity.export() {
                        log::error!("Error exporting {filename}: {e}");
                        skipped_files += 1;
                        continue;
                    }
                }

                gpx_activities.activities_list.push(activity);
            }
            "tcx" => {
                let file = match File::open(filename) {
                    Ok(f) => f,
                    Err(e) => {
                        log::error!("Error opening {filename}: {e}");
                        skipped_files += 1;
                        continue;
                    }
                };
                let mut tcdb = match tcx::read(&mut BufReader::new(file)) {
                    Ok(t) => t,
                    Err(e) => {
                        log::error!("Error parsing {filename}: {e}");
                        skipped_files += 1;
                        continue;
                    }
                };
                tcdb.calc_heartrates();

                // In debug mode, export the raw TCX database to JSON
                if cli_args.value_source("debug") == Some(ValueSource::CommandLine)
                    || cli_args.value_source("debug")
                        == Some(ValueSource::EnvVariable)
                {
                    let outfile = set_extension(filename, "json");
                    log::trace!(
                        "Exporting {filename} to {} for debugging.",
                        outfile.as_str()
                    );
                    if let Err(e) = tcdb.export_json(outfile.as_str()) {
                        log::error!("Error exporting debug JSON for {filename}: {e}");
                    }
                }

                let Some(activities) = tcdb.activities else {
                    log::warn!("No activities found in {filename}. Skipping.");
                    skipped_files += 1;
                    continue;
                };

                let mut curr_activities =
                    TCXActivity::from_activities(&activities);
                curr_activities.filename = Some(filename.to_string());

                if print_summary {
                    curr_activities.print(false);
                }

                if !detail_off {
                    if let Err(e) = curr_activities.export_json() {
                        log::error!("Error exporting JSON for {filename}: {e}");
                        skipped_files += 1;
                        continue;
                    }

                    let tp_list =
                        TCXTrackpointList::from_activities(&activities);
                    if let Err(e) = tp_list.export_csv(
                        &set_extension(filename, "trackpoints.csv"),
                    ) {
                        log::error!("Error exporting trackpoints for {filename}: {e}");
                        skipped_files += 1;
                        continue;
                    }
                }

                tcx_activities.activities.push(curr_activities);
            }
            other => {
                log::error!(
                    "Unsupported file format '.{other}' for {filename}. \
                     Supported: .fit, .gpx, .tcx"
                );
                skipped_files += 1;
                continue;
            }
        }

        processed_files += 1;
    }

    log::info!("Total files examined:        {total_files:6}");
    log::info!("Files processed:             {processed_files:6}");
    log::info!("Files skipped due to errors: {skipped_files:6}");

    // Export per-format summary CSVs
    if write_summary {
        if !fit_activities.activities_list.is_empty() {
            let name = set_extension(summary_base, "fit.csv");
            log::info!("FIT summary written to: {name}");
            fit_activities.export_summary_csv(&name)?;
        }
        if !gpx_activities.activities_list.is_empty() {
            let name = set_extension(summary_base, "gpx.csv");
            log::info!("GPX summary written to: {name}");
            gpx_activities.export_csv(&name)?;
        }
        if !tcx_activities.activities.is_empty() {
            let name = set_extension(summary_base, "tcx.csv");
            log::info!("TCX summary written to: {name}");
            tcx_activities.export_csv(&name)?;
        }
    }

    // In trace mode, export TCX summary JSON
    if cli_args.get_count("debug") > 1 && !tcx_activities.activities.is_empty() {
        let json_name = set_extension(summary_base, "tcx.json");
        log::trace!("Exporting TCX summary JSON: {json_name}");
        tcx_activities.export_json(&json_name)?;
    }

    Ok(())
}

/// The actual executable function that gets called when the program is invoked.
fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            log::error!("{}", err.to_string().replace('\"', ""));
            1
        }
    });
}
