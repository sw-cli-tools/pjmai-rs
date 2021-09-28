use log::info;
use pjm1::args;
use pjm1::command;
use pjm1::config;

fn main() {
    env_logger::init();
    info!(target:"pjm1::main", "env_logger initialized");
    let config = config::init();
    match &config.command {
        args::Subcommands::Add {
            project,
            file_or_dir,
        } => command::add(project, file_or_dir),
        args::Subcommands::Aliases {} => command::aliases(),
        args::Subcommands::Change { project } => command::change(project),
        args::Subcommands::List {} => command::list(),
        args::Subcommands::Prompt {} => command::prompt(),
        args::Subcommands::Remove { project } => command::remove(project),
        args::Subcommands::Show {} => command::show(),
    }
    info!(target:"pjm1::main", "finished");
}
