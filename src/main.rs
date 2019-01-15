#[macro_use]
extern crate clap;
extern crate dirs;
extern crate yaml_rust;

use clap::App;

use std::fs;
use std::fs::{create_dir_all};
use std::path::Path;
use std::error::Error;
use yaml_rust::YamlLoader;

fn main() -> Result<(), Box<Error>> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
       .version(crate_version!())
       .author(crate_authors!())
       .get_matches();

    // Should never panic because Location has a default value.
    let config_dir = matches.value_of("location").expect("Location not provided!");
    let config_dir = config_dir.replace("~",
                                dirs::home_dir().expect("Could not find home dir!")
                                                .to_str().expect("Failed to stringify home dir"));
    let configs = load_or_init_config(&config_dir)?;
    println!("{:?}", configs);
    Ok(())
}

fn load_or_init_config(path: &str) -> Result<Vec<yaml_rust::Yaml>, Box<Error>> {
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
    Ok(YamlLoader::load_from_str(&config)?)
}
