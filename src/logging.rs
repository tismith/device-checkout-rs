use clap;
use stderrlog;

///This sets up logging, and takes the output from the commandline
///options
pub fn configure_logger(config: &clap::ArgMatches) {
    let verbose = config.occurrences_of("verbosity") as usize;
    let quiet = config.is_present("quiet");
    let ts = match config.value_of("timestamp") {
        Some("ns") => stderrlog::Timestamp::Nanosecond,
        Some("ms") => stderrlog::Timestamp::Microsecond,
        Some("sec") => stderrlog::Timestamp::Second,
        Some("none") | None => stderrlog::Timestamp::Off,
        Some(_) => clap::Error {
            message: "invalid value for 'timestamp'".into(),
            kind: clap::ErrorKind::InvalidValue,
            info: None,
        }.exit(),
    };

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .timestamp(ts)
        .init()
        .unwrap();
}
