/// Defines the `GpxMetadata` struct whih holds the metadata information about the file and its contents, with associated functions.
use gpx;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use crate::types::duration::Duration;
use crate::types::timestamp::TimeStamp;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the metadata information about the file and its contents
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct GpxMetadata {
    /// THe name of the GPX file from which the information was read.
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
    pub time: Option<TimeStamp>,

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

impl GpxMetadata {
    /// Initialize Session with default empty values
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
    /// # Parameters
    ///
    /// `src: &gpx::Gpx` -- The original contents of the GPX file being parsed.
    ///
    /// `filename: &str` -- The filename of the GPX file being read.
    ///
    /// # Returns
    ///
    /// `Self` -- A `GpxMetadata` struct filled with the metadata and copyright contents from the orginal `Gpx` struct.
    pub fn from_header(src: &gpx::Gpx, filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut dest = Self::new();
        dest.set_filename(filename);

        // Parse the GPX header
        dest.version = Some(gpx_ver_to_string(&src.version));
        if let Some(creator) = &src.creator {
            dest.creator = Some(creator.to_string());
        }

        // Parse the source metadata
        let src_meta = src.metadata.as_ref().unwrap();
        if let Some(activity) = &src_meta.name {
            dest.activity = Some(activity.to_string());
        }
        if let Some(description) = &src_meta.description {
            dest.description = Some(description.to_string());
        }
        if let Some(keywords) = &src_meta.keywords {
            dest.keywords = Some(keywords.to_string());
        }
        if let Some(time) = &src_meta.time {
            dest.time = Some(TimeStamp(*time));
        }

        // For now, only read the first href in the list of links (if there is one)
        if !src_meta.links.is_empty() {
            dest.links_href = Some(src_meta.links[0].href.to_string());
            if let Some(text) = &src_meta.links[0].text {
                dest.links_text = Some(text.to_string());
            }
        }

        // Parse the copyright information if there is any.
        match &src_meta.copyright {
            Some(cr_data) => {
                if let Some(author) = &cr_data.author {
                    dest.copyright_author = Some(author.to_string())
                };
                if let Some(license) = &cr_data.license {
                    dest.copyright_license = Some(license.to_string())
                };
                if let Some(year) = cr_data.year {
                    dest.copyright_year = Some(year)
                };
            }
            None => {
                log::trace!(
                    "parsers::parse_gpx_header() -- Copyright information not found. No need to parse."
                );
            }
        }

        // Parse src_meta.author if there is anything there.
        match &src_meta.author {
            Some(author) => {
                if let Some(name) = &author.name {
                    dest.author_name = Some(name.to_string())
                };
                if let Some(email) = &author.email {
                    dest.author_email = Some(email.to_string())
                };
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

        log::debug!("GpxMetadata::from_header() -- Metadata: {:?}", dest);

        // return the src_meta struct
        Ok(dest)
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
    /// Nothing if OK, otherwise `Error`.
    pub fn export_json(&self) -> Result<(), Box<dyn Error>> {
        let mut filename = self.filename.as_ref().unwrap().to_path_buf();
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

impl Default for GpxMetadata {
    /// Set defaults to be either `None` or zero.
    fn default() -> Self {
        GpxMetadata {
            filename: None,
            version: None,
            creator: None,
            activity: None,
            description: None,
            author_name: None,
            author_email: None,
            links_href: None,
            links_text: None,
            keywords: None,
            time: None,
            duration: None,
            copyright_author: None,
            copyright_year: None,
            copyright_license: None,
            num_waypoints: 0,
            num_tracks: 0,
            num_routes: 0,
        }
    }
}

/// Turn the `gpx::GpxVersion` enum into a string
pub fn gpx_ver_to_string(version: &gpx::GpxVersion) -> String {
    match version {
        gpx::GpxVersion::Gpx10 => "Gpx10".to_string(),
        gpx::GpxVersion::Gpx11 => "Gpx11".to_string(),
        _ => "unknown".to_string(),
    }
}
