use sentry;
use stderrlog;
use utils::types;

///This sets up logging, and takes the output from the commandline
///options
pub fn configure_logger(config: &types::Settings) {
    let mut logger = stderrlog::new();

    logger
        .quiet(config.quiet)
        .verbosity(config.verbosity)
        .timestamp(config.timestamp);

    if config.reporting {
        sentry::init((
            "https://145efbb2a99d408c9394596c5b25b14f@sentry.io/1240440",
            sentry::ClientOptions {
                release: sentry_crate_release!(),
                ..Default::default()
            },
        ));

        sentry::integrations::panic::register_panic_handler();

        let options = sentry::integrations::log::LoggerOptions {
            ..Default::default()
        };

        sentry::integrations::log::init(Some(Box::new(logger)), options);
    } else {
        logger.init().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_configure_logger() {
        //test that we don't panic creating a default logger
        configure_logger(&types::Settings::new());
    }
}
