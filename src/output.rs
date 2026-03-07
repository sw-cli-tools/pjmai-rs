//! Output formatting for human and JSON output modes

use crate::util;
use serde::Serialize;
use std::collections::HashMap;

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
    /// Project description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Project tags
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Primary programming language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Project group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Last used timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<String>,
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
    /// Subdirectory path within the project (if navigating to a subdir)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdir: Option<String>,
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

/// JSON output for the context command
#[derive(Debug, Serialize)]
pub struct ContextOutput {
    /// Project name
    pub name: String,
    /// Project path
    pub path: String,
    /// Whether this is a directory or file
    #[serde(rename = "type")]
    pub path_type: String,
    /// Project description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Project tags
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Primary programming language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Project group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Project notes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    /// Key files found in project
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub key_files: Vec<KeyFile>,
}

/// Key file information for context
#[derive(Debug, Serialize)]
pub struct KeyFile {
    /// File name
    pub name: String,
    /// Description of the file's purpose
    pub purpose: String,
}

/// JSON output for notes list
#[derive(Debug, Serialize)]
pub struct NotesOutput {
    /// Project name
    pub project: String,
    /// All notes
    pub notes: Vec<NoteEntry>,
}

/// A single note entry
#[derive(Debug, Serialize)]
pub struct NoteEntry {
    /// Note index (1-based)
    pub index: usize,
    /// Note text
    pub text: String,
}

/// JSON output for tags list
#[derive(Debug, Serialize)]
pub struct TagsOutput {
    /// Project name
    pub project: String,
    /// All tags
    pub tags: Vec<String>,
}

/// JSON output for metadata update
#[derive(Debug, Serialize)]
pub struct MetaOutput {
    /// Success indicator
    pub success: bool,
    /// Project name
    pub project: String,
    /// Updated fields
    pub updated: Vec<String>,
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

/// JSON output for config export command
#[derive(Debug, Serialize)]
pub struct ConfigExportOutput {
    /// Configuration version
    pub version: String,
    /// Current project name
    pub current_project: String,
    /// All projects
    pub projects: Vec<ProjectOutput>,
    /// Project stack
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stack: Vec<String>,
}

/// JSON output for config import command
#[derive(Debug, Serialize)]
pub struct ConfigImportOutput {
    /// Whether import was successful
    pub success: bool,
    /// Number of projects added
    pub added: usize,
    /// Number of projects skipped (already exist)
    pub skipped: usize,
    /// Number of projects updated (merge mode)
    pub updated: usize,
    /// Names of added projects
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub added_projects: Vec<String>,
    /// Names of skipped projects
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skipped_projects: Vec<String>,
    /// Names of updated projects
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub updated_projects: Vec<String>,
}

/// JSON output for env show command
#[derive(Debug, Serialize)]
pub struct EnvShowOutput {
    /// Project name
    pub project: String,
    /// Environment variables
    pub vars: HashMap<String, String>,
    /// Commands to run on project entry
    pub on_enter: Vec<String>,
    /// Commands to run when leaving the project
    pub on_exit: Vec<String>,
    /// Paths to prepend to PATH
    pub path_prepend: Vec<String>,
}

/// JSON output for env modification commands
#[derive(Debug, Serialize)]
pub struct EnvModifyOutput {
    /// Success indicator
    pub success: bool,
    /// Operation performed
    pub operation: String,
    /// Project name affected
    pub project: String,
}

/// JSON output for env auto-detect command
#[derive(Debug, Serialize)]
pub struct EnvAutoDetectOutput {
    /// Project name
    pub project: String,
    /// Whether changes were applied (false if dry_run)
    pub applied: bool,
    /// Detected features
    pub detected: Vec<DetectedFeature>,
}

/// A detected environment feature
#[derive(Debug, Serialize)]
pub struct DetectedFeature {
    /// Feature name (e.g., "python-venv", "node-nvm", "direnv")
    pub feature: String,
    /// File or directory that triggered detection
    pub source: String,
    /// Commands/paths that would be added
    pub config: DetectedConfig,
}

/// Configuration that would be applied for a detected feature
#[derive(Debug, Serialize)]
pub struct DetectedConfig {
    /// Paths to prepend to PATH
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub path_prepend: Vec<String>,
    /// Commands to run on enter
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub on_enter: Vec<String>,
    /// Commands to run on exit
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub on_exit: Vec<String>,
}

/// JSON output for group list command
#[derive(Debug, Serialize)]
pub struct GroupListOutput {
    /// All groups
    pub groups: Vec<GroupSummary>,
    /// Current group name (if any)
    pub current_group: Option<String>,
    /// Total number of groups
    pub total: usize,
}

/// Summary of a group
#[derive(Debug, Serialize)]
pub struct GroupSummary {
    /// Group name (parent directory name)
    pub name: String,
    /// Optional alias for the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Path to the group directory
    pub path: String,
    /// Number of projects in this group
    pub project_count: usize,
    /// Whether this is the current group
    pub is_current: bool,
    /// Project names (only included if all=true)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<String>,
}

/// JSON output for group show command
#[derive(Debug, Serialize)]
pub struct GroupShowOutput {
    /// Group name
    pub name: String,
    /// Optional alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Path to the group directory
    pub path: String,
    /// Number of projects
    pub project_count: usize,
    /// Project names (only included if all=true)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<String>,
}

/// JSON output for group prompt command
#[derive(Debug, Serialize)]
pub struct GroupPromptOutput {
    /// Group name (or alias if --alias flag used)
    pub name: String,
}

/// JSON output for group alias command
#[derive(Debug, Serialize)]
pub struct GroupAliasOutput {
    /// Success indicator
    pub success: bool,
    /// Operation performed
    pub operation: String,
    /// Group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Alias name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

/// JSON output for listing group aliases
#[derive(Debug, Serialize)]
pub struct GroupAliasListOutput {
    /// All group aliases (group_name -> alias)
    pub aliases: HashMap<String, String>,
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
