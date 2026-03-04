//! Project Management tool
#![deny(warnings, missing_docs)]

use colored::Colorize;
use log::info;
use pjmai::args;
use pjmai::command;
use pjmai::config;
use pjmai::error::Result;

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
    let json = config.json;
    match &config.command {
        args::Subcommands::Add {
            project,
            file_or_dir,
        } => command::add(project, file_or_dir, json)?,
        args::Subcommands::Aliases {} => command::aliases(json),
        args::Subcommands::Change { project } => command::change(project, json)?,
        args::Subcommands::Complete { target } => command::complete(target)?,
        args::Subcommands::Completions { .. } => unreachable!(), // handled above
        args::Subcommands::List {} => command::list(json)?,
        args::Subcommands::Prompt {} => command::prompt(json)?,
        args::Subcommands::Remove { project } => command::remove(project, json)?,
        args::Subcommands::Rename { from, to } => command::rename(from, to, json)?,
        args::Subcommands::Scan {
            dir,
            depth,
            ignore,
            dry_run,
            add_all,
        } => command::scan(dir, *depth, ignore.clone(), *dry_run, *add_all, json)?,
        args::Subcommands::Setup {
            shell,
            shell_only,
            completions_only,
        } => command::setup(*shell, *shell_only, *completions_only, json)?,
        args::Subcommands::Show {} => command::show(json)?,
    }
    info!(target:"pjmai::main", "finished");
    Ok(())
}

/// Entry point
fn main() {
    env_logger::init();
    info!(target:"pjmai::main", "env_logger initialized");

    if let Err(e) = run() {
        eprintln!("{}: {}", "error".red().bold(), e);
        std::process::exit(1);
    }
}
