// This file contains functions that export data to JSON or CSV
use super::types;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Export the activity into its constituent JSON and CSV parts:
///
///    _Session_ gets exported to `fitfilename.session.json`
///    _Laps_ get exported to `fitfilename.laps.csv`
///   _Records_ get exported to `fitfilename.records.csv`
///
/// **Parameters:**
///
///    `activity: &types::Activity` -- A struct containing all the information parsed from the FIT file.
///
/// **Returns:**
///
///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
pub fn export_activity(activity: &types::Activity) -> Result<(), Box<dyn Error>> {
    export_session_json(activity)?;
    export_laps_csv(activity)?;
    export_records_csv(activity)?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Export the session infromation to a JSON file name based on the FIT file name.
///
/// **Parameters:**
///
///    `activity: &types::Activity` -- A struct containing all the information parsed from the FIT file.
///
/// **Returns:**
///
///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
pub fn export_session_json(activity: &types::Activity) -> Result<(), Box<dyn Error>> {
    let outfile = activity
        .session
        .filename
        .as_ref()
        .unwrap()
        .to_lowercase()
        .replace(".fit", ".session.json");
    log::trace!("Writing JSON file {}", &outfile);
    serde_json::to_writer_pretty(&File::create(&outfile)?, &activity.session)
        .expect("Unable to write session info to JSON file.");

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Export the laps information to a CSV file named after the FIT file with the _.fit_ extension replaced by _.laps.csv_
///
/// **Parameters:**
///
///    `activity: &types::Activity` -- A struct containing all the information parsed from the FIT file.
///
/// **Returns:**
///
///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
pub fn export_laps_csv(activity: &types::Activity) -> Result<(), Box<dyn Error>> {
    let outfile = activity
        .session
        .filename
        .as_ref()
        .unwrap()
        .to_lowercase()
        .replace(".fit", ".laps.csv");
    log::trace!("Writing lap CSV file {}", &outfile);

    // Create a buffer for the CSV
    let mut lap_writer = WriterBuilder::new().has_headers(false).from_path(outfile)?;

    // Write the header separately since types::Duration doesn't get serialized properly
    lap_writer.write_record(&[
        "filename",
        "lap_num",
        "cadence_avg",
        "cadence_max",
        "heartrate_min",
        "heartrate_avg",
        "heartrate_max",
        "speed_avg",
        "speed_max",
        "power_avg",
        "power_max",
        "lat_start",
        "lon_start",
        "lat_end",
        "lon_end",
        "stance_time_avg_secs",
        "vertical_oscillation_avg",
        "ascent",
        "descent",
        "calories",
        "distance",
        "duration_secs",
        "duration_active_secs",
        "duration_moving_secs",
        "start_time",
        "finish_time",
        "heart_rate_zone0_secs",
        "heart_rate_zone1_secs",
        "heart_rate_zone2_secs",
        "heart_rate_zone3_secs",
        "heart_rate_zone4_secs",
    ])?;

    // Now write the actual laps
    for lap in activity.laps.iter() {
        lap_writer.serialize(lap)?;
    }

    // Write the file
    lap_writer.flush()?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Export the records information to a CSV file named after the FIT file with the _.fit_ extension replaced by _.records.csv_
///
/// **Parameters:**
///
///    `activity: &types::Activity` -- A struct containing all the information parsed from the FIT file.
///
/// **Returns:**
///
///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
pub fn export_records_csv(activity: &types::Activity) -> Result<(), Box<dyn Error>> {
    let outfile = activity
        .session
        .filename
        .as_ref()
        .unwrap()
        .to_lowercase()
        .replace(".fit", ".records.csv");
    log::trace!("Writing records CSV file {}", &outfile);
    // Create a buffer for the CSV
    let mut rec_writer = WriterBuilder::new().has_headers(false).from_path(outfile)?;

    // Write the header separately since types::Duration doesn't get serialized properly
    rec_writer.write_record(&[
        "timestamp",
        "duration_seconds",
        "distance",
        "altitude",
        "stance_time_seconds",
        "vertical_oscillation",
        "cadence",
        "speed",
        "power",
        "heartrate",
        "calories",
        "lat",
        "lon",
    ])?;

    // Now write the actual laps
    for rec in activity.records.iter() {
        rec_writer.serialize(rec)?;
    }

    // Write the file
    rec_writer.flush()?;

    Ok(())
}
