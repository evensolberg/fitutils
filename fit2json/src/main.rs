//! Read one or more FIT files and dump their contents as JSON

use std::{error::Error, fs::File, path::PathBuf};

use clap::Parser;

// Application-specific types
mod types;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

/// Command-line options
#[derive(Debug, Parser)]
#[command(name = "fit2json")]
struct Cli {
    /// FIT files to convert to JSON
    files: Vec<PathBuf>,

    /// Output location, if not provided, the JSON file will be output alongside the input file. If a
    /// directory is provided, all FIT files will be written there using the same filename as the FIT file,
    /// but with a '.json' extension. If multiple FIT files are provided and the output path isn't a
    /// directory, the JSON array will store all records present in the order they were read. Using
    /// a "-" as the output file name will result in all content being printed to STDOUT.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

/// Performs the actual work.
fn run() -> Result<(), Box<dyn Error>> {
    // Configure logging
    Builder::new()
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    let cli = Cli::parse();
    let output_loc = cli
        .output
        .map_or(types::OutputLocation::Inplace, types::OutputLocation::new);
    let collect_all = matches!(output_loc, types::OutputLocation::LocalFile(_));

    // Expand glob patterns in-app so quoted globs work consistently across
    // shells and on Windows.
    //
    // fit2json receives PathBufs from clap, which can include non-UTF-8 paths
    // on Unix.  Strategy:
    //   • Path exists on disk   → add as-is (no string conversion, non-UTF-8 safe)
    //   • Non-existent + UTF-8  → pass to expand_globs() as a glob pattern
    //   • Non-existent + non-UTF-8 → warn and skip (cannot be a valid glob)
    let mut files: Vec<PathBuf> = Vec::new();
    for p in &cli.files {
        if p.exists() {
            files.push(p.clone());
        } else if let Some(s) = p.to_str() {
            files.extend(
                utilities::expand_globs(&[s.to_owned()])
                    .into_iter()
                    .map(PathBuf::from),
            );
        } else {
            log::warn!("Skipping non-UTF-8 path that does not exist: {}", p.to_string_lossy());
        }
    }
    files.sort();
    files.dedup();

    // If no files have been provided, read from STDIN
    if cli.files.is_empty() {
        log::info!("No files supplied. Reading from STDIN.");
        // Force Stdout mode when reading from STDIN with Inplace output
        let effective_output = match &output_loc {
            types::OutputLocation::Inplace => types::OutputLocation::Stdout,
            other => other.clone(),
        };
        effective_output.write_json_file(
            &PathBuf::from("<stdin>"),
            fitparser::from_reader(&mut std::io::stdin())?,
        )?;
        return Ok(());
    }

    // If file args were supplied but none matched (or all are missing),
    // exit cleanly — expand_globs() already warned for each unmatched entry.
    if files.is_empty() {
        return Ok(());
    }

    // Read each FIT file and output it
    let mut all_fit_data: Vec<fitparser::FitDataRecord> = Vec::new();
    for file in files {
        // open file and parse data
        log::info!("Processing file: {}", &file.to_str().unwrap_or_default());
        let mut fp = File::open(&file)?;
        let mut data = fitparser::from_reader(&mut fp)?;

        // output a single fit file's data into a single output file
        if collect_all {
            all_fit_data.append(&mut data);
        } else {
            output_loc.write_json_file(&file, data)?;
        }
    }
    // output fit data from all files into a single file
    if collect_all {
        log::info!("Summary information collected in specified output location.");
        output_loc.write_json_file(&PathBuf::new(), all_fit_data)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn expand_globs_empty_on_no_match() {
        // Use an isolated empty temp directory so the glob is guaranteed to
        // match nothing, regardless of any stale files in the OS temp dir.
        let tmp = std::env::temp_dir()
            .join(format!("fitutils_f2j_nomatch_{}", std::process::id()));
        if tmp.exists() {
            fs::remove_dir_all(&tmp).expect("remove stale temp dir");
        }
        fs::create_dir_all(&tmp).expect("create temp dir");
        let pattern = tmp.join("*.fit").to_string_lossy().into_owned();
        let result = utilities::expand_globs(&[pattern]);
        assert!(result.is_empty());
        fs::remove_dir_all(tmp).expect("clean up temp dir");
    }
}

/// Main executable entry point. Hands off to the `run` function.
fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Error: {}", err.to_string().replace('\"', ""));
            1
        }
    });
}
