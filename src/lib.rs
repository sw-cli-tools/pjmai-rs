//! library modules
#![deny(warnings, missing_docs)]

/// Argument Parsing
pub mod args;

/// Command Processing
pub mod command;

/// Configuration
pub mod config;

/// Error types
pub mod error;

/// I/O functions
pub mod io;

/// Output formatting (human and JSON)
pub mod output;

/// Projects Model
pub mod projects;

/// Utility functions
pub mod util;

/// Project Name String
type ProjectName = String;

/// Project Path String (directory or file)
type ProjectPath = String;

/// Serialized Registry as String
type SerializedRegistry = String;

/// Environment variable name for custom config directory
pub const PJMAI_CONFIG_DIR_ENV: &str = "PJMAI_CONFIG_DIR";

/// Configuration for PJM paths
#[derive(Debug, Clone)]
pub struct PjmConfig {
    /// The directory containing the config file (e.g., ~/.pjmai)
    pub config_dir: String,
}

impl PjmConfig {
    /// Create a new PjmConfig by checking the environment variable or using the default
    pub fn new() -> Self {
        let config_dir = std::env::var(PJMAI_CONFIG_DIR_ENV).unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME environment variable not set");
            format!("{}/.pjmai", home)
        });
        PjmConfig { config_dir }
    }

    /// Get the full path to the config.toml file
    pub fn config_file_path(&self) -> String {
        format!("{}/config.toml", self.config_dir)
    }
}

impl Default for PjmConfig {
    fn default() -> Self {
        Self::new()
    }
}
