use std::io::Error;

use crate::config::global::GlobalConfig;
use crate::config::Writable;
use crate::Context;

pub fn handler(context: Context) -> Result<(), Error> {
    let helper = context
        .matches
        .value_of("aur-helper")
        .map(|val| String::from(val));
    let path = context
        .matches
        .value_of("path")
        .map(|val| String::from(val));

    GlobalConfig { helper, path }.write_to_file(&context.global_config_path);

    Ok(())
}
