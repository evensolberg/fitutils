//! Defines the `Activity` struct which contains the parsed contents of a GPX file, and associated functions.
use chrono::{Local, TimeZone};
use csv::WriterBuilder;
use gpx::Gpx;
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::gpx::gpxmetadata::GPXMetadata;
use crate::gpx::route::GPXRoute;
use crate::gpx::track::GPXTrack;
use crate::gpx::waypoint::GPXWaypoint;
use crate::Duration;

/// High-level construct that contains the entirety of the GPX file
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct GPXActivity {
    /// High-level metadata about the activity such as start time, duration, number of tracks, etc.
    pub metadata: GPXMetadata,

    /// A list of waypoints that we have marked special.
    pub waypoints: Vec<GPXWaypoint>,

    /// A list of routes, each with a list of point-by-point directions.
    pub routes: Vec<GPXRoute>,

    /// A list of tracks with waypoints indicating point-in-time position and other data.
    pub tracks: Vec<GPXTrack>,
}

impl GPXActivity {
    /// Create a new, empty Activity.
    #[must_use]
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
    /// `Result<Self, Box<dyn Error>>` -- Returns an instance of the `Activity` struct if successful, otherwise an `Error`.
    ///
    /// # Errors
    ///
    /// Reading the GPX file can fail. Setting the duration can fail.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::types::activity::Activity;
    ///
    /// let my_activity = Activity::from_file("running.gpx")?;
    /// ```
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let gpx: Gpx = gpx::read(BufReader::new(File::open(filename)?))?;
        log::debug!("activity::from_file() -- gpx.metadata = {:?}", gpx.metadata);
        log::trace!("\nactivity::from_file() -- gpx = {gpx:?}");

        let mut activity = Self::new();

        // Fill the GPX Header info so we can serialize it later
        activity.metadata = GPXMetadata::from_header(&gpx, filename);
        log::trace!(
            "main::run() -- GPX Metadata header: {:?}",
            activity.metadata
        );

        for curr_track in gpx.tracks {
            let mut track = GPXTrack::from_gpx_track(&curr_track, filename);
            track.track_num += 1;
            log::debug!(
                "main::run() -- track::Number of segments: {} / waypoints: {}",
                track.num_segments,
                track.num_waypoints
            );

            log::trace!("\nmain::run() -- track = {track:?}");
            activity.tracks.push(track);
        }

        // Set the total duration to be the sum of the track durations
        activity.set_duration();

        Ok(activity)
    }

    /// Exports all the relevant data for the activity.
    /// Calls the `GpxMetadata::export_json()` function and its own
    /// `export_tracks_csv()` and `export_waypoints_csv()` functions.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    /// # Errors
    ///
    /// Various export functions may return errors which get propagated.
    ///
    /// # Panics
    ///
    /// None.
    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        self.metadata.export_json()?;
        self.export_tracks_csv()?;
        self.export_waypoints_csv()?;

        Ok(())
    }

    /// Iterates through the tracks and calculates a total activity duration.
    /// Should only be used after the track data has been gathered.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    /// # Errors
    ///
    /// Returns an error if there are no tracks from which to read total activity duration.
    ///
    /// # Panics
    ///
    /// None.
    ///
    fn set_duration(&mut self) {
        let mut duration = Duration::from_secs_f64(0.0);

        // If there are no tracks, protest!
        if self.tracks.is_empty() {
            self.metadata.duration = Some(Duration::default());
        } else {
            // Iterate through the tracks
            for track in &self.tracks {
                // If the track has duration data, add it to the total.
                if let Some(trackduration) = track.duration {
                    duration += trackduration;
                }
            }

            // Set the total duration
            self.metadata.duration = Some(duration);
        }
    }

    /// Export the tracks to CSV
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful.
    ///
    /// # Errors
    ///
    /// Creating the `WriterBuilder` may fail. Serializing the track data may fail. Flushing the writer may fail.
    ///
    /// # Panics
    ///
    /// None.
    fn export_tracks_csv(&self) -> Result<(), Box<dyn Error>> {
        let tracks = &self.tracks;
        if tracks.is_empty() {
            return Err("track::export_tracks_csv() -- No Tracks in the Activity.".into());
        }

        // Change the file extension
        let mut outfile = PathBuf::from(
            self.metadata
                .filename
                .as_ref()
                .unwrap_or(&PathBuf::from("export")),
        );
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
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful.
    ///
    /// # Errors
    ///
    /// Errors if there are no tracks in the activity. The `WriterBuilder` may fail. Serialization may fail. Flushing the writer may fail.
    ///
    /// # Panics
    ///
    /// None.
    fn export_waypoints_csv(&self) -> Result<(), Box<dyn Error>> {
        let tracks = &self.tracks;
        if tracks.is_empty() {
            return Err("track::export_waypoints_csv() -- No Tracks in the Activity.".into());
        }

        // Change the file extension
        let mut outfile = PathBuf::from(
            self.metadata
                .filename
                .as_ref()
                .unwrap_or(&PathBuf::from("export")),
        );
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

    /// Prints the metadata information about the activity
    pub fn print(&self, detailed: bool) {
        let unknown = String::new();

        println!(
            "\nFile:              {}",
            self.metadata
                .filename
                .as_ref()
                .unwrap_or(&Path::new("unknown").to_path_buf())
                .to_string_lossy()
        );
        println!(
            "GPX Version:       {}",
            self.metadata.version.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Creator:           {}",
            self.metadata.creator.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Activity:          {}",
            self.metadata.activity.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Time:              {}",
            self.metadata
                .time
                .as_ref()
                .unwrap_or(&Local.timestamp_opt(0, 0).unwrap())
        );
        println!(
            "Duration:          {}",
            self.metadata
                .duration
                .as_ref()
                .unwrap_or(&Duration::default())
        );
        println!(
            "Description:       {}",
            self.metadata.description.as_ref().unwrap_or(&unknown)
        );
        println!("Waypoints:         {}", self.metadata.num_waypoints);
        println!("Tracks:            {}", self.metadata.num_tracks);
        println!("Routes:            {}", self.metadata.num_routes);
        if detailed {
            println!(
                "Author Name:       {}",
                self.metadata.author_name.as_ref().unwrap_or(&unknown)
            );
            println!(
                "Author Email:      {}",
                self.metadata.author_email.as_ref().unwrap_or(&unknown)
            );
            println!(
                "Links Text:        {}",
                self.metadata.links_text.as_ref().unwrap_or(&unknown)
            );
            println!(
                "Links Href:        {}",
                self.metadata.links_href.as_ref().unwrap_or(&unknown)
            );
            println!(
                "Keywords:          {}",
                self.metadata.keywords.as_ref().unwrap_or(&unknown)
            );
            println!(
                "Copyright Author:  {}",
                self.metadata.copyright_author.as_ref().unwrap_or(&unknown)
            );
            println!(
                "Copyright Year:    {}",
                self.metadata.copyright_year.as_ref().unwrap_or(&0)
            );
            println!(
                "Copyright License: {}",
                self.metadata.copyright_license.as_ref().unwrap_or(&unknown)
            );
        }
    }
}

impl Default for GPXActivity {
    /// Sets up the Activity with empty data placeholders.
    fn default() -> Self {
        Self {
            metadata: GPXMetadata::new(),
            waypoints: Vec::new(),
            routes: Vec::new(),
            tracks: Vec::new(),
        }
    }
}
