use csv::WriterBuilder;
use std::error::Error;
use std::path::PathBuf;

use crate::types::activity::Activity;

/// Export the tracks to CSV
pub fn export_tracks_csv(activity: &Activity) -> Result<(), Box<dyn Error>> {
    let tracks = &activity.tracks;
    if tracks.len() == 0 {
        return Err("track::export_tracks_csv() -- No Tracks in the Activity.".into());
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

pub fn export_waypoints_csv(activity: &Activity) -> Result<(), Box<dyn Error>> {
    let tracks = &activity.tracks;
    if tracks.len() == 0 {
        return Err("track::export_waypoints_csv() -- No Tracks in the Activity.".into());
    }

    // Change the file extension
    let mut outfile = PathBuf::from(activity.metadata.filename.as_ref().unwrap());
    outfile.set_extension("waypointss.csv");

    // Create a buffer for the CSV
    let mut writer = WriterBuilder::new().has_headers(true).from_path(outfile)?;

    // Export the tracks sans the waypoints
    for curr_track in tracks {
        for curr_wpt in &curr_track.waypoints {
            writer.serialize(curr_wpt)?;
        }
    }

    writer.flush()?;

    // Return safely
    Ok(())
}
