use std::path::PathBuf;

/// Get the extension part of the filename and return it as a string
///
/// # Parameters
///
/// `filename: &str` -- The filename to get the extension from.
///
/// # Returns
///
/// `String` -- The extension part of the filename.
///
/// # Example
///
/// ```
/// let extension = get_extension("running.gpx");
/// assert_eq!(extension, "gpx".to_string());
/// ```
#[must_use]
pub fn get_extension(filename: &str) -> String {
    std::path::Path::new(&filename)
        .extension()
        .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Change the file extension
///
/// # Parameters
///
/// `filename: &str` -- The filename to change.
/// `extension: &str` -- The new extension to use.
///
/// # Returns
///
/// `String` -- The new filename with the new extension.
///
/// # Example
///
/// ```
/// let new_filename = set_extension("running.gpx", "tcx");
/// assert_eq!(new_filename, "running.tcx".to_string());
/// ```
#[must_use]
pub fn set_extension(filename: &str, extension: &str) -> String {
    let mut filename = PathBuf::from(&filename);
    filename.set_extension(extension);

    String::from(filename.as_os_str().to_str().unwrap_or("unknown"))
}
