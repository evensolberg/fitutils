mod duration;
mod tcx;
mod timestamp;

pub use crate::tcx::activities::{TCXActivitiesList, TCXActivitiesSummary};
pub use crate::tcx::trackpoints::{TCXTrackpoint, TCXTrackpointList};
pub use crate::{duration::Duration, timestamp::TimeStamp};
use std::path::PathBuf;

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
