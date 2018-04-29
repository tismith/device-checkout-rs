//reexport Timestamp, so other modules don't need to use stderrlog
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
