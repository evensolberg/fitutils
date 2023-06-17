//! Defines the `Track` struct which holds information about each track, including summary data and waypoint details.

use serde::Serialize;
use std::path::PathBuf;

use chrono::{DateTime, Local};

use crate::gpx::waypoint::GPXWaypoint;
use crate::set_string_field; // from the macros crate
use crate::Duration; // from the macros crate

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the information about each track. Includes summary data and the details of each waypoint in the track.
#[derive(Serialize, Debug, Default)]
#[serde(default)]
#[allow(clippy::module_name_repetitions)]
pub struct GPXTrack {
    /// The original file name containing the track
    pub filename: Option<PathBuf>,

    /// The track number if the overall file. Often cordestponds to Lap Number.
    pub track_num: usize,

    /// GPS name of track.
    pub name: Option<String>,

    /// Start time for the track
    pub start_time: Option<DateTime<Local>>,

    /// Duration for the track
    pub duration: Option<Duration>,

    /// GPS comment for track.
    pub comment: Option<String>,

    /// User description of track.
    pub description: Option<String>,

    /// Source of data. Included to give user some idea of reliability and accuracy of data.
    pub source: Option<String>,

    /// The URL of the first link to external information about the track.
    pub links_href: Option<String>,

    /// The description of the first link to external information about the track.
    pub links_text: Option<String>,

    /// Type (classification) of track.
    pub t_type: Option<String>,

    /// Number of track segments within this track
    pub num_segments: usize,

    /// Total number of waypoints within this track
    pub num_waypoints: usize,

    /// The list of waypoints in this track (not serialized)
    #[serde(skip)] // Do not serialize - we'll handle it in the export. Maybe.
    pub waypoints: Vec<GPXWaypoint>,
}

impl GPXTrack {
    /// Sets the filename in the `Track` struct.
    ///
    /// # Parameters
    ///
    /// `filename: &str` -- The file name of the GPX file from which the track is taken.
    ///
    /// # Example
    ///
    /// ```
    /// track.set_filename("running.gpx");
    pub fn set_filename(&mut self, filename: &str) {
        self.filename = Some(PathBuf::from(filename));
    }

    /// Returns a new `Track` instance with data filled from the parameters given.
    ///
    /// # Parameters
    ///
    /// `src: &gpx::Track` -- A `gpx::Track` struct from the original file.
    ///
    /// `filename: &str` -- The name of the GPX file from which the Track was taken.
    ///
    /// # Returns
    ///
    /// `Self` -- A `Track` struct instance.
    ///
    /// # Example
    ///
    /// ```text
    /// for curr_track in gpx.tracks {
    ///   let mut track = Track::from_gpx_track(&curr_track, filename);
    ///   ...
    /// ```
    #[must_use]
    #[allow(clippy::used_underscore_binding)]
    pub fn from_gpx_track(src: &gpx::Track, filename: &str) -> Self {
        let mut dest = Self::default();
        dest.set_filename(filename);

        set_string_field!(src, name, dest);
        set_string_field!(src, comment, dest);
        set_string_field!(src, description, dest);
        set_string_field!(src, source, dest);
        set_string_field!(src, type_, dest, t_type);

        // See if we have links
        get_links(src, &mut dest);

        // Count the number of segments
        dest.num_segments = src.segments.len();

        // Get the list of segments and their waypoints
        get_waypoints(src, &mut dest);

        // Set the start time and duration
        set_times(&mut dest);

        dest
    }
}

/// Reads the waypoints from the original file and adds them to the `Track` struct.
///
/// # Arguments
///
/// - `src: &gpx::Track` -- The original `gpx::Track` struct.
/// - `dest: &mut Track` -- The `Track` struct to which the waypoints are added.
fn get_waypoints(src: &gpx::Track, dest: &mut GPXTrack) {
    let mut segnum: usize = 0;
    for curr_seg in &src.segments {
        segnum += 1;
        let mut wptnum: usize = 0;
        for curr_wpt in &curr_seg.points {
            let mut wpt = GPXWaypoint::from_gpx_waypoint(curr_wpt);
            wptnum += 1;
            wpt.segment_num = segnum;
            wpt.waypoint_mum = wptnum;
            dest.waypoints.push(wpt);
        }
    }

    dest.num_waypoints = dest.waypoints.len();
    log::debug!(
        "from_gpx_track() -- dest.num_waypoints = {}",
        dest.num_waypoints
    );
}

/// Sets the start time and duration for the track based on the waypoints in the track. Note that this needs the number of waypoints to be set.
///
/// # Arguments
///
/// - `dest: &mut Track` -- The `Track` struct to which the times are added. This also contains the waypoint information.
fn set_times(dest: &mut GPXTrack) {
    if dest.num_waypoints > 0 {
        let t_now = Local::now();

        // Get the start time
        let t_start = dest.waypoints[0].time.unwrap_or(t_now);
        dest.start_time = Some(t_start);

        // Get the end time
        let t_finish = &dest.waypoints[dest.num_waypoints - 1]
            .time
            .as_ref()
            .unwrap_or(&t_now);

        dest.duration = Some(Duration::between(&t_start, t_finish));
    }
}

/// Sets the links in the `Track` struct, if the source `gpx::Track` has any.
///
/// # Arguments
///
/// - `src: &gpx::Track` -- The source `gpx::Track` struct.
/// - `dest: &mut GPXTrack` -- The destination `GPXTrack` struct.
fn get_links(src: &gpx::Track, dest: &mut GPXTrack) {
    if !src.links.is_empty() {
        dest.links_href = Some(src.links[0].href.to_string());
        if let Some(text) = &src.links[0].text {
            dest.links_text = Some(text.to_string());
        }
    }
}
