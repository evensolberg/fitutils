use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

use chrono::{DateTime, Datelike, Timelike};

use crate::tcxx::activities::ActivitiesSummary;
mod activities;

pub fn process_tcx(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut values = HashMap::<String, String>::new();

    let tcdb = tcx::read(&mut BufReader::new(File::open(&filename).unwrap()))?;

    if let Some(activities) = tcdb.activities {
        let mut act = ActivitiesSummary::from_activities(&activities);
        act.filename = Some(filename.to_string());

        log::debug!("act = {:?}", act);

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
            values.insert("%month".to_string(), format!("{:02}", tc.month0()));
            values.insert("%mo".to_string(), format!("{:02}", tc.month0()));
            values.insert("%day".to_string(), format!("{:02}", tc.day0()));
            values.insert("%dy".to_string(), format!("{:02}", tc.day0()));

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

    Ok(values)
}
