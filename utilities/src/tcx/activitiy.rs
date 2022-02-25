use csv::WriterBuilder;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::path::PathBuf;
use std::{error::Error, io::BufReader};
use tcx::{self};

use crate::{set_extension, Duration};

/// Holds a summary of the activities in the file
#[derive(Serialize, Debug, Clone)]
pub struct TCXActivity {
    /// Filename of the original file from which the data was read
    pub filename: Option<String>,

    /// Number of activities in the file - typically 1
    pub num_activities: Option<usize>,

    /// Sport
    pub sport: Option<String>,

    /// Activity ID - usually denoted by the start time for the activity
    pub start_time: Option<String>,

    /// Total activity duration in seconds.
    pub duration: Option<Duration>,

    /// Notes - if there are any
    pub notes: Option<String>,

    /// Number of laps within the activity
    pub num_laps: Option<usize>,

    /// Total number of tracks within the activity
    pub num_tracks: Option<usize>,

    /// Total number of trackpoints within the activity
    pub num_trackpoints: Option<usize>,

    /// Total distance covered during the lap in meters.
    pub distance_meters: Option<f64>,

    /// Max ascent in meters from start
    pub start_altitude: Option<f64>,

    /// Max ascent in meters from start
    pub max_altitude: Option<f64>,

    /// Max ascent in meters from start
    pub ascent_meters: Option<f64>,

    /// Average speed in Meters/Second for the activity
    pub average_speed: Option<f64>,

    /// Maximum speed in Meters/Second obtained during the activity.
    pub maximum_speed: Option<f64>,

    /// Number of calories burned during the activity.
    pub calories: Option<u16>,

    /// Average heart rate in Beats per Minute (BPM) for the activity
    pub average_heart_rate: Option<f64>,

    /// Maximum heart rate in Beats per Minute (BPM) for the activity
    pub maximum_heart_rate: Option<f64>,

    /// Average cadence (typically in Steps, Revolutions or Strokes per Minute) for the activity.
    pub average_cadence: Option<f64>,

    /// Maximum cadence (typically in Steps, Revolutions or Strokes per Minute) for the activity.
    pub maximum_cadence: Option<u8>,
}

impl TCXActivity {
    /// Create a new, empty Activities Summary
    pub fn new() -> Self {
        TCXActivity::default()
    }

    pub fn from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut tcdb = tcx::read(&mut BufReader::new(File::open(&filename).unwrap()))?;
        tcdb.calc_heartrates();

        let mut act;

        if let Some(activities) = tcdb.activities {
            act = Self::from_activities(&activities);
            let file_name = filename.to_string();
            act.filename = Some(file_name.clone());
        } else {
            act = Self::new();
        }

