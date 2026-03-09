use crate::PjmConfig;
use crate::args;
use crate::error::Result;
use crate::util;
use colored::Colorize;
use log::info;

/// The configuration
#[derive(Debug)]
pub struct Config {
    /// The subcommands
    pub command: args::Subcommands,
    /// Output in JSON format
    pub json: bool,
    /// Assume yes to all prompts
    pub yes: bool,
}

/// Establish the configuration from pre-parsed arguments
pub fn init_with_args(args: args::Args) -> Result<Config> {
    info!("initializing config");

    if args.logging {
        info!("-l args {:?}", args);
    }

    // Initialize the global PjmConfig (reads from PJMAI_CONFIG_DIR env var or uses default)
    let pjm_config = PjmConfig::new();
    info!("using config dir: {}", pjm_config.config_dir);

    // Print debug info before initializing config (if debug flag is set)
    if args.debug {
        print_debug_info(&pjm_config);
    }

    util::init_config(pjm_config);

    util::check(args.yes)?;
    Ok(Config {
        command: args.command,
        json: args.json,
        yes: args.yes,
    })
}

/// Print debug information to stderr
fn print_debug_info(pjm_config: &PjmConfig) {
    eprintln!("{}", "=== PJMAI Debug Info ===".cyan().bold());

    // Version
    eprintln!("{}: {}", "Version".cyan(), crate::args::VERSION);

    // Environment
    eprintln!("{}", "\n--- Environment ---".cyan());
    if let Ok(val) = std::env::var("PJMAI_CONFIG_DIR") {
        eprintln!("{}: {}", "PJMAI_CONFIG_DIR".cyan(), val);
    } else {
        eprintln!("{}: {}", "PJMAI_CONFIG_DIR".cyan(), "(not set)".dimmed());
    }
    if let Ok(val) = std::env::var("SHELL") {
        eprintln!("{}: {}", "SHELL".cyan(), val);
    }
    if let Ok(val) = std::env::var("HOME") {
        eprintln!("{}: {}", "HOME".cyan(), val);
    }

    // Config paths
    eprintln!("{}", "\n--- Configuration ---".cyan());
    eprintln!("{}: {}", "Config directory".cyan(), &pjm_config.config_dir);
    let config_file = pjm_config.config_file_path();
    eprintln!("{}: {}", "Config file".cyan(), &config_file);

    let config_exists = std::path::Path::new(&config_file).exists();
    if config_exists {
        eprintln!("{}: {}", "Config file exists".cyan(), "yes".green());

        // Try to read project info
        if let Ok(contents) = std::fs::read_to_string(&config_file)
            && let Ok(registry) = toml::from_str::<crate::projects::ProjectsRegistry>(&contents)
        {
            eprintln!("{}", "\n--- Projects ---".cyan());
            eprintln!("{}: {}", "Total projects".cyan(), registry.project.len());
            if !registry.current_project.is_empty() {
                eprintln!(
                    "{}: {}",
                    "Current project".cyan(),
                    registry.current_project.green()
                );
            } else {
                eprintln!("{}: {}", "Current project".cyan(), "(none)".dimmed());
            }

            // List projects
            if !registry.project.is_empty() {
                eprintln!("{}", "\n--- Project List ---".cyan());
                for project in &registry.project {
                    let marker = if project.name == registry.current_project {
                        ">".green()
                    } else {
                        " ".normal()
                    };
                    eprintln!(
                        "{} {}: {}",
                        marker,
                        project.name.yellow(),
                        project.action.file_or_dir
                    );
                }
            }
        }
    } else {
        eprintln!(
            "{}: {}",
            "Config file exists".cyan(),
            "no (will be created)".yellow()
        );
    }

    eprintln!("{}", "\n========================".cyan().bold());
}
