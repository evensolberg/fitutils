//! Defines the `Activity` struct which contains the parsed contents of a GPX file, and associated functions.
use csv::WriterBuilder;
use gpx::Gpx;
use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

use crate::types::{GpxMetadata, Route, Track, Waypoint};
use utilities::Duration;

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

    /// Read the activity and associated tracks and waypoints from the file given.
    ///
    /// # Parameters
    ///
    /// `filename: &str` -- The name of the GPX file we wish to read.
    ///
    /// # Returns
    ///
    /// `Result<Self, Box<dyn Error>> -- Returns an instance of the `Activity` struct if successful, otherwise an `Error`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::types::activity::Activity;
    ///
    /// let my_activity = Activity::from_file("running.gpx")?;
    /// ```
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let gpx: Gpx = gpx::read(BufReader::new(File::open(&filename)?))?;
        log::debug!("activity::from_file() -- gpx.metadata = {:?}", gpx.metadata);
        log::trace!("\nactivity::from_file() -- gpx = {:?}", gpx);

        let mut activity = Self::new();

        // Fill the GPX Header info so we can serialize it later
        activity.metadata = GpxMetadata::from_header(&gpx, filename)?;
        log::trace!(
            "main::run() -- GPX Metadata header: {:?}",
            activity.metadata
        );

        for curr_track in gpx.tracks {
            let mut track = Track::from_gpx_track(&curr_track, filename)?;
            track.track_num += 1;
            log::debug!(
                "main::run() -- track::Number of segments: {} / waypoints: {}",
                track.num_segments,
                track.num_waypoints
            );

            log::trace!("\nmain::run() -- track = {:?}", track);
            activity.tracks.push(track);
        }

        // Set the total duration to be the sum of the track durations
        activity.set_duration()?;

        Ok(activity)
    }

    /// Exports all the relevant data for the activity.
    /// Calls the `GpxMetadata::export_json()` function and its own
    /// `export_tracks_csv()` and `export_waypoints_csv()` functions.
    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        self.metadata.export_json()?;
        self.export_tracks_csv()?;
        self.export_waypoints_csv()?;

        Ok(())
    }

    /// Iterates through the tracks and calculates a total activity duration.
    /// Should only be used after the track data has been gathered.
    pub fn set_duration(&mut self) -> Result<(), Box<dyn Error>> {
        let mut duration = Duration::from_secs_f64(0.0);

        // If there are no tracks, protest!
        if self.tracks.is_empty() {
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
        if tracks.is_empty() {
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
        if tracks.is_empty() {
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