        // return safely
        Ok(act)
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
                            act_s.start_altitude = Some(num);
                            act_s.max_altitude = Some(num);
                        }
                    }
                }
            }
        }

        for activity in &activities.activities {
            act_s.num_activities = Some(act_s.num_activities.unwrap_or(0) + 1);
            act_s.sport = Some(activity.sport.clone());
            act_s.start_time = Some(activity.id.clone()); // TODO: https://github.com/evensolberg/fitparser/projects/6#card-71437698
            act_s.notes = activity.notes.clone();

            for lap in &activity.laps {
                act_s.num_laps = Some(act_s.num_laps.unwrap_or(0) + 1);
                act_s.duration = Some(
                    act_s.duration.unwrap_or_default()
                        + Duration::from_secs_f64(lap.total_time_seconds),
                );
                act_s.distance_meters =
                    Some(act_s.distance_meters.unwrap_or(0.0) + lap.distance_meters);
                act_s.calories = Some(act_s.calories.unwrap_or(0) + lap.calories);
                if let Some(max_speed) = lap.maximum_speed {
                    if act_s.maximum_speed.unwrap_or(0.0) < max_speed {
                        act_s.maximum_speed = Some(max_speed);
                    }
                }

                for track in &lap.tracks {
                    act_s.num_tracks = Some(act_s.num_tracks.unwrap_or(0) + 1);
                    act_s.num_trackpoints =
                        Some(act_s.num_trackpoints.unwrap_or(0) + track.trackpoints.len());
                    // Check to see if max HR for the lap > current recorded max
                    if let Some(mhr) = lap.maximum_heart_rate {
                        if act_s.maximum_heart_rate.unwrap_or(0.0) < mhr {
                            act_s.maximum_heart_rate = Some(mhr);
                        }
                    }

                    for trackpoint in &track.trackpoints {
                        // Check if there is a cadence and if it's greater than the current max
                        if let Some(curr_cad) = trackpoint.cadence {
                            if act_s.maximum_cadence.unwrap_or(0) <= curr_cad {
                                act_s.maximum_cadence = Some(curr_cad);
                            }
                            cad += curr_cad as f64;
                        }

                        // Check if there is a heart rate and record it
                        if let Some(curr_hr) = &trackpoint.heart_rate {
                            hr += curr_hr.value;
                            if act_s.maximum_heart_rate.unwrap_or(0.0) < curr_hr.value {
                                act_s.maximum_heart_rate = Some(curr_hr.value);
                            }
                        }

                        // Check if there is altitude data and calculate
                        if let Some(altitude) = trackpoint.altitude_meters {
                            if act_s.max_altitude.unwrap_or(0.0) < altitude {
                                act_s.max_altitude = Some(altitude);
                            }
                        }
                    }
                }
            }
        }

        act_s.ascent_meters =
            Some(act_s.max_altitude.unwrap_or(0.0) - act_s.start_altitude.unwrap_or(0.0));
        if act_s.duration.is_some() {
            act_s.average_speed = Some(
                act_s.distance_meters.unwrap_or(0.0)
                    / act_s.duration.unwrap_or_default().0.as_secs() as f64,
            );
        }

        // Calculate averages for the whole activity set
        if act_s.num_trackpoints.unwrap_or(0) > 0 {
            act_s.average_cadence = Some(cad / act_s.num_trackpoints.unwrap_or(1) as f64);
            act_s.average_heart_rate = Some(hr / act_s.num_trackpoints.unwrap_or(1) as f64);
        }

        // If maximum_cadence = None then set it to the same as average
        if act_s.maximum_cadence.is_none() && act_s.average_cadence.is_some() {
            act_s.maximum_cadence = Some(act_s.average_cadence.unwrap() as u8);
        }
        // return it
        act_s
    } // pub fn from_activities

    /// Export the activity summary as a JSON file
    pub fn export_json(&self) -> Result<(), Box<dyn Error>> {
        if self.filename.is_none() {
            return Err("No filename specified in the ActivitySummary. Unable to export.".into());
        }

        let out_file = set_extension(
            self.filename
                .as_ref()
                .unwrap_or(&"tcx_activity".to_string()),
            "activity.json",
        );
        serde_json::to_writer_pretty(
            &std::fs::File::create(&std::path::PathBuf::from(&out_file))?,
            &self,
        )?;

        Ok(())
    }

    /// Print the details of the activity
    pub fn print(&self, _detailed: bool) {
        let unknown = "".to_string();

        println!(
            "\nFile:                  {}",
            self.filename.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Sport:                 {}",
            self.sport.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Start time:            {}",
            self.start_time.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Duration:              {}",
            self.duration.as_ref().unwrap_or(&Duration::default())
        );
        println!(
            "Notes:                {}",
            self.notes.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Activities:           {:>9}",
            self.num_activities.unwrap_or_default()
        );
        println!(
            "Laps:                 {:>9}",
            self.num_laps.unwrap_or_default()
        );
        println!(
            "Trakcs:               {:>9}",
            self.num_tracks.unwrap_or_default()
        );
        println!(
            "Trackpoints:          {:>9}",
            self.num_trackpoints.unwrap_or_default()
        );
        println!(
            "Distance (m):         {:>9.2}",
            self.distance_meters.unwrap_or_default()
        );
        println!(
            "Start altitude (m):   {:>9.2}",
            self.start_altitude.unwrap_or_default()
        );
        println!(
            "Max altitude (m):     {:>9.2}",
            self.max_altitude.unwrap_or_default()
        );
        println!(
            "Ascent (m):           {:>9.2}",
            self.ascent_meters.unwrap_or_default()
        );
        println!(
            "Avg Speed (m/s):      {:>9.2}",
            self.average_speed.unwrap_or_default()
        );
        println!(
            "Max Speed (m/s):      {:>9.2}",
            self.maximum_speed.unwrap_or_default()
        );
        println!(
            "Calories Burned:      {:>9}",
            self.calories.unwrap_or_default()
        );
        println!(
            "Avg Heart Rate (bpm): {:>9.1}",
            self.average_heart_rate.unwrap_or_default()
        );
        println!(
            "Max Heart Rate (bpm): {:>9.1}",
            self.maximum_heart_rate.unwrap_or_default()
        );
        println!(
            "Avg Cadence (bpm):    {:>9.1}",
            self.average_cadence.unwrap_or_default()
        );
        println!(
            "Max Cadence (bpm):    {:>9.1}",
            self.maximum_cadence.unwrap_or_default() as f64
        );
    }
}

