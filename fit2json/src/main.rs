//! Read one or more FIT files and dump their contents as JSON
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

// Application-specific types
pub mod types;
use crate::types::OutputLocation;

/// Parse FIT formatted files and output their data in the JSON format
#[derive(Debug, StructOpt)]
#[structopt(name = "fit_to_json")]
struct Cli {
    /// FIT files to convert to JSON
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,

    /// Output location, if not provided, the JSON file will be output alongside the input file. If a
    /// directory is provided, all FIT files will be written there using the same filename as the FIT file,
    /// but with a '.json' extension. If multiple FIT files are provided and the output path isn't a
    /// directory, the JSON array will store all records present in the order they were read. Using
    /// a "-" as the output file name will result in all content being printed to STDOUT.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn run() -> Result<(), Box<dyn Error>> {
    let opt = Cli::from_args();
    let output_loc = opt
        .output
        .map_or(OutputLocation::Inplace, OutputLocation::new);
    let collect_all = matches!(output_loc, OutputLocation::LocalFile(_));
    if opt.files.is_empty() {
        let mut stdin = io::stdin();
        let data = fitparser::from_reader(&mut stdin)?;
        output_loc.write_json_file(&PathBuf::from("<stdin>"), data)?;
        return Ok(());
    }

    // Read each FIT file and output it
    let mut all_fit_data: Vec<fitparser::FitDataRecord> = Vec::new();
    for file in opt.files {
        // open file and parse data
        let mut fp = File::open(&file)?;
        let mut data = fitparser::from_reader(&mut fp)?;

        // output a single fit file's data into a single output file
        if collect_all {
            all_fit_data.append(&mut data);
        } else {
            output_loc.write_json_file(&file, data)?;
        }
    }
    // output fit data from all files into a single file
    if collect_all {
        output_loc.write_json_file(&PathBuf::new(), all_fit_data)?;
    }

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
