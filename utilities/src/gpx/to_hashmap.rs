use convert_case::{Case, Casing};
use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

use gpx::Gpx;

use crate::datetime_keys::{insert_datetime_keys, insert_duration_keys};
use crate::GPXMetadata;

/// Parses a GPX file and returns the relevant metadata
///
/// # Arguments
///
/// `filename: &str` - The name of the file to be read and fed into the `HashMap`
///
/// # Returns
///
/// `Result<HashMap<String, String>>` with key/value pairs on success.
///
/// # Errors
///
/// Reading the GPX file may fail.
///
/// # Panics
///
///
///
/// # Examples
///
///
///
#[allow(clippy::module_name_repetitions)]
pub fn gpx_to_hashmap(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let gpx: Gpx = gpx::read(BufReader::new(File::open(filename)?))?;
    log::debug!("process_gpx::gpx = {gpx:?}");
    let gpxmeta = GPXMetadata::from_header(&gpx, filename);
    log::debug!("process_gpx::gpxmeta = {gpxmeta:?}");

    let mut values = HashMap::<String, String>::with_capacity(32);
    let mf = gpxmeta
        .creator
        .unwrap_or_else(|| "unknown".to_string())
        .to_case(Case::Title);
    values.insert("%manufacturer".to_string(), mf.clone());
    values.insert("%mf".to_string(), mf.clone());
    values.insert("%product".to_string(), mf.clone());
    values.insert("%pr".to_string(), mf);

    let sn = gpxmeta
        .description
        .unwrap_or_else(|| "unknown".to_string())
        .replace("GPX File Created by ", "")
        .trim()
        .to_string()
        .to_case(Case::Title);
    values.insert("%serial_number".to_string(), sn.clone());
    values.insert("%sn".to_string(), sn);

    let ac = gpxmeta
        .activity
        .unwrap_or_else(|| "unknown".to_string())
        .to_case(Case::Title);
    values.insert("%activity".to_string(), ac.clone());
    values.insert("%at".to_string(), ac);

    let ad = "Unknown".to_string();
    values.insert("%activity_detailed".to_string(), ad.clone());
    values.insert("%ad".to_string(), ad);

    insert_datetime_keys(&mut values, gpxmeta.time.as_ref());
    insert_duration_keys(&mut values, gpxmeta.duration);

    Ok(values)
}
