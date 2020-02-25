use pjm1::args;
use pjm1::command;
use pjm1::config;

fn main() {
    let config = config::init();
    match &config.command {
        args::Subcommands::Add { project, file_or_dir } => command::add(project, file_or_dir),
        args::Subcommands::Aliases { } => command::aliases(),
        args::Subcommands::Change { project } => command::change(project),
        args::Subcommands::List { } => command::list(),
        args::Subcommands::Remove { project } => command::remove(project),
    }
}
