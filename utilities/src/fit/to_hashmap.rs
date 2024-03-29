use crate::fit::session::FITSession;
use chrono::{Datelike, Timelike};
use convert_case::{Case, Casing};
use fitparser::profile::field_types::MesgNum;
use std::{collections::HashMap, error::Error, fs::File};

/// Process a .FIT file and return the results
///
/// # Arguments
///
/// `filename: &str` -- The name of the FIT file to be read
///
/// # Returns
///
/// A `Result<HashMap<String, String>>` - a `HashMap` with mappings of keys (i.e., manufacturer, activity, etc.) and values.
///
/// # Errors
///
/// Reading the file may fail.
///
/// # Panics
///
/// None.
#[allow(clippy::module_name_repetitions, clippy::too_many_lines)]
pub fn fit_to_hashmap(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut values = HashMap::<String, String>::new();

    // open the file and deserialize it - return error if unable.
    let mut fp = File::open(filename)?;
    let file = fitparser::from_reader(&mut fp)?;

    // Create a bunch of placeholder variables.
    let mut my_session = FITSession::with_filename(filename);
    let mut num_sessions = 0;

    // This is where the actual parsing happens
    for data in file {
        // for each FitDataRecord
        match data.kind() {
            // Figure out what kind it is and parse accordingly
            MesgNum::FileId => {
                // File header
                my_session.parse_header(data.fields());
            }
            MesgNum::Session => {
                my_session.parse_session(data.fields());
                num_sessions += 1;
                my_session.num_sessions = Some(num_sessions);
            }
            _ => (),
        } // match
    } // for data

    // Push the data into the HashMap for later use.
    let mf = my_session
        .manufacturer
        .unwrap_or_else(|| "unknown".to_string())
        .to_case(Case::Title);
    values.insert("%manufacturer".to_string(), mf.clone());
    values.insert("%mf".to_string(), mf);

    let pr = my_session
        .product
        .unwrap_or_else(|| "unknown".to_string())
        .to_case(Case::Title);
    values.insert("%product".to_string(), pr.clone());
    values.insert("%pr".to_string(), pr);

    let sn = my_session
        .serial_number
        .unwrap_or_else(|| "unknown".to_string());
    values.insert("%serial_number".to_string(), sn.clone());
    values.insert("%sn".to_string(), sn);

    let ac = my_session
        .activity_type
        .unwrap_or_else(|| "unknown".to_string())
        .to_case(Case::Title);
    values.insert("%activity".to_string(), ac.clone());
    values.insert("%at".to_string(), ac);

    let ad = my_session
        .activity_detailed
        .unwrap_or_else(|| "unknown".to_string())
        .to_case(Case::Title);
    values.insert("%activity_detailed".to_string(), ad.clone());
    values.insert("%ad".to_string(), ad);

    if let Some(tc) = my_session.time_created {
        values.insert("%year".to_string(), format!("{:04}", tc.year()));
        values.insert("%yr".to_string(), format!("{:04}", tc.year()));
        values.insert("%month".to_string(), format!("{:02}", tc.month()));
        values.insert("%mn".to_string(), format!("{:02}", tc.month()));
        values.insert("%day".to_string(), format!("{:02}", tc.day()));
        values.insert("%dy".to_string(), format!("{:02}", tc.day()));

        values.insert("%hour".to_string(), format!("{:02}", tc.hour()));
        values.insert("%hr".to_string(), format!("{:02}", tc.hour()));
        values.insert("%24hour".to_string(), format!("{:02}", tc.hour()));
        values.insert("%24".to_string(), format!("{:02}", tc.hour()));

        let (am, hrs) = tc.hour12();
        let hr = format!("{hrs:02}");
        values.insert("%hour12".to_string(), hr.clone());
        values.insert("%12".to_string(), hr);
        if am {
            values.insert("%ampm".to_string(), "pm".to_string());
            values.insert("%ap".to_string(), "pm".to_string());
        } else {
            values.insert("%ampm".to_string(), "am".to_string());
            values.insert("%ap".to_string(), "am".to_string());
        }

        values.insert("%minute".to_string(), format!("{:02}", tc.minute()));
        values.insert("%mt".to_string(), format!("{:02}", tc.minute()));
        values.insert("%second".to_string(), format!("{:02}", tc.second()));
        values.insert("%sc".to_string(), format!("{:02}", tc.second()));
        values.insert("%weekday".to_string(), tc.weekday().to_string());
        values.insert("%wd".to_string(), tc.weekday().to_string());
    } else {
        values.insert("%year".to_string(), "0000".to_string());
        values.insert("%yr".to_string(), "0000".to_string());
        values.insert("%month".to_string(), "00".to_string());
        values.insert("%mn".to_string(), "00".to_string());
        values.insert("%day".to_string(), "00".to_string());
        values.insert("%dy".to_string(), "00".to_string());
        values.insert("%hour".to_string(), "00".to_string());
        values.insert("%hr".to_string(), "00".to_string());
        values.insert("%24hour".to_string(), "00".to_string());
        values.insert("%24".to_string(), "00".to_string());
        values.insert("%12hour".to_string(), "00".to_string());
        values.insert("%12".to_string(), "00".to_string());
        values.insert("%ampm".to_string(), "ampm".to_string());
        values.insert("%ap".to_string(), "ampm".to_string());
        values.insert("%minute".to_string(), "00".to_string());
        values.insert("%mt".to_string(), "00".to_string());
        values.insert("%second".to_string(), "00".to_string());
        values.insert("%sc".to_string(), "00".to_string());
        values.insert("%weekday".to_string(), "00".to_string());
        values.insert("%wd".to_string(), "00".to_string());
    }

    if let Some(dur) = my_session.duration {
        values.insert("%duration".to_string(), dur.to_string());
        values.insert("%du".to_string(), dur.to_string());
    } else {
        values.insert("%duration".to_string(), "0".to_string());
        values.insert("%du".to_string(), "0".to_string());
    }

    log::debug!("values = {values:?}");

    // return safely
    Ok(values)
}

#[cfg(test)]
///
mod tests {
    use super::*;

    #[test]
    ///
    fn test_process_fit() {
        let filename = "../data/test.fit";
        let fm = fit_to_hashmap(filename).unwrap();

        // File contents only get printed if run with cargo test -- --nocapture
        println!("tm = {fm:?}");
        println!("tm.len() = {}", fm.len());
    }
}
