use convert_case::{Case, Casing};
use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

use chrono::{Datelike, Timelike};
use gpx::Gpx;

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
    log::debug!("process_gpx::gpx = {:?}", gpx);
    let gpxmeta = GPXMetadata::from_header(&gpx, filename);
    log::debug!("process_gpx::gpxmeta = {:?}", gpxmeta);

    let mut values = HashMap::<String, String>::new();
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
    values.insert("%ac".to_string(), ac);

    let ad = "Unknown".to_string();
    values.insert("%activity_detailed".to_string(), ad.clone());
    values.insert("%ad".to_string(), ad);

    if let Some(tc) = gpxmeta.time {
        values.insert("%year".to_string(), format!("{:04}", tc.year()));
        values.insert("%yr".to_string(), format!("{:04}", tc.year()));
        values.insert("%month".to_string(), format!("{:02}", tc.month()));
        values.insert("%mo".to_string(), format!("{:02}", tc.month()));
        values.insert("%day".to_string(), format!("{:02}", tc.day()));
        values.insert("%dy".to_string(), format!("{:02}", tc.day()));

        values.insert("%hour".to_string(), format!("{:02}", tc.hour()));
        values.insert("%hr".to_string(), format!("{:02}", tc.hour()));
        values.insert("%hour24".to_string(), format!("{:02}", tc.hour()));
        values.insert("%h24".to_string(), format!("{:02}", tc.hour()));

        let (am, hrs) = tc.hour12();
        let hr = format!("{hrs:02}");
        values.insert("%hour12".to_string(), hr.clone());
        values.insert("%h12".to_string(), hr);
        if am {
            values.insert("%ampm".to_string(), "pm".to_string());
            values.insert("%ap".to_string(), "pm".to_string());
        } else {
            values.insert("%ampm".to_string(), "am".to_string());
            values.insert("%ap".to_string(), "am".to_string());
        }

        values.insert("%minute".to_string(), format!("{:02}", tc.minute()));
        values.insert("%mi".to_string(), format!("{:02}", tc.minute()));
        values.insert("%second".to_string(), format!("{:02}", tc.second()));
        values.insert("%se".to_string(), format!("{:02}", tc.second()));
        values.insert("%weekday".to_string(), tc.weekday().to_string());
        values.insert("%wd".to_string(), tc.weekday().to_string());
    } else {
        values.insert("%year".to_string(), "0000".to_string());
        values.insert("%yr".to_string(), "0000".to_string());
        values.insert("%month".to_string(), "00".to_string());
        values.insert("%mo".to_string(), "00".to_string());
        values.insert("%day".to_string(), "00".to_string());
        values.insert("%dy".to_string(), "00".to_string());
        values.insert("%hour".to_string(), "00".to_string());
        values.insert("%hr".to_string(), "00".to_string());
        values.insert("%hour24".to_string(), "00".to_string());
        values.insert("%h24".to_string(), "00".to_string());
        values.insert("%hour12".to_string(), "00".to_string());
        values.insert("%h12".to_string(), "00".to_string());
        values.insert("%ampm".to_string(), "ampm".to_string());
        values.insert("%ap".to_string(), "ampm".to_string());
        values.insert("%minute".to_string(), "00".to_string());
        values.insert("%mi".to_string(), "00".to_string());
        values.insert("%second".to_string(), "00".to_string());
        values.insert("%se".to_string(), "00".to_string());
        values.insert("%weekday".to_string(), "00".to_string());
        values.insert("%wd".to_string(), "00".to_string());
    }

    if let Some(dur) = gpxmeta.duration {
        values.insert("%duration".to_string(), dur.to_string());
        values.insert("%du".to_string(), dur.to_string());
    } else {
        values.insert("%duration".to_string(), "0".to_string());
        values.insert("%du".to_string(), "0".to_string());
    }

    Ok(values)
}
