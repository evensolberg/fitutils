//! Defines the `Activities` struct which is used to hold a list of all activities, and associated functions.

use csv::WriterBuilder;
use std::error::Error;
use std::path::PathBuf;

use crate::types::activity::Activity;

/// Holds a list of all activities. Used to export session totals.
#[derive(Debug)]
pub struct Activities {
    /// A list of activities.
    pub activities_list: Vec<Activity>,
}

impl Activities {
    /// Create a new, empty Activities list.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::types::activities::Activities;
    ///
    /// activities = Activities::new();
    /// ```
    pub fn new() -> Self {
        Activities::default()
    }

    /// Export the summary list of session information to a CSV file.
    ///
    /// # Parameters
    ///
    /// `sessionfile: &str`: The name of the file into which the sessions summary information is to be written.
    ///
    /// # Returns
    ///
    /// `Result<(), Box<dyn Error>>`: Nothing if everying went OK, or an `Error` if not.
    ///
    /// # Example
    ///
    /// ```
    /// activities.export_summary_csv("session_summary.csv")?;
    /// ```
    pub fn export_summary_csv(&self, sessionfile: &str) -> Result<(), Box<dyn Error>> {
        // Create a buffer for the CSV
        let outfile = PathBuf::from(sessionfile);
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_path(&outfile)?;

        // Write the header separately since types::Duration doesn't get serialized properly
        writer.write_record(&[
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
        for activity in self.activities_list.iter() {
            log::trace!("activities::export_summary_csv() -- serializing: {:?}", activity);
            writer.serialize(&activity.session)?;
        }

        log::trace!("activities::export_summary_csv() -- session information to be written: {:?}", writer);

        // Write the file
        writer.flush()?;

        // Return safely
        Ok(())
    }
}

impl Default for Activities {
    /// Sets up the Activities with empty data placeholders.
    fn default() -> Self {
        Self {
            activities_list: Vec::new(),
        }
    }
}
