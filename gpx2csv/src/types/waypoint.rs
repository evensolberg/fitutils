use crate::types::gpxmetadata::fix_to_string;
use crate::types::timestamp::TimeStamp;
use serde::Serialize;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Waypoint represents a waypoint, point of interest, or named feature on a map.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Waypoint {
    pub track_num: usize,
    pub route_num: usize,
    pub segment_num: usize,
    pub waypoint_mum: usize,

    /// The geographical point.
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,

    /// Elevation (in meters) of the point.
    pub elevation: Option<f64>,

    /// Speed (in meters per second) (only in GPX 1.0)
    pub speed: Option<f64>,

    /// Creation/modification DateTime<Utc> for element. Date and time in are in
    /// Univeral Coordinated Time (UTC), not local time! Conforms to ISO 8601
    /// specification for date/time repdestentation. Fractional seconds are
    /// allowed for millisecond timing in tracklogs.
    pub time: Option<TimeStamp>,

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

        dest.time = Some(TimeStamp(src.time.unwrap()));

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

        if !src.links.is_empty() {
            dest.links_href = Some(src.links[0].href.to_string());
            if let Some(text) = &src.links[0].text {
                dest.links_text = Some(text.to_string());
            }
        }

        if let Some(fix) = &src.fix {
            dest.fix = Some(fix_to_string(fix))
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
            track_num: 0,
            route_num: 0,
            segment_num: 0,
            waypoint_mum: 0,
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
