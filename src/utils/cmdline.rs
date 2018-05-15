use clap;
use utils::types;

pub fn parse_cmdline() -> types::Settings {
    let matches = arg_matcher().get_matches();
    parse_matcher(&matches)
}

fn parse_matcher(matches: &clap::ArgMatches) -> types::Settings {
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
    let database = matches.value_of("database").unwrap_or_else(|| {
        clap::Error {
            message: "invalid value for 'database'".into(),
            kind: clap::ErrorKind::InvalidValue,
            info: None,
        }.exit()
    });

    types::Settings {
        verbosity,
        quiet,
        timestamp,
        port,
        database_url: database.to_string(),
        ..Default::default()
    }
}

fn arg_matcher<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new(crate_name!())
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
        .arg(
            clap::Arg::with_name("database")
                .short("d")
                .long("database")
                .help("path to database file")
                .default_value("devices.db")
                .takes_value(true),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port() {
        let m = arg_matcher()
            .get_matches_from_safe(vec!["", "--port", "1234"])
            .unwrap();
        let s = parse_matcher(&m);

        assert_eq!(s.port, 1234u16);
    }

    #[test]
    fn test_database() {
        let m = arg_matcher()
            .get_matches_from_safe(vec!["", "--database", "somefile.txt"])
            .unwrap();
        let s = parse_matcher(&m);

        assert_eq!(s.database_url, "somefile.txt".to_string());
    }
}
