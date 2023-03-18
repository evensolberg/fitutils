use chrono::{DateTime, Local, TimeZone};
use csv::WriterBuilder;
use serde::Serialize;
use std::error::Error;
use std::path::PathBuf;
use tcx;

use crate::Duration;

/// Holds each Trackpoint as a Record
#[derive(Serialize, Debug, Clone)]
pub struct TCXTrackpoint {
    /// Sport
    pub sport: String,

    /// Activity ID - usually denoted by the start time for the activity
    pub start_time: DateTime<Local>,

    /// Total activity duration in seconds.
    pub time: DateTime<Local>,

    /// How far into the exercise we are
    pub duration: Duration,

    /// Activity Number
    pub activity_num: usize,

    /// Lap Number
    pub lap_num: usize,

    /// Track number within the lap
    pub track_num: usize,

    /// Trackpoint number within the Track
    pub trackpoint_num: usize,

    /// Trackpoint GPS latitude
    pub latitude: Option<f64>,

    /// Trackpoint GPS longitude
    pub longitude: Option<f64>,

    /// Trackpoint GPS altitude in meters
    pub altitude_meters: Option<f64>,

    /// Trackpoint GPS distance in meters from start
    pub distance_meters: Option<f64>,

    /// Heart rate in BPM
    pub heart_rate: Option<f64>,

    /// Cadence in BPM, RPM or SPM.
    pub cadence: Option<u8>,
}

impl TCXTrackpoint {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TCXTrackpoint {
    fn default() -> Self {
        Self {
            sport: String::new(),
            start_time: Local.timestamp(0, 0),
            time: Local.timestamp(0, 0),
            duration: Duration::default(),
            activity_num: 0,
            lap_num: 0,
            track_num: 0,
            trackpoint_num: 0,
            latitude: None,
            longitude: None,
            altitude_meters: None,
            distance_meters: None,
            heart_rate: None,
            cadence: None,
        }
    }
}

/// Contains the list of activity trackpoints from the TCX file
#[derive(Serialize, Debug, Clone, Default)]
pub struct TCXTrackpointList {
    /// The list of individual trackpoints
    pub trackpoints: Vec<TCXTrackpoint>,
}

impl TCXTrackpointList {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            trackpoints: Vec::new(),
        }
    }

    #[must_use]
    pub fn from_activities(activities: &tcx::Activities) -> Self {
        let mut tpl = Self::new();

        let mut a_num = 0;

        for activity in &activities.activities {
            a_num += 1;
            let mut l_num = 0;
            for lap in &activity.laps {
                l_num += 1;
                let mut track_num = 0;
                for track in &lap.tracks {
                    track_num += 1;
                    let mut trackpoint_num = 0;
                    for trackpoint in &track.trackpoints {
                        trackpoint_num += 1;

                        // Extract a new Trackpoint
                        let mut tp = TCXTrackpoint::new();
                        tp.sport = activity.sport.clone();
                        tp.start_time = DateTime::parse_from_rfc3339(&activity.id)
                            .unwrap_or_else(|_| Local.timestamp(0, 0).into())
                            .into();
                        tp.time = trackpoint.time.with_timezone(&chrono::Local);
                        tp.duration = Duration::between(&tp.start_time, &tp.time);
                        tp.activity_num = a_num;
                        tp.lap_num = l_num;
                        tp.track_num = track_num;
                        tp.trackpoint_num = trackpoint_num;
                        if let Some(pos) = &trackpoint.position {
                            tp.latitude = Some(pos.latitude);
                            tp.longitude = Some(pos.longitude);
                        }
                        tp.altitude_meters = trackpoint.altitude_meters;
                        tp.distance_meters = trackpoint.distance_meters;
                        if let Some(hr) = &trackpoint.heart_rate {
                            tp.heart_rate = Some(hr.value);
                        }
                        if let Some(cad) = &trackpoint.cadence {
                            tp.cadence = Some(*cad);
                        }

                        // Push the TP into the list
                        tpl.trackpoints.push(tp);
                    }
                }
            }
        }

        // Return it
        tpl
    }

    /// Export the activity summary as a CSV file
    ///
    /// # Arguments
    ///
    /// `filename: &str` -- the filename into which the contents are to be exported.
    ///
    /// # Returns
    ///
    /// `Ok(())` if everything went well.
    ///
    /// # Errors
    ///
    /// The `WriterBuilder` may fail. Serialization may fail. Writer flush may fail.
    ///
    /// # Panics
    ///
    ///
    ///
    /// # Examples
    ///
    ///
    ///
    pub fn export_csv(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        // Create a buffer for the CSV
        let outfile = PathBuf::from(filename);
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_path(&outfile)?;

        writer.write_record([
            "sport",
            "start_time",
            "time",
            "duration",
            "activity_num",
            "lap_num",
            "track_num",
            "trackpoint_num",
            "latitude",
            "longitude",
            "altitude_meters",
            "distance_meters",
            "heart_rate",
            "cadence",
        ])?;

        for trackpoint in &self.trackpoints {
            log::trace!("TrackpointsList::export_csv() -- serializing: {trackpoint:?}");
            writer.serialize(trackpoint)?;
        }

        log::trace!("TrackpointsList::export_csv() -- information to be written: {writer:?}");

        // Write the file
        writer.flush()?;

        Ok(())
    }
}
