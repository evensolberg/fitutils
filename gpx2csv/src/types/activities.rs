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

impl Default for Activities {
    /// Sets up the Activities with empty data placeholders.
    fn default() -> Self {
        Self {
            activities_list: Vec::new(),
        }
    }
}
