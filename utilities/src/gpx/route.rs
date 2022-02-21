use serde::Serialize;

use crate::gpx::{link::GPXLink, waypoint::GPXWaypoint};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Route repdestents an ordered list of waypoints representing a series of turn points leading to a destination.
#[derive(Clone, Default, Debug, PartialEq, Serialize)]
pub struct GPXRoute {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub links: Vec<GPXLink>,
    pub number: Option<u32>,
    pub _type: Option<String>,
    #[serde(skip)] // Do not serialize - we'll handle it in the export. Maybe.
    pub points: Vec<GPXWaypoint>,
}
