use clap;

pub fn parse_cmdline() -> clap::ArgMatches<'static> {
    clap::App::new(crate_name!())
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
        .get_matches()
}
