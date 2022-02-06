use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

use chrono::{Datelike, Timelike};

use crate::tcxx::activities::ActivitiesSummary;
use tcx;
mod activities;

pub fn process_tcx(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut values = HashMap::<String, String>::new();

    let tcdb = tcx::read(&mut BufReader::new(File::open(&filename).unwrap()))?;

    if let Some(activities) = tcdb.activities {
        let mut curr_activities = ActivitiesSummary::from_activities(&activities);
        curr_activities.filename = Some(filename.to_string());
    }

    Ok(values)
}
