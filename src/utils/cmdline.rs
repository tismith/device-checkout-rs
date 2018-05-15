use clap;
use utils::types;

pub fn parse_cmdline() -> types::Settings {
    let matches = matcher().get_matches();
    match parse(&matches) {
        Ok(s) => s,
        Err(e) => e.exit(),
    }
}

fn parse(matches: &clap::ArgMatches) -> Result<types::Settings, clap::Error> {
    let verbosity = matches.occurrences_of("verbosity") as usize;
    if verbosity > 4 {
        Err(clap::Error {
            message: "invalid number of 'v' flags".into(),
            kind: clap::ErrorKind::InvalidValue,
            info: None,
        })?
    }
    let quiet = matches.is_present("quiet");
    let timestamp = match matches.value_of("timestamp") {
        Some("ns") => types::Timestamp::Nanosecond,
        Some("ms") => types::Timestamp::Microsecond,
        Some("sec") => types::Timestamp::Second,
        Some("none") | None => types::Timestamp::Off,
        Some(_) => Err(clap::Error {
            message: "invalid value for 'timestamp'".into(),
            kind: clap::ErrorKind::InvalidValue,
            info: None,
        })?,
    };

    let port = value_t!(matches.value_of("port"), u16)?;
    let database = matches.value_of("database").ok_or_else(|| clap::Error {
        message: "invalid value for 'database'".into(),
        kind: clap::ErrorKind::InvalidValue,
        info: None,
    })?;

    Ok(types::Settings {
        verbosity,
        quiet,
        timestamp,
        port,
        database_url: database.to_string(),
        ..Default::default()
    })
}

fn matcher<'a, 'b>() -> clap::App<'a, 'b> {
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
        let m = matcher()
            .get_matches_from_safe(vec!["", "--port", "1234"])
            .unwrap();
        let s = parse(&m).unwrap();

        assert_eq!(s.port, 1234u16);
    }

    #[test]
    fn test_database() {
        let m = matcher()
            .get_matches_from_safe(vec!["", "--database", "somefile.txt"])
            .unwrap();
        let s = parse(&m).unwrap();

        assert_eq!(s.database_url, "somefile.txt".to_string());
    }

    #[test]
    fn test_too_much_verbosity() {
        let m = matcher().get_matches_from_safe(vec!["", "-vvvvv"]).unwrap();
        assert!(parse(&m).is_err());
    }

    #[test]
    fn test_just_enough_verbosity() {
        let m = matcher().get_matches_from_safe(vec!["", "-vvv"]).unwrap();
        let s = parse(&m).unwrap();

        assert_eq!(s.verbosity, 3);
    }

    #[test]
    fn test_timestamps() {
        let m = matcher()
            .get_matches_from_safe(vec!["", "-t", "sec"])
            .unwrap();
        let s = parse(&m).unwrap();

        match s.timestamp {
            types::Timestamp::Second => (),
            _ => panic!("unexpected parse"),
        }
    }

    #[test]
    fn test_bogus_timestamps() {
        assert!(
            matcher()
                .get_matches_from_safe(vec!["", "-t", "bogus"])
                .is_err()
        );
    }
}
