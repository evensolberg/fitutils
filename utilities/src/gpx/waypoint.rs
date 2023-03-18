//! Defines the `Waypoint` struct (waypoints, points of interest, or named feature on a map), and associated functions.

use chrono::{DateTime, Local, TimeZone};
use serde::Serialize;

use crate::set_string_field;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Waypoint represents a waypoint, point of interest, or named feature on a map.
#[derive(Default, Clone, Debug, PartialEq, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct GPXWaypoint {
    /// Track number to which this waypoint belongs - `0` if part of Route or separate Waypoint.
    pub track_num: usize,

    /// Route number - `0` if not relevant.
    pub route_num: usize,

    /// Segment number - `0` if not relevant.
    pub segment_num: usize,

    /// Waypoint number - Typically incremeents in fixed time durations.
    pub waypoint_mum: usize,

    /// The geographical point - longitude.
    pub longitude: Option<f64>,

    /// The geographical point - latitude.
    pub latitude: Option<f64>,

    /// Elevation (in meters) of the point.
    pub elevation: Option<f64>,

    /// Speed (in meters per second) (only in GPX 1.0)
    pub speed: Option<f64>,

    /// Creation/modification DateTime<Utc> for element. Date and time in are in
    /// Universal Coordinated Time (UTC), not local time! Conforms to ISO 8601
    /// specification for date/time repdestentation. Fractional seconds are
    /// allowed for millisecond timing in tracklogs.
    pub time: Option<DateTime<Local>>,

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

    /// Number of links to additional information about the waypoint.
    pub num_links: usize,

    /// URL for the first link to additional information about the waypoint.
    pub links_href: Option<String>,

    /// Descriptive text about the first link to additional information about the waypoint.
    pub links_text: Option<String>,

    /// Text of GPS symbol name. For interchange with other programs, use the
    /// exact spelling of the symbol as displayed on the GPS. If the GPS
    /// abbreviates words, spell them out.
    pub symbol: Option<String>,

    /// Type (classification) of the waypoint.
    pub w_type: Option<String>,

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

    /// Placeholder: Heart Rate in Beats per Minute.
    pub heart_rate: Option<u16>,

    /// Placeholder: Cadence in Beats/Revolutions/Strokes per Minute
    pub cadence: Option<u16>,
}

impl GPXWaypoint {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    #[allow(clippy::used_underscore_binding)]
    pub fn from_gpx_waypoint(src: &gpx::Waypoint) -> Self {
        let mut dest = Self::new();

        dest.longitude = Some(src.point().x());
        dest.latitude = Some(src.point().y());

        dest.elevation = src.elevation;
        dest.speed = src.speed;

        dest.time = Some(time_to_dt_local(src));

        set_string_field!(src, name, dest);
        set_string_field!(src, comment, dest);
        set_string_field!(src, description, dest);
        set_string_field!(src, source, dest);
        set_string_field!(src, symbol, dest);
        set_string_field!(src, symbol, dest);

        if let Some(w_type) = &src._type {
            dest.w_type = Some(w_type.to_string());
        }

        if !src.links.is_empty() {
            dest.links_href = Some(src.links[0].href.to_string());
            if let Some(text) = &src.links[0].text {
                dest.links_text = Some(text.to_string());
            }
        }

        if let Some(fix) = &src.fix {
            dest.fix = Some(fix_to_string(fix));
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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Converts the `Gpx::Fix` struct to a string for easier export
///
/// # Parameters
///
/// `src: &gpx::Fix` a `Gpx::Fox` enum
///
/// # Returns
///
/// `String` -- A String containing the name of the Enum value as a string.
///
/// # Example
///
/// ```rust
/// use utilities::GPXFix;
///
/// let src: = &gpx::Fix;
///
/// if let Some(fix) = &src.fix {
///    dest.fix = Some(fix.to_string(&fix))
/// }
/// ```
///
/// # References
///
/// - [Gpx Fix](https://docs.rs/gpx/0.8.3/gpx/enum.Fix.html) enum documentation.
fn fix_to_string(src: &gpx::Fix) -> String {
    match src {
        gpx::Fix::None => "None".to_string(),
        gpx::Fix::TwoDimensional => "TwoDimensional".to_string(),
        gpx::Fix::ThreeDimensional => "ThreeDimensional".to_string(),
        gpx::Fix::DGPS => "DGPS".to_string(),
        gpx::Fix::PPS => "PPS".to_string(),
        gpx::Fix::Other(str) => format!("Other({})", str.clone()),
    }
}

/// Converts `gpx::parser::time::Time` to `DateTime`<Local>
fn time_to_dt_local(src: &gpx::Waypoint) -> DateTime<Local> {
    // let t = src.time.unwrap().format().unwrap_or_default();

    src.time.map_or_else(
        || Local.timestamp(0, 0),
        |t| {
            let tfs = t.format("%FT%TZ%z");
            let tf = tfs.to_string();

            DateTime::parse_from_str(tf.as_str(), "%FT%TZ%z")
                .map_or_else(|_x| Local.timestamp(0, 0), |tp| tp.with_timezone(&Local))
        },
    )
}
