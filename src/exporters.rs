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
///    `Box<dyn Error>` -- propagates error handling up if something goes wrong.
pub fn export_laps_csv(lap_vec: &[types::Lap], filename: &str) -> Result<(), Box<dyn Error>> {
    let mut lap_writer = WriterBuilder::new()
        .has_headers(false)
        .from_path(filename)?;

    // Write the header separately since types::Duration doesn't get serialized properly
    lap_writer.write_record(&[
        "lap_num",
        "cadence_avg",
        "cadence_max",
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
        "ascent",
        "descent",
        "calories",
        "distance",
        "duration_secs",
        "duration_nanos",
        "duration_active_secs",
        "duration_active_nanos",
    ])?;

    for lap in lap_vec.iter() {
        lap_writer.serialize(lap)?;
    }
    lap_writer.flush()?;

    Ok(())
}
