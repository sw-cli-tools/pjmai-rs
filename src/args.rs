use crate::{ProjectName, ProjectPath};
use clap::{Parser, Subcommand};
use log::info;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// Project Management Tool (first draft) - manage and switch between projects
///
/// for more details:
///  adpj --help
///  chpj --help
///  lspj --help
///  rmpj --help
///  shpj --help
#[derive(Debug, PartialEq, Parser)]
#[command(version = generated_version())]
pub struct Args {
    /// Prints debugging info. -d must precede subcommands
    #[arg(long, short)]
    pub debug: bool,

    /// Logs to a log file. -l must precede subcommands
    #[arg(long, short)]
    pub logging: bool,

    /// The specified command
    #[command(subcommand)]
    pub command: Subcommands,
}

/// Subcommands
#[derive(Debug, PartialEq, Subcommand)]
pub enum Subcommands {
    /// Shows help for aliases: adpj, chpj, hlpj, lspj, and rmpj; alias hlpj
    #[command(name = "aliases")]
    Aliases {},

    /// Adds a new project to the projects configuration (~/.pjm/config.toml); alias adpj
    #[command(name = "add", alias = "a")]
    Add {
        /// Names the project (a short alias for the project)
        #[arg(long, short)]
        project: ProjectName,

        /// File name to be sourced for project (e.g., to set environment variables)
        #[arg(long, short)]
        file_or_dir: ProjectPath,
    },

    /// Changes to the specified project (changes directory or sources file); alias chpj
    #[command(name = "change", alias = "c")]
    Change {
        /// Project to switch to
        #[arg(long, short)]
        project: ProjectName,
    },

    /// Lists the previously added projects; alias lspj
    #[command(name = "list", alias = "l")]
    List {},

    /// Prompt string for previously changed-to project; alias prpj
    #[command(name = "prompt", alias = "p")]
    Prompt {},

    /// Removes a previously created project from the projects configuration; alias rmpj
    #[command(name = "remove", alias = "r")]
    Remove {
        /// Removes project with this name
        #[arg(long, short)]
        project: ProjectName,
    },

    /// Shows the previously changed-to project; alias shpj
    #[command(name = "show", alias = "s")]
    Show {},
}

/// Parse supplied arguments
pub fn parse_args() -> Args {
    info!("parsing args...");
    let r = Args::parse();
    info!("args: {:?}", &r);
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Args {
                command: Subcommands::Add {
                    project: "pjm1".to_string(),
                    file_or_dir: "~/gh/wma/pjm1".to_string(),
                },
                debug: false,
                logging: false,
            },
            Args::try_parse_from(["test", "add", "-p", "pjm1", "-f", "~/gh/wma/pjm1"]).unwrap()
        );
    }
}
