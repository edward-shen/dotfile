#[macro_use]
extern crate clap;
extern crate dirs;
extern crate tokio;

mod config;
mod subcommands;

use std::io::Error;
use std::path::PathBuf;

use dirs::config_dir;

use clap::App;
use clap::ArgMatches;

use crate::config::global::{load_config as load_global_config, GlobalConfig};
use crate::config::local::{load_config as load_local_config, LocalConfig};

#[derive(Debug)]
pub struct Context<'a> {
    global_config_path: PathBuf,
    global_config: GlobalConfig,
    local_config_path: Option<PathBuf>,
    local_config: Option<LocalConfig>,
    matches: ArgMatches<'a>,
}

/// Parses top-level CLI arguments, loads the dotfile config, and then passes
/// the loaded configs into the subcommand handler
fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(&yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    let global_config_path = match matches.value_of("location") {
        Some(e) => PathBuf::from(e),
        None => config_dir().unwrap().join("./dotfile/config.toml"),
    };

    let global_config = load_global_config(&global_config_path);

    let local_config_path = (&global_config.path)
        .clone()
        .and_then(|path| Some(PathBuf::from(path)));

    let local_config = (local_config_path)
        .clone()
        .and_then(|path| Some(load_local_config(&path)));

    let context = Context {
        global_config_path: global_config_path,
        global_config: global_config,
        local_config_path: local_config_path,
        local_config: local_config,
        matches: matches.clone(),
    };

    match matches.subcommand_name().unwrap_or_default() {
        "init" => subcommands::init::handler(context),
        "use" => subcommands::use_cmd::handler(context),
        "add" => subcommands::add::handler(context),
        "remove" => subcommands::remove::handler(context),
        "group" => subcommands::group::handler(context),
        "install" => subcommands::install::handler(context),
        _ => panic!("clap-rs failed to handle invalid input!"),
    }
}
