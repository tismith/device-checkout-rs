use clap;
use types;

pub fn parse_cmdline() -> types::Settings {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            clap::Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity"),
        )
        .arg(
            clap::Arg::with_name("quiet")
                .short("q")
                .help("Silence all output"),
        )
        .arg(
            clap::Arg::with_name("timestamp")
                .short("t")
                .help("prepend log lines with a timestamp")
                .takes_value(true)
                .possible_values(&["none", "sec", "ms", "ns"]),
        )
        .get_matches();

    let verbose = matches.occurrences_of("verbosity") as usize;
    let quiet = matches.is_present("quiet");
    let ts = match matches.value_of("timestamp") {
        Some("ns") => types::Timestamp::Nanosecond,
        Some("ms") => types::Timestamp::Microsecond,
        Some("sec") => types::Timestamp::Second,
        Some("none") | None => types::Timestamp::Off,
        Some(_) => clap::Error {
            message: "invalid value for 'timestamp'".into(),
            kind: clap::ErrorKind::InvalidValue,
            info: None,
        }.exit(),
    };

    types::Settings {
        verbosity: verbose,
        quiet: quiet,
        timestamp: ts,
        .. Default::default()
    }
}
