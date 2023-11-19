//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build() -> Command {
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
}

#[cfg(test)]
/// Tests for the CLI module
mod tests {
    use super::*;

    /// Test the CLI build function
    #[test]
    fn test_cli_build() {
        // Long form
        let args = build().get_matches_from(vec![
            "--read",
            "test.fit",
            "--debug",
            "--debug",
            "--print-summary",
            "--print-detail",
            "--quiet",
        ]);

        assert!(args.contains_id("read"));
        assert!(args.contains_id("debug"));
        assert!(args.contains_id("print-summary"));
        assert!(args.contains_id("print-detail"));
        assert!(args.contains_id("quiet"));
        assert_eq!(args.get_count("debug"), 2);

        // Short form
        let args2 =
            build().get_matches_from(vec!["--read", "test.fit", "-d", "-d", "-s", "-l", "-q"]);

        assert!(args2.contains_id("debug"));
        assert!(args2.contains_id("print-summary"));
        assert!(args2.contains_id("print-detail"));
        assert!(args2.contains_id("quiet"));
        assert_eq!(args2.get_count("debug"), 2);
    }
}
