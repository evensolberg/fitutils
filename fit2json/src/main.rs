//! Read one or more FIT files and dump their contents as JSON

use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

// Application-specific types
pub mod types;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

/// Command-line options
#[derive(Debug, StructOpt)]
#[structopt(name = "fit2json")]
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

/// Performs the actual work.
fn run() -> Result<(), Box<dyn Error>> {
    // Configure logging
    Builder::new()
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    let opt = Cli::from_args();
    let output_loc = opt
        .output
        .map_or(types::OutputLocation::Inplace, types::OutputLocation::new);
    let collect_all = matches!(output_loc, types::OutputLocation::LocalFile(_));

    // If no files have been provided, read from STDIN
    if opt.files.is_empty() {
        log::info!("No files supplied. Reading from STDIN.");
        let mut stdin = io::stdin();
        let data = fitparser::from_reader(&mut stdin)?;
        output_loc.write_json_file(&PathBuf::from("<stdin>"), data)?;
        return Ok(());
    }

    // Read each FIT file and output it
    let mut all_fit_data: Vec<fitparser::FitDataRecord> = Vec::new();
    for file in opt.files {
        // open file and parse data
        log::info!("Processing file: {}", &file.to_str().unwrap());
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
        log::info!("Summary information collected in specified output location.");
        output_loc.write_json_file(&PathBuf::new(), all_fit_data)?;
    }

    Ok(())
}

/// Main executable entry point. Hands off to the `run` function.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            Builder::new()
                .filter_level(LevelFilter::Error)
                .target(Target::Stdout)
                .init();
            log::error!("{}", err.to_string().replace("\"", ""));
            1
        }
    });
}
