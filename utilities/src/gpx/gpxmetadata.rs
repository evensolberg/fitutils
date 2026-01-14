use chrono::{DateTime, Datelike, Local};
/// Defines the `GpxMetadata` struct whih holds the metadata information about the file and its contents, with associated functions.
use gpx;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::PathBuf};

use crate::set_string_field; // From the macros crate.
use crate::Duration;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the metadata information about the file and its contents
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct GPXMetadata {
    /// The name of the GPX file from which the information was read.
    pub filename: Option<PathBuf>,

    /// Gpx version used (`Gpx10`, `Gpx11`, or `Unknown`) in this file.
    pub version: Option<String>,

    /// Creator name or URL of the software that created the GPX document.
    pub creator: Option<String>,

    /// The name of the GPX file -- this usually corresponds to an activity.
    pub activity: Option<String>,

    /// A description of the contents of the GPX file.
    pub description: Option<String>,

    /// The name of the person or organization who created the GPX file.
    pub author_name: Option<String>,

    /// The email address for the person or organization who created the GPX file.
    pub author_email: Option<String>,

    /// The first URL associated with the location described in the file.
    pub links_href: Option<String>,

    /// The descriptive text for the first URL associated with this file.
    pub links_text: Option<String>,

    /// Keywords associated with the file. Search engines or databases can use this information to classify the data.
    pub keywords: Option<String>,

    /// The creation date of the file.
    pub time: Option<DateTime<Local>>,

    /// The total duration of the activities found in this file.
    pub duration: Option<Duration>,

    /// The name of the person or company the holds the copyright for this GPX file.
    pub copyright_author: Option<String>,

    /// The year the copyright for this file was put in place.
    pub copyright_year: Option<i32>,

    /// The license terms for the GPX file.
    pub copyright_license: Option<String>,

    /// The total number of waypoints (in tracks) found in this GPX file.
    pub num_waypoints: usize,

    /// The number of tracks found in this file.
    pub num_tracks: usize,

    /// The number of routes found in this file.
    pub num_routes: usize,
}

impl GPXMetadata {
    /// Initialize Session with default empty values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `filename` field in the struct to the value provided.
    ///
    /// # Parameters
    ///
    /// `filename: &str` -- The name of the GPX file being read.
    ///
    /// # Example
    ///
    /// ```
    /// let input_file = "running.gpx";
    ///
    /// gpx_meta.set_filename(input_file);
    /// ```
    pub fn set_filename(&mut self, filename: &str) {
        self.filename = Some(PathBuf::from(&filename));
    }

    /// Create a new Session instance based on the original metadata and file name.
    ///
    /// # Arguments
    ///
    /// `src: &gpx::Gpx` -- The original contents of the GPX file being parsed.
    ///
    /// `filename: &str` -- The filename of the GPX file being read.
    ///
    /// # Returns
    ///
    /// `Self` -- A `GPXMetadata` struct filled with the metadata and copyright contents from the original `Gpx` struct.
    ///
    /// # Panics
    ///
    /// If somehow the `src` argument doesn't contain metadata, things could panic. This is highly unlikely. Ditto if there are no tracks.
    #[must_use]
    pub fn from_header(src: &gpx::Gpx, filename: &str) -> Self {
        let mut dest = Self::new();
        dest.set_filename(filename);

        // Parse the GPX header
        dest.version = Some(src.version.to_string());
        set_string_field!(src, creator, dest);

        // Parse the source metadata
        let md_new = gpx::Metadata::default();
        let src_meta = src.metadata.as_ref().unwrap_or(&md_new);
        set_activity(src_meta, &mut dest, src);

        set_string_field!(src_meta, description, dest);
        set_string_field!(src_meta, keywords, dest);
        set_time(src_meta, &mut dest);

        // For now, only read the first href in the list of links (if there is one)
        set_link(src_meta, &mut dest);

        // Parse the copyright information if there is any.
        match &src_meta.copyright {
            Some(cr_data) => {
                set_string_field!(cr_data, author, dest, copyright_author);
                set_string_field!(cr_data, license, dest, copyright_license);

                if let Some(year) = cr_data.year {
                    dest.copyright_year = Some(year);
                }
            }
            None => {
                log::trace!(
                    "parsers::parse_gpx_header() -- Copyright information not found. No need to parse."
                );
            }
        }

        set_copyright_year(&mut dest);

        // Parse src_meta.author if there is anything there.
        match &src_meta.author {
            Some(author) => {
                set_string_field!(author, name, dest, author_name);
                set_string_field!(author, email, dest, author_email);
            }
            None => {
                log::trace!(
                    "parsers::parse_gpx_header() -- Author information not found. No need to parse."
                );
            }
        }

        // Find the number of waypoints, tracks and segments
        dest.num_waypoints = src.waypoints.len();
        dest.num_tracks = src.tracks.len();
        dest.num_routes = src.routes.len();

        log::debug!("GpxMetadata::from_header() -- Metadata: {dest:?}");

        // return the src_meta struct
        dest
    }

