// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

//standard includes
//#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate clap;

mod utils;

fn main() {
    let mut config = utils::cmdline::parse_cmdline();
    config.module_path = Some(module_path!().into());
    utils::logging::configure_logger(&config);

    if let Err(ref e) = run(&config) {
        use failure::Fail;
        let mut fail: &Fail = e.cause();
        error!("{}", fail);

        while let Some(cause) = fail.cause() {
            error!("caused by: {}", cause);
            fail = cause;
        }
        std::process::exit(1);
    }
}

fn run(_config: &utils::types::Settings) -> Result<(), failure::Error> {
    trace!("Entry to top level run()");
    //DO STUFF

//--------------------------------------------------
//     use failure::ResultExt;
//     std::fs::File::open("foo.txt").context("Failed to open foo.txt")?;
//-------------------------------------------------- 

    Ok(())
}
