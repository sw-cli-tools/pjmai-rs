use crate::projects;
use crate::util;
use colored::Colorize;
use log::info;

/// Add a project
pub fn add(project_name: &str, file_name: &str) {
    info!("adding {} -f {}", &project_name, &file_name);
    let mut projects = util::projects();
    if is_dup(project_name) {
        info!("adding dup failed");
        panic!("cannot add duplicate project name {}", &project_name);
    }
    info!("push");
    projects.project.push(projects::ChangeToProject {
        action: projects::Action {
            file_or_dir: file_name.to_string(),
        },
        name: project_name.to_string(),
    });
    if projects.project.len() == 1 {
        info!("switch to only project");
        projects.current_project = project_name.to_string();
    }
    util::save_config_toml(&projects.ser());
    info!("add done");
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
pub fn change(project_name: &str) {
    info!("changing to project {}", &project_name);
    let mut projects = util::projects();
    for project in &projects.project {
        if project.name == project_name {
            let file_path = util::expand_file_path(&project.action.file_or_dir);
            if util::is_file_found(&file_path) {
                projects.current_project = project.name.to_string();
                util::save_config_toml(&projects.ser());
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
    }
    println!("project '{}' not found", &project_name);
    info!("change done 4b");
    std::process::exit(4) // bash wrapper will echo the error
}

/// Lists known projects
pub fn list() {
    info!("listing all projects");
    let mut projects = util::projects();
    projects.project.sort_by(|a, b| a.name.cmp(&b.name));
    for project in &projects.project {
        let short_path = util::shorten_path(&project.action.file_or_dir);
        let colored_name = if project.name == projects.current_project {
            project.name.italic().green()
        } else {
            project.name.normal()
        };
        let colored_short_path = if project.name == projects.current_project {
            short_path.italic().green()
        } else {
            short_path.normal()
        };
        let current = if project.name == projects.current_project {
            ">".to_string().italic().green()
        } else {
            " ".to_string().normal()
        };
        println!("{}{:8} {}", current, colored_name, colored_short_path);
    }
    info!("listing done");
}

/// Return the prompt indicator for the current project
pub fn prompt() {
    info!("prompt");
    let projects = util::projects();
    for project in &projects.project {
        if project.name == projects.current_project {
            println!("{}", project.name.to_string());
        }
    }
    info!("prompt done");
}

/// Remove the specified project
pub fn remove(unwanted_project_name: &str) {
    info!("remove {}", &unwanted_project_name);
    let mut dirty = false;
    let old_projects = util::projects();
    let mut new_projects = projects::ProjectsRegistry::new();
    for project in &old_projects.project {
        if project.name != unwanted_project_name {
            info!("keeping {}", &project.name);
            new_projects.project.push(projects::ChangeToProject {
                action: projects::Action {
                    file_or_dir: project.action.file_or_dir.to_string(),
                },
                name: project.name.to_string(),
            });
        } else if project.name != old_projects.current_project {
            info!("discarding {}", &project.name);
            dirty = true;
            new_projects.current_project = old_projects.current_project.to_string();
        }
    }
    if dirty {
        info!("saving changes");
        util::save_config_toml(&new_projects.ser());
    }
    info!("remove done");
}

/// Show the name and path for the current project
pub fn show() {
    info!("showing current project");
    let projects = util::projects();
    for project in &projects.project {
        if project.name == projects.current_project {
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
}

fn is_dup(check_project_name: &str) -> bool {
    info!("is_dup {}", &check_project_name);
    for project in &util::projects().project {
        if project.name == check_project_name {
            return true;
        }
    }
    false
}
