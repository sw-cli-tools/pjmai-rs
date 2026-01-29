use crate::error::{PjmError, Result};
use crate::projects;
use crate::util;
use colored::Colorize;
use log::info;

/// Add a project
pub fn add(project_name: &str, file_name: &str) -> Result<()> {
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
    info!("add done");
    Ok(())
}

/// List aliases
pub fn aliases() {
    info!("aliases");
    println!("adpj <name> -f <dir|file> # alias for pjm1 add");
    println!("chpj <name>               # alias for pjm1 change");
    println!("hlpj                      # alias for pjm1 aliases");
    println!("lspj                      # alias for pjm1 list");
    println!("prpj                      # alias for pjm1 prompt");
    println!("rmpj <name>               # alias for pjm1 remove");
    println!("shpj                      # alias for pjm1 show");
    info!("aliases done");
}

/// Changes to the specified project
pub fn change(project_name: &str) -> Result<()> {
    info!("changing to project {}", &project_name);
    let mut registry = util::projects()?;

    // Find matching project(s) using fuzzy matching
    let matched = find_matching_project(project_name, &registry);

    match matched {
        MatchResult::Exact(project) | MatchResult::Unique(project) => {
            let file_path = util::expand_file_path(&project.action.file_or_dir);
            if util::is_file_found(&file_path) {
                registry.current_project = project.name.to_string();
                util::save_config_toml(&registry.ser()?)?;
                print!("{}", &file_path); // path parameter for bash cd or source command
                if util::is_file_dir(&file_path) {
                    info!("change done 2");
                    std::process::exit(2); // bash wrapper will cd to the above printed path
                } else {
                    info!("change done 3");
                    std::process::exit(3); // bash wrapper will source the above printed path
                }
            } else {
                println!("dir or file '{}' not found", &file_path);
                info!("change done 4a");
                std::process::exit(4); // bash wrapper will echo the error
            }
        }
        MatchResult::Ambiguous(matches) => {
            println!(
                "ambiguous project name '{}', matches: {}",
                project_name,
                matches.join(", ")
            );
            info!("change done 4c");
            std::process::exit(4);
        }
        MatchResult::None => {
            println!("project '{}' not found", &project_name);
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
pub fn list() -> Result<()> {
    info!("listing all projects");
    let mut registry = util::projects()?;
    registry.project.sort_by(|a, b| a.name.cmp(&b.name));
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
    info!("listing done");
    Ok(())
}

/// Return the prompt indicator for the current project
pub fn prompt() -> Result<()> {
    info!("prompt");
    let registry = util::projects()?;
    if !registry.current_project.is_empty() {
        println!("{}", registry.current_project);
    }
    info!("prompt done");
    Ok(())
}

/// Remove the specified project
pub fn remove(unwanted_project_name: &str) -> Result<()> {
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
    }
    info!("remove done");
    Ok(())
}

/// Show the name and path for the current project
pub fn show() -> Result<()> {
    info!("showing current project");
    let registry = util::projects()?;
    for project in &registry.project {
        if project.name == registry.current_project {
            info!("current {}", project.name);
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
