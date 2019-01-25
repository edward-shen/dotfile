use std::io::Error;

use crate::config::global::GlobalConfig;
use crate::config::Writable;
use crate::Context;

use crate::subcommands::get_arg_err_msg;

pub fn handler(context: Context) -> Result<(), Error> {
    let args = context
        .matches
        .subcommand_matches("use")
        .expect(&get_arg_err_msg("use"));

    let helper = args.value_of("helper").map(|val| String::from(val));
    let path = args.value_of("path").map(|val| String::from(val));

    GlobalConfig { helper, path }.write_to_file(&context.global_config_path);

    Ok(())
}
