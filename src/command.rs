use crate::error::{PjmError, Result};
use crate::output::{
    self, AliasOutput, AliasesOutput, ChangeOutput, ErrorOutput, ListOutput, ProjectOutput,
    PromptOutput, PushPopOutput, SetupAction, SetupOutput, ShowOutput, SuccessOutput,
};
use crate::projects;
use crate::util;
use clap::CommandFactory;
use clap_complete::Shell;
use colored::Colorize;
use log::info;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

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
                alias: "mvpj".to_string(),
                command: "pjmai rename".to_string(),
                description: "Rename a project".to_string(),
            },
            AliasOutput {
                alias: "popj".to_string(),
                command: "pjmai pop".to_string(),
                description: "Pop from project stack".to_string(),
            },
            AliasOutput {
                alias: "prpj".to_string(),
                command: "pjmai prompt".to_string(),
                description: "Get current project name for prompt".to_string(),
            },
            AliasOutput {
                alias: "pspj".to_string(),
                command: "pjmai push".to_string(),
                description: "Push current and switch to project".to_string(),
            },
            AliasOutput {
                alias: "rmpj".to_string(),
                command: "pjmai remove".to_string(),
                description: "Remove a project".to_string(),
            },
            AliasOutput {
                alias: "scpj".to_string(),
                command: "pjmai scan".to_string(),
                description: "Scan for git repositories".to_string(),
            },
            AliasOutput {
                alias: "shpj".to_string(),
                command: "pjmai show".to_string(),
                description: "Show current project".to_string(),
            },
            AliasOutput {
                alias: "srcpj".to_string(),
                command: "source .pjmai.sh".to_string(),
                description: "Source project env file (explicit opt-in)".to_string(),
            },
        ];
        output::print_json(&AliasesOutput { aliases });
    } else {
        println!("adpj <name> -f <dir|file> # alias for pjmai add");
        println!("chpj <name>               # alias for pjmai change");
        println!("hlpj                      # alias for pjmai aliases");
        println!("lspj                      # alias for pjmai list");
        println!("mvpj <old> <new>          # alias for pjmai rename");
        println!("popj                      # alias for pjmai pop");
        println!("prpj                      # alias for pjmai prompt");
        println!("pspj <name>               # alias for pjmai push");
        println!("rmpj <name>               # alias for pjmai remove");
        println!("scpj [dir]                # alias for pjmai scan");
        println!("shpj                      # alias for pjmai show");
        println!("srcpj                     # source .pjmai.sh (opt-in env)");
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
        // Include stack depth if > 0
        if registry.stack.is_empty() {
            println!("{}", registry.current_project);
        } else {
            println!("{}:{}", registry.current_project, registry.stack.len());
        }
    }

    info!("prompt done");
    Ok(())
}