    /// Export the session data to a JSON file using the filename specified in the struct,
    /// with the extension changed to `.session.json`.
    ///
    /// # Parameters
    ///
    /// None (`&self` is implied)
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful.
    ///
    /// # Errors
    ///
    /// Writing the session data may fail.
    pub fn export_json(&self) -> Result<(), Box<dyn Error>> {
        let mut filename = self
            .filename
            .as_ref()
            .unwrap_or(&PathBuf::from("export"))
            .clone();
        filename.set_extension("session.json");
        log::trace!(
            "exporter::export_session_json() -- Writing JSON file {:?}",
            &filename.to_str()
        );

        // Write the session data to JSON
        serde_json::to_writer_pretty(&File::create(&filename)?, &self)?;

        Ok(())
    }
}

/// Sets the copyright year based on the timestamp found in the metadata. If the
/// timestamp is not found, set it to the year the activity started. If that isn't found either,
/// set it to the current year.
fn set_copyright_year(dest: &mut GPXMetadata) {
    // Debatable if this is kosher, but I'm going with it for now.
    // If the copyright year is none, set it to the year the activity started.
    if dest.copyright_year.is_none() {
        let year = dest.time.as_ref().unwrap_or(&Local::now()).year();
        log::debug!("copyright_year = {year}");
        dest.copyright_year = Some(year);
    }
}

/// Sets the links field in the `GPXMetadata` struct based on the links in the `gpx::Metadata` struct.
/// Only the first link is used.
///
/// # Arguments
///
/// - `src_meta: &gpx::Metadata` -- The metadata from the original GPX file.
/// - `dest: &mut GPXMetadata` -- The destination `GPXMetadata` struct, which gets modified.
fn set_link(src_meta: &gpx::Metadata, dest: &mut GPXMetadata) {
    if !src_meta.links.is_empty() {
        dest.links_href = Some(src_meta.links[0].href.clone());
        if let Some(text) = &src_meta.links[0].text {
            dest.links_text = Some(text.clone());
        }
    }
}

/// Sets the time field in the `GPXMetadata` struct based on the time in the `gpx::Metadata` struct.
///
/// # Arguments
///
/// - `src_meta: &gpx::Metadata` -- The metadata from the original GPX file.
/// - `dest: &mut GPXMetadata` -- The destination `GPXMetadata` struct, which gets modified.
fn set_time(src_meta: &gpx::Metadata, dest: &mut GPXMetadata) {
    if let Some(time) = &src_meta.time {
        let t = time.format().unwrap_or_default();

        if let Ok(ltz_w) = DateTime::parse_from_rfc3339(t.as_str()) {
            let ltz = ltz_w.with_timezone(&Local);
            dest.time = Some(ltz);
        } else {
            log::warn!("Unable to parse the time from the header. Yikes.");
        }
    }
}

/// Set the activity based on either the activity from the metadata,
/// or if that's not found, from the first track activity.
///
/// # Arguments
///
/// - `src_meta: &gpx::Metadata` -- The metadata from the original GPX file.
/// - `dest: &mut GPXMetadata` -- The destination `GPXMetadata` struct, which gets modified.
/// - `src: &gpx::Gpx` -- The original contents of the GPX file being parsed.
fn set_activity(src_meta: &gpx::Metadata, dest: &mut GPXMetadata, src: &gpx::Gpx) {
    if let Some(activity) = &src_meta.name {
        dest.activity = Some(activity.clone());
    } else if src
        .tracks
        .first()
        .unwrap_or(&gpx::Track::default())
        .name
        .is_some()
    {
        dest.activity = Some(
            src.tracks
                .first()
                .unwrap_or(&gpx::Track::default())
                .name
                .as_ref()
                .unwrap_or(&"Unknown".to_string())
                .clone(),
        );
    }
}
