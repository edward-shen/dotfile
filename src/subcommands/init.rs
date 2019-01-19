use std::env::current_dir;
use std::fs::{create_dir_all, read_dir, write};
use std::io::{stdin, stdout, Error, ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use dirs::home_dir;

pub fn handler((_, args): (&yaml_rust::Yaml, &clap::ArgMatches)) -> Result<(), Error> {
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
            adopt_repository(&path)
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

fn prep_dir(path: &PathBuf) -> Result<(), Error> {
    if !path.exists() {
        create_dir_all(&path)
    } else {
        Ok(())
    }
}

fn init_dotfile_config(path: &PathBuf) -> Result<(), Error> {
    let init_string = format!("version: {}", crate_version!());
    write(path.join("dotfile.yaml"), &init_string)
}

fn init_vcs(path: &PathBuf) -> Result<(), Error> {
    let exit_code = Command::new("git")
        .args(&["init", path.to_str().unwrap()])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute git. Is it installed?")
        .code();

    match exit_code {
        Some(0) => Ok(()),
        Some(val) => panic!("git exited with exit code {}", val),
        None => panic!("git process terminated by signal"),
    }
}

fn adopt_repository(path: &PathBuf) -> Result<(), Error> {
    let should_continue = get_confirmation();

    if !should_continue {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "User denied permission to modify directory!",
        ));
    }
    Ok(())
}

fn get_confirmation() -> bool {
    let mut input = String::new();

    while input != "y" && input != "n" {
        input.clear();

        print!("Warning: this option will modify the contents of the directory. Proceed? [y/N] ");
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
