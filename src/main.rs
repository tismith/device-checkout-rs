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
mod logging;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
    //TODO add custom or mapped error types here
}
use errors::*;

fn main() {
    let config = cmdline::parse_cmdline();
    logging::configure_logger(&config);

    if let Err(ref e) = run(&config) {
        use error_chain::ChainedError; // trait which holds `display_chain`
        error!("{}", e.display_chain());
        ::std::process::exit(1);
    }
}

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run(_config: &clap::ArgMatches) -> Result<()> {
    trace!("Entry to top level run()");
    //DO STUFF

    Ok(())
}
