use std::env::current_dir;
use std::error::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use dirs::home_dir;

pub fn handler(
  (dotfile_config, dotfile_dir_config, args): (yaml_rust::Yaml, yaml_rust::Yaml, clap::ArgMatches),
) {
  let args = args.subcommand_matches("init").unwrap();
  let path = args.value_of("PATH").expect("invalid path arg");
  let path = current_dir()
    .expect("could not get current directory")
    .join(path.replace("~", home_dir().unwrap().to_str().unwrap()));

  match can_init(&path) {
    Ok(true) => {
      println!("I can initialize the repo now at {}", path.display());
      init_repository();
    }
    Ok(false) => {
      if args.is_present("stow_dir") {
        println!("trying to adopt from stow");
      } else {
        println!("Specified directory is not empty!");
      }
    }
    Err(msg) => eprintln!("{}", msg),
  }
}

/// Checks if the specified path is a path we
fn can_init(path: &PathBuf) -> Result<bool, String> {
  if !path.exists() {
    return Ok(true);
  }

  if path.is_file() {
    return Err(String::from("Cannot initialize to a file!"));
  }

  match read_dir(path) {
    Ok(contents) => Ok(contents.collect::<Vec<_>>().len() == 0),
    Err(e) => Err(format!(
      "Failed to read directory {}: {}",
      path.display(),
      e.description()
    )),
  }
}

fn init_repository() {}
