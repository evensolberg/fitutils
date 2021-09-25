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
    pub author: Option<String>,
    pub links_href: Option<String>,
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
            author: None,
            links_href: None,
            time: None,
            copyright_author: None,
            copyright_year: None,
            copyright_license: None,
        }
    }
}
