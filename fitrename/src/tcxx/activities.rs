use tcx;

/// Holds a summary of the activities in the file
#[derive(Debug, Clone)]
pub struct ActivitiesSummary {
    /// Filename of the original file from which the data was read
    pub filename: Option<String>,

    /// Number of activities in the file - typically 1
    pub num_activities: Option<usize>,

    /// Sport
    pub sport: Option<String>,

    /// Activity ID - usually denoted by the start time for the activity
    pub start_time: Option<String>,

    /// Total activity duration in seconds.
    pub total_time_seconds: Option<f64>,

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
                act_s.total_time_seconds =
                    Some(act_s.total_time_seconds.unwrap_or(0.0) + lap.total_time_seconds);
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
                            if act_s.maximum_cadence.unwrap_or(0) < curr_cad {
                                act_s.maximum_cadence = Some(curr_cad);
                            }
                            cad += curr_cad as f64;
                        }

                        // Check if there is a heart rate and record it
                        if let Some(curr_hr) = &trackpoint.heart_rate {
                            hr += curr_hr.value;
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
        if act_s.total_time_seconds.is_some() {
            act_s.average_speed = Some(
                act_s.distance_meters.unwrap_or(0.0) / act_s.total_time_seconds.unwrap_or(1.0),
            );
        }

        // Calculate averages for the whole activity set
        if act_s.num_trackpoints.unwrap_or(0) > 0 {
            act_s.average_cadence = Some(cad / act_s.num_trackpoints.unwrap_or(1) as f64);
            act_s.average_heart_rate = Some(hr / act_s.num_trackpoints.unwrap_or(1) as f64);
        }
        // return it
        act_s
    } // pub fn from_activities
}

impl Default for ActivitiesSummary {
    /// Sets up the ActivitiesSummary with defaults or empty fields
    fn default() -> Self {
        Self {
            filename: None,
            num_activities: None,
            sport: None,
            start_time: None,
            total_time_seconds: None,
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
