use crate::args;
use crate::util;
use log::info;

#[derive(Debug)]
pub struct Config {
    pub command: args::Subcommands,
}

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
    util::check();
    Config {
        command: args.command,
    }
}
