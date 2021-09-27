use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the metadata information about the file and its contents
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
    pub time: Option<DateTime<Utc>>,
    pub copyright_author: Option<String>,
    pub copyright_year: Option<i32>,
    pub copyright_license: Option<String>,
}

impl GpxMetadata {
    /// Initialize Session with default empty values
    pub fn new() -> Self {
        GpxMetadata::default()
    }
}

impl Default for GpxMetadata {
    /// Set defaults to be either empty or zero.
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
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the high-level information about each track
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Track {
    pub activity: Option<String>,
    pub comment: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub links_href: Option<String>,
    pub links_text: Option<String>,
    pub _type: Option<String>,
    // pub segments: Vec<gpx::TrackSegment>,
}

impl Track {
    pub fn new() -> Self {
        Track::default()
    }
}

impl Default for Track {
    /// Set defaults to be either empty or zero.
    fn default() -> Self {
        Track {
            activity: None,
            comment: None,
            description: None,
            source: None,
            links_href: None,
            links_text: None,
            _type: None,
        }
    }
}
