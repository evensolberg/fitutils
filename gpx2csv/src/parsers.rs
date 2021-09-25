use gpx::Gpx;
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

    // FIXME: There has to be a cleaner way to do this!
    gpx_meta.version = Some(gpx_ver_to_string(&gpx_data.version));
    gpx_meta.creator = Some(gpx_data.creator.as_ref().unwrap().to_string());

    let metadata = gpx_data.metadata.as_ref().unwrap();
    gpx_meta.activity = Some(metadata.name.as_ref().unwrap().to_string());
    gpx_meta.description = Some(metadata.description.as_ref().unwrap().to_string());
    gpx_meta.author = Some(metadata.name.as_ref().unwrap().to_string());
    gpx_meta.links_href = Some(metadata.links[0].href.to_string());
    gpx_meta.time = Some(metadata.time.unwrap());

    // return it
    Ok(gpx_meta)
}
