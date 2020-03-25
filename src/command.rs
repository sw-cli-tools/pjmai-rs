use colored::Colorize;

use crate::projects;
use crate::util;

pub fn add(project_name: &str, file_name: &str) {
    let mut projects = util::projects();
    projects.project.push(projects::ChangeToProject {
        action: projects::Action { file_or_dir: file_name.to_string() },
        name: project_name.to_string(),
    });
    if projects.project.len() == 1 {
        projects.current_project = project_name.to_string();
    }
    util::save_config_toml(&projects.ser());
}

pub fn aliases() {
    println!("adpj <name> -f <dir|file> # alias for pjm1 add");
    println!("chpj <name>               # alias for pjm1 change");
    println!("hlpj                      # alias for pjm1 aliases");
    println!("lspj                      # alias for pjm1 list");
    println!("rmpj <name>               # alias for pjm1 remove");
    println!("shpj                      # alias for pjm1 show");
}

pub fn change(project_name: &str) {
    let mut projects = util::projects();
    for project in &projects.project {
        if project.name == project_name {
            let file_path = util::expand_file_path(&project.action.file_or_dir);
            if util::is_file_found(&file_path) {
                projects.current_project = project.name.to_string();
                util::save_config_toml(&projects.ser());
                print!("{}", &file_path); // path parameter for bash cd or source command
                if util::is_file_dir(&file_path) {
                    std::process::exit(2); // bash wrapper will cd to the above printed path
                } else {
                    std::process::exit(3); // bash wrapper will source the above printed path
                }
            } else {
                println!("dir or file '{}' not found", &file_path);
                std::process::exit(4); // bash wrapper will echo the error
            }
        }
    }
    println!("project '{}' not found", &project_name);
    std::process::exit(4) // bash wrapper will echo the error
}

pub fn list() {
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
}

pub fn remove(unwanted_project_name: &str) {
    let old_projects = util::projects();
    let mut new_projects = projects::ProjectsRegistry::new();
    for project in &old_projects.project {
        if project.name != unwanted_project_name {
            new_projects.project.push(projects::ChangeToProject {
                action: projects::Action { file_or_dir: project.action.file_or_dir.to_string() },
                name: project.name.to_string(),
            });
        } else if project.name != old_projects.current_project {
                new_projects.current_project = old_projects.current_project.to_string();
        }
    }
    util::save_config_toml(&new_projects.ser());
}

pub fn show() {
    let projects = util::projects();
    for project in &projects.project {
        if project.name == projects.current_project {
            println!("{}", format!(">{:8} {}",
                                   project.name,
                                   util::shorten_path(&project.action.file_or_dir)).green());
        }
    }
}
