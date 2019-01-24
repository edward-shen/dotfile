use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::config::global::load_config as load_global_config;
use crate::config::global::Config as GlobalConfig;
use crate::config::local::{load_config as load_local_config, Group};
use crate::config::Writable;

pub fn handler(
    (global_config_path, _, args): (&PathBuf, &GlobalConfig, &clap::ArgMatches),
) -> Result<(), Error> {
    let args = args
        .subcommand_matches("group")
        .expect("clap misparsed subcommand!");

    let local_config_path = load_global_config(global_config_path)
        .path
        .and_then(|path| Some(PathBuf::from(path)));

    if local_config_path.is_none() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Global config does not have path set, was init called?",
        ));
    }

    let local_config_path = local_config_path.unwrap();

    if args.is_present("rename") {
        rename_group(&local_config_path, args.values_of("rename").unwrap());
    }

    if args.is_present("new_group") {
        create_groups(&local_config_path, args.values_of("new_group").unwrap());
    }

    if args.is_present("to_delete") {
        delete_groups(&local_config_path, args.values_of("to_delete").unwrap());
    }

    Ok(())
}

fn create_groups(local_config_path: &PathBuf, args: clap::Values) {
    let mut local_config = load_local_config(local_config_path);
    let args: Vec<_> = args.collect();

    for group_name in args {
        local_config
            .groups
            .insert(String::from(group_name), Group { packages: vec![] });
    }

    local_config.write_to_file(local_config_path);
}

fn delete_groups(local_config_path: &PathBuf, args: clap::Values) {
    let mut local_config = load_local_config(local_config_path);
    let args: Vec<_> = args.collect();

    for group_name in args {
        local_config.groups.remove(&String::from(group_name));
    }

    local_config.write_to_file(local_config_path);
}

fn rename_group(local_config_path: &PathBuf, args: clap::Values) {
    let mut local_config = load_local_config(local_config_path);
    let mut args: Vec<_> = args.collect();

    let new_group_name = String::from(args.pop().unwrap());
    let old_group_name = String::from(args.pop().unwrap());

    let group_to_move = local_config.groups.remove(&old_group_name).unwrap();
    local_config.groups.insert(new_group_name, group_to_move);

    local_config.write_to_file(local_config_path);
}
