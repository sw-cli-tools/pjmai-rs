use crate::error::{PjmError, Result};
use crate::output::{
    self, AliasOutput, AliasesOutput, ChangeOutput, ErrorOutput, ListOutput, ProjectOutput,
    PromptOutput, ShowOutput, SuccessOutput,
};
use crate::projects;
use crate::util;
use colored::Colorize;
use log::info;

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
