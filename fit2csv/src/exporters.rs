// This file contains functions that export data to JSON or CSV
use super::types;
use csv::WriterBuilder;
use std::error::Error;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Export the laps vector to a CSV file with the file name specified.
///
/// **Parameters:**
///
///    `lap_vec: &[types::Lap]` -- A vector of laps as parsed by the parse_laps function.<br>
///    `filename: &str` -- The name of the CSV file into which we wish to write the contents of the laps.
///
/// **Returns:**
///
///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
pub fn export_laps_csv(lap_vec: &[types::Lap], filename: &str) -> Result<(), Box<dyn Error>> {
    // Create a buffer for the CSV
    let mut lap_writer = WriterBuilder::new()
        .has_headers(false)
        .from_path(filename)?;

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
        "stance_time_avg_seconds",
        "stance_time_avg_nanos",
        "vertical_oscillation_avg",
        "ascent",
        "descent",
        "calories",
        "distance",
        "duration_secs",
        "duration_nanos",
        "duration_active_secs",
        "duration_active_nanos",
        "duration_moving_secs",
        "duration_moving_nanos",
        "start_time",
        "finish_time",
        "heart_rate_zone0_seconds",
        "heart_rate_zone0_nanos",
        "heart_rate_zone1_seconds",
        "heart_rate_zone1_nanos",
        "heart_rate_zone2_seconds",
        "heart_rate_zone2_nanos",
        "heart_rate_zone3_seconds",
        "heart_rate_zone3_nanos",
        "heart_rate_zone4_seconds",
        "heart_rate_zone4_nanos",
    ])?;

    // Now write the actual laps
    for lap in lap_vec.iter() {
        lap_writer.serialize(lap)?;
    }

    // Write the file
    lap_writer.flush()?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Export the records vector to a CSV file with the file name specified.
///
/// **Parameters:**
///
///    `rec_vec: &[types::Records]` -- A vector of records as parsed by the parse_laps function.<br>
///    `filename: &str` -- The name of the CSV file into which we wish to write the contents of the laps.
///
/// **Returns:**
///
///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
pub fn export_records_csv(rec_vec: &[types::Record], filename: &str) -> Result<(), Box<dyn Error>> {
    // Create a buffer for the CSV
    let mut rec_writer = WriterBuilder::new()
        .has_headers(false)
        .from_path(filename)?;

    // Write the header separately since types::Duration doesn't get serialized properly
    rec_writer.write_record(&[
        "timestamp",
        "duration_seconds",
        "duration_nanos",
        "distance",
        "altitude",
        "stance_time_seconds",
        "stance_time_nanos",
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
    for rec in rec_vec.iter() {
        rec_writer.serialize(rec)?;
    }

    // Write the file
    rec_writer.flush()?;

    Ok(())
}
