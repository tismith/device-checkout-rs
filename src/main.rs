// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

//standard includes
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate clap;

mod cmdline;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
    //TODO add custom or mapped error types here
}
use errors::*;

fn main() {
    let config = cmdline::parse_cmdline();
    configure_logger(&config);

    if let Err(ref e) = run(&config) {
        use error_chain::ChainedError; // trait which holds `display_chain`
        error!("{}", e.display_chain());
        ::std::process::exit(1);
    }
}

fn configure_logger(config: &clap::ArgMatches) {
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

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run(_config: &clap::ArgMatches) -> Result<()> {
    trace!("Entry to top level run()");
    //DO STUFF

    Ok(())
}
