use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{create_dir, create_dir_all, read_dir, rename, write, DirEntry};
use std::io::{stdin, stdout, Error, ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use dirs::home_dir;

use super::super::config::dotfile::Config as GlobalConfig;
use super::super::config::local::{Config as LocalConfig, Group};

const COMMON_DIR: &'static str = "common";

pub fn handler((_, args): (&GlobalConfig, &clap::ArgMatches)) -> Result<(), Error> {
    let args = args
        .subcommand_matches("init")
        .expect("Clap-rs gave us incorrect subcommand!");
    let home_dir = home_dir().expect("Could not locate home directory!");
    let home_dir = home_dir.to_str().expect("Could not stringify home path!");
    let path = args.value_of("PATH").expect("invalid path arg");
    let path = current_dir()
        .expect("could not get current directory")
        .join(path.replace("~", home_dir));

    if can_init(&path)? {
        init_repository(&path)
    } else {
        if args.is_present("stow_dir") {
            adopt_repository(&path, &args)
        } else {
            Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("{}: Directory is not empty.", path.display()),
            ))
        }
    }
}

/// Checks if `path` is either an empty directory or one that doesn't exist.
fn can_init(path: &PathBuf) -> Result<bool, Error> {
    if !path.exists() {
        return Ok(true);
    }

    read_dir(path).and_then(|dir| Ok(dir.collect::<Vec<_>>().len() == 0))
}

/// Preforms all steps required to initialize a dotfile repository.
fn init_repository(path: &PathBuf) -> Result<(), Error> {
    prep_dir(&path)?;
    init_dotfile_config(&path)?;
    init_vcs(&path)?;
    Ok(())
}

/// Creates the directory and/or the common directory if it doesn't exist.
fn prep_dir(path: &PathBuf) -> Result<(), Error> {
    if !path.exists() {
        create_dir_all(&path.join(COMMON_DIR))
    } else {
        Ok(())
    }
}

fn init_dotfile_config(path: &PathBuf) -> Result<(), Error> {
    let groups = HashMap::new();
    groups.insert(
        "common",
        Group {
            packages: Vec::new(),
        },
    );
    let config = LocalConfig {
        version: crate_version!().to_string(),
        groups,
    };

    write(path.join("dotfile.toml"), toml::to_string(&config).unwrap())
}

fn init_vcs(path: &PathBuf) -> Result<(), Error> {
    let exit_code = Command::new("git")
        .current_dir(path)
        .arg("init")
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute git. Is it installed?")
        .code();

    match exit_code {
        Some(0) => (),
        Some(val) => panic!("git exited with exit code {}", val),
        None => panic!("git process terminated by signal"),
    };

    Command::new("git")
        .current_dir(path)
        .args(&["add", "."])
        .stdout(Stdio::null())
        .output()
        .expect("Error while adding all directories to git.");

    Command::new("git")
        .current_dir(path)
        .args(&["commit", "-m", "Initial commit"])
        .stdout(Stdio::null())
        .output()
        .expect("Failed to commit all files");

    Ok(())
}

/// Converts a standard stow repository to a dotfile repository by initalizing
/// a dotfile config file and moving all non-hidden folders to a common
/// subfolder.
fn adopt_repository(path: &PathBuf, args: &clap::ArgMatches) -> Result<(), Error> {
    if !get_confirmation(path) {
        return Err(Error::new(
            ErrorKind::PermissionDenied, // I mean this is correct, but probably a bad idea later
            "User denied permission to modify directory!",
        ));
    }

    let common_dir_path = path.join(COMMON_DIR);
    create_dir(&common_dir_path).expect("Could not create folder \"common\" in folder to adopt.");

    let ignore_list: Vec<&str> = match args.values_of("ignore") {
        Some(e) => e.collect(),
        _ => vec![],
    };

    let path_iter = path
        .read_dir()
        .expect("Could not read directory")
        .filter_map(|ele| {
            let ele = ele.unwrap();
            if ignore_list.contains(&ele.file_name().to_str().unwrap()) {
                None
            } else {
                Some(ele)
            }
        });

    for item in path_iter {
        if can_move(&item, &common_dir_path) {
            stow_move(&item, &common_dir_path);
        }
    }

    init_dotfile_config(&path)
}

/// Gets confirmation for us to mutate the user directory. Will continuously ask
/// for input until user explicitly gives yes or no. If no input was provided,
/// input is defaulted to no.
fn get_confirmation(path: &PathBuf) -> bool {
    let mut input = String::new();

    while input != "y" && input != "n" {
        input.clear();

        print!(
            "Warning: this option will modify the contents of {}. Proceed? [y/N] ",
            path.display()
        );
        // Stdout is line buffered by default, need to flush for it to be printed.
        stdout().flush().expect("Could not flush stdout!");

        stdin()
            .read_line(&mut input)
            .expect("Could not read from input!");

        input = input.trim_end().to_ascii_lowercase();

        // Default to no
        if input.is_empty() {
            input = String::from("n");
        }
    }

    input == "y"
}

/// Checks if the item is not equal to the target directory, if its a directory
/// (versus a regular file), and if it's not a hidden file. Returns `true` if
/// all cases are true.
fn can_move(item: &DirEntry, common_path: &PathBuf) -> bool {
    &item.path() != common_path
        && item.file_type().expect("Could not get filetype!").is_dir()
        && item
            .file_name()
            .to_str()
            .expect("Could not stringify filename")
            .get(0..1)
            .unwrap_or_else(|| panic!("Got a 0 length filename?!"))
            != "."
}

/// Unstows all dotfiles from the home directory and restows them after moving
/// them to the common directory.
fn stow_move(src: &DirEntry, dest: &PathBuf) {
    let dir_name = src.file_name();
    let dir_name = dir_name.to_str().expect("");

    Command::new("stow")
        .current_dir(dest.parent().unwrap())
        .args(&["-D", dir_name])
        .output()
        .expect("Failed to execute stow! Is it installed?");

    let dest_path = &dest.join(src.file_name());
    rename(src.path(), dest_path).expect("Failed to move folder!");

    Command::new("stow")
        .current_dir(dest)
        .args(&["-t", home_dir().unwrap().to_str().unwrap(), "-S", dir_name])
        .output()
        .expect("Could not execute stow!");
}
