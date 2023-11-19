//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build() -> Command {
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
}

#[cfg(test)]
/// Tests for the CLI module
mod tests {
    use super::*;

    /// Test the CLI build function
    #[test]
    fn test_cli_build() {
        let args = build().get_matches_from(vec![
            "--read",
            "test.gpx",
            "--debug",
            "--debug",
            "--quiet",
            "--detail-off",
            "--summary-file",
            "test.csv",
        ]);

        assert!(args.contains_id("read"));
        assert!(args.contains_id("debug"));
        assert!(args.contains_id("debug"));
        assert!(args.contains_id("quiet"));
        assert!(args.contains_id("detail-off"));
        assert!(args.contains_id("summary-file"));
        assert_eq!(args.get_count("debug"), 2);

        let args2 = build().get_matches_from(vec![
            "--read", "test.gpx", "-d", "-d", "-q", "-o", "-s", "test.csv",
        ]);

        assert!(args2.contains_id("read"));
        assert!(args2.contains_id("debug"));
        assert!(args2.contains_id("quiet"));
        assert!(args2.contains_id("detail-off"));
        assert!(args2.contains_id("summary-file"));
        assert_eq!(args2.get_count("debug"), 2);
    }
}
