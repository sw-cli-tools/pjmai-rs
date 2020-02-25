use crate::args;
use crate::util;

#[derive(Debug)]
pub struct Config {
    pub command: args::Subcommands,
}

pub fn init() -> Config {
    let args = args::parse_args();
    if args.debug {
        unimplemented!("debugging not yet implemented");
    }
    if args.logging {
        unimplemented!("logging not yet implemented");
    }
    util::check();
    Config { command: args.command, }
}
