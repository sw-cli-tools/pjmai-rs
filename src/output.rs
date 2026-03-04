//! Output formatting for human and JSON output modes

use crate::util;
use serde::Serialize;

/// A project in JSON output format
#[derive(Debug, Serialize)]
pub struct ProjectOutput {
    /// Project name
    pub name: String,
    /// Project path (file or directory)
    pub path: String,
    /// Whether this is a directory or file
    #[serde(rename = "type")]
    pub path_type: String,
    /// Whether this is the current project
    pub is_current: bool,
}

/// JSON output for the list command
#[derive(Debug, Serialize)]
pub struct ListOutput {
    /// All projects
    pub projects: Vec<ProjectOutput>,
    /// Current project name (empty string if none)
    pub current_project: String,
    /// Total number of projects
    pub total: usize,
}

/// JSON output for the show command
#[derive(Debug, Serialize)]
pub struct ShowOutput {
    /// Project name
    pub name: String,
    /// Project path
    pub path: String,
    /// Whether this is a directory or file
    #[serde(rename = "type")]
    pub path_type: String,
    /// Stack of projects (for pop navigation)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stack: Vec<String>,
}

/// JSON output for push/pop operations
#[derive(Debug, Serialize)]
pub struct PushPopOutput {
    /// Project name switched to
    pub name: String,
    /// Project path
    pub path: String,
    /// Whether this is a directory or file
    #[serde(rename = "type")]
    pub path_type: String,
    /// Action for shell (cd or source)
    pub action: String,
    /// Current stack depth after operation
    pub stack_depth: usize,
}

/// JSON output for the prompt command
#[derive(Debug, Serialize)]
pub struct PromptOutput {
    /// Current project name (empty string if none)
    pub current_project: String,
}

/// JSON output for successful add/remove operations
#[derive(Debug, Serialize)]
pub struct SuccessOutput {
    /// Success indicator
    pub success: bool,
    /// Operation performed
    pub operation: String,
    /// Project name affected
    pub project: String,
}

/// JSON output for the change command
#[derive(Debug, Serialize)]
pub struct ChangeOutput {
    /// Project name
    pub name: String,
    /// Project path
    pub path: String,
    /// Whether this is a directory or file
    #[serde(rename = "type")]
    pub path_type: String,
    /// Action for shell (cd or source)
    pub action: String,
}

/// JSON output for errors
#[derive(Debug, Serialize)]
pub struct ErrorOutput {
    /// Error code for programmatic handling
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Similar projects (for not found errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similar_projects: Option<Vec<String>>,
    /// Hint for fixing the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// JSON output for aliases command
#[derive(Debug, Serialize)]
pub struct AliasOutput {
    /// Alias name
    pub alias: String,
    /// Command it maps to
    pub command: String,
    /// Description
    pub description: String,
}

/// JSON output for the aliases command
#[derive(Debug, Serialize)]
pub struct AliasesOutput {
    /// All aliases
    pub aliases: Vec<AliasOutput>,
}

/// JSON output for the setup command
#[derive(Debug, Serialize)]
pub struct SetupOutput {
    /// Whether setup was successful
    pub success: bool,
    /// Shell that was configured
    pub shell: String,
    /// Actions performed
    pub actions: Vec<SetupAction>,
    /// Path to shell rc file modified (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rc_file: Option<String>,
    /// Path to completions file installed (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions_file: Option<String>,
}

/// Individual setup action
#[derive(Debug, Serialize)]
pub struct SetupAction {
    /// Action name
    pub action: String,
    /// Whether it succeeded
    pub success: bool,
    /// Details or error message
    pub message: String,
}

/// Determine if a path points to a directory or file
pub fn path_type(path: &str) -> String {
    let expanded = util::expand_file_path(path);
    if util::is_file_found(&expanded) {
        if util::is_file_dir(&expanded) {
            "directory".to_string()
        } else {
            "file".to_string()
        }
    } else {
        "unknown".to_string()
    }
}

/// Print JSON to stdout
pub fn print_json<T: Serialize>(value: &T) {
    println!("{}", serde_json::to_string_pretty(value).expect("JSON serialization failed"));
}
