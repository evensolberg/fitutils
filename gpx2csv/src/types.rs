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
            filename: Some(PathBuf::new()),
            version: Some("".to_string()),
            creator: Some("".to_string()),
            activity: Some("".to_string()),
            description: Some("".to_string()),
            author: Some("".to_string()),
            links_href: Some("".to_string()),
            time: Some(chrono::offset::Utc::now()),
        }
    }
}
