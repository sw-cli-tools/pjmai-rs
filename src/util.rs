use crate::io;
use crate::projects;
use crate::{ProjectPath, SerializedConfig};
use log::info;

pub fn check() {
    info!("create or use config file (or quit)");
    if !is_file_found(&projects_file_path()) {
        info!("config file not found");
        if prompt_create_yes_no() {
            info!("creating config file");
            save_config_toml(&initial_config_toml());
        } else {
            info!("cannot continue without config file");
            std::process::exit(0);
        }
    }
    info!("using config file at {}", &projects_file_path());
}

pub fn expand_file_path(file_path: &str) -> ProjectPath {
    info!("expand file path {}", &file_path);
    if file_path.starts_with('~') {
        let home = std::env::var("HOME").unwrap();
        let new_file_path = &file_path.to_string().replacen('~', &home, 1);
        new_file_path.to_string()
    } else {
        file_path.to_string()
    }
}

pub fn is_file_dir(file_path: &str) -> bool {
    info!("is file dir? {}", &file_path);
    std::fs::metadata(file_path).unwrap().is_dir()
}

pub fn is_file_found(file_path: &str) -> bool {
    info!("is file found? {}", &file_path);
    std::path::Path::new(&file_path).exists()
}

pub fn projects() -> projects::ProjectsRegistry {
    info!("projects");
    projects::ProjectsRegistry::deser(projects_file_contents())
}

pub fn save_config_toml(projects_string: &str) {
    info!("save config toml");
    match io::write(projects_string, &projects_file_path()) {
        Ok(()) => (),
        Err(e) => panic!("unable to write file, e={}", e),
    }
}

pub fn shorten_path(long_path: &str) -> String {
    info!("shorten_path {}", &long_path);
    let short_path;
    let home = std::env::var("HOME").unwrap_or_else(|_| "".to_string());
    if long_path.starts_with(&home) {
        let mut skip = home.len();
        let mut result = vec![];
        for char in long_path.chars() {
            if skip > 0 {
                skip -= 1;
            } else {
                result.push(char);
            }
        }
        short_path = "~".to_string() + &result.iter().collect::<ProjectPath>();
    } else {
        short_path = long_path.to_string();
    }
    short_path
}

fn env_home() -> String {
    info!("env_home");
    match std::env::var("HOME") {
        Ok(home) => home,
        Err(e) => panic!("couldn't read environment variable HOME, e={}", e),
    }
}

fn initial_config_toml() -> SerializedConfig {
    info!("initial config toml");
    projects::ProjectsRegistry::new().ser()
}

fn projects_file_contents() -> SerializedConfig {
    info!("projects file contents");
    match io::read(projects_file_path()) {
        Ok(projects_string) => projects_string,
        Err(e) => panic!("unable to read projects, e={}", e),
    }
}

fn projects_file_path() -> String {
    info!("projects file path");
    format!("{}/.pjm/config.toml", env_home())
}

fn prompt_create_yes_no() -> bool {
    info!("create yes/no");
    use std::io::{stdin, stdout, Write};

    print!("create Y/n> ");
    stdout().flush().expect("flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("read stdin");
    matches!(input.as_ref(), "Y\n" | "y\n" | "\n")
}
