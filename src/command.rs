use colored::Colorize;

use crate::projects;
use crate::util;

pub fn add(project_name: &String, file_name: &String) {
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

pub fn change(project_name: &String) {
    let mut projects = util::projects();
    for project in &projects.project {
        if project.name == project_name.to_string() {
            let file_path = util::expand_file_path(&project.action.file_or_dir);
            if util::is_file_found(&file_path) {
                projects.current_project = project.name.to_string();
                util::save_config_toml(&projects.ser());
                println!("{}", &file_path);
                if util::is_file_dir(&file_path) {
                    std::process::exit(2); // wrapper will cd to the above printed path
                } else {
                    std::process::exit(3); // wrapper will source the above printed path
                }
            } else {
                println!("dir or file '{}' not found", &file_path);
                std::process::exit(4); // wrapper will echo the error
            }
        }
    }
    println!("project '{}' not found", &project_name);
    std::process::exit(4) // wrapper will echo the error
}

pub fn list() {
    let mut projects = util::projects();
    projects.project.sort_by(|a, b| a.name.cmp(&b.name));
    let mut current;
    for project in &projects.project {
        let short_path = util::shorten_path(&project.action.file_or_dir);
        let colored_name;
        let colored_short_path;
        if project.name == projects.current_project {
            current = ">".to_string().italic().green();
            colored_name = project.name.italic().green();
            colored_short_path = short_path.italic().green();
        } else {
            current = " ".to_string().normal();
            colored_name = project.name.normal();
            colored_short_path = short_path.normal();
        }
        println!("{}{:8} {}", current, colored_name, colored_short_path);
    }
}

pub fn remove(unwanted_project_name: &String) {
    let old_projects = util::projects();
    let mut new_projects = projects::ProjectsRegistry::new();
    for project in &old_projects.project {
        if project.name != unwanted_project_name.to_string() {
            new_projects.project.push(projects::ChangeToProject {
                action: projects::Action { file_or_dir: project.action.file_or_dir.to_string() },
                name: project.name.to_string(),
            });
        } else {
            if project.name != old_projects.current_project {
                new_projects.current_project = old_projects.current_project.to_string();
            }
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
