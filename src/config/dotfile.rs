use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use toml::from_str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub helper: Option<String>,
    pub path: Option<String>,
}

pub fn load_config(path: &PathBuf) -> Config {
    let configs = match read_to_string(path) {
        Ok(config) => config,
        _ => init_config(&path),
    };

    from_str(&configs).expect("Malformed config file!")
}

fn init_config(path: &PathBuf) -> String {
    if !path.parent().unwrap().exists() {
        create_dir_all(path.parent().unwrap()).expect("Cannot create config directory!");
    }

    let config = Config {
        helper: None,
        path: None,
    };

    write_config(config, path);

    // Unwrapping should be safe here.
    read_to_string(path).unwrap()
}

pub fn update_config(config: Config, path: &PathBuf) {
    let mut new_config = load_config(path);

    if config.helper.is_some() {
        new_config.helper = config.helper;
    }

    if config.path.is_some() {
        new_config.path = config.path;
    }

    write_config(new_config, path);
}

pub fn write_config(config: Config, path: &PathBuf) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Could not create config file");
    file.write(toml::to_string(&config).unwrap().as_bytes())
        .expect("Could not write to file!");
}
