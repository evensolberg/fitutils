use crate::types::lap::Lap;
use crate::types::record::Record;
use crate::types::session::Session;

use csv::WriterBuilder;
use fitparser::profile::field_types::MesgNum;
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
    /// Parses the input file into its constituent parts.
    ///
    /// **Parameters:***
    ///
    ///    `filename: &str` -- The filename for the FIT file to be parsed.
    ///
    /// **Returns:**
    ///
    ///    `Result<Activity, Box<dyn Error>>` -- Ok(Activity) if succesful, otherwise an error.
    ///
    /// **Example:**
    ///
    ///   ```rust
    ///    mod parsers;
    ///
    ///    let my_activity = parsers::parse_fitfile("fitfile.fit")?;
    ///   ```
    pub fn from_fitfile(filename: &str) -> Result<Activity, Box<dyn Error>> {
        // open the file - return error if unable.
        let mut fp = File::open(filename)?;
        log::trace!(
            "parsers::parse_fitfile() -- {} was read OK. File pointer name: {:?}",
            filename,
            fp
        );

        // Deserialize the file contents
        log::trace!("parsers::parse_fitfile() -- Deserializing file.");
        let file = fitparser::from_reader(&mut fp)?;

        log::debug!(
            "parsers::parse_fitfile() -- Data was deserialized. Total number of records: {}",
            file.len()
        );

        let mut my_session = Session::new();
        my_session.filename = Some(filename.to_string());

        // This is the main file parsing loop. This will definitely get expanded.
        let mut num_records = 0;
        let mut num_sessions = 0;
        let mut lap_num = 0;
        let mut lap_vec: Vec<Lap> = Vec::new(); // Lap information vector
        let mut records_vec: Vec<Record> = Vec::new();

        // This is where the actual parsing happens
        log::debug!("parsers::parse_fitfile() -- Parsing data.");
        for data in file {
            // for each FitDataRecord
            match data.kind() {
                // Figure out what kind it is and parse accordingly
                MesgNum::FileId => {
                    // File header
                    my_session.parse_header(data.fields())?;
                    log::debug!(
                        "parsers::parse_fitfile() -- Session after parsing header: {:?}",
                        my_session
                    );
                }
                MesgNum::Session => {
                    my_session.parse_session(data.fields())?;
                    log::debug!("parsers::parse_fitfile() -- Session: {:?}", my_session);
                    num_sessions += 1;
                    my_session.num_sessions = Some(num_sessions);
                }
                MesgNum::Lap => {
                    let mut lap = Lap::from_fit_lap(data.fields(), &my_session)?;
                    lap_num += 1;
                    lap.lap_num = Some(lap_num);
                    log::debug!("parsers::parse_fitfile() -- Lap {:3}: {:?}", lap_num, lap);
                    lap_vec.push(lap); // push the lap onto the vector
                }
                MesgNum::Record => {
                    // FIXME: This is inefficient since we're instantiating this for every record
                    let record = Record::from_fit_record(data.fields(), &my_session)?;
                    log::debug!("parsers::parse_fitfile() -- Record: {:?}", record);
                    records_vec.push(record);
                    num_records += 1;
                    my_session.num_records = Some(num_records);
                }
                _ => (),
            } // match
        } // for data

        // Build the activity
        let my_activity = Activity {
            session: my_session,
            laps: lap_vec,
            records: records_vec,
        };

        // Return the activity struct
        Ok(my_activity)
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
        self.session.export_json()?;
        Self::export_laps_csv(self)?;
        Self::export_records_csv(self)?;

        // return safely
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
