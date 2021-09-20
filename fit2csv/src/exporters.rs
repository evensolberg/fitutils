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
        "cadence_avg_bpm",
        "cadence_max_bpm",
        "heartrate_min_bpm",
        "heartrate_avg_bpm",
        "heartrate_max_bpm",
        "speed_avg_ms",
        "speed_max_ms",
        "power_avg_w",
        "power_max_w",
        "lat_start",
        "lon_start",
        "lat_end",
        "lon_end",
        "stance_time_avg_sec",
        "vertical_oscillation_avg",
        "ascent_m",
        "descent_m",
        "calories",
        "distance_m",
        "duration_secs",
        "duration_active_sec",
        "duration_moving_sec",
        "start_time",
        "finish_time",
        "heart_rate_zone0_sec",
        "heart_rate_zone1_sec",
        "heart_rate_zone2_sec",
        "heart_rate_zone3_sec",
        "heart_rate_zone4_sec",
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
        "duration_sec",
        "distance_m",
        "altitude_m",
        "stance_time_sec",
        "vertical_oscillation",
        "cadence_bpm",
        "speed_ms",
        "power_w",
        "heartrate_bpm",
        "calories",
        "lat_deg",
        "lon_deg",
    ])?;

    // Now write the actual laps
    for rec in activity.records.iter() {
        rec_writer.serialize(rec)?;
    }

    // Write the file
    rec_writer.flush()?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Export the session summary information to a CSV file named _sessions.csv_ in the invocation directory.
///
/// **Parameters:**
///
///    `session_vec: &[types::Session]` -- A vector containing all the sessions found
///
/// **Returns:**
///
///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
pub fn export_sessions_csv(session_vec: &[types::Session]) -> Result<(), Box<dyn Error>> {
    // Create a buffer for the CSV
    let mut sess_writer = WriterBuilder::new()
        .has_headers(false)
        .from_path("sessions.csv")?;

    // Write the header separately since types::Duration doesn't get serialized properly
    sess_writer.write_record(&[
        "filename",
        "manufacturer",
        "product",
        "serial_number",
        "time_created",
        "activity_type",
        "activity_detailed",
        "num_sessions",
        "num_laps",
        "num_records",
        "cadence_avg_bpm",
        "cadence_max_bpm",
        "heartrate_avg_bpm",
        "heartrate_max_bpm",
        "heartrate_min_bpm",
        "speed_avg_ms",
        "speed_max_ms",
        "power_avg_w",
        "power_max_w",
        "power_threshold_w",
        "nec_lat_deg",
        "nec_lon_deg",
        "swc_lat_deg",
        "swc_lon_deg",
        "stance_time_avg",
        "vertical_oscillation_avg",
        "ascent_m",
        "descent_m",
        "calories",
        "distance_m",
        "duration_sec",
        "duration_active_sec",
        "duration_moving_sec",
        "start_time",
        "finish_time",
        "time_in_hr_zone_0_sec",
        "time_in_hr_zone_1_sec",
        "time_in_hr_zone_2_sec",
        "time_in_hr_zone_3_sec",
        "time_in_hr_zone_4_sec",
    ])?;

    // Now write the actual laps
    for sess in session_vec.iter() {
        sess_writer.serialize(sess)?;
    }

    // Write the file
    sess_writer.flush()?;

    Ok(())
}
