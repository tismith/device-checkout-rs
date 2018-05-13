//reexport Timestamp, so other modules don't need to use stderrlog
use failure;
use std;
pub use stderrlog::Timestamp;

#[derive(Debug)]
pub struct Settings {
    pub verbosity: usize,
    pub quiet: bool,
    pub timestamp: Timestamp,
    pub module_path: Option<String>,
    pub database_url: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            verbosity: 0,
            quiet: false,
            timestamp: Timestamp::Off,
            module_path: None,
            database_url: "devices.db".to_string(),
            port: 8000,
        }
    }
}

pub struct ExitFailure(failure::Error);

impl std::fmt::Debug for ExitFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use failure::Fail;
        let mut fail: &Fail = self.0.cause();
        write!(f, "{}", fail)?;
        while let Some(cause) = fail.cause() {
            write!(f, "\ncaused by: {}", cause)?;
            fail = cause;
        }
        Ok(())
    }
}

impl std::convert::From<failure::Error> for ExitFailure {
    fn from(error: failure::Error) -> Self {
        ExitFailure(error)
    }
}
