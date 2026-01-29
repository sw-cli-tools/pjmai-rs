use crate::args;
use crate::error::Result;
use crate::util;
use crate::PjmConfig;
use log::info;

/// The configuration
#[derive(Debug)]
pub struct Config {
    /// The subcommands
    pub command: args::Subcommands,
}

/// Establish the configuration from pre-parsed arguments
pub fn init_with_args(args: args::Args) -> Result<Config> {
    info!("initializing config");

    if args.logging {
        info!("-l args {:?}", args);
    }
    if args.debug {
        info!("-d debug not implemented");
        unimplemented!("debugging not yet implemented");
    }

    // Initialize the global PjmConfig (reads from PJM_CONFIG_DIR env var or uses default)
    let pjm_config = PjmConfig::new();
    info!("using config dir: {}", pjm_config.config_dir);
    util::init_config(pjm_config);

    util::check()?;
    Ok(Config {
        command: args.command,
    })
}
