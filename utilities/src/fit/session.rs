//! Defines the `Session` struct which holds summary information about the workout session, and associated functions.

use crate::Duration;
use crate::{
    fit::constfunc::{
        map_float64, map_sint32, map_string, map_uint16, map_uint8, LATLON_MULTIPLIER,
    },
    FITHrZones,
};

use chrono::{DateTime, Local, TimeZone};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use convert_case::{Case, Casing};
use fitparser::FitDataField;
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
#[allow(clippy::module_name_repetitions)]
pub struct FITSession {
    pub filename: Option<String>, // TODO: Switch to PathBuf
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub serial_number: Option<String>,
    pub time_created: Option<DateTime<Local>>,
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
    pub start_time: Option<DateTime<Local>>,
    pub finish_time: Option<DateTime<Local>>,
    pub time_in_hr_zones: FITHrZones,
}

impl FITSession {
    /// Initialize Session with default empty values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new, empty session and seft the `filename` value to the filename supplied.
    ///
    /// # Arguments
    ///
    /// `filename: &str` - The name of the file to be read.
    ///
    /// # Returns
    ///
    /// `Self` - A new `FITSession` with the file name field populated.
    ///
    /// # Errors
    ///
    ///
    #[must_use]
    pub fn with_filename(filename: &str) -> Self {
        let mut session = Self::new();
        session.filename = Some(filename.to_string());

        session
    }

    /// Output details about the session
    pub fn print_summary(&self) {
        let unknown = String::from("Unknown");

        println!(
            "\n{} summary:\n",
            self.filename.as_ref().unwrap_or(&String::from(&unknown))
        );
        println!(
            "Manufacturer: {}    Time created: {}",
            self.manufacturer.as_ref().unwrap_or(&unknown),
            self.time_created
                .as_ref()
                .unwrap_or(&Local.timestamp_opt(0, 0).unwrap())
        );
        println!(
            "Sessions: {}      Laps: {:2}      Records: {}",
            self.num_sessions.unwrap_or_default(),
            self.num_laps.unwrap_or_default(),
            self.num_records.unwrap_or_default()
        );
        println!(
            "Total duration:  {}      Calories Burned: {}",
            self.duration.unwrap_or_default(),
            self.calories.unwrap_or_default()
        );
        println!("\nTime in Zones:");
        println!(
            "Speed/Power: {}",
            self.time_in_hr_zones.hr_zone_4.unwrap_or_default()
        );
        println!(
            "Anaerobic:   {}",
            self.time_in_hr_zones.hr_zone_3.unwrap_or_default()
        );
        println!(
            "Aerobic:     {}",
            self.time_in_hr_zones.hr_zone_2.unwrap_or_default()
        );
        println!(
            "Fat Burning: {}",
            self.time_in_hr_zones.hr_zone_1.unwrap_or_default()
        );
        println!(
            "Warmup:      {}",
            self.time_in_hr_zones.hr_zone_0.unwrap_or_default()
        );
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Extract manufacturer and session creation time from the FIT data file header
    ///
    /// # Arguments
    ///
    ///    `fields: &[FitDataField]` -- See the fitparser crate for details: <https://docs.rs/fitparser/0.4.0/fitparser/struct.FitDataField.html><br>
    ///
    /// # Returns
    ///
    ///    `Result<(), Box<dyn Error>>` -- Returns nothing if OK, error if problematic.
    ///
    /// # Errors
    ///
    ///
    pub fn parse_header(&mut self, fields: &[FitDataField]) {
        let field_map: HashMap<&str, &fitparser::Value> =
            fields.iter().map(|x| (x.name(), x.value())).collect();

        self.manufacturer = field_map.get("manufacturer").and_then(map_string);
        self.product = field_map.get("product").and_then(map_string);
        self.serial_number = field_map.get("serial_number").and_then(map_string);

        if let Some(fitparser::Value::Timestamp(ft)) = field_map.get("time_created") {
            self.time_created = Some(*ft);
        } else {
            self.time_created = None;
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Parses session information into more detail.
    ///
    /// **Parameters:**
    ///
    ///    `session: &mut types::Session` -- An empty session struct to be filled in. See `types.rs` for details on this struct.
    ///
    /// **Returns:**
    ///
    ///    `Result<(), Box<dyn Error>>` -- Returns nothing if OK, error if problematic.
    pub fn parse_session(&mut self, fields: &[FitDataField]) {
        let field_map: HashMap<&str, &fitparser::Value> =
            fields.iter().map(|x| (x.name(), x.value())).collect();
        log::trace!(
            "Sparsers::parse_session() -- ession field_map = {:?}",
            field_map
        );

        self.activity_type = field_map.get("sport").and_then(map_string);
        self.activity_type = Some(
            self.activity_type
                .as_ref()
                .unwrap_or(&"unknown".to_string())
                .to_case(Case::Title),
        );
        self.activity_detailed = field_map.get("sub_sport").and_then(map_string);
        self.activity_detailed = Some(
            self.activity_detailed
                .as_ref()
                .unwrap_or(&"unknown".to_string())
                .to_case(Case::Title),
        );

        self.cadence_avg = field_map.get("avg_cadence").and_then(map_uint8);
        self.cadence_max = field_map.get("max_cadence").and_then(map_uint8);

        self.heartrate_avg = field_map.get("avg_heart_rate").and_then(map_uint8);
        self.heartrate_max = field_map.get("max_heart_rate").and_then(map_uint8);
        self.heartrate_min = field_map.get("min_heart_rate").and_then(map_uint8);

        self.stance_time_avg = field_map.get("avg_stance_time").and_then(map_float64);
        self.vertical_oscillation_avg = field_map
            .get("avg_vertical_oscillation")
            .and_then(map_float64);

        self.speed_avg = field_map
            .get("enhanced_avg_speed")
            .and_then(map_float64)
            .map(Velocity::new::<meter_per_second>);
        self.speed_max = field_map
            .get("enhanced_max_speed")
            .and_then(map_float64)
            .map(Velocity::new::<meter_per_second>);

        self.power_avg = field_map.get("avg_power").and_then(map_uint16);
        self.power_max = field_map.get("max_power").and_then(map_uint16);
        self.power_threshold = field_map.get("threshold_power").and_then(map_uint16);

        // GPS - NEC = North East Corner, SWC = South West Corner
        self.nec_lat = field_map
            .get("nec_lat")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);
        self.nec_lon = field_map
            .get("nec_long")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);
        self.swc_lat = field_map
            .get("swc_lat")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);
        self.swc_lon = field_map
            .get("swc_long")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * LATLON_MULTIPLIER);

