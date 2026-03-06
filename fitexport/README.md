# fitexport

Reads .fit, .gpx, and .tcx activity files and exports session/metadata to .json, detailed data to .csv, and optionally a summary .csv file. File format is detected automatically from the extension.

## Usage

```text
Usage: fitexport [OPTIONS] <FILE(S)>...

Arguments:
  <FILE(S)>...  One or more .fit, .gpx, or .tcx files to process. Wildcards and mixed formats are supported.

Options:
  -q, --quiet                              Don't produce any output except errors while working.
  -p, --print-summary                      Print summary detail for each activity processed.
  -o, --detail-off                         Don't export detailed information from each file parsed.
  -s, --summary-file <summary output file> Summary output file name.
  -h, --help                               Print help.
  -V, --version                            Print version.
```
