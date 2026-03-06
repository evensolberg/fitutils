//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build() -> Command {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .long_about(
            "Reads .fit, .gpx, and .tcx activity files and exports session/metadata \
             to .json, detailed data to .csv, and optionally a summary .csv file. \
             File format is detected automatically from the extension.",
        )
        .arg(
            Arg::new("read")
                .value_name("FILE(S)")
                .help(
                    "One or more .fit, .gpx, or .tcx files to process. \
                     Wildcards and mixed formats are supported.",
                )
                .num_args(1..)
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Output debug information. Supply twice for trace-level logs.")
                .env("FIT_DEBUG")
                .num_args(0)
                .action(ArgAction::Count)
                .hide(true),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Don't produce any output except errors while working.")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("print-summary")
                .short('p')
                .long("print-summary")
                .help("Print summary detail for each activity processed.")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("detail-off")
                .short('o')
                .long("detail-off")
                .help("Don't export detailed information from each file parsed.")
                .requires("summary-file")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("summary-file")
                .short('s')
                .value_name("summary output file name")
                .long("summary-file")
                .help("Summary output file name.")
                .num_args(1)
                .required(false)
                .action(ArgAction::Set),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_build_long() {
        let args = build().get_matches_from(vec![
            "--read",
            "test.fit",
            "--debug",
            "--debug",
            "--quiet",
            "--print-summary",
            "--detail-off",
            "--summary-file",
            "summary.csv",
        ]);
        assert!(args.contains_id("read"));
        assert!(args.contains_id("debug"));
        assert!(args.contains_id("quiet"));
        assert!(args.contains_id("print-summary"));
        assert!(args.contains_id("detail-off"));
        assert!(args.contains_id("summary-file"));
        assert_eq!(args.get_count("debug"), 2);
    }

    #[test]
    fn test_cli_build_short() {
        let args = build().get_matches_from(vec![
            "--read",
            "test.gpx",
            "-d",
            "-d",
            "-q",
            "-p",
            "-o",
            "-s",
            "summary.csv",
        ]);
        assert!(args.contains_id("read"));
        assert!(args.contains_id("debug"));
        assert!(args.contains_id("quiet"));
        assert!(args.contains_id("print-summary"));
        assert!(args.contains_id("detail-off"));
        assert!(args.contains_id("summary-file"));
        assert_eq!(args.get_count("debug"), 2);
    }
}
