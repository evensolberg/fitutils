mod duration;
mod timestamp;

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
