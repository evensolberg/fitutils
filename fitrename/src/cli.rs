//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, ArgMatches, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build() -> ArgMatches {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        // .author(clap::crate_authors!("\n"))
        .long_about("This program will rename FIT, GPX and TCX files based on metadata about the activity in the file and a pattern provided for the file name.")
        .arg(
            Arg::new("read")
                .value_name("FILE(S)")
                .help("One or more .fit, .gpx or .tcx file(s) to process. Wildcards and multiple_occurrences files (e.g. 2019*.fit 2020*.gpx) are supported.")
                .num_args(1..)
                .required(true)
                .action(ArgAction::Append),
        )
        .arg( // Rename pattern}
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .help("The pattern for new file names.")
                .num_args(1)
                .action(ArgAction::Set)
                .required(true)
                .hide(false),
        )
        .arg( // Hidden debug parameter
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .env("FIT_DEBUG")
                .hide(true)
                .num_args(0)
                .action(ArgAction::Count)
        )
        .arg( // Don't print any information
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Don't produce any output except errors while working.")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
        .arg( // Print summary information
            Arg::new("print-summary")
                .short('s')
                .long("print-summary")
                .help("Print a summary of the number of files processed, errors, etc.")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
        .arg( // Dry-run
            Arg::new("dry-run")
                .short('r')
                .long("dry-run")
                .help("Perform a dry-run. This will output what the result will be without performing the actual rename operation.")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
        .get_matches()
}
