use std::{collections::HashMap, error::Error};

use crate::datetime_keys::{insert_datetime_keys, insert_duration_keys};
use crate::TCXActivity;
use chrono::DateTime;
use convert_case::{Case, Casing};

/// Iterates through a TCX file and saves the information to a `HashMap`
///
/// # Arguments
///
/// `filename: &str` -- the name of the TCX file to be processed.
///
/// # Returns
///
/// `HashMap(String, String)` containing key (manufacturer, product, etc.)/value mappings.
///
/// # Errors
///
/// Opening the file may fail.
///
/// # Panics
///
/// None.
#[allow(clippy::module_name_repetitions, clippy::unwrap_used)]
pub fn tcx_to_hashmap(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut values = HashMap::<String, String>::new();

    // Make sure we can open the file correctly
    let tcdb = match tcx::read_file(filename) {
        Ok(res) => res,
        Err(err) => return Err(format!("Unable to open {filename}. Error: {err}").into()),
    };

    if let Some(activities) = tcdb.activities {
        let mut act = TCXActivity::from_activities(&activities);
        act.filename = Some(filename.to_string());

        log::debug!("act = {act:?}");

        // Insert values into HashMap
        // Insert "unknown" into all the fields that don't have a corresponding field in the TCX.
        let unknown = "unknown".to_string();
        values.insert("%manufacturer".to_string(), unknown.to_case(Case::Title));
        values.insert("%unknown".to_string(), unknown.clone());
        values.insert("%product".to_string(), unknown.to_case(Case::Title));
        values.insert("%pr".to_string(), unknown.to_case(Case::Title));
        values.insert("%serial_number".to_string(), unknown.clone());
        values.insert("%sn".to_string(), unknown.clone());

        let ac = act
            .sport
            .unwrap_or_else(|| "unknown".to_string())
            .to_case(Case::Title);
        values.insert("%activity".to_string(), ac.clone());
        values.insert("%at".to_string(), ac);

        values.insert("%activity_detailed".to_string(), unknown.clone());
        values.insert("%ad".to_string(), unknown);

        let parsed_time = act
            .start_time
            .as_ref()
            .and_then(|st| DateTime::parse_from_rfc3339(st).ok())
            .map(|dt| dt.with_timezone(&chrono::Local));
        insert_datetime_keys(&mut values, parsed_time.as_ref());
        insert_duration_keys(&mut values, act.duration);
    }

    // Return safely
    Ok(values)
}

#[cfg(test)]
///
mod tests {
    use super::*;
    /// Test the processing of a TCX file
    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_process_tcx() {
        // Read the file
        let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/running.tcx");
        let tm = tcx_to_hashmap(filename).unwrap();

        // File contents only get printed if run with cargo test -- --nocapture
        println!("tm = {tm:?}");
        println!("tm.len() = {}", tm.len());

        // Perform the actual tests
        assert_eq!(tm.len(), 32);
        assert_eq!(
            tm.get("%activity").unwrap().to_string(),
            "Running".to_string()
        );
        assert_eq!(tm.get("%at").unwrap().to_string(), "Running".to_string());
        assert_eq!(
            tm.get("%activity_detailed").unwrap().to_string(),
            "unknown".to_string()
        );
        assert_eq!(tm.get("%ad").unwrap().to_string(), "unknown".to_string());
        assert_eq!(tm.get("%ampm").unwrap().to_string(), "am".to_string());
        assert_eq!(tm.get("%ap").unwrap().to_string(), "am".to_string());
        assert_eq!(tm.get("%day").unwrap().to_string(), "15".to_string());
        assert_eq!(tm.get("%du").unwrap().to_string(), "1325".to_string());
        assert_eq!(tm.get("%duration").unwrap().to_string(), "1325".to_string());
        assert_eq!(tm.get("%dy").unwrap().to_string(), "15".to_string());
        assert_eq!(tm.get("%hr").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%12").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%24").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%hour").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%12hour").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%24hour").unwrap().to_string(), "06".to_string());
        assert_eq!(
            tm.get("%manufacturer").unwrap().to_string(),
            "Unknown".to_string()
        );
        assert_eq!(tm.get("%minute").unwrap().to_string(), "35".to_string());
        assert_eq!(tm.get("%mt").unwrap().to_string(), "35".to_string());
        assert_eq!(tm.get("%month").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%mo").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%pr").unwrap().to_string(), "Unknown".to_string());
        assert_eq!(
            tm.get("%product").unwrap().to_string(),
            "Unknown".to_string()
        );
        assert_eq!(tm.get("%sc").unwrap().to_string(), "49".to_string());
        assert_eq!(tm.get("%second").unwrap().to_string(), "49".to_string());
        assert_eq!(
            tm.get("%serial_number").unwrap().to_string(),
            "unknown".to_string()
        );
        assert_eq!(tm.get("%sn").unwrap().to_string(), "unknown".to_string());
        assert_eq!(
            tm.get("%unknown").unwrap().to_string(),
            "unknown".to_string()
        );
        assert_eq!(tm.get("%wd").unwrap().to_string(), "Fri".to_string());
        assert_eq!(tm.get("%weekday").unwrap().to_string(), "Fri".to_string());
        assert_eq!(tm.get("%year").unwrap().to_string(), "2018".to_string());
        assert_eq!(tm.get("%yr").unwrap().to_string(), "2018".to_string());
    }
}
