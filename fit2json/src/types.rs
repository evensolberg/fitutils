use serde::Serialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

/// Alternate serialization format
#[derive(Clone, Debug, Serialize)]
pub struct FitDataMap {
    kind: fitparser::profile::MesgNum,
    fields: BTreeMap<String, fitparser::ValueWithUnits>,
}

impl FitDataMap {
    fn new(record: fitparser::FitDataRecord) -> Self {
        FitDataMap {
            kind: record.kind(),
            fields: record
                .into_vec()
                .into_iter()
                .map(|f| (f.name().to_owned(), fitparser::ValueWithUnits::from(f)))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OutputLocation {
    Inplace,
    LocalDirectory(PathBuf),
    LocalFile(PathBuf),
    Stdout,
}

impl OutputLocation {
    pub fn new(location: PathBuf) -> Self {
        if location.is_dir() {
            OutputLocation::LocalDirectory(location)
        } else if location.as_os_str() == "-" {
            OutputLocation::Stdout
        } else {
            OutputLocation::LocalFile(location)
        }
    }

    pub fn write_json_file(
        &self,
        filename: &Path,
        data: Vec<fitparser::FitDataRecord>,
    ) -> Result<(), Box<dyn Error>> {
        // convert data to a name: {value, units} map before serializing
        let data: Vec<FitDataMap> = data.into_iter().map(FitDataMap::new).collect();
        let json = serde_json::to_string(&data)?;

        let outname = match self {
            Self::Inplace => filename.with_extension("json"),
            Self::LocalDirectory(dest) => dest
                .clone()
                .join(filename.file_name().unwrap())
                .with_extension("json"),
            Self::LocalFile(dest) => dest.clone(),
            Self::Stdout => {
                println!("{}", json);
                return Ok(());
            }
        };
        let mut fp = File::create(outname)?;
        match fp.write_all(json.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
