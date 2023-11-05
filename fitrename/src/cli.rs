//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build() -> Command {
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
        .arg( // FIle move pattern}
            Arg::new("move")
                .short('m')
                .long("move")
                .help("Move the file to the directory specified (patterns allowed).")
                .num_args(1)
                .action(ArgAction::Set)
                .required(false)
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
}

#[cfg(test)]
///
mod tests {
    use super::*;

    /// Test that the CLI is built correctly
    #[test]
    fn test_cli_build() {
        // Test using long form arguments/flags.
        let args = build().get_matches_from(vec![
            "--read",
            "test.fit",
            "--pattern",
            "{%year}-{%month}-{%day} {%24hour}{%minute}{%second}",
            "--move",
            "{%year}",
            "--debug",
            "--quiet",
            "--print-summary",
            "--dry-run",
        ]);

        assert!(args.contains_id("read"));
        assert!(args.contains_id("pattern"));
        assert!(args.contains_id("move"));
        assert!(args.contains_id("debug"));
        assert!(args.contains_id("quiet"));
        assert!(args.contains_id("print-summary"));
        assert!(args.contains_id("dry-run"));

        // Test using short form arguments/flags.
        let args2 = build().get_matches_from(vec![
            "--read",
            "test.fit",
            "-p",
            "{%yr}-{%mn}-{%dy} {%24}{%mt}{%sc}",
            "-m",
            "{%yr}",
            "-d",
            "-q",
            "-s",
            "-r",
        ]);

        assert!(args2.contains_id("read"));
        assert!(args2.contains_id("pattern"));
        assert!(args2.contains_id("move"));
        assert!(args2.contains_id("debug"));
        assert!(args2.contains_id("quiet"));
        assert!(args2.contains_id("print-summary"));
        assert!(args2.contains_id("dry-run"));
    }
}
