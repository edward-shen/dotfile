use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use tokio::prelude::*;
use tokio::timer::Interval;

use crate::config::global::{load_config as load_global_config, Config as GlobalConfig};
use crate::config::local::load_config as load_local_config;

pub fn handler(
    (global_config_path, _, args): (&PathBuf, &GlobalConfig, &clap::ArgMatches),
) -> Result<(), Error> {
    let run_scripts = !args.is_present("no_scripts");
    let global_config = load_global_config(global_config_path);
    let local_config_path = global_config
        .path
        .and_then(|path| Some(PathBuf::from(path)))
        .expect("Global config does not have custom dotfile path");
    let helper = global_config.helper;

    start_sudo_timer();

    update_arch();

    if !args.is_present("bool") {
        install_group(&local_config_path, &helper, vec!["common"], run_scripts);
    }

    if args.is_present("group") {
        let group_names: Vec<_> = args.values_of("group").unwrap().collect();
        install_group(&local_config_path, &helper, group_names, run_scripts);
    }

    if args.is_present("GROUPS") {
        let group_names: Vec<_> = args.values_of("groups").unwrap().collect();
        install_group(&local_config_path, &helper, group_names, run_scripts);
    }

    Ok(())
}

fn start_sudo_timer() {
    let task = Interval::new(Instant::now(), Duration::from_secs(60))
        .for_each(|_| {
            Command::new("sudo")
                .arg("-v")
                .output()
                .expect("Could not run sudo!");
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
}

fn update_arch() {
    Command::new("sudo")
        .args(&["pacman", "-Syyu", "--noconfirm", "--needed"])
        .output()
        .expect("Could not execute sudo. Is it installed?");
}

fn install_group(
    dotfile_dir_path: &PathBuf,
    helper: &Option<String>,
    group_names: Vec<&str>,
    run_scripts: bool,
) {
    if run_scripts {
        run_script(&dotfile_dir_path.join("pre.sh"));
    }

    let mut installer = match helper {
        Some(aur_helper) => Command::new(aur_helper),
        None => {
            let mut default_helper = Command::new("sudo");
            default_helper.args(&["pacman", "-S"]);
            default_helper
        }
    };

    let local_config = load_local_config(dotfile_dir_path);

    for group_name in group_names {
        let config_group = local_config
            .groups
            .get(group_name)
            .expect("Group was not found!");
        installer
            .args(&["--noconfirm", "--needed"])
            .args(&config_group.packages)
            .output()
            .expect("Specified helper does not exist!");
    }

    // TODO: Stow dotfiles

    if run_scripts {
        run_script(&dotfile_dir_path.join("post.sh"));
    }
}

fn run_script(path: &PathBuf) {
    Command::new("sh")
        .arg(path)
        .output()
        .expect("failed to run sh!");
}
