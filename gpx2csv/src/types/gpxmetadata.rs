use gpx;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use crate::types::timestamp::TimeStamp;
use crate::types::ExportJSON;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the src_meta information about the file and its contents
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct GpxMetadata {
    pub filename: Option<PathBuf>,
    pub version: Option<String>,
    pub creator: Option<String>,
    pub activity: Option<String>,
    pub description: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub links_href: Option<String>,
    pub links_text: Option<String>,
    pub keywords: Option<String>,
    pub time: Option<TimeStamp>,
    pub copyright_author: Option<String>,
    pub copyright_year: Option<i32>,
    pub copyright_license: Option<String>,
    pub num_waypoints: usize,
    pub num_tracks: usize,
    pub num_routes: usize,
}

impl GpxMetadata {
    /// Initialize Session with default empty values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new Session instance with the filename set from the parameter
    pub fn set_filename(&mut self, filename: &str) {
        self.filename = Some(PathBuf::from(&filename));
    }

    /// Create a new Session instance based on the original metadata and file name.
    pub fn from_header(src: &gpx::Gpx, filename: &str) -> Self {
        let mut dest = Self::new();
        dest.set_filename(&filename);

        // Parse the GPX header
        log::trace!("parsers::parse_gpx_header() -- Parsing the GPX header information.");
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
        if src_meta.links.len() > 0 {
            dest.links_href = Some(src_meta.links[0].href.to_string());
            if let Some(text) = &src_meta.links[0].text {
                dest.links_text = Some(text.to_string());
            }
        }

        // Parse the copyright information if there is any.
        match &src_meta.copyright {
            Some(cr_data) => {
                log::trace!("parsers::parse_gpx_header() -- Copyright information found. Parsing.");
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
                log::trace!("parsers::parse_gpx_header() -- Author information found. Parsing.");
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

        // return the src_meta struct
        dest
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
            copyright_author: None,
            copyright_year: None,
            copyright_license: None,
            num_waypoints: 0,
            num_tracks: 0,
            num_routes: 0,
        }
    }
}

impl ExportJSON for GpxMetadata {
    /// Export the session data to a JSON file using the filename specified in the struct,
    /// with the extension changed to `.session.json`.
    ///
    /// **Parameters:**
    ///
    /// None (`&self` is implied)
    ///
    /// **Returns:**<br>
    ///
    /// Nothing if OK<br>
    /// `Error` if not OK
    fn export_json(&self) -> Result<(), Box<dyn Error>> {
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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Converts the Gpx::Fix struct to a string for easier export
///
/// **Parameters**:
///     `src: &gpx::Fix` a [Gpx Fix](https://docs.rs/gpx/0.8.3/gpx/enum.Fix.html) enum
///
/// **Returns**:
///     `String` -- A String containing the name of the Enum value as a string.
///
/// **Example**:
///
/// ```rust
/// let src: = &gpx::Fix;
///
/// if let Some(fix) = &src.fix {
///    dest.fix = Some(fix_to_string(&fix))
/// }
/// ```
pub fn fix_to_string(src: &gpx::Fix) -> String {
    match src {
        gpx::Fix::None => "None".to_string(),
        gpx::Fix::TwoDimensional => "TwoDimensional".to_string(),
        gpx::Fix::ThreeDimensional => "ThreeDimensional".to_string(),
        gpx::Fix::DGPS => "DGPS".to_string(),
        gpx::Fix::PPS => "PPS".to_string(),
        gpx::Fix::Other(str) => format!("Other({})", str.to_owned()),
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
