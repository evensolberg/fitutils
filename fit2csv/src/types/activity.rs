use crate::types::lap::Lap;
use crate::types::record::Record;
use crate::types::session::Session;

use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the all the information about the file and its contents
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Activity {
    pub session: Session,
    pub laps: Vec<Lap>,
    pub records: Vec<Record>,
}

impl Activity {
    pub fn new() -> Self {
        Self::default()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Export the activity into its constituent JSON and CSV parts:
    ///
    ///    _Session_ gets exported to `fitfilename.session.json`
    ///    _Laps_ get exported to `fitfilename.laps.csv`
    ///   _Records_ get exported to `fitfilename.records.csv`
    ///
    /// **Returns:**
    ///
    ///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        Self::export_session_json(&self)?;
        Self::export_laps_csv(&self)?;
        Self::export_records_csv(&self)?;

        // return safely
        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Export the session infromation to a JSON file name based on the FIT file name.
    ///
    /// **Returns:**
    ///
    ///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
    fn export_session_json(&self) -> Result<(), Box<dyn Error>> {
        // Change the file extension
        let mut export_path = PathBuf::from(&self.session.filename.as_ref().unwrap());
        export_path.set_extension("session.json");
        log::trace!(
            "exporter::export_session_json() -- Writing JSON file {}",
            &export_path.to_str().unwrap()
        );

        // Write the session data to JSON
        serde_json::to_writer_pretty(&File::create(&export_path)?, &self.session)
            .expect("Unable to write session info to JSON file.");

        // Everything is OK
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
    fn export_laps_csv(&self) -> Result<(), Box<dyn Error>> {
        // Change the file extension
        let mut outfile = PathBuf::from(&self.session.filename.as_ref().unwrap());
        outfile.set_extension("laps.csv");
        log::trace!(
            "exporter::export_laps_csv() -- Writing lap CSV file {}",
            &outfile.to_str().unwrap()
        );

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
        for lap in self.laps.iter() {
            lap_writer.serialize(lap)?;
        }

        // Write the file
        lap_writer.flush()?;

        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Export the records information to a CSV file named after the FIT file with the _.fit_ extension replaced by _.records.csv_
    ///
    /// **Returns:**
    ///
    ///    `Result<(), Box<dyn Error>>` -- OK if successful, propagates error handling up if something goes wrong.
    pub fn export_records_csv(&self) -> Result<(), Box<dyn Error>> {
        // Change the file extension
        let mut outfile = PathBuf::from(&self.session.filename.as_ref().unwrap());
        outfile.set_extension("laps.csv");
        log::trace!(
            "exporter::export_records_csv() -- Writing records CSV file {}",
            &outfile.to_str().unwrap()
        );

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
        for rec in self.records.iter() {
            rec_writer.serialize(rec)?;
        }

        // Write the file
        rec_writer.flush()?;

        Ok(())
    }

    // end impl Activity
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            session: Session::new(),
            laps: Vec::new(),
            records: Vec::new(),
        }
    }
}
