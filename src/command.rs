use crate::error::{PjmError, Result};
use crate::output::{
    self, AliasOutput, AliasesOutput, ChangeOutput, ErrorOutput, ListOutput, ProjectOutput,
    PromptOutput, SetupAction, SetupOutput, ShowOutput, SuccessOutput,
};
use crate::projects;
use crate::util;
use clap::CommandFactory;
use clap_complete::Shell;
use colored::Colorize;
use log::info;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Add a project
pub fn add(project_name: &str, file_name: &str, json: bool) -> Result<()> {
    info!("adding {} -f {}", &project_name, &file_name);
    let mut registry = util::projects()?;

    // Check for duplicate project name (using already loaded projects)
    if is_dup(project_name, &registry) {
        info!("adding dup failed");
        return Err(PjmError::DuplicateProject(project_name.to_string()));
    }

    // Validate that the file or directory exists
    let expanded_path = util::expand_file_path(file_name);
    if !util::is_file_found(&expanded_path) {
        return Err(PjmError::PathNotFound(expanded_path));
    }

    info!("push");
    registry.project.push(projects::ChangeToProject {
        action: projects::Action {
            file_or_dir: file_name.to_string(),
        },
        name: project_name.to_string(),
    });
    if registry.project.len() == 1 {
        info!("switch to only project");
        registry.current_project = project_name.to_string();
    }
    util::save_config_toml(&registry.ser()?)?;

    if json {
        output::print_json(&SuccessOutput {
            success: true,
            operation: "add".to_string(),
            project: project_name.to_string(),
        });
    }

    info!("add done");
    Ok(())
}

/// List aliases
pub fn aliases(json: bool) {
    info!("aliases");

    if json {
        let aliases = vec![
            AliasOutput {
                alias: "adpj".to_string(),
                command: "pjmai add".to_string(),
                description: "Add a new project".to_string(),
            },
            AliasOutput {
                alias: "chpj".to_string(),
                command: "pjmai change".to_string(),
                description: "Change to a project".to_string(),
            },
            AliasOutput {
                alias: "hlpj".to_string(),
                command: "pjmai aliases".to_string(),
                description: "Show all aliases".to_string(),
            },
            AliasOutput {
                alias: "lspj".to_string(),
                command: "pjmai list".to_string(),
                description: "List all projects".to_string(),
            },
            AliasOutput {
                alias: "prpj".to_string(),
                command: "pjmai prompt".to_string(),
                description: "Get current project name for prompt".to_string(),
            },
            AliasOutput {
                alias: "rmpj".to_string(),
                command: "pjmai remove".to_string(),
                description: "Remove a project".to_string(),
            },
            AliasOutput {
                alias: "shpj".to_string(),
                command: "pjmai show".to_string(),
                description: "Show current project".to_string(),
            },
        ];
        output::print_json(&AliasesOutput { aliases });
    } else {
        println!("adpj <name> -f <dir|file> # alias for pjmai add");
        println!("chpj <name>               # alias for pjmai change");
        println!("hlpj                      # alias for pjmai aliases");
        println!("lspj                      # alias for pjmai list");
        println!("prpj                      # alias for pjmai prompt");
        println!("rmpj <name>               # alias for pjmai remove");
        println!("shpj                      # alias for pjmai show");
    }

    info!("aliases done");
}

