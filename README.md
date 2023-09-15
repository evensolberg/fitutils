# fitutils

This is a project consisting of several sub-modules:

|Directory|Description|
|:--------|:----------|
**fit2csv**|Converts FIT files to CSV, exporting session, lap and records information in separate files
**fit2json**|Dumps a FIT file to JSON. This is mostly meant for debugging and digging out information for use in *fit2csv*
**gpx2csv**|Converts GPX files to CSV, exporting metadata, tracks and segments, routes, and waypoints into separate files.
**tcx2csv**|Converts TCX files to CSV, exporting activities summaries and laps into separate files.
**fitrename**|Renames FIT, GPX and TCX files based on metadata information in the files.
**fitshow**|Displays the (activity) metadata contents of FIT, GPX and TCX files.

More files may come in the future.

Help for each utility can be found by running it with the `-h` or `--help` flag, e.g., `fitview --help`.

See the [Kanban Boards](https://github.com/evensolberg/fit2csv/projects) for the overall roadmap and To Do lists.

NOTE: This repository uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for PRs.

## Installation

1. Make sure you have Rust installed: <https://www.rust-lang.org/learn/get-started>
2. In the main repository directory, run `cargo build --release`
3. Copy the binaries found in `target/release/` to a directory in your path, for example `/usr/local/bin`.

## Handy tools

To process the output from these utilities, the following tools may be of use:

- [XSV](https://github.com/BurntSushi/xsv) - slice and dice CSV files
- [jq](https://stedolan.github.io/jq/) - slice and dice JSON files
