use csv::WriterBuilder;
use std::error::Error;
use std::path::PathBuf;

use crate::types::activity::Activity;

/// Holds a list of all activities. Used to export session totals
#[derive(Debug)]
pub struct Activities {
    /// A list of activities
    pub activities_list: Vec<Activity>,
}

impl Activities {
    /// Create a new, empty Activities list
    pub fn new() -> Self {
        Activities::default()
    }

    /// Export the list of session information to a CSV file
    pub fn export_csv(&self, sessionfile: &str) -> Result<(), Box<dyn Error>> {
        // Create a buffer for the CSV
        let outfile = PathBuf::from(sessionfile);
        let mut sess_writer = WriterBuilder::new()
            .has_headers(false)
            .from_path(&outfile)?;

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
        for activity in self.activities_list.iter() {
            sess_writer.serialize(&activity.session)?;
        }

        // Write the file
        sess_writer.flush()?;

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
