//! Contains the various types used in the parsing of FIT files, and their associated functions.

// publish the types
pub use crate::types::{
    activities::Activities, activity::Activity, hrzones::HrZones, lap::Lap, record::Record,
    session::Session,
};

// This is where the types are defined
mod activities;
mod activity;
mod constfunc;
mod hrzones;
mod lap;
mod record;
mod session;
