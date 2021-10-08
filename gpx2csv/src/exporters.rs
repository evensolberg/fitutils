use csv::WriterBuilder;
use std::error::Error;
use std::path::PathBuf;

use crate::types::activity::Activity;

/// Export the tracks to CSV
pub fn export_tracks_csv(activity: &Activity) -> Result<(), Box<dyn Error>> {
    let tracks = &activity.tracks;
    if tracks.len() == 0 {
        return Err("track::export_csv() -- No Tracks in the Activity.".into());
    }

    // Change the file extension
    let mut outfile = PathBuf::from(activity.metadata.filename.as_ref().unwrap());
    outfile.set_extension("tracks.csv");

    // Create a buffer for the CSV
    let mut writer = WriterBuilder::new().has_headers(true).from_path(outfile)?;

    // Export the tracks sans the waypoints
    for curr_track in tracks {
        writer.serialize(curr_track)?;
    }

    writer.flush()?;

    // Return safely
    Ok(())
}
