
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::env::current_dir;

use dirs::home_dir;

pub fn handler((dotfile_config, dotfile_dir_config, args): (yaml_rust::Yaml, yaml_rust::Yaml, clap::ArgMatches)) {
    let args = args.subcommand_matches("init").unwrap();
    let path = args.value_of("PATH").expect("invalid path arg");
    let path = current_dir().expect("could not get current directory").join(path.replace("~", home_dir().unwrap().to_str().unwrap()));

    if can_init(&path) {
        println!("I can initialize the repo now at {}", path.display());
        init_repository();
    } else {
        if args.is_present("from") {
            println!("trying to adopt from stow");
        } else {
            eprintln!("Directory is not empty!");
        }
    }
}

fn can_init(path: &PathBuf) -> bool {

    if !path.exists() {
      return true;  
    } 

    match read_dir(path) {
        Ok(contents) => {
            contents.collect::<Vec<_>>().len() == 0
        }, 
        Err(e) => {
            panic!("Failed to read directory {}: {}", path.display(), e.description());
        } 
    }
}

fn init_repository() {

}