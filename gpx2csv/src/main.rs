use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use clap::{App, Arg}; // Command line

// Logging
use log::LevelFilter;
use simple_logger::SimpleLogger;

// Read GPX
use gpx::Gpx;

// Local modules
pub mod types;
use crate::types::gpxmetadata::GpxMetadata;
use crate::types::track::Track;
use crate::types::ExportJSON;

fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new("gpx2csv")
        .about("Parses .GPX files to .JSON and .CSV")
        .version("0.0.1")
        .long_about("This program will read one or more .gpx file and output session information to a .json file, the lap information (if any is found) to a .laps.csv file, and the individual records to a .records.csv file. Additionally, a summary sessions.csv file will be produced.")
        .arg(
            Arg::with_name("read")
                .value_name("FILE(S)")
                .help("One or more .gpx file(s) to process. Wildcards and multiple files (e.g. 2019*.gpx 2020*.gpx) are supported.")
                .takes_value(true)
                .multiple(true),
        )
        .arg( // Hidden debug parameter
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .multiple(true)
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .takes_value(false)
                .hidden(true),
        )
        .arg( // Don't print any information
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .multiple(false)
                .help("Don't produce any output except errors while working.")
                .takes_value(false)
        )
        .arg( // Print summary information
            Arg::with_name("print-summary")
                .short("s")
                .long("print-summary")
                .multiple(false)
                .help("Print summary detail for each session processed.")
                .takes_value(false)
        )
        .arg( // Don't print any information
            Arg::with_name("summary-only")
                .short("o")
                .long("summary-only")
                .multiple(false)
                .help("Don't produce detail files for each session processed. Only create the summary file sessions.csv")
                .takes_value(false)
        )
        .get_matches();

    // Set up logging according to the number of times the debug flag has been supplied
    let log_level = cli_args.occurrences_of("debug"); // Will pass this to functions in the future.
    match log_level {
        0 => SimpleLogger::new().with_level(LevelFilter::Info).init()?,
        1 => SimpleLogger::new().with_level(LevelFilter::Debug).init()?,
        _ => SimpleLogger::new().with_level(LevelFilter::Trace).init()?, // More than 1
    }

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let filename = "../data/running.gpx";
    let gpx: Gpx = gpx::read(BufReader::new(File::open(&filename)?))?;
    log::debug!("main::run() -- gpx.metadata = {:?}", gpx.metadata);
    log::trace!("\nmain::run() -- gpx = {:?}", gpx);

    // Fill the GPX Header info so we can serialize it later
    let metadata = GpxMetadata::from_header(&gpx, &filename);
    log::debug!("main::run() -- GPX Metadata header: {:?}", metadata);

    metadata.export_json()?;

    // Each GPX file has multiple "tracks", this takes the first one.
    log::debug!(
        "main::run() -- gpx:Number of waypoints: {}",
        metadata.num_waypoints
    );
    log::debug!(
        "main::run() -- gpx:Number of tracks: {}",
        metadata.num_tracks
    );
    log::debug!(
        "main::run() -- gpx:Number of routes: {}",
        metadata.num_routes
    );

    let mut track = Track::from_gpx_track(&gpx.tracks[0], &filename);
    track.track_num += 1;
    log::debug!(
        "main::run() -- track::Number of segments: {}",
        track.num_segments
    );
    log::debug!(
        "main::run() -- track::Number of waypoints: {}",
        track.num_waypoints
    );

    log::trace!("\nmain::run() -- track = {:?}", track);

    // Everything is a-okay in the end
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory
        Err(err) => {
            // Houston, This file contains a problem
            log::error!("{}", Box::new(err)); // Say what's wrong and
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