/// Changes to the specified project
pub fn change(project_name: &str, json: bool) -> Result<()> {
    info!("changing to project {}", &project_name);
    let mut registry = util::projects()?;

    // Find matching project(s) using fuzzy matching
    let matched = find_matching_project(project_name, &registry);

    match matched {
        MatchResult::Exact(project) | MatchResult::Unique(project) => {
            // Clone needed values before modifying registry
            let proj_name = project.name.clone();
            let file_path = util::expand_file_path(&project.action.file_or_dir);

            if util::is_file_found(&file_path) {
                registry.current_project = proj_name.clone();
                util::save_config_toml(&registry.ser()?)?;

                let is_dir = util::is_file_dir(&file_path);
                let action = if is_dir { "cd" } else { "source" };
                let path_type = if is_dir { "directory" } else { "file" };

                if json {
                    output::print_json(&ChangeOutput {
                        name: proj_name,
                        path: file_path.clone(),
                        path_type: path_type.to_string(),
                        action: action.to_string(),
                    });
                } else {
                    print!("{}", &file_path); // path parameter for bash cd or source command
                }

                if is_dir {
                    info!("change done 2");
                    std::process::exit(2); // bash wrapper will cd to the above printed path
                } else {
                    info!("change done 3");
                    std::process::exit(3); // bash wrapper will source the above printed path
                }
            } else {
                if json {
                    output::print_json(&ErrorOutput {
                        code: "TARGET_NOT_FOUND".to_string(),
                        message: format!("Project target '{}' not found", &file_path),
                        similar_projects: None,
                        hint: Some("The project exists but its target path is missing".to_string()),
                    });
                } else {
                    println!("dir or file '{}' not found", &file_path);
                }
                info!("change done 4a");
                std::process::exit(4); // bash wrapper will echo the error
            }
        }
        MatchResult::Ambiguous(matches) => {
            if json {
                output::print_json(&ErrorOutput {
                    code: "AMBIGUOUS_PROJECT".to_string(),
                    message: format!("Ambiguous project name '{}'", project_name),
                    similar_projects: Some(matches.clone()),
                    hint: Some("Specify a more unique project name".to_string()),
                });
            } else {
                println!(
                    "ambiguous project name '{}', matches: {}",
                    project_name,
                    matches.join(", ")
                );
            }
            info!("change done 4c");
            std::process::exit(4);
        }
        MatchResult::None => {
            // Collect similar project names for suggestions
            let similar: Vec<String> = registry
                .project
                .iter()
                .map(|p| p.name.clone())
                .collect();

            if json {
                output::print_json(&ErrorOutput {
                    code: "PROJECT_NOT_FOUND".to_string(),
                    message: format!("Project '{}' not found", project_name),
                    similar_projects: if similar.is_empty() {
                        None
                    } else {
                        Some(similar)
                    },
                    hint: Some("Use 'pjmai list' to see all projects".to_string()),
                });
            } else {
                println!("project '{}' not found", &project_name);
            }
            info!("change done 4b");
            std::process::exit(4);
        }
    }
}

/// Result of fuzzy project matching
enum MatchResult<'a> {
    /// Exact match found
    Exact(&'a projects::ChangeToProject),
    /// Unique partial/fuzzy match found
    Unique(&'a projects::ChangeToProject),
    /// Multiple matches found (ambiguous)
    Ambiguous(Vec<String>),
    /// No match found
    None,
}

/// Find a project by name using fuzzy matching
/// Priority: exact match > prefix match > substring match
fn find_matching_project<'a>(
    query: &str,
    registry: &'a projects::ProjectsRegistry,
) -> MatchResult<'a> {
    let query_lower = query.to_lowercase();

    // First, try exact match
    for project in &registry.project {
        if project.name == query {
            return MatchResult::Exact(project);
        }
    }

    // Try case-insensitive exact match
    for project in &registry.project {
        if project.name.to_lowercase() == query_lower {
            return MatchResult::Exact(project);
        }
    }

    // Try prefix match
    let prefix_matches: Vec<_> = registry
        .project
        .iter()
        .filter(|p| p.name.to_lowercase().starts_with(&query_lower))
        .collect();

    if prefix_matches.len() == 1 {
        return MatchResult::Unique(prefix_matches[0]);
    }
    if prefix_matches.len() > 1 {
        return MatchResult::Ambiguous(prefix_matches.iter().map(|p| p.name.clone()).collect());
    }

    // Try substring match
    let substring_matches: Vec<_> = registry
        .project
        .iter()
        .filter(|p| p.name.to_lowercase().contains(&query_lower))
        .collect();

    if substring_matches.len() == 1 {
        return MatchResult::Unique(substring_matches[0]);
    }
    if substring_matches.len() > 1 {
        return MatchResult::Ambiguous(substring_matches.iter().map(|p| p.name.clone()).collect());
    }

    MatchResult::None
}

/// Lists known projects
pub fn list(json: bool) -> Result<()> {
    info!("listing all projects");
    let mut registry = util::projects()?;
    registry.project.sort_by(|a, b| a.name.cmp(&b.name));

    if json {
        let projects: Vec<ProjectOutput> = registry
            .project
            .iter()
            .map(|p| {
                let expanded = util::expand_file_path(&p.action.file_or_dir);
                ProjectOutput {
                    name: p.name.clone(),
                    path: expanded.clone(),
                    path_type: output::path_type(&p.action.file_or_dir),
                    is_current: p.name == registry.current_project,
                }
            })
            .collect();
        let total = projects.len();
        output::print_json(&ListOutput {
            projects,
            current_project: registry.current_project.clone(),
            total,
        });
    } else {
        for project in &registry.project {
            let short_path = util::shorten_path(&project.action.file_or_dir);
            let colored_name = if project.name == registry.current_project {
                project.name.italic().green()
            } else {
                project.name.normal()
            };
            let colored_short_path = if project.name == registry.current_project {
                short_path.italic().green()
            } else {
                short_path.normal()
            };
            let current = if project.name == registry.current_project {
                ">".to_string().italic().green()
            } else {
                " ".to_string().normal()
            };
            println!("{}{:8} {}", current, colored_name, colored_short_path);
        }
    }

    info!("listing done");
    Ok(())
}

