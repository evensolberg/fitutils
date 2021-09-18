# fit2json

The base code for this was shamelessly stolen from Matthew Stadelman's excellent [fitparse-rs](https://github.com/stadelmanma/fitparse-rs) crate.

This program will read one or more FIT files and output the corresponding JSON file containing a dump of the entire file.

- If the output location is not provided, the JSON file will be output alongside the input file.
- If a directory is provided, all FIT files will be written there using the same filename as the FIT file, but with a '.json' extension.
- If multiple FIT files are provided and the output path isn't a directory, the JSON array will store all records present in the order they were read.
- Using a "-" as the output file name will result in all content being printed to STDOUT.
