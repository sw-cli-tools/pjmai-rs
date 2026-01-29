use crate::args;
use crate::util;
use crate::PjmConfig;
use log::info;

/// The configuration
#[derive(Debug)]
pub struct Config {
    /// The subcommands
    pub command: args::Subcommands,
}

/// Establish the configuration
pub fn init() -> Config {
    info!("initializing config");
    let args = args::parse_args();

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

    util::check();
    Config {
        command: args.command,
    }
}