/// Return the prompt indicator for the current project
pub fn prompt(json: bool) -> Result<()> {
    info!("prompt");
    let registry = util::projects()?;

    if json {
        output::print_json(&PromptOutput {
            current_project: registry.current_project.clone(),
        });
    } else if !registry.current_project.is_empty() {
        println!("{}", registry.current_project);
    }

    info!("prompt done");
    Ok(())
}

/// Remove the specified project
pub fn remove(unwanted_project_name: &str, json: bool) -> Result<()> {
    info!("remove {}", &unwanted_project_name);
    let old_registry = util::projects()?;
    let mut new_registry = projects::ProjectsRegistry::new();
    let mut found = false;

    for project in &old_registry.project {
        if project.name != unwanted_project_name {
            info!("keeping {}", &project.name);
            new_registry.project.push(projects::ChangeToProject {
                action: projects::Action {
                    file_or_dir: project.action.file_or_dir.to_string(),
                },
                name: project.name.to_string(),
            });
        } else {
            info!("discarding {}", &project.name);
            found = true;
        }
    }

    if found {
        // Preserve current_project unless we just removed it
        if old_registry.current_project != unwanted_project_name {
            new_registry.current_project = old_registry.current_project.clone();
        }
        info!("saving changes");
        util::save_config_toml(&new_registry.ser()?)?;

        if json {
            output::print_json(&SuccessOutput {
                success: true,
                operation: "remove".to_string(),
                project: unwanted_project_name.to_string(),
            });
        }
    } else if json {
        output::print_json(&ErrorOutput {
            code: "PROJECT_NOT_FOUND".to_string(),
            message: format!("Project '{}' not found", unwanted_project_name),
            similar_projects: None,
            hint: Some("Use 'pjmai list' to see all projects".to_string()),
        });
    }

    info!("remove done");
    Ok(())
}

/// Show the name and path for the current project
pub fn show(json: bool) -> Result<()> {
    info!("showing current project");
    let registry = util::projects()?;

    for project in &registry.project {
        if project.name == registry.current_project {
            info!("current {}", project.name);

            if json {
                let expanded = util::expand_file_path(&project.action.file_or_dir);
                output::print_json(&ShowOutput {
                    name: project.name.clone(),
                    path: expanded.clone(),
                    path_type: output::path_type(&project.action.file_or_dir),
                });
            } else {
                println!(
                    "{}",
                    format!(
                        ">{:8} {}",
                        project.name,
                        util::shorten_path(&project.action.file_or_dir)
                    )
                    .green()
                );
            }
            info!("show done");
            return Ok(());
        }
    }

    // No current project
    if json {
        output::print_json(&ErrorOutput {
            code: "NO_CURRENT_PROJECT".to_string(),
            message: "No current project set".to_string(),
            similar_projects: None,
            hint: Some("Use 'pjmai change <project>' to set a current project".to_string()),
        });
    }

    info!("show done");
    Ok(())
}

fn is_dup(check_project_name: &str, registry: &projects::ProjectsRegistry) -> bool {
    info!("is_dup {}", &check_project_name);
    for project in &registry.project {
        if project.name == check_project_name {
            return true;
        }
    }
    false
}

/// Complete project or command names for shell tab completion
pub fn complete(target: &crate::args::CompleteTarget) -> Result<()> {
    use crate::args::CompleteTarget;

    match target {
        CompleteTarget::Projects { prefix } => {
            complete_projects(prefix.as_deref());
        }
        CompleteTarget::Commands { prefix } => {
            complete_commands(prefix.as_deref());
        }
    }

    Ok(())
}

/// Output project names matching a prefix (one per line)
fn complete_projects(prefix: Option<&str>) {
    info!("complete projects prefix={:?}", prefix);

    // Load projects, silently fail if can't load
    let Ok(registry) = util::projects() else {
        return;
    };

    let prefix_lower = prefix.map(|s| s.to_lowercase());

    for project in &registry.project {
        let matches = match &prefix_lower {
            Some(p) => project.name.to_lowercase().starts_with(p),
            None => true,
        };
        if matches {
            println!("{}", project.name);
        }
    }
}

