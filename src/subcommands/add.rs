use std::io::Error;
use std::path::PathBuf;

use crate::config::dotfile::Config as GlobalConfig;

pub fn handler(
    (global_config_path, _, args): (&PathBuf, &GlobalConfig, &clap::ArgMatches),
) -> Result<(), Error> {
    dbg!(global_config_path);
    dbg!(args);
    unimplemented!("Add command is complicated, so this will be done later")
}
