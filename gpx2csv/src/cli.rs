//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, ArgMatches, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build() -> ArgMatches {
    Command::new(clap::crate_name!())
    .about(clap::crate_description!())
    .version(clap::crate_version!())
    // .author(clap::crate_authors!("\n"))
    .long_about("This program will read one or more .gpx file and output session information to a .json file, the lap information (if any is found) to a .laps.csv file, and the individual records to a .records.csv file. Additionally, a summary sessions.csv file will be produced.")
    .arg(
        Arg::new("read")
            .value_name("FILE(S)")
            .help("One or more .gpx file(s) to process. Wildcards and multiple_occurrences files (e.g. 2019*.gpx 2020*.gpx) are supported.")
            .num_args(1..)
            .required(true)
            .action(ArgAction::Append)
    )
    .arg( // Hidden debug parameter
        Arg::new("debug")
            .short('d')
            .long("debug")
            .help("Output debug information as we go. Supply it twice for trace-level logs.")
            .num_args(0)
            .action(ArgAction::Count)
            .hide(true),
    )
    .arg( // Don't print any information
        Arg::new("quiet")
            .short('q')
            .long("quiet")
            .help("Don't produce any output except errors while working.")
            .num_args(0)
            .action(ArgAction::SetTrue)
    )
    .arg( // Don't export detail information
        Arg::new("detail-off")
            .short('o')
            .long("detail-off")
            .help("Don't export detailed information from each file parsed.")
            .num_args(0)
            .action(ArgAction::SetTrue)
            .requires("summary-file")
    )
    .arg( // Summary file name
        Arg::new("summary-file")
            .short('s')
            .value_name("summary output file name")
            .long("summary-file")
            .help("Summary output file name.")
            .num_args(1)
            .action(ArgAction::Set)
    )
    .get_matches()
}
