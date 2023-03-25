//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, ArgMatches, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build() -> ArgMatches {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        // .author(clap::crate_authors!("\n"))
        .long_about("This show the metadata contents of FIT, GPX and TCX files.")
        .arg(
            Arg::new("read")
                .value_name("FILE(S)")
                .help("One or more .fit, .gpx or .tcx file(s) to process. Wildcards and multiple_occurrences files (e.g. 2019*.fit 2020*.gpx) are supported.")
                .num_args(1..)
                .required(true)
                .action(ArgAction::Append)
        )
        .arg( // Hidden debug parameter
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .env("FIT_DEBUG")
                .num_args(0)
                .action(ArgAction::Count)
                .hide(true),
        )
        .arg( // Print summary information
            Arg::new("print-summary")
                .short('s')
                .long("print-summary")
                .help("Print a summary of the number of files processed, errors, etc.")
                .num_args(0)
        )
        .arg( // Print summary information
            Arg::new("print-detail")
                .short('l')
                .long("print-detail")
                .help("Print more detail for each file processed.")
                .action(ArgAction::SetTrue)
        )
        .arg( // Don't print any information
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Don't produce any output except errors while working.")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .hide(true)
        )
        .get_matches()
}
