use crate::io;
use crate::projects;

pub fn check() {
    if !is_file_found(&projects_file_path()) {
        if prompt_create_yes_no() {
            save_config_toml(&initial_config_toml());
        } else {
            std::process::exit(0);
        }
    }
}
pub fn expand_file_path(file_path: &String) -> String {
    if file_path.starts_with("~") {
        let home = std::env::var("HOME").unwrap();
        let new_file_path = &file_path.to_string().replacen("~", &home.to_string(), 1).to_string();
        new_file_path.to_string()
    } else {
        file_path.to_string()
    }
}

pub fn is_file_dir(file_path: &str) -> bool {
    std::fs::metadata(file_path).unwrap().is_dir()
}

pub fn is_file_found(file_path: &str) -> bool {
    std::path::Path::new(&file_path).exists()
}

pub fn projects() -> projects::ProjectsRegistry {
    projects::ProjectsRegistry::deser(projects_file_contents())
}

pub fn save_config_toml(projects_string: &String) {
    match io::write(projects_string, &projects_file_path()) {
        Ok(()) => (),
        Err(e) => panic!("unable to write file, e={}", e),
    }
}


pub fn shorten_path(long_path: &String) -> String {
    let short_path;
    let home = std::env::var("HOME").unwrap_or("".to_string());
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
        short_path = "~".to_string() + &result.iter().collect::<String>().to_string();
    } else {
        short_path = long_path.to_string();
    }
    short_path.to_string()
}

fn env_home() -> String {
    match std::env::var("HOME") {
        Ok(home) => return home.to_string(),
        Err(e) => panic!("couldn't read environment variable HOME, e={}", e),
    }
}

fn initial_config_toml() -> String {
    projects::ProjectsRegistry::new().ser()
}

fn projects_file_contents() -> String {
    match io::read(projects_file_path()) {
        Ok(projects_string) => projects_string,
        Err(e) => panic!("unable to read projects, e={}", e),
    }
}

fn projects_file_path() -> String {
    format!("{}/.pjm/config.toml", env_home())    
}

fn prompt_create_yes_no() -> bool {
    use std::io::{stdin, stdout, Write};

    print!("create Y/n> ");
    stdout().flush().expect("flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("read stdin");
    match input.as_ref() {
        "Y\n" | "y\n" | "\n" => true,
        _ => false,
    }
}

