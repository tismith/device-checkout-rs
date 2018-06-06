//reexport Timestamp, so other modules don't need to use stderrlog
pub use stderrlog::Timestamp;

#[derive(Debug, Clone)]
pub struct Settings {
    pub verbosity: usize,
    pub quiet: bool,
    pub timestamp: Timestamp,
    pub module_path: Option<String>,
    pub template_dir: Option<String>,
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
            template_dir: None,
            database_url: "devices.db".to_string(),
            port: 8000,
        }
    }
}
