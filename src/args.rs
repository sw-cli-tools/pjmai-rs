use crate::{ProjectName, ProjectPath};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use log::info;
use std::io;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// Project Management Tool (AI enhanced) - manage and switch between projects
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

    /// Output in JSON format for machine parsing
    #[arg(long, short = 'j')]
    pub json: bool,

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
    /// Shows help for aliases: adpj, chpj, hlpj, lspj, prpj, rmpj, shpj; alias hlpj
    #[command(name = "aliases")]
    Aliases {},

    /// Adds a new project to the projects configuration (~/.pjmai/config.toml); alias adpj
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

    /// Generate shell completions for bash, zsh, fish, elvish, or powershell
    #[command(name = "completions")]
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Fast completion helper for shells (outputs matching names, one per line)
    #[command(name = "complete")]
    Complete {
        /// What to complete
        #[command(subcommand)]
        target: CompleteTarget,
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

    /// Configure shell integration (adds to shell rc file and installs completions)
    #[command(name = "setup")]
    Setup {
        /// Target shell (auto-detected if not specified)
        #[arg(value_enum)]
        shell: Option<Shell>,

        /// Only install shell integration (source-pjm.sh)
        #[arg(long, conflicts_with = "completions_only")]
        shell_only: bool,

        /// Only install shell completions
        #[arg(long, conflicts_with = "shell_only")]
        completions_only: bool,
    },

    /// Scan directories for git repositories and add them as projects; alias scpj
    #[command(name = "scan")]
    Scan {
        /// Starting directory to scan (default: ~/)
        #[arg(default_value = "~")]
        dir: String,

        /// Maximum depth to recurse (default: 3)
        #[arg(long, default_value = "3")]
        depth: usize,

        /// Comma-separated directory names to skip (in addition to .gitignore)
        #[arg(long, value_delimiter = ',')]
        ignore: Option<Vec<String>>,

        /// Show what would be found without adding anything
        #[arg(long)]
        dry_run: bool,

        /// Add all found projects without confirmation
        #[arg(long)]
        add_all: bool,
    },
}

/// Completion targets for the `complete` subcommand
#[derive(Debug, PartialEq, Subcommand)]
pub enum CompleteTarget {
    /// Complete project names
    #[command(name = "projects")]
    Projects {
        /// Optional prefix to filter projects
        prefix: Option<String>,
    },

    /// Complete command names
    #[command(name = "commands")]
    Commands {
        /// Optional prefix to filter commands
        prefix: Option<String>,
    },
}

/// Generate shell completions to stdout
pub fn print_completions(shell: Shell) {
    let mut cmd = Args::command();
    generate(shell, &mut cmd, "pjmai", &mut io::stdout());
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
                    project: "pjmai".to_string(),
                    file_or_dir: "~/gh/wma/pjmai".to_string(),
                },
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "add", "-p", "pjmai", "-f", "~/gh/wma/pjmai"]).unwrap()
        );
    }

    #[test]
    fn test_add_with_alias() {
        assert_eq!(
            Args {
                command: Subcommands::Add {
                    project: "myproject".to_string(),
                    file_or_dir: "/tmp/project".to_string(),
                },
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "a", "-p", "myproject", "-f", "/tmp/project"]).unwrap()
        );
    }

    #[test]
    fn test_change() {
        assert_eq!(
            Args {
                command: Subcommands::Change {
                    project: "myproject".to_string(),
                },
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "change", "-p", "myproject"]).unwrap()
        );
    }

    #[test]
    fn test_change_with_alias() {
        assert_eq!(
            Args {
                command: Subcommands::Change {
                    project: "foo".to_string(),
                },
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "c", "-p", "foo"]).unwrap()
        );
    }

    #[test]
    fn test_list() {
        assert_eq!(
            Args {
                command: Subcommands::List {},
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "list"]).unwrap()
        );
    }

    #[test]
    fn test_list_with_alias() {
        assert_eq!(
            Args {
                command: Subcommands::List {},
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "l"]).unwrap()
        );
    }

    #[test]
    fn test_remove() {
        assert_eq!(
            Args {
                command: Subcommands::Remove {
                    project: "oldproject".to_string(),
                },
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "remove", "-p", "oldproject"]).unwrap()
        );
    }

    #[test]
    fn test_remove_with_alias() {
        assert_eq!(
            Args {
                command: Subcommands::Remove {
                    project: "bar".to_string(),
                },
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "r", "-p", "bar"]).unwrap()
        );
    }

    #[test]
    fn test_show() {
        assert_eq!(
            Args {
                command: Subcommands::Show {},
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "show"]).unwrap()
        );
    }

    #[test]
    fn test_show_with_alias() {
        assert_eq!(
            Args {
                command: Subcommands::Show {},
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "s"]).unwrap()
        );
    }

    #[test]
    fn test_prompt() {
        assert_eq!(
            Args {
                command: Subcommands::Prompt {},
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "prompt"]).unwrap()
        );
    }

    #[test]
    fn test_prompt_with_alias() {
        assert_eq!(
            Args {
                command: Subcommands::Prompt {},
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "p"]).unwrap()
        );
    }

    #[test]
    fn test_aliases() {
        assert_eq!(
            Args {
                command: Subcommands::Aliases {},
                debug: false,
                json: false,
                logging: false,
            },
            Args::try_parse_from(["test", "aliases"]).unwrap()
        );
    }

    #[test]
    fn test_logging_flag() {
        assert_eq!(
            Args {
                command: Subcommands::List {},
                debug: false,
                json: false,
                logging: true,
            },
            Args::try_parse_from(["test", "-l", "list"]).unwrap()
        );
    }

    #[test]
    fn test_long_logging_flag() {
        assert_eq!(
            Args {
                command: Subcommands::List {},
                debug: false,
                json: false,
                logging: true,
            },
            Args::try_parse_from(["test", "--logging", "list"]).unwrap()
        );
    }

    #[test]
    fn test_debug_flag_present() {
        let args = Args::try_parse_from(["test", "-d", "list"]).unwrap();
        assert!(args.debug);
    }

    #[test]
    fn test_json_flag() {
        assert_eq!(
            Args {
                command: Subcommands::List {},
                debug: false,
                json: true,
                logging: false,
            },
            Args::try_parse_from(["test", "--json", "list"]).unwrap()
        );
    }

    #[test]
    fn test_json_short_flag() {
        assert_eq!(
            Args {
                command: Subcommands::List {},
                debug: false,
                json: true,
                logging: false,
            },
            Args::try_parse_from(["test", "-j", "list"]).unwrap()
        );
    }
}