/// Push current project to stack and switch to specified project
pub fn push(project_name: &str, json: bool) -> Result<()> {
    info!("push to project {}", project_name);
    let mut registry = util::projects()?;

    // Find matching project using fuzzy matching
    let matched = find_matching_project(project_name, &registry);

    match matched {
        MatchResult::Exact(project) | MatchResult::Unique(project) => {
            let proj_name = project.name.clone();
            let file_path = util::expand_file_path(&project.action.file_or_dir);

            if util::is_file_found(&file_path) {
                // Push current project to stack (if we have one)
                if !registry.current_project.is_empty() {
                    registry.stack.push(registry.current_project.clone());
                }

                // Switch to new project
                registry.current_project = proj_name.clone();
                util::save_config_toml(&registry.ser()?)?;

                let is_dir = util::is_file_dir(&file_path);
                let action = if is_dir { "cd" } else { "source" };
                let path_type = if is_dir { "directory" } else { "file" };

                if json {
                    output::print_json(&PushPopOutput {
                        name: proj_name,
                        path: file_path.clone(),
                        path_type: path_type.to_string(),
                        action: action.to_string(),
                        stack_depth: registry.stack.len(),
                    });
                } else {
                    print!("{}", &file_path);
                }

                if is_dir {
                    info!("push done 2");
                    std::process::exit(2); // cd
                } else {
                    info!("push done 3");
                    std::process::exit(3); // source
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
                std::process::exit(4);
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
            std::process::exit(4);
        }
        MatchResult::None => {
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
                println!("project '{}' not found", project_name);
            }
            std::process::exit(4);
        }
    }
}

/// Pop from project stack and switch to the popped project
pub fn pop(json: bool) -> Result<()> {
    info!("pop from stack");
    let mut registry = util::projects()?;

    // Check if stack is empty
    if registry.stack.is_empty() {
        if json {
            output::print_json(&ErrorOutput {
                code: "STACK_EMPTY".to_string(),
                message: "Project stack is empty".to_string(),
                similar_projects: None,
                hint: Some("Use 'pspj <project>' to push projects to the stack first".to_string()),
            });
        } else {
            eprintln!(
                "{}: Stack is empty, staying in '{}'",
                "warning".yellow().bold(),
                registry.current_project
            );
        }
        return Ok(());
    }

    // Pop from stack
    let popped_project = registry.stack.pop().unwrap();

    // Find the popped project
    let project = registry
        .project
        .iter()
        .find(|p| p.name == popped_project);

    match project {
        Some(project) => {
            let proj_name = project.name.clone();
            let file_path = util::expand_file_path(&project.action.file_or_dir);

            if util::is_file_found(&file_path) {
                registry.current_project = proj_name.clone();
                util::save_config_toml(&registry.ser()?)?;

                let is_dir = util::is_file_dir(&file_path);
                let action = if is_dir { "cd" } else { "source" };
                let path_type = if is_dir { "directory" } else { "file" };

                if json {
                    output::print_json(&PushPopOutput {
                        name: proj_name,
                        path: file_path.clone(),
                        path_type: path_type.to_string(),
                        action: action.to_string(),
                        stack_depth: registry.stack.len(),
                    });
                } else {
                    print!("{}", &file_path);
                }

                if is_dir {
                    info!("pop done 2");
                    std::process::exit(2); // cd
                } else {
                    info!("pop done 3");
                    std::process::exit(3); // source
                }
            } else {
                // Save the modified stack even though we can't switch
                util::save_config_toml(&registry.ser()?)?;

                if json {
                    output::print_json(&ErrorOutput {
                        code: "TARGET_NOT_FOUND".to_string(),
                        message: format!("Project target '{}' not found", &file_path),
                        similar_projects: None,
                        hint: Some("The project was popped but its target path is missing".to_string()),
                    });
                } else {
                    eprintln!(
                        "{}: Project '{}' target '{}' not found",
                        "error".red().bold(),
                        popped_project,
                        file_path
                    );
                }
                std::process::exit(4);
            }
        }
        None => {
            // Project was removed from registry but was still on stack
            // Save the modified stack and try to pop again
            util::save_config_toml(&registry.ser()?)?;

            if json {
                output::print_json(&ErrorOutput {
                    code: "PROJECT_NOT_FOUND".to_string(),
                    message: format!("Stacked project '{}' no longer exists", popped_project),
                    similar_projects: None,
                    hint: Some("The project was removed. Try 'popj' again.".to_string()),
                });
            } else {
                eprintln!(
                    "{}: Project '{}' was removed. Stack cleared of this entry.",
                    "warning".yellow().bold(),
                    popped_project
                );
            }
            Ok(())
        }
    }
}

/// Rename a project's nickname
pub fn rename(from: &str, to: &str, json: bool) -> Result<()> {
    info!("renaming {} to {}", from, to);
    let mut registry = util::projects()?;

    // Check if target name already exists
    if registry.project.iter().any(|p| p.name == to) {
        if json {
            output::print_json(&ErrorOutput {
                code: "DUPLICATE_PROJECT".to_string(),
                message: format!("Project '{}' already exists", to),
                similar_projects: None,
                hint: Some("Choose a different name".to_string()),
            });
        } else {
            eprintln!(
                "{}: Project '{}' already exists",
                "error".red().bold(),
                to
            );
        }
        return Ok(());
    }

    // Find and rename the project
    let mut found = false;
    for project in &mut registry.project {
        if project.name == from {
            project.name = to.to_string();
            found = true;

            // Update current_project if it was the renamed one
            if registry.current_project == from {
                registry.current_project = to.to_string();
            }
            break;
        }
    }

    if found {
        util::save_config_toml(&registry.ser()?)?;

        if json {
            output::print_json(&SuccessOutput {
                success: true,
                operation: "rename".to_string(),
                project: to.to_string(),
            });
        } else {
            println!(
                "{} '{}' {} '{}'",
                "Renamed".green(),
                from,
                "->".green(),
                to.green().bold()
            );
        }
    } else if json {
        output::print_json(&ErrorOutput {
            code: "PROJECT_NOT_FOUND".to_string(),
            message: format!("Project '{}' not found", from),
            similar_projects: None,
            hint: Some("Use 'pjmai list' to see all projects".to_string()),
        });
    } else {
        eprintln!(
            "{}: Project '{}' not found",
            "error".red().bold(),
            from
        );
    }

    info!("rename done");
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
                    stack: registry.stack.clone(),
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

                // Display stack if non-empty
                if !registry.stack.is_empty() {
                    println!(
                        "{}",
                        format!(" Stack ({}): {}", registry.stack.len(), registry.stack.join(" <- "))
                            .cyan()
                    );
                }
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
    prompt: bool,
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
    let do_prompt = prompt;

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

    // Install prompt integration
    if do_prompt {
        match install_prompt(detected_shell) {
            Ok((path, already_installed)) => {
                let message = if already_installed {
                    format!("Prompt integration already present in {}", path)
                } else {
                    format!("Added prompt integration to {}", path)
                };
                actions.push(SetupAction {
                    action: "prompt".to_string(),
                    success: true,
                    message: message.clone(),
                });
                if !json {
                    println!("{} {}", "✓".green(), message);
                }
            }
            Err(e) => {
                let message = format!("Failed to install prompt integration: {}", e);
                actions.push(SetupAction {
                    action: "prompt".to_string(),
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

/// Install prompt integration to RC file
/// Returns (rc_file_path, already_installed)
fn install_prompt(shell: Shell) -> std::result::Result<(String, bool), String> {
    let rc_file = get_rc_file(shell);
    let rc_path_str = rc_file.to_string_lossy().to_string();

    let marker = "# PJMAI prompt integration";

    // Shell-specific prompt configuration
    let prompt_code = match shell {
        Shell::Zsh => {
            r#"
# PJMAI prompt integration
_pjm_prompt() {
  local proj=$(prpj 2>/dev/null)
  [[ -n "$proj" ]] && echo "[$proj] "
}
setopt PROMPT_SUBST
PROMPT='$(_pjm_prompt)%~ %# '
"#
        }
        Shell::Bash => {
            r#"
# PJMAI prompt integration
_pjm_prompt() {
  local proj=$(prpj 2>/dev/null)
  [[ -n "$proj" ]] && echo "[$proj] "
}
PS1='$(_pjm_prompt)\w \$ '
"#
        }
        Shell::Fish => {
            return Err("Fish prompt integration not yet supported. Add to your fish_prompt function: set -l proj (prpj 2>/dev/null); and echo -n \"[$proj] \"".to_string());
        }
        _ => {
            return Err(format!("Prompt integration for {:?} not supported", shell));
        }
    };

    // Read existing rc file content
    let existing_content = fs::read_to_string(&rc_file).unwrap_or_default();

    // Check if already installed
    if existing_content.contains(marker) {
        return Ok((rc_path_str, true));
    }

    // Ensure parent directory exists
    if let Some(parent) = rc_file.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Append to rc file
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&rc_file)
        .map_err(|e| format!("Failed to open {}: {}", rc_path_str, e))?;

    writeln!(file, "{}", prompt_code)
        .map_err(|e| format!("Failed to write to {}: {}", rc_path_str, e))?;

    Ok((rc_path_str, false))
}

// ============================================================
// Scan command implementation
// ============================================================

/// Information about a discovered git repository
#[derive(Debug)]
struct DiscoveredRepo {
    /// Full path to the repository
    path: PathBuf,
    /// Suggested nickname (from repo name or dir name)
    suggested_name: String,
    /// Git remote origin URL (if any)
    remote_url: Option<String>,
    /// Parsed owner/organization from remote
    owner: Option<String>,
    /// Parsed repository name from remote
    repo_name: Option<String>,
    /// Host (e.g., "github.com")
    host: Option<String>,
}

/// Scan directories for git repositories
pub fn scan(
    start_dir: &str,
    max_depth: usize,
    ignore_dirs: Option<Vec<String>>,
    dry_run: bool,
    add_all: bool,
    json: bool,
) -> Result<()> {
    info!("scanning {} with depth {}", start_dir, max_depth);

    let expanded_dir = util::expand_file_path(start_dir);
    let start_path = PathBuf::from(&expanded_dir);

    if !start_path.exists() {
        return Err(PjmError::PathNotFound(expanded_dir));
    }

    // Load existing projects to check for duplicates
    let existing_registry = util::projects().unwrap_or_else(|_| projects::ProjectsRegistry::new());
    let existing_paths: HashSet<String> = existing_registry
        .project
        .iter()
        .map(|p| util::expand_file_path(&p.action.file_or_dir))
        .collect();

    // Build ignore set
    let mut ignore_set: HashSet<String> = HashSet::new();
    // Default ignores
    for dir in &["node_modules", "target", "vendor", ".git", "dist", "build", "__pycache__", ".venv", "venv"] {
        ignore_set.insert(dir.to_string());
    }
    // User-specified ignores
    if let Some(ignores) = ignore_dirs {
        for dir in ignores {
            ignore_set.insert(dir);
        }
    }

    // Discover repositories
    eprintln!("{}", format!("Scanning {}...", start_dir).cyan());
    let repos = discover_repos(&start_path, max_depth, &ignore_set, &existing_paths);

    if repos.is_empty() {
        eprintln!("{}", "No new git repositories found.".yellow());
        return Ok(());
    }

    // Generate unique nicknames
    let repos_with_nicknames = generate_unique_nicknames(repos, &existing_registry);

    // Group by owner/host for display
    display_discovered_repos(&repos_with_nicknames);

    if dry_run {
        eprintln!("\n{}", "[DRY RUN] No projects were added.".yellow());
        return Ok(());
    }

    // Confirm and add
    let should_add = if add_all {
        true
    } else {
        prompt_add_all(repos_with_nicknames.len())
    };

    if should_add {
        add_discovered_repos(&repos_with_nicknames, json)?;
        eprintln!(
            "\n{}",
            format!("Added {} project(s).", repos_with_nicknames.len())
                .green()
                .bold()
        );
    } else {
        eprintln!("{}", "No projects added.".yellow());
    }

    Ok(())
}

/// Recursively discover git repositories
fn discover_repos(
    dir: &Path,
    max_depth: usize,
    ignore_set: &HashSet<String>,
    existing_paths: &HashSet<String>,
) -> Vec<DiscoveredRepo> {
    let mut repos = Vec::new();
    discover_repos_recursive(dir, max_depth, 0, ignore_set, existing_paths, &mut repos);
    repos
}

fn discover_repos_recursive(
    dir: &Path,
    max_depth: usize,
    current_depth: usize,
    ignore_set: &HashSet<String>,
    existing_paths: &HashSet<String>,
    repos: &mut Vec<DiscoveredRepo>,
) {
    if current_depth > max_depth {
        return;
    }

    // Check if this directory is a git repo
    let git_dir = dir.join(".git");
    if git_dir.exists() {
        let path_str = dir.to_string_lossy().to_string();

        // Skip if already registered
        if existing_paths.contains(&path_str) {
            info!("skipping already registered: {}", path_str);
            return; // Don't recurse into registered repos
        }

        // Parse git remote
        if let Some(repo) = parse_git_repo(dir) {
            repos.push(repo);
        }
        return; // Don't recurse into git repos
    }

    // Read directory entries
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    // Load .gitignore patterns for this directory
    let gitignore_patterns = load_gitignore(dir);

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let dir_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        // Skip hidden directories (except we already handled .git above)
        if dir_name.starts_with('.') {
            continue;
        }

        // Skip if in ignore set
        if ignore_set.contains(dir_name) {
            continue;
        }

        // Skip if matches gitignore pattern
        if matches_gitignore(dir_name, &gitignore_patterns) {
            continue;
        }

        // Recurse
        discover_repos_recursive(
            &path,
            max_depth,
            current_depth + 1,
            ignore_set,
            existing_paths,
            repos,
        );
    }
}

/// Load .gitignore patterns from a directory
fn load_gitignore(dir: &Path) -> Vec<String> {
    let gitignore_path = dir.join(".gitignore");
    if !gitignore_path.exists() {
        return Vec::new();
    }

    match fs::read_to_string(&gitignore_path) {
        Ok(content) => content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| line.trim().trim_end_matches('/').to_string())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Check if a directory name matches any gitignore pattern
fn matches_gitignore(dir_name: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        // Simple matching: exact match or glob-style
        if pattern == dir_name {
            return true;
        }
        // Handle simple wildcards like *.log -> skip, we're matching dirs
        // Handle patterns like build/ or dist
        let pattern_clean = pattern.trim_start_matches('/');
        if pattern_clean == dir_name {
            return true;
        }
    }
    false
}

/// Parse a git repository to extract remote info
fn parse_git_repo(dir: &Path) -> Option<DiscoveredRepo> {
    let path = dir.to_path_buf();
    let dir_name = dir.file_name()?.to_str()?.to_string();

    // Try to read git config for remote origin
    let git_config_path = dir.join(".git/config");
    let remote_url = if git_config_path.exists() {
        parse_git_config_origin(&git_config_path)
    } else {
        None
    };

    // Parse the remote URL
    let (host, owner, repo_name) = if let Some(ref url) = remote_url {
        parse_remote_url(url)
    } else {
        (None, None, None)
    };

    // Suggested name: prefer repo name from remote, fall back to dir name
    let suggested_name = repo_name.clone().unwrap_or_else(|| dir_name.clone());

    Some(DiscoveredRepo {
        path,
        suggested_name,
        remote_url,
        owner,
        repo_name: repo_name.or(Some(dir_name)),
        host,
    })
}

/// Parse git config file to extract origin URL
fn parse_git_config_origin(config_path: &Path) -> Option<String> {
    let content = fs::read_to_string(config_path).ok()?;

    let mut in_origin_section = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[remote \"origin\"]" {
            in_origin_section = true;
            continue;
        }
        if trimmed.starts_with('[') {
            in_origin_section = false;
            continue;
        }
        if in_origin_section && trimmed.starts_with("url = ") {
            return Some(trimmed.trim_start_matches("url = ").to_string());
        }
    }
    None
}

/// Parse a git remote URL to extract host, owner, and repo name
/// Handles:
///   git@github.com:owner/repo.git
///   git@github.com-work:owner/repo.git  (SSH alias)
///   https://github.com/owner/repo.git
///   ssh://git@github.com/owner/repo.git
fn parse_remote_url(url: &str) -> (Option<String>, Option<String>, Option<String>) {
    // SSH format: git@host:owner/repo.git
    if url.starts_with("git@") {
        let rest = url.trim_start_matches("git@");
        if let Some((host_part, path_part)) = rest.split_once(':') {
            // Normalize host (handle aliases like github.com-work)
            let host = normalize_host(host_part);
            let path = path_part.trim_end_matches(".git");
            if let Some((owner, repo)) = path.split_once('/') {
                return (Some(host), Some(owner.to_string()), Some(repo.to_string()));
            }
        }
    }

    // HTTPS format: https://host/owner/repo.git
    if url.starts_with("https://") || url.starts_with("http://") {
        let rest = url
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 3 {
            let host = normalize_host(parts[0]);
            let owner = parts[1].to_string();
            let repo = parts[2].trim_end_matches(".git").to_string();
            return (Some(host), Some(owner), Some(repo));
        }
    }

    // SSH URL format: ssh://git@host/owner/repo.git
    if url.starts_with("ssh://") {
        let rest = url.trim_start_matches("ssh://");
        let rest = rest.trim_start_matches("git@");
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 3 {
            let host = normalize_host(parts[0]);
            let owner = parts[1].to_string();
            let repo = parts[2].trim_end_matches(".git").to_string();
            return (Some(host), Some(owner), Some(repo));
        }
    }

    (None, None, None)
}

/// Normalize host name (handles SSH aliases like github.com-work)
fn normalize_host(host: &str) -> String {
    // TODO: Read host_aliases from config.toml
    // For now, strip common suffixes like -work, -personal
    if let Some(base) = host.split('-').next()
        && base.contains('.')
    {
        return base.to_string();
    }
    host.to_string()
}

/// Generate unique nicknames for discovered repos, handling collisions
fn generate_unique_nicknames(
    repos: Vec<DiscoveredRepo>,
    existing_registry: &projects::ProjectsRegistry,
) -> Vec<DiscoveredRepo> {
    let mut used_names: HashSet<String> = existing_registry
        .project
        .iter()
        .map(|p| p.name.clone())
        .collect();

    let mut result = Vec::new();

    for mut repo in repos {
        let base_name = repo.suggested_name.clone();
        let mut final_name = base_name.clone();
        let mut counter = 2;

        while used_names.contains(&final_name) {
            final_name = format!("{}{}", base_name, counter);
            counter += 1;
        }

        used_names.insert(final_name.clone());
        repo.suggested_name = final_name;
        result.push(repo);
    }

    result
}

/// Display discovered repositories grouped by host/owner
fn display_discovered_repos(repos: &[DiscoveredRepo]) {
    // Group by host/owner
    let mut groups: HashMap<String, Vec<&DiscoveredRepo>> = HashMap::new();

    for repo in repos {
        let key = match (&repo.host, &repo.owner) {
            (Some(host), Some(owner)) => format!("{}/{}", host, owner),
            (Some(host), None) => host.clone(),
            _ => "(no remote)".to_string(),
        };
        groups.entry(key).or_default().push(repo);
    }

    eprintln!(
        "\n{}",
        format!("Found {} git repositories:", repos.len())
            .green()
            .bold()
    );

    // Sort groups for consistent output
    let mut group_keys: Vec<_> = groups.keys().collect();
    group_keys.sort();

    for key in group_keys {
        let group_repos = &groups[key];
        eprintln!("  {}", key.cyan());

        for repo in group_repos {
            let path_display = util::shorten_path(&repo.path.to_string_lossy());
            let name_display = if repo.suggested_name
                != repo.repo_name.as_ref().unwrap_or(&String::new()).as_str()
            {
                // Name was modified (collision)
                format!("{} (renamed)", repo.suggested_name).yellow()
            } else {
                repo.suggested_name.clone().green()
            };
            // Show remote URL for repos without parsed host (helps debugging)
            let remote_hint = if repo.host.is_none() {
                repo.remote_url
                    .as_ref()
                    .map(|u| format!(" [{}]", u.dimmed()))
                    .unwrap_or_default()
            } else {
                String::new()
            };
            eprintln!(
                "    {:12} {}{}",
                name_display,
                path_display.dimmed(),
                remote_hint
            );
        }
    }
}

/// Prompt user to add all discovered repos
fn prompt_add_all(count: usize) -> bool {
    use std::io::{stderr, stdin, Write};

    eprint!("\nAdd all {} project(s)? [Y/n] ", count);
    stderr().flush().expect("flush stderr");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("read stdin");
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes" | "")
}

/// Add discovered repos to the project registry
fn add_discovered_repos(repos: &[DiscoveredRepo], json: bool) -> Result<()> {
    let mut registry = util::projects()?;

    for repo in repos {
        let path_str = repo.path.to_string_lossy().to_string();
        registry.project.push(projects::ChangeToProject {
            action: projects::Action {
                file_or_dir: path_str.clone(),
            },
            name: repo.suggested_name.clone(),
        });

        if !json {
            eprintln!(
                "  {} {} -> {}",
                "+".green(),
                repo.suggested_name.green(),
                util::shorten_path(&path_str)
            );
        }
    }

    util::save_config_toml(&registry.ser()?)?;
    Ok(())
}
