use chrono::{Datelike, Timelike};
use std::collections::HashMap;

use crate::Duration;

/// Insert standardized datetime key/value pairs into a `HashMap`.
///
/// Canonical keys:
/// `%year`/`%yr`, `%month`/`%mo`, `%day`/`%dy`,
/// `%hour`/`%hr`, `%24hour`/`%24`, `%12hour`/`%12`,
/// `%ampm`/`%ap`, `%minute`/`%mt`, `%second`/`%sc`, `%weekday`/`%wd`
pub fn insert_datetime_keys<Tz: chrono::TimeZone>(
    values: &mut HashMap<String, String>,
    datetime: Option<&chrono::DateTime<Tz>>,
) where
    Tz::Offset: std::fmt::Display,
{
    if let Some(tc) = datetime {
        values.insert("%year".into(), format!("{:04}", tc.year()));
        values.insert("%yr".into(), format!("{:04}", tc.year()));
        values.insert("%month".into(), format!("{:02}", tc.month()));
        values.insert("%mo".into(), format!("{:02}", tc.month()));
        values.insert("%day".into(), format!("{:02}", tc.day()));
        values.insert("%dy".into(), format!("{:02}", tc.day()));

        values.insert("%hour".into(), format!("{:02}", tc.hour()));
        values.insert("%hr".into(), format!("{:02}", tc.hour()));
        values.insert("%24hour".into(), format!("{:02}", tc.hour()));
        values.insert("%24".into(), format!("{:02}", tc.hour()));

        let (is_pm, hrs) = tc.hour12();
        let hr = format!("{hrs:02}");
        values.insert("%12hour".into(), hr.clone());
        values.insert("%12".into(), hr);
        if is_pm {
            values.insert("%ampm".into(), "pm".into());
            values.insert("%ap".into(), "pm".into());
        } else {
            values.insert("%ampm".into(), "am".into());
            values.insert("%ap".into(), "am".into());
        }

        values.insert("%minute".into(), format!("{:02}", tc.minute()));
        values.insert("%mt".into(), format!("{:02}", tc.minute()));
        values.insert("%second".into(), format!("{:02}", tc.second()));
        values.insert("%sc".into(), format!("{:02}", tc.second()));
        values.insert("%weekday".into(), tc.weekday().to_string());
        values.insert("%wd".into(), tc.weekday().to_string());
    } else {
        values.insert("%year".into(), "0000".into());
        values.insert("%yr".into(), "0000".into());
        for key in [
            "%month", "%mo", "%day", "%dy", "%hour", "%hr", "%24hour", "%24", "%12hour", "%12",
            "%minute", "%mt", "%second", "%sc", "%weekday", "%wd",
        ] {
            values.insert(key.into(), "00".into());
        }
        values.insert("%ampm".into(), "am".into());
        values.insert("%ap".into(), "am".into());
    }
}

/// Insert duration key/value pairs into a `HashMap`.
pub fn insert_duration_keys(values: &mut HashMap<String, String>, duration: Option<Duration>) {
    if let Some(dur) = duration {
        values.insert("%duration".into(), dur.0.as_secs().to_string());
        values.insert("%du".into(), dur.0.as_secs().to_string());
    } else {
        values.insert("%duration".into(), "0".into());
        values.insert("%du".into(), "0".into());
    }
}
