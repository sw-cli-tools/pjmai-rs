use crate::error::{PjmError, Result};
use crate::io;
use crate::projects;
use crate::{PjmConfig, ProjectPath, SerializedRegistry};
use log::info;
use std::sync::OnceLock;

/// Global config instance
static CONFIG: OnceLock<PjmConfig> = OnceLock::new();

/// Initialize the global config
pub fn init_config(config: PjmConfig) {
    CONFIG.set(config).expect("Config already initialized");
}

/// Get the global config
fn get_config() -> &'static PjmConfig {
    CONFIG.get().expect("Config not initialized")
}

/// Check for serialized registry
pub fn check(assume_yes: bool) -> Result<()> {
    info!("create or use registry file (or quit)");
    let config = get_config();
    let config_file = config.config_file_path();

    // Create config directory if it doesn't exist
    if !is_file_found(&config.config_dir) {
        info!("config directory not found, creating: {}", &config.config_dir);
        std::fs::create_dir_all(&config.config_dir)
            .map_err(|e| PjmError::ConfigDirCreation(config.config_dir.clone(), e))?;
    }

    if !is_file_found(&config_file) {
        info!("registry file not found");
        if assume_yes || prompt_create_yes_no() {
            info!("creating registry file");
            save_config_toml(&initial_config_toml()?)?;
        } else {
            info!("cannot continue without registry file");
            std::process::exit(0);
        }
    }
    info!("using registry file at {}", &config_file);
    Ok(())
}

/// Expand file path to be absolute
pub fn expand_file_path(file_path: &str) -> ProjectPath {
    info!("expand file path {}", &file_path);
    if file_path.starts_with('~') {
        let home = std::env::var("HOME").unwrap();
        let new_file_path = &file_path.to_string().replacen('~', &home, 1);
        new_file_path.to_string()
    } else {
        file_path.to_string()
    }
}

/// Determine if a path is a file or a directory
pub fn is_file_dir(file_path: &str) -> bool {
    info!("is file dir? {}", &file_path);
    std::fs::metadata(file_path).unwrap().is_dir()
}

/// Determine if the file or directory exists
pub fn is_file_found(file_path: &str) -> bool {
    info!("is file found? {}", &file_path);
    std::path::Path::new(&file_path).exists()
}

/// Reload a registry
pub fn projects() -> Result<projects::ProjectsRegistry> {
    info!("projects");
    projects::ProjectsRegistry::deser(projects_file_contents()?)
}

/// Save the projects registry
pub fn save_config_toml(projects_string: &str) -> Result<()> {
    info!("save config toml");
    let config_file = get_config().config_file_path();
    io::write(projects_string, &config_file).map_err(PjmError::ConfigWrite)
}

/// Shorten an absolute path to one relative to $HOME
pub fn shorten_path(long_path: &str) -> String {
    info!("shorten_path {}", &long_path);
    let home = std::env::var("HOME").unwrap_or_default();
    if !home.is_empty() && let Some(suffix) = long_path.strip_prefix(&home) {
        return format!("~{}", suffix);
    }
    long_path.to_string()
}

fn initial_config_toml() -> Result<SerializedRegistry> {
    info!("initial config toml");
    projects::ProjectsRegistry::new().ser()
}

fn projects_file_contents() -> Result<SerializedRegistry> {
    info!("projects file contents");
    let config_file = get_config().config_file_path();
    io::read(config_file).map_err(PjmError::ConfigRead)
}

fn prompt_create_yes_no() -> bool {
    info!("create yes/no");
    use std::io::{Write, stderr, stdin};

    // Print to stderr so prompt is visible even when stdout is captured
    eprint!("Create config file? [Y/n] ");
    stderr().flush().expect("flush stderr");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("read stdin");
    matches!(input.as_ref(), "Y\n" | "y\n" | "\n")
}
