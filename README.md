# fitutils

This is a project consisting of several sub-modules:

| Directory | Description |
|:----------|:------------|
| **fitexport** | Exports FIT, GPX, and TCX files to JSON metadata and CSV data files. Replaces the former `fit2csv`, `gpx2csv`, and `tcx2csv` binaries. |
| **fit2json** | Dumps a FIT file to raw JSON. Mostly meant for debugging and digging out information. |
| **fitrename** | Renames FIT, GPX and TCX files based on metadata information in the files. |
| **fitview** | Displays the (activity) metadata contents of FIT, GPX and TCX files. |

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

- [xan](https://github.com/medialab/xan) - The CSV magician
- [QSV](https://github.com/jqnatividad/qsv) - slice and dice CSV files
- [jq](https://stedolan.github.io/jq/) - slice and dice JSON files
