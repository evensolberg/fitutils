// pub mod types {
use chrono::{offset::Local, DateTime};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub};
pub use uom::si::{
    f64::{Length as Length_f64, Velocity},
    length::{foot, kilometer, meter, mile},
    u16::Length as Length_u16,
    velocity::{foot_per_second, kilometer_per_hour, meter_per_second, mile_per_hour},
};




/// Units of measure to be used
#[derive(Clone)]
pub enum Unit {
    Metric,
    Imperial,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Unit::Metric => "km",
            Unit::Imperial => "mi",
        };
        write!(f, "{}", value)
    }
}

/// Functionality to display various types of units of measure.
pub trait DisplayUnit {
    /// Display kilometers or miles per hour (placeholder)
    fn display_km_mi(&self, unit: &Unit) -> String;
    /// Display meters or feet per second (placeholder)
    fn display_m_ft(&self, unit: &Unit) -> String;
}

/// Get metrics with units of measure
pub trait GetWithUnit {
    /// Get the value with unit of measure (placeholder)
    fn get_with_unit(&self, unit: &Unit) -> f64;
}

impl GetWithUnit for Length_f64 {
    /// Get the value with unit of measure for type Length_f64
    fn get_with_unit(&self, unit: &Unit) -> f64 {
        match unit {
            Unit::Metric => self.get::<kilometer>(),
            Unit::Imperial => self.get::<mile>(),
        }
    }
}

impl DisplayUnit for Velocity {
    /// Display kilometers or miles per hour for type Velocity
    fn display_km_mi(&self, unit: &Unit) -> String {
        if let Unit::Metric = unit {
            format!(
                "{:.2}",
                (*self).into_format_args(kilometer_per_hour, uom::fmt::DisplayStyle::Abbreviation)
            )
        } else {
            format!(
                "{:.2}",
                (*self).into_format_args(mile_per_hour, uom::fmt::DisplayStyle::Abbreviation)
            )
        }
    }

    /// Display meters or feet per second for type Velocity
    fn display_m_ft(&self, unit: &Unit) -> String {
        if let Unit::Metric = unit {
            format!(
                "{:.2}",
                (*self).into_format_args(meter_per_second, uom::fmt::DisplayStyle::Abbreviation)
            )
        } else {
            format!(
                "{:.2}",
                (*self).into_format_args(foot_per_second, uom::fmt::DisplayStyle::Abbreviation)
            )
        }
    }
}

impl DisplayUnit for Length_f64 {
    /// Display kilometers or miles per hour for type Length_f64
    fn display_km_mi(&self, unit: &Unit) -> String {
        if let Unit::Metric = unit {
            format!(
                "{:.2}",
                (*self).into_format_args(kilometer, uom::fmt::DisplayStyle::Abbreviation)
            )
        } else {
            format!(
                "{:.2}",
                (*self).into_format_args(mile, uom::fmt::DisplayStyle::Abbreviation)
            )
        }
    }

    /// Display meters or feet per second for type Length_f64
    fn display_m_ft(&self, unit: &Unit) -> String {
        if let Unit::Metric = unit {
            format!(
                "{:.2}",
                (*self).into_format_args(meter, uom::fmt::DisplayStyle::Abbreviation)
            )
        } else {
            format!(
                "{:.2}",
                (*self).into_format_args(foot, uom::fmt::DisplayStyle::Abbreviation)
            )
        }
    }
}

impl DisplayUnit for Length_u16 {
    /// Display kilometers or miles per hour for type Length_u16
    fn display_km_mi(&self, unit: &Unit) -> String {
        if let Unit::Metric = unit {
            format!(
                "{:.2}",
                (*self).into_format_args(kilometer, uom::fmt::DisplayStyle::Abbreviation)
            )
        } else {
            format!(
                "{:.2}",
                (*self).into_format_args(mile, uom::fmt::DisplayStyle::Abbreviation)
            )
        }
    }

    /// Display meters or feet per second for type Length_u16
    fn display_m_ft(&self, unit: &Unit) -> String {
        if let Unit::Metric = unit {
            format!(
                "{:.2}",
                (*self).into_format_args(meter, uom::fmt::DisplayStyle::Abbreviation)
            )
        } else {
            format!(
                "{:.2}",
                (*self).into_format_args(foot, uom::fmt::DisplayStyle::Abbreviation)
            )
        }
    }
}




/// Wrapper for chrono::DateTime.
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeStamp(pub DateTime<Local>);

impl Default for TimeStamp {
    /// Initialize TimeStamp to current time.
    fn default() -> TimeStamp {
        TimeStamp(Local::now())
    }
}

impl std::fmt::Display for TimeStamp {
    /// Format time to `%d.%m.%Y %H:%M`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%d.%m.%Y %H:%M"))
    }
}




/// Wrapper for std::time::Duration.
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Duration(std::time::Duration);

impl Duration {
    /// Get duration from seconds.
    pub fn from_secs_f64(secs: f64) -> Self {
        Duration(std::time::Duration::from_secs_f64(secs))
    }

    /// Calculate the duration between two TimeStamps
    pub fn between(ts1: &TimeStamp, ts2: &TimeStamp) -> Self {
        Duration(
            chrono::Duration::to_std(&ts1.0.signed_duration_since(ts2.0))
                .expect("Duration out of bounds"),
        )
    }
}

impl Add for Duration {
    type Output = Self;
    /// Implements the `+` operation for Duration.
    fn add(self, rhs: Duration) -> Self::Output {
        Duration(
            self.0
                .checked_add(rhs.0)
                .expect("overflow when adding durations."),
        )
    }
}

impl AddAssign for Duration {
    /// Implements the `+=` operation for Duration.
    fn add_assign(&mut self, rhs: Duration) {
        self.0 = self.0 + rhs.0;
    }
}

impl Sub for Duration {
    type Output = Duration;
    /// Implements the `-` operation for Duration.
    fn sub(self, rhs: Duration) -> Duration {
        Duration(
            self.0
                .checked_sub(rhs.0)
                .expect("overflow when subtracting durations"),
        )
    }
}

impl std::fmt::Display for Duration {
    /// Impements a way to format (and hence display) Duration.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.as_secs();
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);
        write!(f, "{:02}:{:02}:{:02}", h, m, s)
    }
}
// } // pub mod types
