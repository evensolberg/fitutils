//! Contains a single function to build the CLI
use clap::{Arg, ArgMatches, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build_cli() -> ArgMatches {
    Command::new(clap::crate_name!())
    .about(clap::crate_description!())
    .version(clap::crate_version!())
    // .author(clap::crate_authors!("\n"))
    .long_about("This program will read one or more .gpx file and output session information to a .json file, the lap information (if any is found) to a .laps.csv file, and the individual records to a .records.csv file. Additionally, a summary sessions.csv file will be produced.")
    .arg(
        Arg::new("read")
            .value_name("FILE(S)")
            .help("One or more .gpx file(s) to process. Wildcards and multiple_occurrences files (e.g. 2019*.gpx 2020*.gpx) are supported.")
            .takes_value(true)
            .required(true)
            .multiple_occurrences(true),
    )
    .arg( // Hidden debug parameter
        Arg::new("debug")
            .short('d')
            .long("debug")
            .multiple_occurrences(true)
            .help("Output debug information as we go. Supply it twice for trace-level logs.")
            .takes_value(false)
            .hide(true),
    )
    .arg( // Don't print any information
        Arg::new("quiet")
            .short('q')
            .long("quiet")
            .multiple_occurrences(false)
            .help("Don't produce any output except errors while working.")
            .takes_value(false)
    )
    .arg( // Don't export detail information
        Arg::new("detail-off")
            .short('o')
            .long("detail-off")
            .multiple_occurrences(false)
            .help("Don't export detailed information from each file parsed.")
            .takes_value(false)
            .requires("summary-file")
    )
    .arg( // Summary file name
        Arg::new("summary-file")
            .short('s')
            .value_name("summary output file name")
            .long("summary-file")
            .multiple_occurrences(false)
            .help("Summary output file name.")
            .takes_value(true)
    )
    .get_matches()
}
