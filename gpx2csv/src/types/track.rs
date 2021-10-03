use crate::types::duration::Duration;
use crate::types::timestamp::TimeStamp;
use crate::types::waypoint::Waypoint;

use serde::Serialize;
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the information about each track. Includes summary data and the details of each waypoint in the track.
#[derive(Serialize, Debug)]
#[serde(default)]
pub struct Track {
    /// The track number if the overall file. Often cordestponds to Lap Number.
    pub tracknum: usize,

    /// GPS name of track.
    pub name: Option<String>,

    /// Start time for the track
    pub start_time: Option<TimeStamp>,

    /// Duration for the track
    pub duration: Option<Duration>,

    /// GPS comment for track.
    pub comment: Option<String>,

    /// User description of track.
    pub description: Option<String>,

    /// Source of data. Included to give user some idea of reliability and accuracy of data.
    pub source: Option<String>,

    /// Links to external information about the track.
    pub links_href: Option<String>,
    pub links_text: Option<String>,

    /// Type (classification) of track.
    pub _type: Option<String>,

    /// Number of track segments within this track
    pub num_segments: usize,
    /// Total number of waypoints within this track
    pub num_waypoints: usize,

    pub waypoints: Vec<Waypoint>,
}

impl Track {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_gpx_track(src: &gpx::Track) -> Self {
        let mut dest = Self::new();

        if let Some(name) = &src.name {
            dest.name = Some(name.to_string())
        }
        if let Some(comment) = &src.comment {
            dest.comment = Some(comment.to_string())
        }
        if let Some(description) = &src.description {
            dest.description = Some(description.to_string())
        }
        if let Some(source) = &src.source {
            dest.source = Some(source.to_string())
        }
        if let Some(_type) = &src._type {
            dest._type = Some(_type.to_string())
        }

        // See if we have links
        if src.links.len() > 0 {
            dest.links_href = Some(src.links[0].href.to_string());
            if let Some(text) = &src.links[0].text {
                dest.links_text = Some(text.to_string());
            }
        }

        // Count the number of segments
        dest.num_segments = src.segments.len();

        // Get the list of segments and their waypoints
        let mut segnum: usize = 0;
        for curr_seg in &src.segments {
            segnum += 1;
            let mut wptnum: usize = 0;
            for curr_wpt in &curr_seg.points {
                let mut wpt = Waypoint::from_gpx_waypoint(&curr_wpt);
                wptnum += 1;
                wpt.segment_num = segnum;
                wpt.waypoint_mum = wptnum;
                dest.waypoints.push(wpt)
            }
        }

        dest.num_waypoints = dest.waypoints.len();
        if dest.num_waypoints > 0 {
            dest.start_time = dest.waypoints[0].time;
            let t_last = &dest.waypoints[dest.num_waypoints - 1]
                .time
                .as_ref()
                .unwrap();
            let t_first = &dest.waypoints[0].time.as_ref().unwrap();

            dest.duration = Some(Duration::between(&t_first, &t_last));
        }

        // return it
        dest
    }
}

impl Default for Track {
    /// Set defaults to be either empty or zero.
    fn default() -> Self {
        Track {
            tracknum: 0,
            name: None,
            start_time: None,
            duration: None,
            comment: None,
            description: None,
            source: None,
            links_href: None,
            links_text: None,
            _type: None,
            num_segments: 0,
            num_waypoints: 0,
            waypoints: Vec::new(),
        }
    }
}
