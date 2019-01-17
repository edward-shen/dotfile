#[macro_use]
extern crate clap;
extern crate dirs;
extern crate yaml_rust;

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

    // println!("dotfile .config: {:?}", dotfile_config["helper"].is_null());
    // let dotfile_dir = Path::new(matches.value_of("location").unwrap_or_else(|| ".")).to_path_buf();
    // println!("{:?}", dotfile_dir_config["version"].as_str().expect("Could not find version string in local dotfile config"));
    // println!("{:?}", matches);

    let params = (&dotfile_config, &matches);

    match matches.subcommand_name().unwrap_or_default() {
        "init" => subcommands::init::handler(params),
        // "use" => subcommands::use_cmd::handler(params),
        // "add" => subcommands::add::handler(params),
        // "remove" => subcommands::remove::handler(params),
        // "group" => subcommands::group::handler(params),
        // "install" => subcommands::install::handler(params),
        _ => panic!("clap-rs failed to handle invalid input!"),
    }
}

// fn load_config(path: PathBuf) -> Result<yaml_rust::Yaml, Box<Error>> {
//     let config_path = path.join("./dotfile.yaml");
//     let resolved_path = Path::new(&config_path).to_str().unwrap();
//     let config = fs::read_to_string(&resolved_path)?;
//     let results = &YamlLoader::load_from_str(&config)?[0];
//     Ok(results.clone())
// }
