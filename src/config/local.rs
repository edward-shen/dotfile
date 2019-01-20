use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::io::Error;
use std::path::PathBuf;

use toml::from_str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    version: String,
    groups: HashMap<String, Group>,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    packages: Vec<String>,
}

pub fn load_config(path: &PathBuf) -> Config {
    from_str(&read_to_string(path).expect("Dotfile config not found!"))
        .expect("Malformed config file!")
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
