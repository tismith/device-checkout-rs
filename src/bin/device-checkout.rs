extern crate device_checkout_lib;
#[macro_use]
extern crate sentry;

use device_checkout_lib::*;
use failure::ResultExt;

fn main() -> Result<(), exitfailure::ExitFailure> {
    sentry::init((
        "https://145efbb2a99d408c9394596c5b25b14f@sentry.io/1240440",
        sentry::ClientOptions {
            release: sentry_crate_release!(),
            ..Default::default()
        },
    ));
    sentry::integrations::panic::register_panic_handler();
    let mut config = utils::cmdline::parse_cmdline();
    config.module_path = Some(module_path!().into());
    utils::logging::configure_logger(&config);
    database::run_migrations(&config).context("Failed to migrate database")?;
    routes::rocket(config)
        .context("Failed to launch Rocket http engine")?
        .launch();
    Ok(())
}
