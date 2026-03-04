//! Error types for PJMAI

use std::fmt;
use std::io;

/// Result type alias for PJM operations
pub type Result<T> = std::result::Result<T, PjmError>;

/// Errors that can occur during PJM operations
#[derive(Debug)]
pub enum PjmError {
    /// Project with this name already exists
    DuplicateProject(String),
    /// Project not found in registry
    ProjectNotFound(String),
    /// File or directory does not exist
    PathNotFound(String),
    /// Failed to create config directory
    ConfigDirCreation(String, io::Error),
    /// Failed to read config file
    ConfigRead(io::Error),
    /// Failed to write config file
    ConfigWrite(io::Error),
    /// Failed to parse config file
    ConfigParse(String),
    /// Failed to serialize config
    ConfigSerialize(String),
    /// File or directory not found when changing to project
    TargetNotFound(String),
}

impl fmt::Display for PjmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PjmError::DuplicateProject(name) => {
                write!(f, "cannot add duplicate project name '{}'", name)
            }
            PjmError::ProjectNotFound(name) => {
                write!(f, "project '{}' not found", name)
            }
            PjmError::PathNotFound(path) => {
                write!(f, "file or directory '{}' does not exist", path)
            }
            PjmError::ConfigDirCreation(path, err) => {
                write!(f, "failed to create config directory '{}': {}", path, err)
            }
            PjmError::ConfigRead(err) => {
                write!(f, "failed to read config file: {}", err)
            }
            PjmError::ConfigWrite(err) => {
                write!(f, "failed to write config file: {}", err)
            }
            PjmError::ConfigParse(msg) => {
                write!(f, "failed to parse config file: {}", msg)
            }
            PjmError::ConfigSerialize(msg) => {
                write!(f, "failed to serialize config: {}", msg)
            }
            PjmError::TargetNotFound(path) => {
                write!(f, "project target '{}' not found", path)
            }
        }
    }
}

impl std::error::Error for PjmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PjmError::ConfigDirCreation(_, err) => Some(err),
            PjmError::ConfigRead(err) => Some(err),
            PjmError::ConfigWrite(err) => Some(err),
            _ => None,
        }
    }
}
