use std::fs::{create_dir_all, read_to_string};
use std::path::PathBuf;

use toml::from_str;

use serde::{Deserialize, Serialize};

use crate::config::Writable;

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub helper: Option<String>,
    pub path: Option<String>,
}

impl Writable for GlobalConfig {}

pub fn load_config(path: &PathBuf) -> GlobalConfig {
    let configs = match read_to_string(path) {
        Ok(config) => config,
        _ => init_config(&path),
    };

    from_str(&configs).expect("Malformed config file!")
}

fn init_config(path: &PathBuf) -> String {
    let global_config_dir = path.parent().unwrap();

    if !global_config_dir.exists() {
        create_dir_all(global_config_dir).expect("Cannot create config directory!");
    }

    let config = GlobalConfig {
        helper: None,
        path: None,
    };

    config.write_to_file(path);

    // Unwrapping should be safe here.
    read_to_string(path).unwrap()
}
