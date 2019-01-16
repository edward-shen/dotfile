use std::env::current_dir;
use std::error::Error;
use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use dirs::home_dir;

pub fn handler((_, _, args): (yaml_rust::Yaml, yaml_rust::Yaml, clap::ArgMatches)) {
  let args = args.subcommand_matches("init").unwrap();
  let path = args.value_of("PATH").expect("invalid path arg");
  let path = current_dir()
    .expect("could not get current directory")
    .join(path.replace("~", home_dir().unwrap().to_str().unwrap()));

  match can_init(&path) {
    Ok(true) => init_repository(path),
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

/// Checks if `path` is either an empty directory or one that doesn't exist.
///
/// # Returns
/// * `Ok(true)` If the directory is empty or nonexistent
/// * `Ok(false)` If the directory is not empty
/// * `Err` If the provided path is a file or there was an issue accessing the
/// directory
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

/// Preforms all steps required to initialize a dotfile repository.
fn init_repository(path: PathBuf) {
  prep_dir(&path);
  init_dotfile_config(&path);
  init_vcs(&path);
}

fn prep_dir(path: &PathBuf) {
  if !path.exists() {
    if let Err(e) = create_dir_all(&path) {
      eprintln!(
        "Error while creating directory {}: {}",
        path.display(),
        e.description()
      )
    }
  }
}

fn init_vcs(path: &PathBuf) {
  let exit_code = Command::new("git")
    .args(&["init", path.to_str().unwrap()])
    .stdout(Stdio::null())
    .status()
    .expect("Failed to execute git. Is it installed?")
    .code();

  match exit_code {
    Some(0) => (),
    Some(val) => eprintln!("git exited with exit code {}", val),
    None => eprintln!("git process terminated by signal"),
  }
}

fn init_dotfile_config(path: &PathBuf) {}
