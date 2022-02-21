//! A list of all the `Activity` structs (i.e.. GPX files) parsed, and associated functions.

use csv::WriterBuilder;
use std::error::Error;
use std::path::PathBuf;

use crate::gpx::activity::GPXActivity;

/// Holds a list of all activities. Used to export session totals
#[derive(Debug)]
pub struct GPXActivities {
    /// A list of activities
    pub activities_list: Vec<GPXActivity>,
}

impl GPXActivities {
    /// Create a new, empty Activities list
    pub fn new() -> Self {
        GPXActivities::default()
    }

    /// Export the list of session information to a CSV file
    ///
    /// # Parameters
    ///
    /// `filename: &str` -- The name of the summary file for all the activity data.
    ///
    /// # Returns
    ///
    /// `Result<(), Box<dyn Error>> -- Returns `Ok(())` if everything went well, otherwise `Error.
    ///
    /// # Example
    ///
    /// - Assumes one or more GPX file(s) already parsed.
    ///
    /// ```
    /// activities.export_csv("gpx-summary.csv")?;
    /// ```
    pub fn export_csv(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        // Create a buffer for the CSV. Assume that the filename is valid.
        let mut writer = WriterBuilder::new()
            .has_headers(true)
            .from_path(PathBuf::from(filename))?;

        // Go through the activities list
        for curr_activity in &self.activities_list {
            // Write the metadata for each activity
            writer.serialize(&curr_activity.metadata)?;
        }

        writer.flush()?;

        // Return safely
        Ok(())
    }
}

impl Default for GPXActivities {
    /// Sets up the Activities with empty data placeholders.
    fn default() -> Self {
        Self {
            activities_list: Vec::new(),
        }
    }
}
