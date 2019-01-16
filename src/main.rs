#[macro_use]
extern crate clap;
extern crate dirs;
extern crate yaml_rust;

use clap::App;

use std::error::Error;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;
use yaml_rust::YamlLoader;

mod subcommands;

/// Reads CLI args and loads config file and passes them into the respective
/// subcommand handler.
fn main() -> Result<(), Box<Error>> {
    let yaml = load_yaml!("cli.yaml");
    let mut app = App::from_yaml(&yaml).version(crate_version!()).author(crate_authors!());
    let matches = app.clone().get_matches();

    let dotfile_config = load_or_init_settings()?;
    let dotfile_dir = parse_config_dir(&matches);
    let dotfile_dir_config = load_or_init_config(&dotfile_dir)?;

    // println!("{:?}", configs["version"].as_str().unwrap());
    // println!("{:?}", matches);

    let params = (dotfile_config, dotfile_dir_config, matches);

    match params.2.subcommand_name().unwrap_or_default() {
        "init" => subcommands::init::handler(params),
        "use" => subcommands::use_cmd::handler(params),
        "add" => subcommands::add::handler(params),
        "remove" => subcommands::remove::handler(params),
        "group" => subcommands::group::handler(params),
        "install" => subcommands::install::handler(params),
        _ => app.print_help()?,
    };

    Ok(())
}

fn load_or_init_settings() -> Result<yaml_rust::Yaml, Box<Error>> {
    let config_dir = dirs::home_dir().expect("Home dir not found!");
    let config_dir = config_dir.to_str().expect("Can't stringify home dir!");
    let config = fs::read_to_string(config_dir.to_owned() + "/.config/dotfile/config.yaml")?;
    Ok(YamlLoader::load_from_str(&config)?[0].clone())
}

/// Parses the input args and attempts to locate the config directory.
///
/// Returns the expanded location path.
///
/// # Arguments
///
/// * `matches` - The matches object obtained from the clap library.
///
fn parse_config_dir(matches: &clap::ArgMatches) -> String {
    // Should never panic because Location has a default value.
    let config_dir = matches
        .value_of("location")
        .expect("Location not provided!");

    // Since the default args uses ~, which is a shell-dependent feature, we
    // need to substitute ~ into the home directory. The dirs package is a
    // platform-independent solution.
    config_dir.replace(
        "~",
        dirs::home_dir()
            .expect("Could not find home dir!")
            .to_str()
            .expect("Failed to stringify home dir!"),
    )
}

/// Generates and/or loads the config file at the provided path location.
///
/// # Arguments
///
/// * `path` - the path of the dotfile directory. Can be relative or absolute.
fn load_or_init_config(path: &str) -> Result<yaml_rust::Yaml, Box<Error>> {
    let path = path.to_owned();
    let config_path = path.clone() + "/dotfile.yaml";
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