impl Default for TCXActivity {
    /// Sets up the ActivitiesSummary with defaults or empty fields
    fn default() -> Self {
        Self {
            filename: None,
            num_activities: None,
            sport: None,
            start_time: None,
            duration: None,
            notes: None,
            num_laps: None,
            num_tracks: None,
            num_trackpoints: None,
            distance_meters: None,
            start_altitude: None,
            max_altitude: None,
            ascent_meters: None,
            average_speed: None,
            maximum_speed: None,
            calories: None,
            average_heart_rate: None,
            maximum_heart_rate: None,
            average_cadence: None,
            maximum_cadence: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct TCXActivitiesList {
    /// The list of activities
    pub activities: Vec<TCXActivity>,
}

impl Default for TCXActivitiesList {
    fn default() -> Self {
        Self::new()
    }
}

impl TCXActivitiesList {
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

#[cfg(test)]
///
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    ///
    fn test_activities_summary_new() {
        let mut act = TCXActivity::new();

        assert!(act.filename.is_none());
        assert!(act.num_activities.is_none());
        assert!(act.sport.is_none());
        assert!(act.start_time.is_none());
        assert!(act.duration.is_none());
        assert!(act.notes.is_none());
        assert!(act.num_laps.is_none());
        assert!(act.num_tracks.is_none());
        assert!(act.num_trackpoints.is_none());
        assert!(act.distance_meters.is_none());
        assert!(act.start_altitude.is_none());
        assert!(act.max_altitude.is_none());
        assert!(act.ascent_meters.is_none());
        assert!(act.average_speed.is_none());
        assert!(act.maximum_speed.is_none());
        assert!(act.calories.is_none());
        assert!(act.average_heart_rate.is_none());
        assert!(act.maximum_heart_rate.is_none());
        assert!(act.average_cadence.is_none());
        assert!(act.maximum_cadence.is_none());

        act.filename = Some("2022-02-22 0700 cycling.fit".to_string());
        act.num_activities = Some(1);
        act.sport = Some("cycling".to_string());
        act.start_time = Some(chrono::Local::now().to_string());
        act.duration = Some(Duration::from_secs_f64(1800.0));
        act.notes = Some("Something about the activity.".to_string());
        act.num_laps = Some(2);
        act.num_tracks = Some(10);
        act.num_trackpoints = Some(20);
        act.distance_meters = Some(16254.0);
        act.start_altitude = Some(102.1);
        act.max_altitude = Some(110.5);
        act.ascent_meters = Some(52.5);
        act.average_speed = Some(29.2);
        act.maximum_speed = Some(32.7);
        act.calories = Some(520);
        act.average_heart_rate = Some(148.0);
        act.maximum_heart_rate = Some(159.0);
        act.average_cadence = Some(84.2);
        act.maximum_cadence = Some(101);

        assert!(act.filename.is_some());
        assert!(act.num_activities.is_some());
        assert!(act.sport.is_some());
        assert!(act.start_time.is_some());
        assert!(act.duration.is_some());
        assert!(act.notes.is_some());
        assert!(act.num_laps.is_some());
        assert!(act.num_tracks.is_some());
        assert!(act.num_trackpoints.is_some());
        assert!(act.distance_meters.is_some());
        assert!(act.start_altitude.is_some());
        assert!(act.max_altitude.is_some());
        assert!(act.ascent_meters.is_some());
        assert!(act.average_speed.is_some());
        assert!(act.maximum_speed.is_some());
        assert!(act.calories.is_some());
        assert!(act.average_heart_rate.is_some());
        assert!(act.maximum_heart_rate.is_some());
        assert!(act.average_cadence.is_some());
        assert!(act.maximum_cadence.is_some());

        assert_eq!(
            act.filename.unwrap(),
            "2022-02-22 0700 cycling.fit".to_string()
        );
        assert_eq!(act.num_activities.unwrap(), 1);
        assert_eq!(act.sport.unwrap(), "cycling".to_string());

        // Time will have passed, so these should not be the same.
        let act_time = act.start_time.unwrap_or("Unknown".to_string());
        assert_ne!(act_time, chrono::Local::now().to_string());
        assert_ne!(act_time, "Unknown".to_string());

        assert_eq!(act.duration.unwrap().0.as_secs(), 1800);
        assert_eq!(
            act.notes.unwrap(),
            "Something about the activity.".to_string()
        );
        assert_eq!(act.num_laps.unwrap(), 2);
        assert_eq!(act.num_tracks.unwrap(), 10);
        assert_eq!(act.num_trackpoints.unwrap(), 20);
        assert_eq!(act.distance_meters.unwrap(), 16254.0);
        assert_eq!(act.start_altitude.unwrap(), 102.1);
        assert_eq!(act.max_altitude.unwrap(), 110.5);
        assert_eq!(act.ascent_meters.unwrap(), 52.5);
        assert_eq!(act.average_speed.unwrap(), 29.2);
        assert_eq!(act.maximum_speed.unwrap(), 32.7);
        assert_eq!(act.calories.unwrap(), 520);
        assert_eq!(act.average_heart_rate.unwrap(), 148.0);
        assert_eq!(act.maximum_heart_rate.unwrap(), 159.0);
        assert_eq!(act.average_cadence.unwrap(), 84.2);
        assert_eq!(act.maximum_cadence.unwrap(), 101);
    }

    #[assay(include = ["/Users/evensolberg/CloudStation/Source/Rust/fitutils/data/running.tcx"])]
    // TODO: Figure out relative paths.
    fn test_from_activities() {
        // Create an empty summary struct and load data into it.
        let mut act = TCXActivity::new();
        let filename = "/Users/evensolberg/CloudStation/Source/Rust/fitutils/data/running.tcx";
        let tcdb = tcx::read_file(filename)?;

        if let Some(activities) = tcdb.activities {
            act = TCXActivity::from_activities(&activities);
            println!("act = {:?}", act);
        }
        act.filename = Some(filename.to_string());

        // Check that everything is OK
        assert!(act.filename.is_some());
        assert!(act.num_activities.is_some());
        assert!(act.sport.is_some());
        assert!(act.start_time.is_some());
        assert!(act.duration.is_some());
        assert!(act.notes.is_none()); // NB!
        assert!(act.num_laps.is_some());
        assert!(act.num_tracks.is_some());
        assert!(act.num_trackpoints.is_some());
        assert!(act.distance_meters.is_some());
        assert!(act.start_altitude.is_some());
        assert!(act.max_altitude.is_some());
        assert!(act.ascent_meters.is_some());
        assert!(act.average_speed.is_some());
        assert!(act.maximum_speed.is_some());
        assert!(act.calories.is_some());
        assert!(act.average_heart_rate.is_some());
        assert!(act.maximum_heart_rate.is_some());
        assert!(act.average_cadence.is_some());
        assert!(act.maximum_cadence.is_some());

        // Verify the actual data - note that the act.notes section is missing since it's None.
        assert_eq!(
            act.filename.unwrap(),
            "/Users/evensolberg/CloudStation/Source/Rust/fitutils/data/running.tcx".to_string()
        );
        assert_eq!(act.num_activities.unwrap(), 1);
        assert_eq!(act.sport.unwrap(), "Running".to_string());
        assert_eq!(act.start_time.unwrap(), "2018-06-15T13:35:49Z".to_string());
        assert_eq!(act.duration.unwrap().0.as_micros(), 1325444009);
        assert_eq!(act.num_laps.unwrap(), 1);
        assert_eq!(act.num_tracks.unwrap(), 1);
        assert_eq!(act.num_trackpoints.unwrap(), 1325);
        assert_eq!(act.distance_meters.unwrap(), 2963.318848);
        assert_eq!(act.start_altitude.unwrap(), 107.041348);
        assert_eq!(act.max_altitude.unwrap(), 133.028357);
        assert_eq!(act.ascent_meters.unwrap(), 25.987009);
        assert_eq!(act.average_speed.unwrap(), 2.2364670550943395);
        assert_eq!(act.maximum_speed.unwrap(), 4.656036);
        assert_eq!(act.calories.unwrap(), 338);
        assert_eq!(act.average_heart_rate.unwrap(), 137.15622641509435);
        assert_eq!(act.maximum_heart_rate.unwrap(), 170.0);
        assert_eq!(act.average_cadence.unwrap(), 0.0);
        assert_eq!(act.maximum_cadence.unwrap(), 0);
    }
}
