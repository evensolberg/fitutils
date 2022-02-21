use std::{collections::HashMap, error::Error};

use chrono::{DateTime, Datelike, Timelike};
use utilities::TCXActivitiesSummary;

pub fn process_tcx(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut values = HashMap::<String, String>::new();
    let tcdb;

    // Make sure we can open the file correctly
    let tcdb_res = tcx::read_file(filename);
    match tcdb_res {
        Ok(res) => tcdb = res,
        Err(err) => return Err(format!("Unable to open {}. Error: {}", filename, err).into()),
    }

    if let Some(activities) = tcdb.activities {
        let mut act = TCXActivitiesSummary::from_activities(&activities);
        act.filename = Some(filename.to_string());

        log::debug!("act = {:?}", act);

        // Insert values into HashMap
        // Insert "unknown" into all the fields that don't have a correponding field in the TCX.
        let unknown = "unknown".to_string();
        values.insert("%manufacturer".to_string(), unknown.clone());
        values.insert("%unknown".to_string(), unknown.clone());
        values.insert("%product".to_string(), unknown.clone());
        values.insert("%pr".to_string(), unknown.clone());
        values.insert("%serial_number".to_string(), unknown.clone());
        values.insert("%sn".to_string(), unknown.clone());

        let ac = act.sport.unwrap_or_else(|| "unknown".to_string());
        values.insert("%activity".to_string(), ac.clone());
        values.insert("%ac".to_string(), ac);

        values.insert("%activity_detailed".to_string(), unknown.clone());
        values.insert("%ad".to_string(), unknown);

        if let Some(st) = act.start_time {
            // TODO: May want to switch to TimeStamp for consistency.
            let tc = DateTime::parse_from_rfc3339(&st)?.with_timezone(&chrono::Local);

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
            let hr = format!("{:02}", hrs);
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

        if let Some(dur) = act.total_time_seconds {
            values.insert("%duration".to_string(), (dur as usize).to_string());
            values.insert("%du".to_string(), (dur as usize).to_string());
        } else {
            values.insert("%duration".to_string(), "0".to_string());
            values.insert("%du".to_string(), "0".to_string());
        }
    }

    // Return safely
    Ok(values)
}

#[cfg(test)]
///
mod tests {
    use super::*;
    use assay::assay;

    #[assay(include = ["/Users/evensolberg/CloudStation/Source/Rust/fitutils/data/running.tcx"])]
    ///
    fn test_process_tcx() {
        // Read the file
        let filename = "/Users/evensolberg/CloudStation/Source/Rust/fitutils/data/running.tcx";
        let tm = process_tcx(filename)?;

        // File contents only get printed if run with cargo test -- --nocapture
        println!("tm = {:?}", tm);
        println!("tm.len() = {}", tm.len());

        // Perform the actual tests
        assert_eq!(tm.len(), 32);
        assert_eq!(
            tm.get("%activity").unwrap().to_string(),
            "Running".to_string()
        );
        assert_eq!(tm.get("%ac").unwrap().to_string(), "Running".to_string());
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
        assert_eq!(tm.get("%h12").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%h24").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%hour").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%hour12").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%hour24").unwrap().to_string(), "06".to_string());
        assert_eq!(
            tm.get("%manufacturer").unwrap().to_string(),
            "unknown".to_string()
        );
        assert_eq!(tm.get("%mi").unwrap().to_string(), "35".to_string());
        assert_eq!(tm.get("%minute").unwrap().to_string(), "35".to_string());
        assert_eq!(tm.get("%mo").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%month").unwrap().to_string(), "06".to_string());
        assert_eq!(tm.get("%pr").unwrap().to_string(), "unknown".to_string());
        assert_eq!(
            tm.get("%product").unwrap().to_string(),
            "unknown".to_string()
        );
        assert_eq!(tm.get("%se").unwrap().to_string(), "49".to_string());
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