/// Output command names matching a prefix (one per line)
fn complete_commands(prefix: Option<&str>) {
    info!("complete commands prefix={:?}", prefix);

    // List of available commands
    let commands = [
        "add",
        "aliases",
        "change",
        "complete",
        "completions",
        "list",
        "prompt",
        "remove",
        "setup",
        "show",
    ];

    let prefix_lower = prefix.map(|s| s.to_lowercase());

    for cmd in commands {
        let matches = match &prefix_lower {
            Some(p) => cmd.starts_with(p),
            None => true,
        };
        if matches {
            println!("{}", cmd);
        }
    }
}

/// Setup shell integration
pub fn setup(
    shell: Option<Shell>,
    shell_only: bool,
    completions_only: bool,
    json: bool,
) -> Result<()> {
    info!("setup");

    // Detect shell if not specified
    let detected_shell = shell.unwrap_or_else(detect_shell);
    let shell_name = shell_to_string(detected_shell);

    let mut actions: Vec<SetupAction> = Vec::new();
    let mut rc_file_path: Option<String> = None;
    let mut completions_file_path: Option<String> = None;

    let do_shell = !completions_only;
    let do_completions = !shell_only;

    // Install shell integration
    if do_shell {
        match install_shell_integration(detected_shell) {
            Ok((path, already_installed)) => {
                rc_file_path = Some(path.clone());
                let message = if already_installed {
                    format!("Shell integration already present in {}", path)
                } else {
                    format!("Added shell integration to {}", path)
                };
                actions.push(SetupAction {
                    action: "shell_integration".to_string(),
                    success: true,
                    message: message.clone(),
                });
                if !json {
                    println!("{} {}", "✓".green(), message);
                }
            }
            Err(e) => {
                let message = format!("Failed to install shell integration: {}", e);
                actions.push(SetupAction {
                    action: "shell_integration".to_string(),
                    success: false,
                    message: message.clone(),
                });
                if !json {
                    println!("{} {}", "✗".red(), message);
                }
            }
        }
    }

    // Install completions
    if do_completions {
        match install_completions(detected_shell) {
            Ok(path) => {
                completions_file_path = Some(path.clone());
                let message = format!("Installed {} completions to {}", shell_name, path);
                actions.push(SetupAction {
                    action: "completions".to_string(),
                    success: true,
                    message: message.clone(),
                });
                if !json {
                    println!("{} {}", "✓".green(), message);
                }
            }
            Err(e) => {
                let message = format!("Failed to install completions: {}", e);
                actions.push(SetupAction {
                    action: "completions".to_string(),
                    success: false,
                    message: message.clone(),
                });
                if !json {
                    println!("{} {}", "✗".red(), message);
                }
            }
        }
    }

    let all_success = actions.iter().all(|a| a.success);

    if json {
        output::print_json(&SetupOutput {
            success: all_success,
            shell: shell_name.clone(),
            actions,
            rc_file: rc_file_path,
            completions_file: completions_file_path,
        });
    } else if all_success {
        println!();
        println!(
            "{}",
            "Setup complete! Restart your shell or run:".green().bold()
        );
        if let Some(ref rc) = rc_file_path {
            println!("  source {}", rc);
        }
    }

    info!("setup done");
    Ok(())
}

/// Detect the current shell from environment
fn detect_shell() -> Shell {
    if let Ok(shell_path) = std::env::var("SHELL") {
        if shell_path.contains("zsh") {
            return Shell::Zsh;
        } else if shell_path.contains("fish") {
            return Shell::Fish;
        } else if shell_path.contains("elvish") {
            return Shell::Elvish;
        } else if shell_path.contains("pwsh") || shell_path.contains("powershell") {
            return Shell::PowerShell;
        }
    }
    // Default to bash
    Shell::Bash
}

/// Convert Shell enum to string
fn shell_to_string(shell: Shell) -> String {
    match shell {
        Shell::Bash => "bash".to_string(),
        Shell::Zsh => "zsh".to_string(),
        Shell::Fish => "fish".to_string(),
        Shell::Elvish => "elvish".to_string(),
        Shell::PowerShell => "powershell".to_string(),
        _ => "unknown".to_string(),
    }
}

/// Get the RC file path for a shell
fn get_rc_file(shell: Shell) -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    match shell {
        Shell::Bash => PathBuf::from(format!("{}/.bashrc", home)),
        Shell::Zsh => PathBuf::from(format!("{}/.zshrc", home)),
        Shell::Fish => PathBuf::from(format!("{}/.config/fish/config.fish", home)),
        _ => PathBuf::from(format!("{}/.bashrc", home)),
    }
}

