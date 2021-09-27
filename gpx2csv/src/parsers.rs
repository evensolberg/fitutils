use gpx::{Gpx, Track, TrackSegment};
use std::error::Error;

// Local modules
use super::types;

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
/// Parse the GPX header and metadata into something that can be serialized.
pub fn parse_gpx_header(gpx_data: &Gpx) -> Result<types::GpxMetadata, Box<dyn Error>> {
    let mut gpx_meta = types::GpxMetadata::new();

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

    // return the metadata struct
    Ok(gpx_meta)
}

pub fn parse_gpx_track(gpx_track: &gpx::Track) -> Result<types::Track, Box<dyn Error>> {
    let return_track = types::Track::new();

    // Return OK
    Ok(return_track)
}
