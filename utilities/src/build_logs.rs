use env_logger::Builder;
use log::LevelFilter;

pub fn build_log(cli_args: &clap::ArgMatches) -> Builder {
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    if cli_args.is_present("quiet") {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.occurrences_of("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }

    // return it
    logbuilder
}
