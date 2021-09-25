use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use clap::{App, Arg}; // Command line

// Logging
use log::LevelFilter;
use simple_logger::SimpleLogger;

// Read GPX
use gpx::{Gpx, Track, TrackSegment};

// pub mod types;
pub mod parsers;

fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new("fit2csv")
        .about("Parses .GPX files to .JSON and .CSV")
        .version("0.0.1")
        .long_about("This program will read a .fit file and output session information to a .json file, the lap information (if any is found) to a .laps.csv file, and the individual records to a .records.csv file. Additionally, a summary sessions.csv file will be produced.")
        .arg(
            Arg::with_name("read")
                .value_name("FILE(S)")
                .help("One or more .fit file(s) to process. Wildcards and multiple files (e.g. 2019*.fit 2020*.fit) are supported.")
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

    let file = File::open("../data/running.gpx")?;
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let gpx: Gpx = gpx::read(reader)?;
    log::trace!("gpx = {:?}", gpx);
    log::debug!("gpx.metadata = {:?}", gpx.metadata);

    println!("GPX Version = {}", parsers::gpx_ver_to_string(&gpx.version));

    // Each GPX file has multiple "tracks", this takes the first one.
    let track: &Track = &gpx.tracks[0];
    log::trace!("track = {:?}", track);

    // Each track will have different segments full of waypoints, where a
    // waypoint contains info like latitude, longitude, and elevation.
    let segment: &TrackSegment = &track.segments[0];
    log::trace!("segment = {:?}", segment);

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
