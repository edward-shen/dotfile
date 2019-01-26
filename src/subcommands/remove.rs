use std::fs::read_dir;
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

use dirs::home_dir;

use crate::config::Writable;
use crate::subcommands::get_arg_err_msg;
use crate::{Context, LocalConfig};

pub fn handler(context: Context) -> Result<(), Error> {
    let args = context
        .matches
        .subcommand_matches("group")
        .expect(&get_arg_err_msg("group"));

    if let None = context.local_config {
        panic!("asdf");
    }

    let keep_config = args.is_present("keep_config");
    let groups = args.values_of("groups").expect("clap-rs gave 0 groups");
    let groups: Vec<_> = groups.collect();

    let mut local_config = context.local_config.unwrap();
    let local_config_path = context.local_config_path.unwrap();

    if !keep_config {
        match args.values_of("config") {
            Some(paths) => {
                let paths: Vec<_> = paths.collect();
                uninstall_configs(&mut local_config, &local_config_path, &groups, &paths);
            }
            _ => (),
        }
    }

    match args.values_of("PKGS") {
        Some(packages) => {
            let packages: Vec<_> = packages.collect();
            uninstall_packages(&mut local_config, &local_config_path, &groups, &packages);
        }
        _ => (),
    };

    Ok(())
}

fn uninstall_configs(
    local_config: &mut LocalConfig,
    local_config_path: &PathBuf,
    groups: &Vec<&str>,
    configs: &Vec<&str>,
) {
    for group in groups {
        let group_name = &String::from(*group);
        match local_config.groups.get_mut(group_name) {
            Some(_) => {
                for config in configs {
                    let config_name = &String::from(*config);
                    let config_path = local_config_path
                        .parent()
                        .unwrap()
                        .join(group_name)
                        .join(config_name);
                    match read_dir(config_path) {
                        Ok(_) => {
                            let home = home_dir().unwrap();
                            let home = home.to_str().unwrap();
                            Command::new("stow")
                                .current_dir(local_config_path.join(group_name))
                                .args(&["-D", config_name, "-t", home])
                                .output()
                                .expect("Failed to execute stow! Is it installed?");
                        }
                        Err(_) => println!(
                            "warning: config {} not found for group {}",
                            config_name, group_name
                        ),
                    }
                }
            }
            None => println!("warning: group {} does not exist", group_name),
        }
    }

    local_config.write_to_file(local_config_path);
}

fn uninstall_packages(
    local_config: &mut LocalConfig,
    local_config_path: &PathBuf,
    groups: &Vec<&str>,
    packages: &Vec<&str>,
) {
    for group in groups {
        let group_name = &String::from(*group);
        match local_config.groups.get_mut(group_name) {
            Some(group) => {
                for package in packages {
                    let package_name = String::from(*package);
                    match group.packages.iter().position(|x| *x == package_name) {
                        Some(index) => {
                            group.packages.remove(index);
                        }
                        None => println!(
                            "warning: {} not found in group {}, ignoring",
                            package_name, group_name
                        ),
                    }
                }
            }
            None => println!("warning: group {} does not exist, ignoring", group_name),
        }
    }

    local_config.write_to_file(local_config_path);
}
