use log::info;
use std::env;

use structopt::StructOpt;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    about = "Project Management Tool (first draft) - manage and switch between projects
for more details:
 adpj --help
 chpj --help
 lspj --help
 rmpj --help
 shpj --help
",
    version = generated_version())]
pub struct Args {
    #[structopt(long, short)]
    /// Prints debugging info.  -d must preceed subcommands
    pub debug: bool,
    #[structopt(long, short)]
    /// Logs to a log file.  -l must preceed subcommands
    pub logging: bool,
    #[structopt(subcommand)]
    pub command: Subcommands,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum Subcommands {
    /// Shows help for aliases: adpj, chpj, hlpj, lspj, and rmpj; alias hlpj
    #[structopt(name = "aliases", help("hlpj # shows xxpj aliases help"))]
    Aliases {},
    /// Adds a new project to the projects configuration (~/.pjm/config.toml); alias adpj
    #[structopt(
        name = "add",
        alias = "a",
        help("adpj name -f dir-or-file # cd dir -or- source file, when running chpj name")
    )]
    Add {
        #[structopt(long, short)]
        /// Names the project (a short alias for the project)
        project: String,
        #[structopt(long, short)]
        /// File name to be sourced for project (e.g., to set environment variables)
        file_or_dir: String,
    },
    /// Changes to the specified project (changes directory or sources file); alias chpj
    #[structopt(
        name = "change",
        alias = "c",
        help("chpj name # project to switch to or set up")
    )]
    Change {
        #[structopt(long, short)]
        /// Removes project with this name
        project: String,
    },
    /// Lists the previously added projects; alias lspj
    #[structopt(
        name = "list",
        alias = "l",
        help("lspj # lists all projects added by adpj")
    )]
    List {},
    /// Prompt string for previously changed-to project; alias prpj
    #[structopt(
        name = "prompt",
        alias = "p",
        help("prpj # prompt for current project switched to by chpj")
    )]
    Prompt {},
    /// Removes a previously created project from the projects configuration; alias rmpj
    #[structopt(
        name = "remove",
        alias = "r",
        help("rmpj name # removes named project")
    )]
    Remove {
        #[structopt(long, short)]
        /// Removes project with this name
        project: String,
    },
    /// Shows the previously changed-to project; alias shpj
    #[structopt(
        name = "show",
        alias = "s",
        help("shpj # show current project switched to by chpj")
    )]
    Show {},
}
pub fn parse_args() -> Args {
    info!("parsing args...");
    let r = Args::from_args();
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
            Args::from_iter(&["test", "add", "-p", "pjm1", "-f", "~/gh/wma/pjm1"])
        );
    }
}
