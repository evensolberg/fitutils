//! Defines the `Activity` struct which holds the information contained in a .FIT file, and associated functions.

use crate::types::{Lap, Record, Session};

use csv::WriterBuilder;
use fitparser::profile::field_types::MesgNum;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the all the information about a FIT file and its contents
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Activity {
    /// High-level session information and summary.
    pub session: Session,
    /// Lists all the `Lap`s.
    pub laps: Vec<Lap>,
    /// Lists all the `Record`s.
    pub records: Vec<Record>,
}

impl Activity {
    /// Creates a new, empty `Activity`.
    pub fn new() -> Self {
        Self::default()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Parses the input file into its constituent parts.
    ///
    /// # Parameters
    ///
    ///    `filename: &str` -- The filename for the FIT file to be parsed.
    ///
    /// # Returns
    ///
    ///    `Result<Activity, Box<dyn Error>>` -- `Ok(Activity)` if succesful, otherwise an `Error`.
    ///
    /// # Example
    ///
    ///   ```rust
    ///    use crate::types::activity::Activity;
    ///
    ///    let my_activity = Activity::from_fitfile("fitfile.fit")?;
    ///   ```
    pub fn from_file(filename: &str) -> Result<Activity, Box<dyn Error>> {
        // open the file and deserialize it - return error if unable.
        let mut fp = File::open(filename)?;
        let file = fitparser::from_reader(&mut fp)?;

        // Create a bunch of placeholder variables.
        let mut my_session = Session::from_filename(filename)?;
        let mut num_records = 0;
        let mut num_sessions = 0;
        let mut lap_num = 0;
        let mut lap_vec: Vec<Lap> = Vec::new(); // Lap information vector
        let mut records_vec: Vec<Record> = Vec::new();

        // This is where the actual parsing happens
        for data in file {
            // for each FitDataRecord
            match data.kind() {
                // Figure out what kind it is and parse accordingly
                MesgNum::FileId => {
                    // File header
                    my_session.parse_header(data.fields())?;
                }
                MesgNum::Session => {
                    my_session.parse_session(data.fields())?;
                    num_sessions += 1;
                    my_session.num_sessions = Some(num_sessions);
                }
                MesgNum::Lap => {
                    let mut lap = Lap::from_fit_lap(data.fields(), &my_session)?;
                    lap_num += 1;
                    lap.lap_num = Some(lap_num);
                    lap_vec.push(lap); // push the lap onto the vector
                }
                MesgNum::Record => {
                    let record = Record::from_fit_record(data.fields(), &my_session)?;
                    records_vec.push(record);
                    num_records += 1;
                }
                _ => (),
            } // match
        } // for data

        // Set the total number of records for the session
        my_session.num_records = Some(num_records);

        // Build and return the activity
        Ok(Activity {
            session: my_session,
            laps: lap_vec,
            records: records_vec,
        })
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Export the activity into its constituent JSON and CSV parts:
    ///
    /// - _Session_ gets exported to `fitfilename.session.json`
    /// - _Laps_ get exported to `fitfilename.laps.csv`
    /// - _Records_ get exported to `fitfilename.records.csv`
    ///
    /// # Parameters
    ///
    /// `&self` - The current activity.
    ///
    /// # Returns
    ///
    /// `Result<(), Box<dyn Error>>` -- OK if successful, `Error` otherwise.
    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        self.session.export_json()?;
        Self::export_laps_csv(self)?;
        Self::export_records_csv(self)?;

        // return safely
        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Export the laps information to a CSV file named after the FIT file with the _.fit_ extension replaced by _.laps.csv_
    ///
    /// # Parameters
    ///
    /// `&self` -- The current activity.
    ///
    /// # Returns
    ///
    /// `Result<(), Box<dyn Error>>` -- `Ok(())` if successful, `Error` otherwise.
    pub fn export_laps_csv(&self) -> Result<(), Box<dyn Error>> {
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
    /// # Parameters
    ///
    /// `&self` -- The current activity.
    ///
    /// # Returns
    ///
    /// `Result<(), Box<dyn Error>>` -- `Ok(())` if successful, `Error` otherwise.
    pub fn export_records_csv(&self) -> Result<(), Box<dyn Error>> {
        // Change the file extension
        let mut outfile = PathBuf::from(&self.session.filename.as_ref().unwrap());
        outfile.set_extension("records.csv");
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
    /// Returns an empty `Activity`.
    fn default() -> Self {
        Self {
            session: Session::new(),
            laps: Vec::new(),
            records: Vec::new(),
        }
    }
}
