use crate::error::{PjmError, Result};
use crate::output::{
    self, AliasOutput, AliasesOutput, ChangeOutput, ContextOutput, DetectedConfig,
    DetectedFeature, EnvAutoDetectOutput, EnvModifyOutput, EnvShowOutput, ErrorOutput, KeyFile,
    ListOutput, MetaOutput, NoteEntry, NotesOutput, ProjectOutput, PromptOutput, PushPopOutput,
    SetupAction, SetupOutput, ShowOutput, SuccessOutput, TagsOutput,
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
pub fn add(
    project_name: &str,
    file_name: &str,
    description: Option<String>,
    tags: Option<Vec<String>>,
    language: Option<String>,
    group: Option<String>,
    json: bool,
) -> Result<()> {
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

    // Build metadata if any fields are provided
    let metadata = if description.is_some() || tags.is_some() || language.is_some() || group.is_some() {
        Some(projects::ProjectMetadata {
            description,
            tags: tags.unwrap_or_default(),
            language,
            group,
            last_used: None,
            notes: Vec::new(),
            environment: None,
        })
    } else {
        None
    };

    info!("push");
    registry.project.push(projects::ChangeToProject {
        action: projects::Action {
            file_or_dir: file_name.to_string(),
        },
        name: project_name.to_string(),
        metadata,
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
                alias: "ctpj".to_string(),
                command: "pjmai context".to_string(),
                description: "Show project context for AI agents".to_string(),
            },
            AliasOutput {
                alias: "evpj".to_string(),
                command: "pjmai env".to_string(),
                description: "Manage project environment config".to_string(),
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
        println!("ctpj [name]               # alias for pjmai context");
        println!("evpj <name> <cmd> [args]  # alias for pjmai env");
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
            // Clone project for env setup check (before registry modification)
            let project_clone = project.clone();

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
                }

                if is_dir {
                    // Check for environment setup
                    if let Some(setup_script) = generate_env_setup(&project_clone, &file_path) {
                        if !json {
                            print!("{}", setup_script);
                        }
                        info!("change done 5");
                        std::process::exit(5); // bash wrapper will eval the setup script
                    }
                    if !json {
                        print!("{}", &file_path); // path parameter for bash cd command
                    }
                    info!("change done 2");
                    std::process::exit(2); // bash wrapper will cd to the above printed path
                } else {
                    if !json {
                        print!("{}", &file_path); // path parameter for bash source command
                    }
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
pub fn list(
    tag_filter: Option<String>,
    group_filter: Option<String>,
    sort_recent: bool,
    json: bool,
) -> Result<()> {
    info!("listing all projects");
    let registry = util::projects()?;

    // Get filtered and sorted projects
    let filtered_projects: Vec<&projects::ChangeToProject> = if let Some(ref tag) = tag_filter {
        registry.projects_with_tag(tag)
    } else if let Some(ref group) = group_filter {
        registry.projects_in_group(group)
    } else if sort_recent {
        registry.projects_by_recency()
    } else {
        let mut sorted: Vec<_> = registry.project.iter().collect();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        sorted
    };

    if json {
        let projects: Vec<ProjectOutput> = filtered_projects
            .iter()
            .map(|p| {
                let expanded = util::expand_file_path(&p.action.file_or_dir);
                let meta = p.metadata.as_ref();
                ProjectOutput {
                    name: p.name.clone(),
                    path: expanded.clone(),
                    path_type: output::path_type(&p.action.file_or_dir),
                    is_current: p.name == registry.current_project,
                    description: meta.and_then(|m| m.description.clone()),
                    tags: meta.map(|m| m.tags.clone()).unwrap_or_default(),
                    language: meta.and_then(|m| m.language.clone()),
                    group: meta.and_then(|m| m.group.clone()),
                    last_used: meta.and_then(|m| m.last_used.clone()),
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
        // Show filter info
        if let Some(ref tag) = tag_filter {
            println!("{}", format!("Projects with tag '{}':", tag).cyan());
        } else if let Some(ref group) = group_filter {
            println!("{}", format!("Projects in group '{}':", group).cyan());
        } else if sort_recent {
            println!("{}", "Projects by recently used:".cyan());
        }

        for project in &filtered_projects {
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

            // Build tags/group suffix
            let meta = project.metadata.as_ref();
            let tags_str = meta
                .map(|m| {
                    if m.tags.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", m.tags.join(", ")).dimmed().to_string()
                    }
                })
                .unwrap_or_default();

            println!(
                "{}{:8} {}{}",
                current, colored_name, colored_short_path, tags_str
            );
        }

        // Only show "no matching" when a filter was applied
        if filtered_projects.is_empty()
            && (tag_filter.is_some() || group_filter.is_some() || sort_recent)
        {
            println!("{}", "(no matching projects)".dimmed());
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
    let mut registry = util::projects()?;
    let initial_len = registry.project.len();

    // Remove the project (preserves all fields including metadata for remaining projects)
    registry
        .project
        .retain(|p| p.name != unwanted_project_name);

    let found = registry.project.len() < initial_len;

    if found {
        // Clear current_project if we just removed it
        if registry.current_project == unwanted_project_name {
            registry.current_project = String::new();
        }
        // Also clean up stack
        registry.stack.retain(|n| n != unwanted_project_name);

        info!("saving changes");
        util::save_config_toml(&registry.ser()?)?;

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
        CompleteTarget::Tags { prefix } => {
            complete_tags(prefix.as_deref());
        }
        CompleteTarget::Groups { prefix } => {
            complete_groups(prefix.as_deref());
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
        "context",
        "list",
        "meta",
        "note",
        "pop",
        "prompt",
        "push",
        "remove",
        "rename",
        "scan",
        "setup",
        "show",
        "tag",
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

/// Output tag names matching a prefix (one per line)
fn complete_tags(prefix: Option<&str>) {
    info!("complete tags prefix={:?}", prefix);

    let Ok(registry) = util::projects() else {
        return;
    };

    let prefix_lower = prefix.map(|s| s.to_lowercase());

    for tag in registry.all_tags() {
        let matches = match &prefix_lower {
            Some(p) => tag.to_lowercase().starts_with(p),
            None => true,
        };
        if matches {
            println!("{}", tag);
        }
    }
}

/// Output group names matching a prefix (one per line)
fn complete_groups(prefix: Option<&str>) {
    info!("complete groups prefix={:?}", prefix);

    let Ok(registry) = util::projects() else {
        return;
    };

    let prefix_lower = prefix.map(|s| s.to_lowercase());

    for group in registry.all_groups() {
        let matches = match &prefix_lower {
            Some(p) => group.to_lowercase().starts_with(p),
            None => true,
        };
        if matches {
            println!("{}", group);
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
// Context command implementation
// ============================================================

/// Output project context for AI agents
pub fn context(project_name: Option<String>, for_agent: bool, json: bool) -> Result<()> {
    info!("context for {:?}", project_name);
    let registry = util::projects()?;

    // Use current project if none specified
    let target_name = project_name.unwrap_or_else(|| registry.current_project.clone());

    if target_name.is_empty() {
        if json {
            output::print_json(&ErrorOutput {
                code: "NO_CURRENT_PROJECT".to_string(),
                message: "No project specified and no current project set".to_string(),
                similar_projects: None,
                hint: Some("Use 'pjmai context -p <project>' or set a current project".to_string()),
            });
        } else {
            eprintln!("{}: No project specified", "error".red().bold());
        }
        return Ok(());
    }

    // Find the project
    let project = registry.find_project(&target_name);

    match project {
        Some(proj) => {
            let expanded_path = util::expand_file_path(&proj.action.file_or_dir);
            let meta = proj.metadata.as_ref();

            // Detect key files in the project directory
            let key_files = if util::is_file_dir(&expanded_path) {
                detect_key_files(&expanded_path)
            } else {
                Vec::new()
            };

            if json || for_agent {
                output::print_json(&ContextOutput {
                    name: proj.name.clone(),
                    path: expanded_path.clone(),
                    path_type: output::path_type(&proj.action.file_or_dir),
                    description: meta.and_then(|m| m.description.clone()),
                    tags: meta.map(|m| m.tags.clone()).unwrap_or_default(),
                    language: meta.and_then(|m| m.language.clone()),
                    group: meta.and_then(|m| m.group.clone()),
                    notes: meta.map(|m| m.notes.clone()).unwrap_or_default(),
                    key_files,
                });
            } else {
                // Human-readable context output
                println!("{}: {}", "Project".cyan().bold(), proj.name.green().bold());
                println!("{}: {}", "Path".cyan(), expanded_path);
                println!(
                    "{}: {}",
                    "Type".cyan(),
                    output::path_type(&proj.action.file_or_dir)
                );

                if let Some(meta) = meta {
                    if let Some(ref desc) = meta.description {
                        println!("{}: {}", "Description".cyan(), desc);
                    }
                    if let Some(ref lang) = meta.language {
                        println!("{}: {}", "Language".cyan(), lang);
                    }
                    if let Some(ref group) = meta.group {
                        println!("{}: {}", "Group".cyan(), group);
                    }
                    if !meta.tags.is_empty() {
                        println!("{}: {}", "Tags".cyan(), meta.tags.join(", "));
                    }
                    if !meta.notes.is_empty() {
                        println!("{}:", "Notes".cyan());
                        for note in &meta.notes {
                            println!("  - {}", note);
                        }
                    }
                }

                if !key_files.is_empty() {
                    println!("{}:", "Key Files".cyan());
                    for kf in key_files {
                        println!("  {}: {}", kf.name.green(), kf.purpose);
                    }
                }
            }
        }
        None => {
            if json {
                output::print_json(&ErrorOutput {
                    code: "PROJECT_NOT_FOUND".to_string(),
                    message: format!("Project '{}' not found", target_name),
                    similar_projects: None,
                    hint: Some("Use 'pjmai list' to see all projects".to_string()),
                });
            } else {
                eprintln!(
                    "{}: Project '{}' not found",
                    "error".red().bold(),
                    target_name
                );
            }
        }
    }

    Ok(())
}

/// Detect key files in a project directory
fn detect_key_files(dir: &str) -> Vec<KeyFile> {
    let path = Path::new(dir);
    let mut key_files = Vec::new();

    let checks: &[(&str, &str)] = &[
        ("README.md", "Project documentation"),
        ("README", "Project documentation"),
        ("CLAUDE.md", "AI assistant instructions"),
        ("Cargo.toml", "Rust package manifest"),
        ("package.json", "Node.js package manifest"),
        ("pyproject.toml", "Python project configuration"),
        ("setup.py", "Python setup script"),
        ("Makefile", "Build automation"),
        ("Dockerfile", "Container configuration"),
        ("docker-compose.yml", "Multi-container Docker"),
        (".env.example", "Environment template"),
        ("CONTRIBUTING.md", "Contribution guidelines"),
        ("LICENSE", "License file"),
        ("Justfile", "Command runner"),
    ];

    for (filename, purpose) in checks {
        if path.join(filename).exists() {
            key_files.push(KeyFile {
                name: filename.to_string(),
                purpose: purpose.to_string(),
            });
        }
    }

    key_files
}

// ============================================================
// Note command implementation
// ============================================================

/// Manage project notes
pub fn note(
    project_name: &str,
    action: &crate::args::NoteAction,
    json: bool,
) -> Result<()> {
    use crate::args::NoteAction;
    info!("note for {} action={:?}", project_name, action);

    let mut registry = util::projects()?;

    // Find the project
    let project_idx = registry
        .project
        .iter()
        .position(|p| p.name == project_name);

    let Some(idx) = project_idx else {
        if json {
            output::print_json(&ErrorOutput {
                code: "PROJECT_NOT_FOUND".to_string(),
                message: format!("Project '{}' not found", project_name),
                similar_projects: None,
                hint: Some("Use 'pjmai list' to see all projects".to_string()),
            });
        } else {
            eprintln!(
                "{}: Project '{}' not found",
                "error".red().bold(),
                project_name
            );
        }
        return Ok(());
    };

    match action {
        NoteAction::Add { text } => {
            // Ensure metadata exists
            if registry.project[idx].metadata.is_none() {
                registry.project[idx].metadata = Some(projects::ProjectMetadata::default());
            }
            registry.project[idx]
                .metadata
                .as_mut()
                .unwrap()
                .notes
                .push(text.clone());
            util::save_config_toml(&registry.ser()?)?;

            if json {
                output::print_json(&SuccessOutput {
                    success: true,
                    operation: "note_add".to_string(),
                    project: project_name.to_string(),
                });
            } else {
                println!("{} Added note to '{}'", "+".green(), project_name);
            }
        }
        NoteAction::List {} => {
            let notes = registry.project[idx]
                .metadata
                .as_ref()
                .map(|m| m.notes.clone())
                .unwrap_or_default();

            if json {
                let entries: Vec<NoteEntry> = notes
                    .iter()
                    .enumerate()
                    .map(|(i, text)| NoteEntry {
                        index: i + 1,
                        text: text.clone(),
                    })
                    .collect();
                output::print_json(&NotesOutput {
                    project: project_name.to_string(),
                    notes: entries,
                });
            } else if notes.is_empty() {
                println!("{}", "(no notes)".dimmed());
            } else {
                println!("{}:", format!("Notes for '{}'", project_name).cyan());
                for (i, note) in notes.iter().enumerate() {
                    println!("  {}. {}", i + 1, note);
                }
            }
        }
        NoteAction::Remove { index } => {
            let notes = registry.project[idx]
                .metadata
                .as_mut()
                .map(|m| &mut m.notes);

            if let Some(notes) = notes {
                if *index == 0 || *index > notes.len() {
                    if json {
                        output::print_json(&ErrorOutput {
                            code: "INVALID_INDEX".to_string(),
                            message: format!("Invalid note index: {}", index),
                            similar_projects: None,
                            hint: Some("Use 'pjmai note -p <project> list' to see note indices".to_string()),
                        });
                    } else {
                        eprintln!("{}: Invalid note index {}", "error".red().bold(), index);
                    }
                    return Ok(());
                }
                notes.remove(*index - 1);
                util::save_config_toml(&registry.ser()?)?;

                if json {
                    output::print_json(&SuccessOutput {
                        success: true,
                        operation: "note_remove".to_string(),
                        project: project_name.to_string(),
                    });
                } else {
                    println!("{} Removed note {} from '{}'", "-".red(), index, project_name);
                }
            }
        }
        NoteAction::Clear {} => {
            if let Some(meta) = registry.project[idx].metadata.as_mut() {
                meta.notes.clear();
            }
            util::save_config_toml(&registry.ser()?)?;

            if json {
                output::print_json(&SuccessOutput {
                    success: true,
                    operation: "note_clear".to_string(),
                    project: project_name.to_string(),
                });
            } else {
                println!("{} Cleared all notes from '{}'", "✓".green(), project_name);
            }
        }
    }

    Ok(())
}

// ============================================================
// Tag command implementation
// ============================================================

/// Manage project tags
pub fn tag(
    project_name: &str,
    action: &crate::args::TagAction,
    json: bool,
) -> Result<()> {
    use crate::args::TagAction;
    info!("tag for {} action={:?}", project_name, action);

    let mut registry = util::projects()?;

    // Find the project
    let project_idx = registry
        .project
        .iter()
        .position(|p| p.name == project_name);

    let Some(idx) = project_idx else {
        if json {
            output::print_json(&ErrorOutput {
                code: "PROJECT_NOT_FOUND".to_string(),
                message: format!("Project '{}' not found", project_name),
                similar_projects: None,
                hint: Some("Use 'pjmai list' to see all projects".to_string()),
            });
        } else {
            eprintln!(
                "{}: Project '{}' not found",
                "error".red().bold(),
                project_name
            );
        }
        return Ok(());
    };

    match action {
        TagAction::Add { tags } => {
            // Ensure metadata exists
            if registry.project[idx].metadata.is_none() {
                registry.project[idx].metadata = Some(projects::ProjectMetadata::default());
            }
            let meta = registry.project[idx].metadata.as_mut().unwrap();
            for tag in tags {
                if !meta.tags.contains(tag) {
                    meta.tags.push(tag.clone());
                }
            }
            meta.tags.sort();
            util::save_config_toml(&registry.ser()?)?;

            if json {
                output::print_json(&SuccessOutput {
                    success: true,
                    operation: "tag_add".to_string(),
                    project: project_name.to_string(),
                });
            } else {
                println!(
                    "{} Added tags to '{}': {}",
                    "+".green(),
                    project_name,
                    tags.join(", ")
                );
            }
        }
        TagAction::List {} => {
            let tags = registry.project[idx]
                .metadata
                .as_ref()
                .map(|m| m.tags.clone())
                .unwrap_or_default();

            if json {
                output::print_json(&TagsOutput {
                    project: project_name.to_string(),
                    tags,
                });
            } else if tags.is_empty() {
                println!("{}", "(no tags)".dimmed());
            } else {
                println!("{}: {}", project_name.green(), tags.join(", "));
            }
        }
        TagAction::Remove { tags } => {
            if let Some(meta) = registry.project[idx].metadata.as_mut() {
                meta.tags.retain(|t| !tags.contains(t));
            }
            util::save_config_toml(&registry.ser()?)?;

            if json {
                output::print_json(&SuccessOutput {
                    success: true,
                    operation: "tag_remove".to_string(),
                    project: project_name.to_string(),
                });
            } else {
                println!(
                    "{} Removed tags from '{}': {}",
                    "-".red(),
                    project_name,
                    tags.join(", ")
                );
            }
        }
        TagAction::Clear {} => {
            if let Some(meta) = registry.project[idx].metadata.as_mut() {
                meta.tags.clear();
            }
            util::save_config_toml(&registry.ser()?)?;

            if json {
                output::print_json(&SuccessOutput {
                    success: true,
                    operation: "tag_clear".to_string(),
                    project: project_name.to_string(),
                });
            } else {
                println!("{} Cleared all tags from '{}'", "✓".green(), project_name);
            }
        }
    }

    Ok(())
}

// ============================================================
// Meta command implementation
// ============================================================

/// Update project metadata
pub fn meta(
    project_name: &str,
    description: Option<String>,
    language: Option<String>,
    group: Option<String>,
    json: bool,
) -> Result<()> {
    info!("meta for {}", project_name);

    let mut registry = util::projects()?;

    // Find the project
    let project_idx = registry
        .project
        .iter()
        .position(|p| p.name == project_name);

    let Some(idx) = project_idx else {
        if json {
            output::print_json(&ErrorOutput {
                code: "PROJECT_NOT_FOUND".to_string(),
                message: format!("Project '{}' not found", project_name),
                similar_projects: None,
                hint: Some("Use 'pjmai list' to see all projects".to_string()),
            });
        } else {
            eprintln!(
                "{}: Project '{}' not found",
                "error".red().bold(),
                project_name
            );
        }
        return Ok(());
    };

    // Ensure metadata exists
    if registry.project[idx].metadata.is_none() {
        registry.project[idx].metadata = Some(projects::ProjectMetadata::default());
    }

    let meta = registry.project[idx].metadata.as_mut().unwrap();
    let mut updated_fields = Vec::new();

    if let Some(desc) = description {
        meta.description = Some(desc);
        updated_fields.push("description".to_string());
    }
    if let Some(lang) = language {
        meta.language = Some(lang);
        updated_fields.push("language".to_string());
    }
    if let Some(grp) = group {
        meta.group = Some(grp);
        updated_fields.push("group".to_string());
    }

    if updated_fields.is_empty() {
        if json {
            output::print_json(&ErrorOutput {
                code: "NO_FIELDS".to_string(),
                message: "No fields to update".to_string(),
                similar_projects: None,
                hint: Some("Specify --description, --language, or --group".to_string()),
            });
        } else {
            eprintln!("{}: No fields to update", "warning".yellow().bold());
        }
        return Ok(());
    }

    util::save_config_toml(&registry.ser()?)?;

    if json {
        output::print_json(&MetaOutput {
            success: true,
            project: project_name.to_string(),
            updated: updated_fields,
        });
    } else {
        println!(
            "{} Updated '{}': {}",
            "✓".green(),
            project_name,
            updated_fields.join(", ")
        );
    }

    Ok(())
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
            metadata: None, // Metadata can be added later with `pjmai meta`
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

/// Export configuration
pub fn config_export(format: &str, json: bool) -> Result<()> {
    info!("config export format={}", format);
    let registry = util::projects()?;

    match format.to_lowercase().as_str() {
        "toml" => {
            // For TOML format, just output the raw config file
            let toml_str = registry.ser()?;
            println!("{}", toml_str);
        }
        "json" => {
            // For JSON format, output structured JSON
            let projects: Vec<output::ProjectOutput> = registry
                .project
                .iter()
                .map(|p| {
                    let expanded = util::expand_file_path(&p.action.file_or_dir);
                    let meta = p.metadata.as_ref();
                    output::ProjectOutput {
                        name: p.name.clone(),
                        path: expanded.clone(),
                        path_type: output::path_type(&p.action.file_or_dir),
                        is_current: p.name == registry.current_project,
                        description: meta.and_then(|m| m.description.clone()),
                        tags: meta.map(|m| m.tags.clone()).unwrap_or_default(),
                        language: meta.and_then(|m| m.language.clone()),
                        group: meta.and_then(|m| m.group.clone()),
                        last_used: meta.and_then(|m| m.last_used.clone()),
                    }
                })
                .collect();

            let export = output::ConfigExportOutput {
                version: registry.version.clone(),
                current_project: registry.current_project.clone(),
                projects,
                stack: registry.stack.clone(),
            };
            output::print_json(&export);
        }
        _ => {
            return Err(PjmError::InvalidFormat(format!(
                "Unknown format '{}'. Supported formats: toml, json",
                format
            )));
        }
    }

    if json && format != "json" {
        // If --json flag is set but format isn't json, output success message
        output::print_json(&output::SuccessOutput {
            success: true,
            operation: "export".to_string(),
            project: format!("config (format: {})", format),
        });
    }

    Ok(())
}

/// Import configuration from a file
pub fn config_import(file: &str, merge: bool, dry_run: bool, json: bool) -> Result<()> {
    info!("config import file={} merge={} dry_run={}", file, merge, dry_run);

    // Read and parse the import file
    let expanded_path = util::expand_file_path(file);
    if !util::is_file_found(&expanded_path) {
        return Err(PjmError::PathNotFound(expanded_path));
    }

    let contents = fs::read_to_string(&expanded_path)
        .map_err(|e| PjmError::IoError(format!("Failed to read {}: {}", file, e)))?;

    // Determine format from extension or content
    let import_registry: projects::ProjectsRegistry = if file.ends_with(".json") {
        serde_json::from_str(&contents)
            .map_err(|e| PjmError::ConfigParse(format!("Failed to parse JSON: {}", e)))?
    } else {
        // Assume TOML
        projects::ProjectsRegistry::deser(contents)?
    };

    let mut registry = util::projects()?;
    let existing_names: HashSet<String> = registry.project.iter().map(|p| p.name.clone()).collect();

    let mut added: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    let mut updated: Vec<String> = Vec::new();

    for import_project in &import_registry.project {
        if existing_names.contains(&import_project.name) {
            if merge {
                // In merge mode, update existing project's metadata
                if let Some(existing) = registry.find_project_mut(&import_project.name) {
                    // Only update metadata if import has metadata
                    if import_project.metadata.is_some() {
                        existing.metadata = import_project.metadata.clone();
                        updated.push(import_project.name.clone());
                    } else {
                        skipped.push(import_project.name.clone());
                    }
                }
            } else {
                // Without merge, skip existing projects
                skipped.push(import_project.name.clone());
            }
        } else {
            // New project, add it
            added.push(import_project.name.clone());
            if !dry_run {
                registry.project.push(import_project.clone());
            }
        }
    }

    // Save if not dry run and we made changes
    if !dry_run && (!added.is_empty() || !updated.is_empty()) {
        util::save_config_toml(&registry.ser()?)?;
    }

    if json {
        output::print_json(&output::ConfigImportOutput {
            success: true,
            added: added.len(),
            skipped: skipped.len(),
            updated: updated.len(),
            added_projects: added,
            skipped_projects: skipped,
            updated_projects: updated,
        });
    } else {
        let action_word = if dry_run { "Would import" } else { "Imported" };
        println!(
            "{} {} project(s)",
            action_word,
            (added.len() + updated.len()).to_string().green()
        );

        if !added.is_empty() {
            println!("{}", "Added:".cyan());
            for name in &added {
                println!("  {} {}", "+".green(), name);
            }
        }

        if !updated.is_empty() {
            println!("{}", "Updated:".cyan());
            for name in &updated {
                println!("  {} {}", "~".yellow(), name);
            }
        }

        if !skipped.is_empty() {
            println!(
                "{} {} project(s) already exist",
                "Skipped:".dimmed(),
                skipped.len()
            );
            for name in &skipped {
                println!("  {} {}", "-".dimmed(), name.dimmed());
            }
        }

        if dry_run {
            println!("\n{}", "(dry run - no changes made)".dimmed());
        }
    }

    Ok(())
}

/// Set an environment variable for a project
pub fn env_set(project_name: &str, key: &str, value: &str, json: bool) -> Result<()> {
    info!("env set {} for {}", key, project_name);
    let mut registry = util::projects()?;

    let project = registry
        .find_project_mut(project_name)
        .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

    // Ensure metadata exists
    if project.metadata.is_none() {
        project.metadata = Some(projects::ProjectMetadata::default());
    }

    let metadata = project.metadata.as_mut().unwrap();

    // Ensure environment exists
    if metadata.environment.is_none() {
        metadata.environment = Some(projects::EnvironmentConfig::default());
    }

    let env = metadata.environment.as_mut().unwrap();

    // Ensure vars exists
    if env.vars.is_none() {
        env.vars = Some(HashMap::new());
    }

    env.vars.as_mut().unwrap().insert(key.to_string(), value.to_string());

    util::save_config_toml(&registry.ser()?)?;

    if json {
        output::print_json(&EnvModifyOutput {
            success: true,
            operation: "set".to_string(),
            project: project_name.to_string(),
        });
    } else {
        println!("Set {}={} for project {}", key, value, project_name);
    }

    Ok(())
}

/// Remove an environment variable from a project
pub fn env_unset(project_name: &str, key: &str, json: bool) -> Result<()> {
    info!("env unset {} for {}", key, project_name);
    let mut registry = util::projects()?;

    let project = registry
        .find_project_mut(project_name)
        .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

    if let Some(ref mut metadata) = project.metadata
        && let Some(ref mut env) = metadata.environment
        && let Some(ref mut vars) = env.vars
    {
        vars.remove(key);
    }

    util::save_config_toml(&registry.ser()?)?;

    if json {
        output::print_json(&EnvModifyOutput {
            success: true,
            operation: "unset".to_string(),
            project: project_name.to_string(),
        });
    } else {
        println!("Unset {} for project {}", key, project_name);
    }

    Ok(())
}

/// Add an on_enter command to a project
pub fn env_on_enter(project_name: &str, command: &str, json: bool) -> Result<()> {
    info!("env on_enter {} for {}", command, project_name);
    let mut registry = util::projects()?;

    let project = registry
        .find_project_mut(project_name)
        .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

    // Ensure metadata exists
    if project.metadata.is_none() {
        project.metadata = Some(projects::ProjectMetadata::default());
    }

    let metadata = project.metadata.as_mut().unwrap();

    // Ensure environment exists
    if metadata.environment.is_none() {
        metadata.environment = Some(projects::EnvironmentConfig::default());
    }

    let env = metadata.environment.as_mut().unwrap();

    // Ensure on_enter exists
    if env.on_enter.is_none() {
        env.on_enter = Some(Vec::new());
    }

    env.on_enter.as_mut().unwrap().push(command.to_string());

    util::save_config_toml(&registry.ser()?)?;

    if json {
        output::print_json(&EnvModifyOutput {
            success: true,
            operation: "on_enter".to_string(),
            project: project_name.to_string(),
        });
    } else {
        println!("Added on_enter command for project {}", project_name);
    }

    Ok(())
}

/// Add an on_exit command to a project
pub fn env_on_exit(project_name: &str, command: &str, json: bool) -> Result<()> {
    info!("env on_exit {} for {}", command, project_name);
    let mut registry = util::projects()?;

    let project = registry
        .find_project_mut(project_name)
        .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

    // Ensure metadata exists
    if project.metadata.is_none() {
        project.metadata = Some(projects::ProjectMetadata::default());
    }

    let metadata = project.metadata.as_mut().unwrap();

    // Ensure environment exists
    if metadata.environment.is_none() {
        metadata.environment = Some(projects::EnvironmentConfig::default());
    }

    let env = metadata.environment.as_mut().unwrap();

    // Ensure on_exit exists
    if env.on_exit.is_none() {
        env.on_exit = Some(Vec::new());
    }

    env.on_exit.as_mut().unwrap().push(command.to_string());

    util::save_config_toml(&registry.ser()?)?;

    if json {
        output::print_json(&EnvModifyOutput {
            success: true,
            operation: "on_exit".to_string(),
            project: project_name.to_string(),
        });
    } else {
        println!("Added on_exit command for project {}", project_name);
    }

    Ok(())
}

/// Prepend a path to PATH for a project
pub fn env_path_prepend(project_name: &str, path: &str, json: bool) -> Result<()> {
    info!("env path_prepend {} for {}", path, project_name);
    let mut registry = util::projects()?;

    let project = registry
        .find_project_mut(project_name)
        .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

    // Ensure metadata exists
    if project.metadata.is_none() {
        project.metadata = Some(projects::ProjectMetadata::default());
    }

    let metadata = project.metadata.as_mut().unwrap();

    // Ensure environment exists
    if metadata.environment.is_none() {
        metadata.environment = Some(projects::EnvironmentConfig::default());
    }

    let env = metadata.environment.as_mut().unwrap();

    // Ensure path_prepend exists
    if env.path_prepend.is_none() {
        env.path_prepend = Some(Vec::new());
    }

    env.path_prepend.as_mut().unwrap().push(path.to_string());

    util::save_config_toml(&registry.ser()?)?;

    if json {
        output::print_json(&EnvModifyOutput {
            success: true,
            operation: "path_prepend".to_string(),
            project: project_name.to_string(),
        });
    } else {
        println!("Added path_prepend '{}' for project {}", path, project_name);
    }

    Ok(())
}

/// Remove a path from the path_prepend list
pub fn env_path_remove(project_name: &str, path: &str, json: bool) -> Result<()> {
    info!("env path_remove {} for {}", path, project_name);
    let mut registry = util::projects()?;

    let project = registry
        .find_project_mut(project_name)
        .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

    if let Some(ref mut metadata) = project.metadata
        && let Some(ref mut env) = metadata.environment
        && let Some(ref mut paths) = env.path_prepend
    {
        paths.retain(|p| p != path);
    }

    util::save_config_toml(&registry.ser()?)?;

    if json {
        output::print_json(&EnvModifyOutput {
            success: true,
            operation: "path_remove".to_string(),
            project: project_name.to_string(),
        });
    } else {
        println!("Removed path_prepend '{}' for project {}", path, project_name);
    }

    Ok(())
}

/// Show environment config for a project
pub fn env_show(project_name: &str, json: bool) -> Result<()> {
    info!("env show for {}", project_name);
    let registry = util::projects()?;

    let project = registry
        .find_project(project_name)
        .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

    let (vars, on_enter, on_exit, path_prepend) = if let Some(ref metadata) = project.metadata {
        if let Some(ref env) = metadata.environment {
            (
                env.vars.clone().unwrap_or_default(),
                env.on_enter.clone().unwrap_or_default(),
                env.on_exit.clone().unwrap_or_default(),
                env.path_prepend.clone().unwrap_or_default(),
            )
        } else {
            (HashMap::new(), Vec::new(), Vec::new(), Vec::new())
        }
    } else {
        (HashMap::new(), Vec::new(), Vec::new(), Vec::new())
    };

    if json {
        output::print_json(&EnvShowOutput {
            project: project_name.to_string(),
            vars,
            on_enter,
            on_exit,
            path_prepend,
        });
    } else {
        println!("Environment config for project {}:", project_name.cyan());
        if vars.is_empty() && on_enter.is_empty() && on_exit.is_empty() && path_prepend.is_empty() {
            println!("  (no environment configuration)");
        } else {
            if !vars.is_empty() {
                println!("  {}:", "Variables".green());
                for (k, v) in &vars {
                    println!("    {}={}", k, v);
                }
            }
            if !path_prepend.is_empty() {
                println!("  {}:", "Path prepend".green());
                for p in &path_prepend {
                    println!("    {}", p);
                }
            }
            if !on_enter.is_empty() {
                println!("  {}:", "On enter".green());
                for cmd in &on_enter {
                    println!("    {}", cmd);
                }
            }
            if !on_exit.is_empty() {
                println!("  {}:", "On exit".green());
                for cmd in &on_exit {
                    println!("    {}", cmd);
                }
            }
        }
    }

    Ok(())
}

/// Clear all environment config for a project
pub fn env_clear(project_name: &str, json: bool) -> Result<()> {
    info!("env clear for {}", project_name);
    let mut registry = util::projects()?;

    let project = registry
        .find_project_mut(project_name)
        .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

    if let Some(ref mut metadata) = project.metadata {
        metadata.environment = None;
    }

    util::save_config_toml(&registry.ser()?)?;

    if json {
        output::print_json(&EnvModifyOutput {
            success: true,
            operation: "clear".to_string(),
            project: project_name.to_string(),
        });
    } else {
        println!("Cleared environment config for project {}", project_name);
    }

    Ok(())
}

/// Auto-detect environment configuration from project files
pub fn env_auto_detect(project_name: &str, dry_run: bool, json: bool) -> Result<()> {
    info!("env auto-detect for {} (dry_run={})", project_name, dry_run);
    let mut registry = util::projects()?;

    let project = registry
        .find_project(project_name)
        .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

    // Get the project path
    let project_path = util::expand_file_path(&project.action.file_or_dir);

    if !util::is_file_dir(&project_path) {
        if json {
            output::print_json(&ErrorOutput {
                code: "NOT_DIRECTORY".to_string(),
                message: format!(
                    "Project '{}' points to a file, not a directory",
                    project_name
                ),
                similar_projects: None,
                hint: Some("Auto-detect only works for directory-based projects".to_string()),
            });
        } else {
            println!(
                "{}",
                "Auto-detect only works for directory-based projects".red()
            );
        }
        return Ok(());
    }

    let project_dir = Path::new(&project_path);
    let mut detected: Vec<DetectedFeature> = Vec::new();

    // Check for Python virtual environment (.venv or venv directory)
    let venv_dir = if project_dir.join(".venv").is_dir() {
        Some(".venv")
    } else if project_dir.join("venv").is_dir() {
        Some("venv")
    } else {
        None
    };

    if let Some(venv) = venv_dir {
        detected.push(DetectedFeature {
            feature: "python-venv".to_string(),
            source: format!("{}/", venv),
            config: DetectedConfig {
                path_prepend: vec![format!("./{}/bin", venv)],
                on_enter: vec![format!("source {}/bin/activate", venv)],
                on_exit: vec!["deactivate".to_string()],
            },
        });
    }

    // Check for pyproject.toml (Python project without existing venv)
    if venv_dir.is_none() && project_dir.join("pyproject.toml").is_file() {
        detected.push(DetectedFeature {
            feature: "python-project".to_string(),
            source: "pyproject.toml".to_string(),
            config: DetectedConfig {
                path_prepend: vec![],
                on_enter: vec![
                    "# Python project detected - run: uv venv && source .venv/bin/activate"
                        .to_string(),
                ],
                on_exit: vec![],
            },
        });
    }

    // Check for .nvmrc (Node version manager)
    if project_dir.join(".nvmrc").is_file() {
        detected.push(DetectedFeature {
            feature: "node-nvm".to_string(),
            source: ".nvmrc".to_string(),
            config: DetectedConfig {
                path_prepend: vec![],
                on_enter: vec!["nvm use".to_string()],
                on_exit: vec![],
            },
        });
    }

    // Check for node_modules/.bin (Node project)
    if project_dir.join("node_modules/.bin").is_dir() {
        detected.push(DetectedFeature {
            feature: "node-modules".to_string(),
            source: "node_modules/.bin/".to_string(),
            config: DetectedConfig {
                path_prepend: vec!["./node_modules/.bin".to_string()],
                on_enter: vec![],
                on_exit: vec![],
            },
        });
    }

    // Check for .envrc (direnv)
    if project_dir.join(".envrc").is_file() {
        detected.push(DetectedFeature {
            feature: "direnv".to_string(),
            source: ".envrc".to_string(),
            config: DetectedConfig {
                path_prepend: vec![],
                on_enter: vec![
                    "# .envrc detected - consider using direnv or: source .envrc".to_string(),
                ],
                on_exit: vec![],
            },
        });
    }

    // Check for Cargo.toml (Rust project)
    if project_dir.join("Cargo.toml").is_file() {
        detected.push(DetectedFeature {
            feature: "rust-cargo".to_string(),
            source: "Cargo.toml".to_string(),
            config: DetectedConfig {
                path_prepend: vec!["./target/debug".to_string()],
                on_enter: vec![],
                on_exit: vec![],
            },
        });
    }

    if detected.is_empty() {
        if json {
            output::print_json(&EnvAutoDetectOutput {
                project: project_name.to_string(),
                applied: false,
                detected: vec![],
            });
        } else {
            println!("No environment features detected for project {}", project_name);
        }
        return Ok(());
    }

    // Apply configuration if not dry_run
    if !dry_run {
        // Re-get mutable reference
        let project = registry
            .find_project_mut(project_name)
            .ok_or_else(|| PjmError::ProjectNotFound(project_name.to_string()))?;

        // Ensure metadata and environment exist
        if project.metadata.is_none() {
            project.metadata = Some(projects::ProjectMetadata::default());
        }
        let metadata = project.metadata.as_mut().unwrap();
        if metadata.environment.is_none() {
            metadata.environment = Some(projects::EnvironmentConfig::default());
        }
        let env = metadata.environment.as_mut().unwrap();

        // Initialize vectors if needed
        if env.path_prepend.is_none() {
            env.path_prepend = Some(Vec::new());
        }
        if env.on_enter.is_none() {
            env.on_enter = Some(Vec::new());
        }
        if env.on_exit.is_none() {
            env.on_exit = Some(Vec::new());
        }

        // Apply detected configurations (avoid duplicates)
        let path_prepend = env.path_prepend.as_mut().unwrap();
        let on_enter = env.on_enter.as_mut().unwrap();
        let on_exit = env.on_exit.as_mut().unwrap();

        for feature in &detected {
            for p in &feature.config.path_prepend {
                if !path_prepend.contains(p) {
                    path_prepend.push(p.clone());
                }
            }
            for cmd in &feature.config.on_enter {
                // Skip comment-only suggestions if they start with #
                if !cmd.starts_with('#') && !on_enter.contains(cmd) {
                    on_enter.push(cmd.clone());
                }
            }
            for cmd in &feature.config.on_exit {
                if !on_exit.contains(cmd) {
                    on_exit.push(cmd.clone());
                }
            }
        }

        util::save_config_toml(&registry.ser()?)?;
    }

    if json {
        output::print_json(&EnvAutoDetectOutput {
            project: project_name.to_string(),
            applied: !dry_run,
            detected,
        });
    } else {
        println!(
            "{}",
            format!("Detected environment features for project {}:", project_name).cyan()
        );
        for feature in &detected {
            println!("  {} (from {})", feature.feature.green(), feature.source);
            if !feature.config.path_prepend.is_empty() {
                println!("    Path prepend: {}", feature.config.path_prepend.join(", "));
            }
            if !feature.config.on_enter.is_empty() {
                println!("    On enter: {}", feature.config.on_enter.join("; "));
            }
            if !feature.config.on_exit.is_empty() {
                println!("    On exit: {}", feature.config.on_exit.join("; "));
            }
        }
        if dry_run {
            println!("\n{}", "(dry run - no changes made)".dimmed());
        } else {
            println!("\n{}", "Configuration applied.".green());
        }
    }

    Ok(())
}

/// Generate environment setup script for a project
fn generate_env_setup(project: &projects::ChangeToProject, file_path: &str) -> Option<String> {
    let env = project.metadata.as_ref()?.environment.as_ref()?;

    // Check if there's anything to set up
    let has_vars = env.vars.as_ref().map(|v| !v.is_empty()).unwrap_or(false);
    let has_on_enter = env.on_enter.as_ref().map(|v| !v.is_empty()).unwrap_or(false);
    let has_on_exit = env.on_exit.as_ref().map(|v| !v.is_empty()).unwrap_or(false);
    let has_path_prepend = env.path_prepend.as_ref().map(|v| !v.is_empty()).unwrap_or(false);

    if !has_vars && !has_on_enter && !has_on_exit && !has_path_prepend {
        return None;
    }

    let mut script = String::new();

    // CD to directory
    script.push_str(&format!("cd '{}'\n", file_path));

    // PATH modifications (before other env vars so commands can use updated PATH)
    if let Some(paths) = &env.path_prepend {
        for p in paths {
            // Escape single quotes in path
            let escaped_p = p.replace('\'', "'\\''");
            script.push_str(&format!("export PATH='{}':\"$PATH\"\n", escaped_p));
        }
    }

    // Environment variables
    if let Some(vars) = &env.vars {
        for (k, v) in vars {
            // Escape single quotes in value
            let escaped_v = v.replace('\'', "'\\''");
            script.push_str(&format!("export {}='{}'\n", k, escaped_v));
        }
    }

    // Store on_exit commands for later (shell will eval this when switching projects)
    if let Some(cmds) = &env.on_exit {
        let exit_script = cmds.join("; ");
        let escaped = exit_script.replace('\'', "'\\''");
        script.push_str(&format!("_PJMAI_ON_EXIT='{}'\n", escaped));
    } else {
        // Clear any previous on_exit
        script.push_str("_PJMAI_ON_EXIT=''\n");
    }

    // On-enter commands
    if let Some(cmds) = &env.on_enter {
        for cmd in cmds {
            script.push_str(&format!("{}\n", cmd));
        }
    }

    Some(script)
}
