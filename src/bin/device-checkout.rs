extern crate device_checkout_lib;
use device_checkout_lib::*;

#[macro_use]
extern crate log;

fn run(config: utils::types::Settings) -> Result<(), failure::Error> {
    trace!("run()");
    database::run_migrations(&config)?;
    routes::rocket(config)?.launch();
    Ok(())
}

fn main() {
    let mut config = utils::cmdline::parse_cmdline();
    config.module_path = Some(module_path!().into());
    utils::logging::configure_logger(&config);

    if let Err(ref e) = run(config) {
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
