use std::error::Error;

// Submodules
pub mod activities;
pub mod activity;
pub mod duration;
// pub mod fix; // Currently not used.
pub mod gpxmetadata;
pub mod link;
// pub mod person; // Currently not used.
pub mod route;
pub mod timestamp;
pub mod track;
pub mod waypoint;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Provides the capability to export the contents of a data type in JSON format.
pub trait ExportJSON {
    fn export_json(&self) -> Result<(), Box<dyn Error>>;
}

/// Provides the capability to export the contents of a data type in CSV format.
pub trait ExportCSV {
    fn export_csv(&self) -> Result<(), Box<dyn Error>>;
}
