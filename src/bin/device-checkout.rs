extern crate device_checkout_lib;
use device_checkout_lib::*;

fn main() -> Result<(), exitfailure::ExitFailure> {
    let mut config = utils::cmdline::parse_cmdline();
    config.module_path = Some(module_path!().into());
    utils::logging::configure_logger(&config);
    database::run_migrations(&config)?;
    routes::rocket(config)?.launch();
    Ok(())
}
