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
                .help("One or more .fit, .gpx or .tcx file(s) to process. Glob patterns (e.g. *.fit, 2024*.gpx) are expanded by the application.")
                .num_args(1..)
                .required(false)
                .action(ArgAction::Append),
        )
        .arg( // Rename pattern}
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .help("The pattern for new file names.")
                .num_args(1)
                .action(ArgAction::Set)
                .required(false)
                .hide(false),
        )
        .arg( // FIle move pattern}
            Arg::new("move")
                .short('m')
                .long("move")
                .help("Move the file to the directory specified by this pattern. Use {%type} or {%ty} to organize by file format (e.g. --move \"{%type}\").")
                .num_args(1)
                .value_name("target_directory")
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
        .arg(
            Arg::new("type-case")
                .short('t')
                .long("type-case")
                .help(
                    "Case for the {%type} and {%ty} pattern variables. \
                     'upper'/'u'/'U' produces 'FIT', 'lower'/'l'/'L' (default) produces 'fit'.",
                )
                .value_name("CASE")
                .value_parser(["upper", "lower", "u", "U", "l", "L"])
                .default_value("lower")
                .num_args(1)
                .action(ArgAction::Set)
                .required(false),
        )
        .arg(
            Arg::new("print-codes")
                .short('c')
                .long("print-codes")
                .help("Print all variable codes available for use in patterns and exit.")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
}

#[cfg(test)]
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
            "--type-case",
            "upper",
        ]);

        assert!(args.contains_id("read"));
        assert!(args.contains_id("pattern"));
        assert!(args.contains_id("move"));
        assert!(args.contains_id("debug"));
        assert!(args.contains_id("quiet"));
        assert!(args.contains_id("print-summary"));
        assert!(args.contains_id("dry-run"));
        assert!(args.contains_id("type-case"));
        assert_eq!(
            args.get_one::<String>("type-case").map(String::as_str),
            Some("upper")
        );

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
        assert_eq!(
            args2.get_one::<String>("type-case").map(String::as_str),
            Some("lower")
        );
    }

    /// Test that `--type-case` accepts the short aliases `u`/`U`/`l`/`L`
    #[test]
    fn test_type_case_short_aliases() {
        for alias in &["u", "U"] {
            let args = build().get_matches_from(vec!["--read", "test.fit", "-p", "{%type}", "--type-case", alias]);
            assert_eq!(
                args.get_one::<String>("type-case").map(String::as_str),
                Some(*alias),
                "expected alias '{alias}' to be accepted",
            );
        }
        for alias in &["l", "L"] {
            let args = build().get_matches_from(vec!["--read", "test.fit", "-p", "{%type}", "--type-case", alias]);
            assert_eq!(
                args.get_one::<String>("type-case").map(String::as_str),
                Some(*alias),
                "expected alias '{alias}' to be accepted",
            );
        }
    }

    /// Test that `-t` is accepted as a short form of `--type-case`
    #[test]
    fn test_type_case_short_flag() {
        let args = build().get_matches_from(vec!["--read", "test.fit", "-p", "{%type}", "-t", "upper"]);
        assert_eq!(
            args.get_one::<String>("type-case").map(String::as_str),
            Some("upper"),
        );

        let args2 = build().get_matches_from(vec!["--read", "test.fit", "-p", "{%type}", "-t", "U"]);
        assert_eq!(
            args2.get_one::<String>("type-case").map(String::as_str),
            Some("U"),
        );
    }

    /// Test that `--print-codes` / `-c` works without requiring FILES or --pattern
    #[test]
    fn test_print_codes_flag() {
        use clap::parser::ValueSource;

        // First element is argv[0] (program name); the actual flag follows.
        let args = build().get_matches_from(vec!["fitrename", "--print-codes"]);
        assert_eq!(
            args.value_source("print-codes"),
            Some(ValueSource::CommandLine),
            "--print-codes should be recognised as a command-line argument",
        );

        let args2 = build().get_matches_from(vec!["fitrename", "-c"]);
        assert_eq!(
            args2.value_source("print-codes"),
            Some(ValueSource::CommandLine),
            "-c should be recognised as a command-line argument",
        );
    }

    /// Test that `--print-codes` also works alongside other args
    #[test]
    fn test_print_codes_with_pattern_args() {
        use clap::parser::ValueSource;

        let args = build().get_matches_from(vec!["--read", "test.fit", "-p", "{%type}", "--print-codes"]);
        assert_eq!(
            args.value_source("print-codes"),
            Some(ValueSource::CommandLine),
        );
    }
}
