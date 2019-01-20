use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use dirs::config_dir;

use toml::from_str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    helper: Option<String>,
    path: Option<String>,
}

pub fn load_config() -> Config {
    // Okay to use unwrap, if a home dir isn't present we have other problems.
    let path = config_dir().unwrap().join("./dotfile/config.toml");
    let configs = match read_to_string(&path) {
        Ok(config) => config,
        Err(_e) => init_config(path),
    };

    from_str(&configs).expect("Malformed config file!")
}

fn init_config(path: PathBuf) -> String {
    if !path.parent().unwrap().exists() {
        create_dir_all(path.parent().unwrap()).expect("Cannot create config directory!");
    }

    let config = Config {
        helper: None,
        path: None,
    };

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .expect("Could not create config file");
    file.write(toml::to_string(&config).unwrap().as_bytes())
        .expect("Could not write to file!");

    // Unwrapping should be safe here.
    read_to_string(path).unwrap()
}
