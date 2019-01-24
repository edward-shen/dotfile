use std::io::Error;
use std::path::PathBuf;

use crate::config::global::GlobalConfig;
use crate::config::Writable;

pub fn handler(
    (global_config_path, _, args): (&PathBuf, &GlobalConfig, &clap::ArgMatches),
) -> Result<(), Error> {
    let helper = args.value_of("aur-helper").map(|val| String::from(val));
    let path = args.value_of("path").map(|val| String::from(val));

    GlobalConfig { helper, path }.write_to_file(global_config_path);

    Ok(())
}
