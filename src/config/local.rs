use std::collections::HashMap;

use std::fs::read_to_string;
use std::io::Error;
use std::path::PathBuf;

use toml::from_str;

use serde::{Deserialize, Serialize};

use crate::config::Writable;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    version: String,
    pub groups: HashMap<String, Group>,
}

impl Writable for LocalConfig {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub packages: Vec<String>,
}

pub fn load_config(path: &PathBuf) -> LocalConfig {
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
    let config = LocalConfig {
        version: crate_version!().to_string(),
        groups,
    };

    config.write_to_file(&path.join("dotfile.toml"));

    Ok(())
}
