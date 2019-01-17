use std::env::current_dir;
use std::error::Error;
use std::fs::{create_dir_all, read_dir, write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use dirs::home_dir;

pub fn handler((_, args): (&yaml_rust::Yaml, &clap::ArgMatches)) -> Result<(), String> {
    let args = args.subcommand_matches("init").unwrap();
    let home_dir = home_dir().expect("Could not locate home directory!");
    let home_dir = home_dir.to_str().expect("Could not stringify home path!");
    let path = args.value_of("PATH").expect("invalid path arg");
    let path = current_dir()
        .expect("could not get current directory")
        .join(path.replace("~", home_dir));

    if let Ok(can_init) = can_init(&path) {
        if can_init {
            init_repository(&path);
        } else {
            if args.is_present("stow_dir") {
                unimplemented!("adopting a stow file");
            } else {
                return Err(String::from("Specified directory is not empty!"));
            }
        }
    }

    Ok(())
}

/// Checks if `path` is either an empty directory or one that doesn't exist.
///
/// # Returns
/// * `Ok(true)` If the directory is empty or nonexistent
/// * `Ok(false)` If the directory is not empty
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
fn init_repository(path: &PathBuf) -> Result<(), String> {
    prep_dir(&path)?;
    init_dotfile_config(&path)?;
    init_vcs(&path)?;
    Ok(())
}

fn prep_dir(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        match create_dir_all(&path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Error while creating directory {}: {}",
                path.display(),
                e.description()
            )),
        }
    } else {
        Ok(())
    }
}

fn init_dotfile_config(path: &PathBuf) -> Result<(), String> {
    let init_string = format!("version: {}", crate_version!());
    if let Err(e) = write(path.join("dotfile.yaml"), &init_string) {
        Err(format!(
            "Could not write local dotfile config file! {}",
            e.description(),
        ))
    } else {
        Ok(())
    }
}

fn init_vcs(path: &PathBuf) -> Result<(), String> {
    let exit_code = Command::new("git")
        .args(&["init", path.to_str().unwrap()])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute git. Is it installed?")
        .code();

    match exit_code {
        Some(0) => (),
        Some(val) => panic!("git exited with exit code {}", val),
        None => panic!("git process terminated by signal"),
    }

    Ok(())
}
