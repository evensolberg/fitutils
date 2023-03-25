use clap::parser::ValueSource;
use env_logger::Builder;
use log::LevelFilter;

#[must_use]
pub fn build_log(cli_args: &clap::ArgMatches) -> Builder {
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    if cli_args.value_source("quiet") != Some(ValueSource::CommandLine) {
        match cli_args.get_count("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    } else {
        logbuilder.filter_level(LevelFilter::Off);
    }

    // return it
    logbuilder
}
