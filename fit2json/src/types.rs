//! Contains types used in the serialization and output selection for the data.

use serde::Serialize;
use std::{
    collections::BTreeMap,
    error::Error,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

/// Alternate serialization format
#[derive(Clone, Debug, Serialize)]
pub struct FitDataMap {
    kind: fitparser::profile::MesgNum,
    fields: BTreeMap<String, fitparser::ValueWithUnits>,
}

impl FitDataMap {
    /// Instantiates a new datamap from a `FitDataRecord`
    fn new(record: fitparser::FitDataRecord) -> Self {
        Self {
            kind: record.kind(),
            fields: record
                .into_vec()
                .into_iter()
                .map(|f| (f.name().to_owned(), fitparser::ValueWithUnits::from(f)))
                .collect(),
        }
    }
}

/// Output location alternatives
#[derive(Clone, Debug)]
pub enum OutputLocation {
    /// Put the output in the same place as the input
    Inplace,

    /// Put the output into a separate directory
    LocalDirectory(PathBuf),

    /// Collate all outputs into a single file
    LocalFile(PathBuf),

    /// Output to `stdout`
    Stdout,
}

impl OutputLocation {
    /// Create a new output location based on the location specified.
    pub fn new(location: PathBuf) -> Self {
        if location.is_dir() {
            Self::LocalDirectory(location)
        } else if location.as_os_str() == "-" {
            Self::Stdout
        } else {
            Self::LocalFile(location)
        }
    }

    /// Write to a JSON file
    ///
    /// # Parameters
    ///
    /// `filename: &Path` -- The file(s) or directory where we wish to save.
    ///
    /// `data: Vec<fitparser::FitDataRecord>` -- A vector (list) of `FitDataRecords`
    ///
    /// # Returns
    ///
    /// - `Ok(()` if everything went well.
    /// - `Error` if problems occurred.
    pub fn write_json_file(
        &self,
        filename: &Path,
        data: Vec<fitparser::FitDataRecord>,
    ) -> Result<(), Box<dyn Error>> {
        // convert data to a name: {value, units} map before serializing
        let data: Vec<FitDataMap> = data.into_iter().map(FitDataMap::new).collect();
        let json = serde_json::to_string(&data)?;

        // Figure out where to send the output
        let outname = match self {
            Self::Inplace => filename.with_extension("json"),
            Self::LocalDirectory(dest) => dest
                .clone()
                .join(filename.file_name().unwrap_or_default())
                .with_extension("json"),
            Self::LocalFile(dest) => dest.clone(),
            Self::Stdout => {
                println!("{json}");
                return Ok(());
            }
        };

        // Write the data to the selected output and return the result.
        let mut fp = File::create(outname)?;
        match fp.write_all(json.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
