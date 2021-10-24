use csv::WriterBuilder;
use serde::Serialize;
use serde_json;
use std::error::Error;
use std::path::PathBuf;
use tcx;

use crate::types::addons::set_extension;

/// Holds a summary of the activities in the file
#[derive(Serialize, Debug, Clone)]
pub struct ActivitiesSummary {
    /// Filename of the original file from which the data was read
    pub filename: String,

    /// Number of activities in the file - typically 1
    pub num_activities: usize,

    /// Sport
    pub sport: String,

    /// Activity ID - usually denoted by the start time for the activity
    pub start_time: String,

    /// Total activity duration in seconds.
    pub total_time_seconds: f64,

    /// Notes - if there are any
    pub notes: Option<String>,

    /// Number of laps within the activity
    pub num_laps: usize,

    /// Total number of tracks within the activity
    pub num_tracks: usize,

    /// Total number of trackpoints within the activity
    pub num_trackpoints: usize,

    /// Total distance covered during the lap in meters.
    pub distance_meters: f64,

    /// Max ascent in meters from start
    pub start_altitude: f64,

    /// Max ascent in meters from start
    pub max_altitude: f64,

    /// Max ascent in meters from start
    pub ascent_meters: f64,

    /// Average speed in Meters/Second for the activity
    pub average_speed: f64,

    /// Maximum speed in Meters/Second obtained during the activity.
    pub maximum_speed: f64,

    /// Number of calories burned during the activity.
    pub calories: u16,

    /// Average heart rate in Beats per Minute (BPM) for the activity
    pub average_heart_rate: f64,

    /// Maximum heart rate in Beats per Minute (BPM) for the activity
    pub maximum_heart_rate: f64,

    /// Average cadence (typically in Steps, Revolutions or Strokes per Minute) for the activity.
    pub average_cadence: f64,

    /// Maximum cadence (typically in Steps, Revolutions or Strokes per Minute) for the activity.
    pub maximum_cadence: u8,
}

impl ActivitiesSummary {
    /// Create a new, empty Activities Summary
    pub fn new() -> Self {
        ActivitiesSummary::default()
    }

    /// Generates a summary from a set of activities in the TCX file.
    /// Assumes that tcx::TrainingCenterDatabase::calc_heartrates() has been run.
    pub fn from_activities(activities: &tcx::Activities) -> Self {
        let mut act_s = Self::new();

        let mut hr: f64 = 0.0;
        let mut cad: f64 = 0.0;

        // Find the altitude of the very first TrackPoint
        if let Some(act) = activities.activities.first() {
            if let Some(lap) = act.laps.first() {
                if let Some(track) = lap.tracks.first() {
                    if let Some(tp) = track.trackpoints.first() {
                        if let Some(num) = tp.altitude_meters {
                            act_s.start_altitude = num;
                            act_s.max_altitude = num;
                        }
                    }
                }
            }
        }

        for activity in &activities.activities {
            act_s.num_activities += 1;
            act_s.sport = activity.sport.clone();
            act_s.start_time = activity.id.clone(); // TODO: https://github.com/evensolberg/fitparser/projects/6#card-71437698
            act_s.notes = activity.notes.clone();

            for lap in &activity.laps {
                act_s.num_laps += 1;
                act_s.total_time_seconds += lap.total_time_seconds;
                act_s.distance_meters += lap.distance_meters;
                act_s.calories += lap.calories;
                if let Some(max_speed) = lap.maximum_speed {
                    if act_s.maximum_speed < max_speed {
                        act_s.maximum_speed = max_speed;
                    }
                }

                for track in &lap.tracks {
                    act_s.num_tracks += 1;
                    act_s.num_trackpoints += track.trackpoints.len();
                    // Check to see if max HR for the lap > current recorded max
                    if let Some(mhr) = lap.maximum_heart_rate {
                        if act_s.maximum_heart_rate < mhr {
                            act_s.maximum_heart_rate = mhr;
                        }
                    }

                    for trackpoint in &track.trackpoints {
                        // Check if there is a cadence and if it's greater than the current max
                        if let Some(curr_cad) = trackpoint.cadence {
                            if act_s.maximum_cadence < curr_cad {
                                act_s.maximum_cadence = curr_cad;
                            }
                            cad += curr_cad as f64;
                        }

                        // Check if there is a heart rate and record it
                        if let Some(curr_hr) = &trackpoint.heart_rate {
                            hr += curr_hr.value;
                        }

                        // Check if there is altitude data and calculate
                        if let Some(altitude) = trackpoint.altitude_meters {
                            if act_s.max_altitude < altitude {
                                act_s.max_altitude = altitude;
                            }
                        }
                    }
                }
            }
        }

        act_s.ascent_meters = act_s.max_altitude - act_s.start_altitude;
        if act_s.total_time_seconds != 0.0 {
            act_s.average_speed = act_s.distance_meters / act_s.total_time_seconds;
        }

        // Calculate averages for the whole activity set
        if act_s.num_trackpoints > 0 {
            act_s.average_cadence = cad / act_s.num_trackpoints as f64;
            act_s.average_heart_rate = hr / act_s.num_trackpoints as f64;
        }
        // return it
        act_s
    } // pub fn from_activities

    /// Export the activity summary as a JSON file
    pub fn export_json(&self) -> Result<(), Box<dyn Error>> {
        if self.filename.is_empty() {
            return Err("No filename specified in the ActivitySummary. Unable to export.".into());
        }

        let out_file = set_extension(&self.filename, "activity.json");
        serde_json::to_writer_pretty(
            &std::fs::File::create(&std::path::PathBuf::from(&out_file))?,
            &self,
        )?;

        Ok(())
    }
}

impl Default for ActivitiesSummary {
    /// Sets up the ActivitiesSummary with defaults or empty fields
    fn default() -> Self {
        Self {
            filename: "".to_string(),
            num_activities: 0,
            sport: "".to_string(),
            start_time: "".to_string(),
            total_time_seconds: 0.0,
            notes: None,
            num_laps: 0,
            num_tracks: 0,
            num_trackpoints: 0,
            distance_meters: 0.0,
            start_altitude: 0.0,
            max_altitude: 0.0,
            ascent_meters: 0.0,
            average_speed: 0.0,
            maximum_speed: 0.0,
            calories: 0,
            average_heart_rate: 0.0,
            maximum_heart_rate: 0.0,
            average_cadence: 0.0,
            maximum_cadence: 0,
        }
    }
}

/// A list of the summarized activities
#[derive(Serialize, Debug)]
pub struct ActivitiesList {
    /// The list of activities
    pub activities: Vec<ActivitiesSummary>,
}

impl ActivitiesList {
    /// Create a new, empty `ActivitiesList`.
    pub fn new() -> Self {
        Self {
            activities: Vec::new(),
        }
    }

    /// Export the activity summary as a JSON file
    pub fn export_json(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        serde_json::to_writer_pretty(
            &std::fs::File::create(&std::path::PathBuf::from(filename))?,
            &self,
        )?;

        Ok(())
    }

    /// Export the activity summary as a CSV file
    pub fn export_csv(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        // Create a buffer for the CSV
        let outfile = PathBuf::from(filename);
        let mut writer = WriterBuilder::new().has_headers(true).from_path(&outfile)?;

        for activity in self.activities.iter() {
            log::trace!(
                "ActivitiesList::export_csv() -- serializing: {:?}",
                activity
            );
            writer.serialize(&activity)?;
        }

        log::trace!(
            "ActivitiesList::export_csv() -- information to be written: {:?}",
            writer
        );

        // Write the file
        writer.flush()?;

        Ok(())
    }
}
