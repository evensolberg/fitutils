//! Defines types used in the collection and serialization of the GPX data.
pub use crate::types::{
    activities::Activities, activity::Activity, duration::Duration, gpxmetadata::GpxMetadata,
    link::Link, route::Route, timestamp::TimeStamp, track::Track, waypoint::Waypoint,
};

// Submodules
mod activities;
mod activity;
mod duration;
// mod fix; // Currently not used.
mod gpxmetadata;
mod link;
// mod person; // Currently not used.
mod route;
mod timestamp;
mod track;
mod waypoint;
