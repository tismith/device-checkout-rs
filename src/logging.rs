use stderrlog;
use types;

///This sets up logging, and takes the output from the commandline
///options
pub fn configure_logger(config: &types::Settings) {
    stderrlog::new()
        //.module(module_path!())
        .quiet(config.quiet)
        .verbosity(config.verbosity)
        .timestamp(config.timestamp)
        .init()
        .unwrap();
}
