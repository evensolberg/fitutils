use std::path::PathBuf;

mod duration;
mod fit;
mod gpx;
mod tcx;
mod timestamp;

pub use crate::fit::{
    activities::FITActivities, activity::FITActivity, hrzones::FITHrZones, lap::FITLap,
    record::FITRecord, session::FITSession,
};

pub use fit::constfunc::*;

pub use crate::gpx::activities::GPXActivities;
pub use crate::gpx::activity::GPXActivity;
pub use crate::gpx::gpxmetadata::GPXMetadata;
pub use crate::gpx::link::GPXLink;
pub use crate::gpx::route::GPXRoute;
pub use crate::gpx::track::GPXTrack;
pub use crate::gpx::waypoint::GPXWaypoint;

pub use crate::tcx::activities::{TCXActivitiesList, TCXActivitiesSummary};
pub use crate::tcx::trackpoints::{TCXTrackpoint, TCXTrackpointList};
pub use crate::{duration::Duration, timestamp::TimeStamp};

////////////////////////////////////////////////////////////////////////////////
/// Get the extension part of the filename and return it as a string
pub fn get_extension(filename: &str) -> String {
    std::path::Path::new(&filename)
        .extension()
        .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Change the file extension
pub fn set_extension(filename: &str, extension: &str) -> String {
    let mut filename = PathBuf::from(&filename);
    filename.set_extension(&extension);

    String::from(filename.as_os_str().to_str().unwrap_or("unknown"))
}