        self.ascent = field_map
            .get("total_ascent")
            .and_then(map_uint16)
            .map(Length_u16::new::<meter>);
        self.descent = field_map
            .get("total_descent")
            .and_then(map_uint16)
            .map(Length_u16::new::<meter>);

        self.calories = field_map.get("total_calories").and_then(map_uint16);
        self.distance = field_map
            .get("total_distance")
            .and_then(map_float64)
            .map(Length_f64::new::<meter>);

        self.duration = field_map
            .get("total_elapsed_time")
            .and_then(map_float64)
            .map(Duration::from_secs_f64);
        self.duration_active = field_map
            .get("total_timer_time")
            .and_then(map_float64)
            .map(Duration::from_secs_f64);
        self.duration_moving = field_map
            .get("total_moving_time")
            .and_then(map_float64)
            .map(Duration::from_secs_f64);

        if let Some(fitparser::Value::Timestamp(st)) = field_map.get("start_time") {
            self.start_time = Some(*st);
        } else {
            self.start_time = None;
        }

        if let Some(fitparser::Value::Timestamp(ft)) = field_map.get("DateTime<Local>") {
            self.finish_time = Some(*ft);
        } else {
            self.finish_time = None;
        }
        self.num_laps = field_map.get("num_laps").and_then(map_uint16);

        self.time_in_hr_zones = FITHrZones::from(field_map.get("time_in_hr_zone"));
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Export the session information to a JSON file name based on the FIT file name.
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful.
    ///
    /// # Errors
    ///
    /// Writing the JSON could fail.
    pub fn export_json(&self) -> Result<(), Box<dyn Error>> {
        // Change the file extension
        let mut export_path = PathBuf::from(
            &self
                .filename
                .as_ref()
                .unwrap_or(&String::from("export-session.json")),
        );
        export_path.set_extension("session.json");
        log::trace!(
            "exporter::export_session_json() -- Writing JSON file {}",
            &export_path.to_str().unwrap_or("<Unknown filename>")
        );

        // Write the session data to JSON
        serde_json::to_writer_pretty(&File::create(&export_path)?, &self)?;

        // Everything is OK
        Ok(())
    }

    // end impl Session
}

impl Default for FITSession {
    /// Set defaulft to be either empty or zero.
    fn default() -> Self {
        Self {
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
            speed_avg: None,
            speed_max: None,
            power_avg: None,
            power_max: None,
            power_threshold: None,
            nec_lat: None,
            nec_lon: None,
            swc_lat: None,
            swc_lon: None,
            stance_time_avg: None,
            vertical_oscillation_avg: None,
            ascent: None,
            descent: None,
            calories: None,
            distance: None,
            duration: None,
            duration_active: None,
            duration_moving: None,
            start_time: None,
            finish_time: None,
            time_in_hr_zones: FITHrZones::default(),
        }
    }
}
