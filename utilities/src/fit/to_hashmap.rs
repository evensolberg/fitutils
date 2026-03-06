use crate::datetime_keys::{insert_datetime_keys, insert_duration_keys};
use crate::fit::session::FITSession;
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
    let mut values = HashMap::<String, String>::with_capacity(32);

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

    insert_datetime_keys(&mut values, my_session.time_created.as_ref());
    insert_duration_keys(&mut values, my_session.duration);

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
        let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/rowing.fit");
        let fm = fit_to_hashmap(filename).unwrap();

        // File contents only get printed if run with cargo test -- --nocapture
        println!("tm = {fm:?}");
        println!("tm.len() = {}", fm.len());
    }
}
