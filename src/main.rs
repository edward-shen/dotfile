#[macro_use]
extern crate clap;
extern crate dirs;
extern crate yaml_rust;

use std::error::Error;
use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use clap::App;

use yaml_rust::YamlLoader;

use dirs::home_dir;

mod config;
mod subcommands;

/// Reads CLI args and loads config file and passes them into the respective
/// subcommand handler.
fn main() -> Result<(), Box<Error>> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(&yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    let dotfile_config = config::dotfile::load_config();
    let dotfile_dir = Path::new(matches.value_of("location").unwrap_or_else(|| ".")).to_path_buf();
    let dotfile_dir_config = load_or_init_config(dotfile_dir)?;

    // println!("{:?}", configs["version"].as_str().unwrap());
    // println!("{:?}", dotfile_config["helper"].is_null());
    // println!("{:?}", matches);

    let params = (dotfile_config, dotfile_dir_config, matches);

    match params.2.subcommand_name().unwrap_or_default() {
        "init" => subcommands::init::handler(params),
        "use" => subcommands::use_cmd::handler(params),
        "add" => subcommands::add::handler(params),
        "remove" => subcommands::remove::handler(params),
        "group" => subcommands::group::handler(params),
        "install" => subcommands::install::handler(params),
        _ => (),
    };

    Ok(())
}

/// Generates and/or loads the config file at the provided path location.
///
/// # Arguments
///
/// * `path` - the path of the dotfile directory. Can be relative or absolute.
fn load_or_init_config(path: PathBuf) -> Result<yaml_rust::Yaml, Box<Error>> {
    let config_path = path.join("./dotfile.yaml");
    let resolved_path = Path::new(&config_path).to_str().unwrap();
    let config = match fs::read_to_string(&resolved_path) {
        Ok(conf) => conf,
        Err(_e) => {
            let init_string = format!("version: {}", crate_version!());
            match fs::write(resolved_path, &init_string) {
                Ok(_ok) => _ok,
                Err(_e) => {
                    create_dir_all(path)?;
                    fs::write(resolved_path, &init_string)?;
                    println!("Generating new configuration file at {}", resolved_path);
                }
            };
            init_string
        }
    };
    let results = &YamlLoader::load_from_str(&config)?[0];
    Ok(results.clone())
}