/// Find the source-pjm.sh script
fn find_source_script() -> Option<PathBuf> {
    // Try relative to the current executable
    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
    {
        // Check in same directory as executable
        let script = exe_dir.join("source-pjm.sh");
        if script.exists() {
            return Some(script);
        }

        // Check in parent directory (for development: target/debug/../source-pjm.sh)
        if let Some(parent) = exe_dir.parent() {
            let script = parent.join("source-pjm.sh");
            if script.exists() {
                return Some(script);
            }
            // Also check parent's parent (target/../source-pjm.sh)
            if let Some(grandparent) = parent.parent() {
                let script = grandparent.join("source-pjm.sh");
                if script.exists() {
                    return Some(script);
                }
            }
        }
    }

    // Try common installation paths
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let common_paths = [
        format!("{}/.local/share/pjmai/source-pjm.sh", home),
        format!("{}/.pjmai/source-pjm.sh", home),
        "/usr/local/share/pjmai/source-pjm.sh".to_string(),
    ];

    for path in &common_paths {
        let p = PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
    }

    None
}

/// Install shell integration to RC file
/// Returns (rc_file_path, already_installed)
fn install_shell_integration(shell: Shell) -> std::result::Result<(String, bool), String> {
    let rc_file = get_rc_file(shell);
    let rc_path_str = rc_file.to_string_lossy().to_string();

    // Find the source script
    let source_script = find_source_script().ok_or_else(|| {
        "Could not find source-pjm.sh. Please ensure it's in the same directory as pjmai or in ~/.local/share/pjmai/".to_string()
    })?;

    let source_line = format!("source {}", source_script.to_string_lossy());
    let marker = "# PJMAI shell integration";
    let full_block = format!("{}\n{}\n", marker, source_line);

    // Read existing rc file content
    let existing_content = fs::read_to_string(&rc_file).unwrap_or_default();

    // Check if already installed
    if existing_content.contains(&source_line) || existing_content.contains(marker) {
        return Ok((rc_path_str, true));
    }

    // Ensure parent directory exists (for fish config)
    if let Some(parent) = rc_file.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Append to rc file
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&rc_file)
        .map_err(|e| format!("Failed to open {}: {}", rc_path_str, e))?;

    // Add a newline before if file doesn't end with one
    let prefix = if existing_content.is_empty() || existing_content.ends_with('\n') {
        ""
    } else {
        "\n"
    };

    writeln!(file, "{}{}", prefix, full_block)
        .map_err(|e| format!("Failed to write to {}: {}", rc_path_str, e))?;

    Ok((rc_path_str, false))
}

/// Install shell completions
fn install_completions(shell: Shell) -> std::result::Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

    let (completions_dir, filename) = match shell {
        Shell::Bash => (
            format!("{}/.local/share/bash-completion/completions", home),
            "pjmai".to_string(),
        ),
        Shell::Zsh => (format!("{}/.zsh/completions", home), "_pjmai".to_string()),
        Shell::Fish => (
            format!("{}/.config/fish/completions", home),
            "pjmai.fish".to_string(),
        ),
        _ => {
            return Err(format!(
                "Completions for {:?} not supported yet",
                shell
            ))
        }
    };

    // Create completions directory
    fs::create_dir_all(&completions_dir)
        .map_err(|e| format!("Failed to create {}: {}", completions_dir, e))?;

    let completions_path = format!("{}/{}", completions_dir, filename);

    // Generate completions
    let mut cmd = crate::args::Args::command();
    let mut buffer = Vec::new();
    clap_complete::generate(shell, &mut cmd, "pjmai", &mut buffer);

    // Write completions file
    fs::write(&completions_path, buffer)
        .map_err(|e| format!("Failed to write {}: {}", completions_path, e))?;

    // For zsh, remind about fpath
    if shell == Shell::Zsh {
        // Check if .zshrc has the fpath line
        let zshrc = format!("{}/.zshrc", home);
        let zshrc_content = fs::read_to_string(&zshrc).unwrap_or_default();
        let fpath_line = "fpath=(~/.zsh/completions $fpath)";
        if !zshrc_content.contains(fpath_line) && !zshrc_content.contains(".zsh/completions") {
            // Add fpath to .zshrc
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&zshrc)
                .map_err(|e| format!("Failed to open {}: {}", zshrc, e))?;
            writeln!(
                file,
                "\n# PJMAI completions\n{}\nautoload -Uz compinit && compinit",
                fpath_line
            )
            .map_err(|e| format!("Failed to write to {}: {}", zshrc, e))?;
        }
    }

    Ok(completions_path)
}
