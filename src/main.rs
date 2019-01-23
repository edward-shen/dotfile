#[macro_use]
extern crate clap;
extern crate dirs;
extern crate tokio;

use std::io::Error;
use std::path::PathBuf;

use dirs::config_dir;

use clap::App;

mod config;
mod subcommands;

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

    let global_config = config::dotfile::load_config(&global_config_path);

    let params = (&global_config_path, &global_config, &matches);

    match matches.subcommand_name().unwrap_or_default() {
        "init" => subcommands::init::handler(params),
        "use" => subcommands::use_cmd::handler(params),
        "add" => subcommands::add::handler(params),
        // "remove" => subcommands::remove::handler(params),
        "group" => subcommands::group::handler(params),
        "install" => subcommands::install::handler(params),
        _ => panic!("clap-rs failed to handle invalid input!"),
    }
}
