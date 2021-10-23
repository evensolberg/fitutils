use serde::Serialize;
use tcx;

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

    /// Generates a summary from a set of activities in the TCX file
    pub fn from_activity(activities: &tcx::Activities) -> Self {
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

            // Find the distance of the last trackpoint in the last track of the last lap - hopefully it doesn't reset for each lap or track.
            if let Some(lap) = activity.laps.last() {
                if let Some(track) = lap.tracks.last() {
                    if let Some(tp) = track.trackpoints.last() {
                        if let Some(num) = tp.distance_meters {
                            act_s.distance_meters += num;
                        }
                    }
                }
            }

            for lap in &activity.laps {
                act_s.num_laps += 1;
                for track in &lap.tracks {
                    act_s.num_tracks += 1;
                    act_s.num_trackpoints += track.trackpoints.len();
                    for trackpoint in &track.trackpoints {
                        // Check if there is a cadence and if it's greater than the current max
                        if let Some(curr_cad) = trackpoint.cadence {
                            if curr_cad > act_s.maximum_cadence {
                                act_s.maximum_cadence = curr_cad;
                                cad += curr_cad as f64;
                            }
                        }

                        // Check if there is a heart rate and record it
                        if let Some(curr_hr) = &trackpoint.heart_rate {
                            if curr_hr.value > act_s.maximum_heart_rate {
                                act_s.maximum_heart_rate = curr_hr.value;
                            }
                            hr += curr_hr.value;
                        }

                        // Check if there is altitude data and calculate
                        if let Some(altitude) = trackpoint.altitude_meters {
                            if altitude > act_s.max_altitude {
                                act_s.max_altitude = altitude;
                                act_s.ascent_meters = act_s.max_altitude - act_s.start_altitude;
                            }
                        }
                    }
                }
            }
        }

        // Calculate averages for the whole activity set
        if act_s.num_trackpoints > 0 {
            act_s.average_cadence = cad / act_s.num_trackpoints as f64;
            act_s.average_heart_rate = hr / act_s.num_trackpoints as f64;
        }
        // return it
        act_s
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
