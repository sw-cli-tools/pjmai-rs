//! Project Management tool
#![deny(warnings, missing_docs)]

use colored::Colorize;
use log::info;
use pjm1::args;
use pjm1::command;
use pjm1::config;
use pjm1::error::Result;

/// Run the application and return a Result
fn run() -> Result<()> {
    // Parse args early to handle completions before config init
    let parsed_args = args::parse_args();

    // Handle completions early (before config init) since they don't need config
    if let args::Subcommands::Completions { shell } = &parsed_args.command {
        args::print_completions(*shell);
        return Ok(());
    }

    let config = config::init_with_args(parsed_args)?;
    match &config.command {
        args::Subcommands::Add {
            project,
            file_or_dir,
        } => command::add(project, file_or_dir)?,
        args::Subcommands::Aliases {} => command::aliases(),
        args::Subcommands::Change { project } => command::change(project)?,
        args::Subcommands::Completions { .. } => unreachable!(), // handled above
        args::Subcommands::List {} => command::list()?,
        args::Subcommands::Prompt {} => command::prompt()?,
        args::Subcommands::Remove { project } => command::remove(project)?,
        args::Subcommands::Show {} => command::show()?,
    }
    info!(target:"pjm1::main", "finished");
    Ok(())
}

/// Entry point
fn main() {
    env_logger::init();
    info!(target:"pjm1::main", "env_logger initialized");

    if let Err(e) = run() {
        eprintln!("{}: {}", "error".red().bold(), e);
        std::process::exit(1);
    }
}
