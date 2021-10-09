use crate::types::duration::Duration;
use crate::types::hrzones::HrZones;
use crate::types::timestamp::TimeStamp;

use serde::{Deserialize, Serialize};
use uom::si::{
    f64::{Length as Length_f64, Velocity},
    // length::{foot, kilometer, meter, mile},
    length::meter,
    u16::Length as Length_u16,
    // velocity::{foot_per_second, kilometer_per_hour, meter_per_second, mile_per_hour},
    velocity::meter_per_second,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Summary information about the workout session
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub filename: Option<String>, // TODO: Switch to PathBuf
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub serial_number: Option<String>,
    pub time_created: Option<TimeStamp>,
    pub activity_type: Option<String>,
    pub activity_detailed: Option<String>,
    pub num_sessions: Option<u16>,
    pub num_laps: Option<u16>,
    pub num_records: Option<u64>,
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub heartrate_min: Option<u8>,
    pub speed_avg: Option<Velocity>,
    pub speed_max: Option<Velocity>,
    pub power_avg: Option<u16>,
    pub power_max: Option<u16>,
    pub power_threshold: Option<u16>,
    pub nec_lat: Option<f64>,
    pub nec_lon: Option<f64>,
    pub swc_lat: Option<f64>,
    pub swc_lon: Option<f64>,
    pub stance_time_avg: Option<f64>,
    pub vertical_oscillation_avg: Option<f64>,
    pub ascent: Option<Length_u16>,
    pub descent: Option<Length_u16>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Option<Duration>,
    pub duration_active: Option<Duration>,
    pub duration_moving: Option<Duration>,
    pub start_time: Option<TimeStamp>,
    pub finish_time: Option<TimeStamp>,
    pub time_in_hr_zones: HrZones,
}

impl Session {
    /// Initialize Session with default empty values
    pub fn new() -> Self {
        Session::default()
    }

    /// Output details about the session
    pub fn print_session(&self) {
        println!("\n{} summary:\n", self.filename.as_ref().unwrap());
        println!(
            "Manufacturer: {}    Time created: {}",
            self.manufacturer.as_ref().unwrap(),
            self.time_created.as_ref().unwrap()
        );
        println!(
            "Sessions: {}      Laps: {:2}      Records: {}",
            self.num_sessions.unwrap(),
            self.num_laps.unwrap(),
            self.num_records.unwrap()
        );
        println!(
            "Total duration:  {}      Calories Burned: {}",
            self.duration.unwrap(),
            self.calories.unwrap()
        );
        println!("\nTime in Zones:");
        println!("Speed/Power: {}", self.time_in_hr_zones.hr_zone_4.unwrap());
        println!("Anaerobic:   {}", self.time_in_hr_zones.hr_zone_3.unwrap());
        println!("Aerobic:     {}", self.time_in_hr_zones.hr_zone_2.unwrap());
        println!("Fat Burning: {}", self.time_in_hr_zones.hr_zone_1.unwrap());
        println!("Warmup:      {}", self.time_in_hr_zones.hr_zone_0.unwrap());
    }
}

impl Default for Session {
    /// Set defaults to be either empty or zero.
    fn default() -> Self {
        Session {
            filename: None,
            manufacturer: None,
            product: None,
            serial_number: None,
            time_created: None,
            activity_type: None,
            activity_detailed: None,
            num_sessions: None,
            num_laps: None,
            num_records: None,
            cadence_avg: None,
            cadence_max: None,
            heartrate_avg: None,
            heartrate_max: None,
            heartrate_min: None,
            speed_avg: Some(Velocity::new::<meter_per_second>(0.0)),
            speed_max: Some(Velocity::new::<meter_per_second>(0.0)),
            power_avg: None,
            power_max: None,
            power_threshold: None,
            nec_lat: None,
            nec_lon: None,
            swc_lat: None,
            swc_lon: None,
            stance_time_avg: None,
            vertical_oscillation_avg: None,
            ascent: Some(Length_u16::new::<meter>(0)),
            descent: Some(Length_u16::new::<meter>(0)),
            calories: None,
            distance: Some(Length_f64::new::<meter>(0.0)),
            duration: None,
            duration_active: None,
            duration_moving: None,
            start_time: None,
            finish_time: None,
            time_in_hr_zones: HrZones::default(),
        }
    }
}
