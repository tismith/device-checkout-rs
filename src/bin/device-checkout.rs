extern crate device_checkout_lib;
use device_checkout_lib::*;
use failure::ResultExt;

fn main() -> Result<(), exitfailure::ExitFailure> {
    let mut config = utils::cmdline::parse_cmdline();
    config.module_path = Some(module_path!().into());
    utils::logging::configure_logger(&config);
    database::run_migrations(&config).context("Failed to migrate database")?;
    routes::rocket(config).context("Failed to launch Rocket http engine")?.launch();
    Ok(())
}
