use std::io::Error;

use crate::config::dotfile::Config as GlobalConfig;

pub fn handler((global_config, args): (&GlobalConfig, &clap::ArgMatches)) -> Result<(), Error> {
    Ok(())
}
