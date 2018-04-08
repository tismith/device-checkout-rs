//reexport Timestamp, so other modules don't need to use stderrlog
pub use stderrlog::Timestamp as Timestamp;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{}
//TODO add custom or mapped error types here

#[derive(Debug)]
pub struct Settings {
    pub verbosity: usize,
    pub quiet: bool,
    pub timestamp: Timestamp,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            verbosity: 0,
            quiet: false,
            timestamp: Timestamp::Off,
        }
    }
}
