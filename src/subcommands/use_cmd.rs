use std::io::Error;
use std::path::PathBuf;

use crate::config::dotfile::{update_config as update_global_config, Config};

pub fn handler(
    (global_config_path, _, args): (&PathBuf, &Config, &clap::ArgMatches),
) -> Result<(), Error> {
    let helper = args.value_of("aur-helper").map(|val| String::from(val));
    let path = args.value_of("path").map(|val| String::from(val));

    update_global_config(Config { helper, path }, global_config_path);

    Ok(())
}
