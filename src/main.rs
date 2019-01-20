#[macro_use]
extern crate clap;
extern crate dirs;

use std::io::Error;

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

    let dotfile_config = config::dotfile::load_config();
    let params = (&dotfile_config, &matches);

    match matches.subcommand_name().unwrap_or_default() {
        "init" => subcommands::init::handler(params),
        "use" => subcommands::use_cmd::handler(params),
        // "add" => subcommands::add::handler(params),
        // "remove" => subcommands::remove::handler(params),
        // "group" => subcommands::group::handler(params),
        // "install" => subcommands::install::handler(params),
        _ => panic!("clap-rs failed to handle invalid input!"),
    }
}
