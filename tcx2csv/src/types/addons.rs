use std::path::PathBuf;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Change the file extension
pub fn set_extension(filename: &str, extension: &str) -> String {
    let mut filename = PathBuf::from(&filename);
    filename.set_extension(&extension);

    String::from(filename.as_os_str().to_str().unwrap_or("unknown"))
}
