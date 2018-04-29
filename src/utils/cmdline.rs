use clap;
use utils::types;

pub fn parse_cmdline() -> types::Settings {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            clap::Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity, maximum 4"),
        )
        .arg(
            clap::Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Silence all output"),
        )
        .arg(
            clap::Arg::with_name("timestamp")
                .short("t")
                .long("timestamp")
                .help("prepend log lines with a timestamp")
                .takes_value(true)
                .possible_values(&["none", "sec", "ms", "ns"]),
        )
        .arg(
            clap::Arg::with_name("port")
                .short("p")
                .long("port")
                .help("tcp port number to listen on")
                .default_value("8000")
                .takes_value(true),
        )
        .get_matches();

    let verbosity = matches.occurrences_of("verbosity") as usize;
    if verbosity > 4 {
        clap::Error {
            message: "invalid number of 'v' flags".into(),
            kind: clap::ErrorKind::InvalidValue,
            info: None,
        }.exit()
    }
    let quiet = matches.is_present("quiet");
    let timestamp = match matches.value_of("timestamp") {
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

    let port = value_t!(matches.value_of("port"), u16).unwrap_or_else(|e| e.exit());

    types::Settings {
        verbosity,
        quiet,
        timestamp,
        port,
        ..Default::default()
    }
}
