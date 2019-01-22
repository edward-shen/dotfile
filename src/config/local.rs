use std::collections::HashMap;
use std::fs::{read_to_string, write, OpenOptions};
use std::io::{Error, Write};
use std::path::PathBuf;

use toml::from_str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    version: String,
    pub groups: HashMap<String, Group>,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub packages: Vec<String>,
}

pub fn load_config(path: &PathBuf) -> Config {
    from_str(&read_to_string(path).expect("Dotfile config not found!"))
        .expect("Malformed config file!")
}

pub fn update_config(config: Config, path: &PathBuf) {
    write_config(config, path);
}

fn write_config(config: Config, path: &PathBuf) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Could not create config file");
    file.write(toml::to_string(&config).unwrap().as_bytes())
        .expect("Could not write to file!");
}

pub fn init_config(path: &PathBuf) -> Result<(), Error> {
    let mut groups = HashMap::new();
    groups.insert(
        String::from("common"),
        Group {
            packages: Vec::new(),
        },
    );
    let config = Config {
        version: crate_version!().to_string(),
        groups,
    };

    write(path.join("dotfile.toml"), toml::to_string(&config).unwrap())
}
