//! Project Management tool
#![deny(warnings, missing_docs)]

use colored::Colorize;
use log::info;
use pjmai_rs::args;
use pjmai_rs::command;
use pjmai_rs::config;
use pjmai_rs::error::Result;

/// Run the application and return a Result
fn run() -> Result<()> {
    // Handle -V/--version before clap parsing (custom output)
    if std::env::args().any(|a| a == "-V" || a == "--version") {
        args::print_version();
        return Ok(());
    }

    // Parse args early to handle completions before config init
    let parsed_args = args::parse_args();

    // Handle completions early (before config init) since they don't need config
    if let args::Subcommands::Completions { shell } = &parsed_args.command {
        args::print_completions(*shell);
        return Ok(());
    }

    let config = config::init_with_args(parsed_args)?;
    let json = config.json;
    let yes = config.yes;
    match &config.command {
        args::Subcommands::Add {
            project,
            file_or_dir,
            description,
            tags,
            language,
            group,
        } => command::add(
            project,
            file_or_dir,
            description.clone(),
            tags.clone(),
            language.clone(),
            group.clone(),
            json,
        )?,
        args::Subcommands::Aliases {} => command::aliases(json),
        args::Subcommands::Change { project, subdirs } => command::change(project, subdirs, json)?,
        args::Subcommands::Complete { target } => command::complete(target)?,
        args::Subcommands::Completions { .. } => unreachable!(), // handled above
        args::Subcommands::Config { action } => match action {
            args::ConfigAction::Export { format } => command::config_export(format, json)?,
            args::ConfigAction::Import {
                file,
                merge,
                dry_run,
            } => command::config_import(file, *merge, *dry_run, json)?,
        },
        args::Subcommands::Env { project, action } => match action {
            args::EnvAction::Set { key, value } => command::env_set(project, key, value, json)?,
            args::EnvAction::Unset { key } => command::env_unset(project, key, json)?,
            args::EnvAction::OnEnter { command: cmd } => {
                command::env_on_enter(project, cmd, json)?
            }
            args::EnvAction::OnExit { command: cmd } => {
                command::env_on_exit(project, cmd, json)?
            }
            args::EnvAction::PathPrepend { path } => {
                command::env_path_prepend(project, path, json)?
            }
            args::EnvAction::PathRemove { path } => {
                command::env_path_remove(project, path, json)?
            }
            args::EnvAction::Show {} => command::env_show(project, json)?,
            args::EnvAction::Clear {} => command::env_clear(project, json)?,
            args::EnvAction::AutoDetect { dry_run } => {
                command::env_auto_detect(project, *dry_run, json)?
            }
        },
        args::Subcommands::Context { project, for_agent } => {
            command::context(project.clone(), *for_agent, json)?
        }
        args::Subcommands::List { tag, group, recent } => {
            command::list(tag.clone(), group.clone(), *recent, json)?
        }
        args::Subcommands::Meta {
            project,
            description,
            language,
            group,
        } => command::meta(project, description.clone(), language.clone(), group.clone(), json)?,
        args::Subcommands::Note { project, action } => command::note(project, action, json)?,
        args::Subcommands::Pop {} => command::pop(json)?,
        args::Subcommands::Stack { action } => match action {
            None | Some(args::StackAction::Show {}) => command::stack_show(json)?,
            Some(args::StackAction::Clear { yes: skip_confirm }) => {
                command::stack_clear(yes || *skip_confirm, json)?
            }
        },
        args::Subcommands::Prompt {} => command::prompt(json)?,
        args::Subcommands::Push { project } => command::push(project, json)?,
        args::Subcommands::Remove {
            project,
            all,
            yes: skip_confirm,
        } => {
            if *all {
                command::remove_all(yes || *skip_confirm, json)?;
            } else {
                command::remove(project.as_deref().unwrap(), json)?;
            }
        }
        args::Subcommands::Rename { from, to } => command::rename(from, to, json)?,
        args::Subcommands::Scan {
            dir,
            depth,
            ignore,
            dry_run,
            add_all,
            reset,
        } => {
            if *reset && !*dry_run {
                command::remove_all(yes, json)?;
            }
            command::scan(dir, *depth, ignore.clone(), *dry_run, *add_all || yes, json)?;
        }
        args::Subcommands::Setup {
            shell,
            shell_only,
            completions_only,
            prompt,
        } => command::setup(*shell, *shell_only, *completions_only, *prompt, json)?,
        args::Subcommands::Show {} => command::show(json)?,
        args::Subcommands::Tag { project, action } => command::tag(project, action, json)?,
        args::Subcommands::History { index } => command::history(index.as_ref(), json)?,
        args::Subcommands::Group { action } => match action {
            args::GroupAction::List { all } => command::group_list(*all, json)?,
            args::GroupAction::Show { name, all } => {
                command::group_show(name.clone(), *all, json)?
            }
            args::GroupAction::Prompt { alias } => command::group_prompt(*alias, json)?,
            args::GroupAction::Alias {
                group,
                alias,
                remove,
                list,
            } => command::group_alias(group.clone(), alias.clone(), *remove, *list, json)?,
        },
    }
    info!(target:"pjmai_rs::main", "finished");
    Ok(())
}

/// Entry point
fn main() {
    env_logger::init();
    info!(target:"pjmai_rs::main", "env_logger initialized");

    if let Err(e) = run() {
        eprintln!("{}: {}", "error".red().bold(), e);
        std::process::exit(1);
    }
}
