use std::path::PathBuf;

mod duration;
mod fit;
mod gpx;
mod rename_file;
mod tcx;
mod timestamp;

pub use crate::fit::{
    activities::FITActivities, activity::FITActivity, hrzones::FITHrZones, lap::FITLap,
    record::FITRecord, session::FITSession, to_hashmap::fit_to_hashmap,
};

pub use crate::gpx::{
    activities::GPXActivities, activity::GPXActivity, gpxmetadata::GPXMetadata, link::GPXLink,
    route::GPXRoute, to_hashmap::gpx_to_hashmap, track::GPXTrack, waypoint::GPXWaypoint,
};

pub use crate::tcx::{
    activities::{TCXActivitiesList, TCXActivitiesSummary},
    to_hashmap::tcx_to_hashmap,
    trackpoints::{TCXTrackpoint, TCXTrackpointList},
};

pub use crate::{duration::Duration, rename_file::rename_file, timestamp::TimeStamp};

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
