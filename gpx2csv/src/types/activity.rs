use csv::WriterBuilder;
use std::error::Error;
use std::path::PathBuf;

use crate::types::duration::Duration;
use crate::types::gpxmetadata::GpxMetadata;
use crate::types::route::Route;
use crate::types::track::Track;
use crate::types::waypoint::Waypoint;

/// High-level construct that contains the entirety of the GPX file
#[derive(Debug)]
pub struct Activity {
    /// High-level metadata about the activity such as start time, duration, number of tracks, etc.
    pub metadata: GpxMetadata,
    /// A list of waypoints that we have marked special.
    pub waypoints: Vec<Waypoint>,
    /// A list of routes, each with a list of point-by-point directions.
    pub routes: Vec<Route>,
    /// A list of tracks with waypoints indicating point-in-time position and other data.
    pub tracks: Vec<Track>,
}

impl Activity {
    /// Create a new, empty Activity.
    pub fn new() -> Self {
        Self::default()
    }

    /// Iterates through the tracks and calculates a total activity duration.
    /// Should only be used after the track data has been gathered.
    pub fn set_duration(&mut self) -> Result<(), Box<dyn Error>> {
        let mut duration = Duration::from_secs_f64(0.0);

        // If there are no tracks, protest!
        if self.tracks.len() == 0 {
            return Err("Activity::set_total_duration() -- Unable to set Activity duration. No tracks found.".into());
        }

        // Iterate through the tracks
        for track in &self.tracks {
            // If the track has duration data, add it to the total.
            if let Some(trackduration) = track.duration {
                duration += trackduration;
            }
        }

        // Set the total duration
        self.metadata.duration = Some(duration);

        // Return safely
        Ok(())
    }

    /// Export the tracks to CSV
    pub fn export_tracks_csv(&self) -> Result<(), Box<dyn Error>> {
        let tracks = &self.tracks;
        if tracks.len() == 0 {
            return Err("track::export_tracks_csv() -- No Tracks in the Activity.".into());
        }

        // Change the file extension
        let mut outfile = PathBuf::from(self.metadata.filename.as_ref().unwrap());
        outfile.set_extension("tracks.csv");

        // Create a buffer for the CSV
        let mut writer = WriterBuilder::new().has_headers(true).from_path(outfile)?;

        // Export the tracks sans the waypoints
        for curr_track in tracks {
            writer.serialize(curr_track)?;
        }

        writer.flush()?;

        // Return safely
        Ok(())
    }

    /// Export all the waypoints for each track to CSV
    pub fn export_waypoints_csv(&self) -> Result<(), Box<dyn Error>> {
        let tracks = &self.tracks;
        if tracks.len() == 0 {
            return Err("track::export_waypoints_csv() -- No Tracks in the Activity.".into());
        }

        // Change the file extension
        let mut outfile = PathBuf::from(self.metadata.filename.as_ref().unwrap());
        outfile.set_extension("waypoints.csv");

        // Create a buffer for the CSV
        let mut writer = WriterBuilder::new().has_headers(true).from_path(outfile)?;

        // Export the tracks sans the waypoints
        for curr_track in tracks {
            for curr_wpt in &curr_track.waypoints {
                writer.serialize(curr_wpt)?;
            }
        }

        writer.flush()?;

        // Return safely
        Ok(())
    }
}

impl Default for Activity {
    /// Sets up the Activity with empty data placeholders.
    fn default() -> Self {
        Activity {
            metadata: GpxMetadata::new(),
            waypoints: Vec::new(),
            routes: Vec::new(),
            tracks: Vec::new(),
        }
    }
}
