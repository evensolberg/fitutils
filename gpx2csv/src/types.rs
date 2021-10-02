use std::path::PathBuf;

use chrono::{DateTime, Utc};
use gpx::Gpx;
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Turn the `gpx::GpxVersion` enum into a string
fn gpx_ver_to_string(version: &gpx::GpxVersion) -> String {
    match version {
        gpx::GpxVersion::Gpx10 => "Gpx10".to_string(),
        gpx::GpxVersion::Gpx11 => "Gpx11".to_string(),
        _ => "unknown".to_string(),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the metadata information about the file and its contents
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct GpxMetadata {
    pub filename: Option<PathBuf>,
    pub version: Option<String>,
    pub creator: Option<String>,
    pub activity: Option<String>,
    pub description: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub links_href: Option<String>,
    pub links_text: Option<String>,
    pub keywords: Option<String>,
    pub time: Option<DateTime<Utc>>,
    pub copyright_author: Option<String>,
    pub copyright_year: Option<i32>,
    pub copyright_license: Option<String>,
    pub num_waypoints: usize,
    pub num_tracks: usize,
    pub num_routes: usize,
}

impl GpxMetadata {
    /// Initialize Session with default empty values
    pub fn new() -> Self {
        Self::default()
    }

    fn from_filename(filename: &str) -> Self {
        GpxMetadata {
            filename: Some(PathBuf::from(&filename)),
            version: None,
            creator: None,
            activity: None,
            description: None,
            author_name: None,
            author_email: None,
            links_href: None,
            links_text: None,
            keywords: None,
            time: None,
            copyright_author: None,
            copyright_year: None,
            copyright_license: None,
            num_waypoints: 0,
            num_tracks: 0,
            num_routes: 0,
        }
    }

    pub fn from_header(gpx_data: &Gpx, filename: &str) -> Self {
        let mut gpx_meta = Self::from_filename(filename);

        // Parse the GPX header
        log::trace!("parsers::parse_gpx_header() -- Parsing the GPX header information.");
        gpx_meta.version = Some(gpx_ver_to_string(&gpx_data.version));
        if let Some(creator) = &gpx_data.creator {
            gpx_meta.creator = Some(creator.to_string());
        }

        // Parse the metadata
        let metadata = gpx_data.metadata.as_ref().unwrap();
        if let Some(activity) = &metadata.name {
            gpx_meta.activity = Some(activity.to_string());
        }
        if let Some(description) = &metadata.description {
            gpx_meta.description = Some(description.to_string());
        }
        if let Some(keywords) = &metadata.keywords {
            gpx_meta.keywords = Some(keywords.to_string());
        }
        if let Some(time) = &metadata.time {
            gpx_meta.time = Some(time.to_owned());
        }

        // For now, only read the first href in the list of links (if there is one)
        if metadata.links.len() > 0 {
            gpx_meta.links_href = Some(metadata.links[0].href.to_string());
            if let Some(text) = &metadata.links[0].text {
                gpx_meta.links_text = Some(text.to_string());
            }
        }

        // See if we have copyright information, and extract it if we do.
        match &metadata.copyright {
            Some(cr_data) => {
                log::trace!("parsers::parse_gpx_header() -- Copyright information found. Parsing.");
                if let Some(author) = &cr_data.author {
                    gpx_meta.copyright_author = Some(author.to_string())
                };
                if let Some(license) = &cr_data.license {
                    gpx_meta.copyright_license = Some(license.to_string())
                };
                if let Some(year) = cr_data.year {
                    gpx_meta.copyright_year = Some(year)
                };
            }
            None => {
                log::trace!(
                    "parsers::parse_gpx_header() -- Copyright information not found. No need to parse."
                );
            }
        }

        // Parse metadata.author if there is anything there.
        match &metadata.author {
            Some(author) => {
                log::trace!("parsers::parse_gpx_header() -- Author information found. Parsing.");
                if let Some(name) = &author.name {
                    gpx_meta.author_name = Some(name.to_string())
                };
                if let Some(email) = &author.email {
                    gpx_meta.author_email = Some(email.to_string())
                };
            }
            None => {
                log::trace!(
                    "parsers::parse_gpx_header() -- Author information not found. No need to parse."
                );
            }
        }

        // Find the number of waypoints, tracks and segments
        gpx_meta.num_waypoints = gpx_data.waypoints.len();
        gpx_meta.num_tracks = gpx_data.tracks.len();
        gpx_meta.num_routes = gpx_data.routes.len();

        // return the metadata struct
        gpx_meta
    }
}

impl Default for GpxMetadata {
    /// Set defaults to be either empty or zero.
    fn default() -> Self {
        GpxMetadata {
            filename: None,
            version: None,
            creator: None,
            activity: None,
            description: None,
            author_name: None,
            author_email: None,
            links_href: None,
            links_text: None,
            keywords: None,
            time: None,
            copyright_author: None,
            copyright_year: None,
            copyright_license: None,
            num_waypoints: 0,
            num_tracks: 0,
            num_routes: 0,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Holds the high-level information about each track
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Track {
    /// The track number if the overall file. Often cordestponds to Lap Number.
    pub tracknum: usize,

    /// GPS name of track.
    pub name: Option<String>,

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
    // pub segments: Vec<gpx::TrackSegment>,
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

        // return it
        dest
    }
}

impl Default for Track {
    /// Set defaults to be either empty or zero.
    fn default() -> Self {
        Track {
            tracknum: 1,
            name: None,
            comment: None,
            description: None,
            source: None,
            links_href: None,
            links_text: None,
            _type: None,
            num_segments: 0,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Link repdestents a link to an external destource.
///
/// An external destource could be a web page, digital photo,
/// video clip, etc., with additional information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    /// URL of hyperlink.
    pub href: String,

    /// Text of hyperlink.
    pub text: Option<String>,

    /// Mime type of content (image/jpeg)
    pub _type: Option<String>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Person repdestents a person or organization.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    /// Name of person or organization.
    pub name: Option<String>,

    /// Email adddests.
    pub email: Option<String>,

    /// Link to Web site or other external information about person.
    pub link: Option<Link>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Type of the GPS fix.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Fix {
    /// The GPS had no fix. To signify "the fix info is unknown", leave out the Fix entirely.
    None,
    /// 2D fix gives only longitude and latitude. It needs a minimum of 3 satellites.
    TwoDimensional,
    /// 3D fix gives longitude, latitude and altitude. It needs a minimum of 4 satellites.
    ThreeDimensional,
    /// Differential Global Positioning System.
    DGPS,
    /// Military signal.
    PPS,
    /// Other values that are not in the specification.
    Other(String),
}

impl ToString for Fix {
    fn to_string(&self) -> String {
        match self {
            Fix::None => "None".to_string(),
            Fix::TwoDimensional => "TwoDimensional".to_string(),
            Fix::ThreeDimensional => "ThreeDimensional".to_string(),
            Fix::DGPS => "DGPS".to_string(),
            Fix::PPS => "PPS".to_string(),
            Fix::Other(_) => "Other".to_string(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Route repdestents an ordered list of waypoints repdestenting a series of turn points leading to a destination.
pub struct Route {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub links: Vec<Link>,
    pub number: Option<u32>,
    pub _type: Option<String>,
    pub points: Vec<Waypoint>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Waypoint repdestents a waypoint, point of intedestt, or named feature on a map.
#[derive(Clone, Debug, PartialEq)]
pub struct Waypoint {
    pub track_num: usize,
    pub route_num: Option<usize>,
    pub segment_num: Option<usize>,
    pub waypoint_mum: usize,

    /// The geographical point.
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,

    /// Elevation (in meters) of the point.
    pub elevation: Option<f64>,

    /// Speed (in meters per second) (only in GPX 1.0)
    pub speed: Option<f64>,

    /// Creation/modification timestamp for element. Date and time in are in
    /// Univeral Coordinated Time (UTC), not local time! Conforms to ISO 8601
    /// specification for date/time repdestentation. Fractional seconds are
    /// allowed for millisecond timing in tracklogs.
    pub time: Option<DateTime<Utc>>,

    /// The GPS name of the waypoint. This field will be transferred to and
    /// from the GPS. GPX does not place desttrictions on the length of this
    /// field or the characters contained in it. It is up to the receiving
    /// application to validate the field before sending it to the GPS.
    pub name: Option<String>,

    /// GPS waypoint comment. Sent to GPS as comment.
    pub comment: Option<String>,

    /// A text description of the element. Holds additional information about
    /// the element intended for the user, not the GPS.
    pub description: Option<String>,

    /// Source of data. Included to give user some idea of reliability and
    /// accuracy of data. "Garmin eTrex", "USGS quad Boston North", e.g.
    pub source: Option<String>,

    /// Links to additional information about the waypoint.
    pub num_links: usize,
    pub links_href: Option<String>,
    pub links_text: Option<String>,

    /// Text of GPS symbol name. For interchange with other programs, use the
    /// exact spelling of the symbol as displayed on the GPS. If the GPS
    /// abbreviates words, spell them out.
    pub symbol: Option<String>,

    /// Type (classification) of the waypoint.
    pub _type: Option<String>,

    // <magvar> degreesType </magvar> [0..1] ?
    /// Height of geoid in meters above WGS 84. This cordestpond to the sea level.
    pub geoidheight: Option<f64>,

    /// Type of GPS fix. `none` means GPS had no fix. To signify "the fix info
    /// is unknown", leave out `fix` entirely. Value comes from the list
    /// `{'none'|'2d'|'3d'|'dgps'|'pps'}`, where `pps` means that the military
    /// signal was used.
    pub fix: Option<String>,

    /// Number of satellites used to calculate the GPX fix.
    pub sat: Option<u64>,

    /// Horizontal dilution of precision.
    pub hdop: Option<f64>,

    /// Vertical dilution of precision.
    pub vdop: Option<f64>,

    /// Positional dilution of precision.
    pub pdop: Option<f64>,

    /// Number of seconds since last DGPS update, from the <ageofdgpsdata> element.
    pub age: Option<f64>,

    /// ID of DGPS station used in differential correction, in the range [0, 1023].
    pub dgpsid: Option<u16>,

    /// Placeholder: Heart Rate
    pub heart_rate: Option<u16>,

    /// Placeholder: Cadence
    pub cadence: Option<u16>,
}

impl Waypoint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_gpx_waypoint(src: &gpx::Waypoint) -> Self {
        let mut dest = Self::new();

        dest.longitude = Some(src.point().x());
        dest.latitude = Some(src.point().y());

        dest.elevation = src.elevation;
        dest.speed = src.speed;
        dest.time = src.time;

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
        if let Some(symbol) = &src.symbol {
            dest.symbol = Some(symbol.to_string())
        }
        if let Some(_type) = &src._type {
            dest._type = Some(_type.to_string())
        }

        if src.links.len() > 0 {
            dest.links_href = Some(src.links[0].href.to_string());
            if let Some(text) = &src.links[0].text {
                dest.links_text = Some(text.to_string());
            }
        }

        if let Some(fix) = &src.fix {
            dest.fix = Some(fix_to_string(&fix))
        }

        dest.sat = src.sat;
        dest.hdop = src.hdop;
        dest.vdop = src.vdop;
        dest.pdop = src.pdop;
        dest.age = src.dgps_age;
        dest.dgpsid = src.dgpsid;

        // We currently don't have any way of extracting heart rate and cadence from
        // the GPX file using the Gpx struct, so we're just omitting those for now.

        // return it
        dest
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Self {
            track_num: 1,
            route_num: None,
            segment_num: None,
            waypoint_mum: 1,
            longitude: None,
            latitude: None,
            elevation: None,
            speed: None,
            time: None,
            name: None,
            comment: None,
            description: None,
            source: None,
            num_links: 0,
            links_href: None,
            links_text: None,
            symbol: None,
            _type: None,
            geoidheight: None,
            fix: None,
            sat: None,
            hdop: None,
            vdop: None,
            pdop: None,
            age: None,
            dgpsid: None,
            heart_rate: None,
            cadence: None,
        }
    }
}

fn fix_to_string(src: &gpx::Fix) -> String {
    match src {
        gpx::Fix::None => "None".to_string(),
        gpx::Fix::TwoDimensional => "TwoDimensional".to_string(),
        gpx::Fix::ThreeDimensional => "ThreeDimensional".to_string(),
        gpx::Fix::DGPS => "DGPS".to_string(),
        gpx::Fix::PPS => "PPS".to_string(),
        gpx::Fix::Other(str) => format!("Other({})", str.to_owned()),
    }
}
