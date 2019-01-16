use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use dirs::home_dir;

use yaml_rust::YamlLoader;

pub fn load_config() -> yaml_rust::Yaml {
    // Okay to use unwrap, if a home dir isn't present we have other problems.
    let path = home_dir().unwrap().join("./.config/dotfile/config.yaml");
    let configs = match read_to_string(&path) {
        Ok(config) => config,
        Err(_e) => init_config(path)
    };

    YamlLoader::load_from_str(&configs).expect("Malformed config file!")[0].clone()
}

fn init_config(path: PathBuf) -> String {
    if !path.parent().unwrap().exists() {
        create_dir_all(path.parent().unwrap()).expect("Cannot create config directory!")
    }

    let mut file = OpenOptions::new().write(true).create_new(true).open(&path).expect("Could not create config file");
    file.write(b"helper: \npath: ~/dotfiles").expect("Could not write to file!");

    // Unwrapping should be safe here.
    read_to_string(path).unwrap()
}

// pub fn load_config(path: String) -> Result<yaml_rust::Yaml, String> {
//     let path = resolve_path(path);

//     File::open
//     let config = if path.exists() {
//         YamlLoader::load_from_str(&read_to_string(path.to_str().unwrap()).unwrap())
//     } else {
//         if path.to_str().unwrap() == resolve_path(String::from("~/.config/dotfile")).to_str().unwrap() {
//             init_config(path)
//         } else {
//             Err(String::from("Could not load config file!"))
//         }
//     };

    

//     Ok(config.unwrap_or("could not read")[0])
// }