//! library modules
#![deny(warnings, missing_docs)]

/// Argument Parsing
pub mod args;

/// Command Processing
pub mod command;

/// Confuration
pub mod config;

/// I/O functions
pub mod io;

/// Projects Model
pub mod projects;

/// Utiltity functions
pub mod util;

/// Project Name String
type ProjectName = String;

/// Project Path String (directory or file)
type ProjectPath = String;

/// Serialized Registry as String
type SerializedRegistry = String;
