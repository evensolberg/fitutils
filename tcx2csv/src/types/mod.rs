//! Defines types used in the collection and serialization of the TCX data.
pub use crate::types::activities_summary::{ActivitiesList, ActivitiesSummary};
pub use crate::types::addons::set_extension;
pub use crate::types::duration::Duration;
pub use crate::types::timestamp::TimeStamp;
pub use crate::types::trackpoints::{Trackpoint, TrackpointList};

mod activities_summary;
mod addons;
mod duration;
mod timestamp;
mod trackpoints;
