// use super::types;

use gpx::{Gpx, Track, TrackSegment};

pub fn gpx_ver_to_string(version: &gpx::GpxVersion) -> &str {
    match version {
        gpx::GpxVersion::Gpx10 => "Gpx10",
        gpx::GpxVersion::Gpx11 => "Gpx11",
        _ => "unknown",
    }
}

// pub fn parse_gpx_header(gpx_data: &Gpx) -> types::GpxMetadata {
//     let mut gpx_meta = types::GpxMetadata::new();

//     // gpx_meta.version = Some(gpx_data.version.to_string());
//     gpx_meta.creator = gpx_data.creator;
//     gpx_meta.activity = gpx_data.metadata.unwrap().name;
//     gpx_meta.description = gpx_data.metadata.unwrap().description;
//     gpx_meta.author = gpx_data.metadata.unwrap().name;
//     gpx_meta.links_href = Some(gpx_data.metadata.unwrap().links[0].href);
//     gpx_meta.time = gpx_data.metadata.unwrap().time;

//     // return it
//     gpx_meta
// }
