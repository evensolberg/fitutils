use crate::types::gpxmetadata::GpxMetadata;
use crate::types::route::Route;
use crate::types::track::Track;
use crate::types::waypoint::Waypoint;

/// High-level construct that contains the entirety of the GPX file
#[derive(Debug)]
pub struct Activity {
    pub metadata: GpxMetadata,
    pub waypoints: Vec<Waypoint>,
    pub routes: Vec<Route>,
    pub tracks: Vec<Track>,
}

impl Activity {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Activity {
    fn default() -> Self {
        Activity {
            metadata: GpxMetadata::new(),
            waypoints: Vec::new(),
            routes: Vec::new(),
            tracks: Vec::new(),
        }
    }
}
