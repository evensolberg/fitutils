//! Defines types used in the collection and serialization of the GPX data.
pub use crate::types::{
    activities::Activities, activity::Activity, gpxmetadata::GpxMetadata, link::Link, route::Route,
    track::Track, waypoint::Waypoint,
};

// Submodules
mod activities;
mod activity;
// mod fix; // Currently not used.
mod gpxmetadata;
mod link;
// mod person; // Currently not used.
mod route;
mod track;
mod waypoint;
