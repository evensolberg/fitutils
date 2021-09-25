use serde_json;
use std::error::Error;
use std::fs::File;

use super::types;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Export the metadata information about the file and its contents to a JSON file
pub fn export_session_json(gpx_data: &types::GpxMetadata) -> Result<(), Box<dyn Error>> {
    // Get the filename from the struct and replace .gpx with .session.json
    let mut filename = gpx_data.filename.as_ref().unwrap().to_path_buf();
    filename.set_extension("session.json");
    log::trace!(
        "exporter::export_session_json() -- Writing JSON file {:?}",
        &filename.to_str()
    );

    // Write the session data to JSON
    serde_json::to_writer_pretty(&File::create(&filename)?, gpx_data)
        .expect("Unable to write session info to JSON file.");

    // Everything is OK
    Ok(())
}
