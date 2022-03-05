mod build_logs;
mod duration;
mod extensions;
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
    activity::{TCXActivitiesList, TCXActivity},
    to_hashmap::tcx_to_hashmap,
    trackpoints::{TCXTrackpoint, TCXTrackpointList},
};

pub use crate::{
    build_logs::build_log,
    duration::Duration,
    extensions::{get_extension, set_extension},
    rename_file::rename_file,
    timestamp::TimeStamp,
};
