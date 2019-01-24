use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use tokio::prelude::*;
use tokio::timer::Interval;

use crate::config::local::LocalConfig;
use crate::Context;

pub fn handler(context: Context) -> Result<(), Error> {
    let run_scripts = !context.matches.is_present("no_scripts");

    start_sudo_timer();

    update_arch();

    let dotfile_dir_path = context
        .local_config_path
        .clone()
        .expect("dotfile location was not specified in global config");
    let dotfile_dir_path = dotfile_dir_path
        .parent()
        .expect("Could not get access to dotfile directory");

    let local_config = context.local_config.expect("Could not load local config");
    let helper = context.global_config.helper;

    if !context.matches.is_present("bool") {
        install_group(
            dotfile_dir_path,
            &local_config,
            &helper,
            vec!["common"],
            run_scripts,
        );
    }

    if context.matches.is_present("group") {
        let group_names: Vec<_> = context.matches.values_of("group").unwrap().collect();
        install_group(
            dotfile_dir_path,
            &local_config,
            &helper,
            group_names,
            run_scripts,
        );
    }

    if context.matches.is_present("GROUPS") {
        let group_names: Vec<_> = context.matches.values_of("groups").unwrap().collect();
        install_group(
            dotfile_dir_path,
            &local_config,
            &helper,
            group_names,
            run_scripts,
        );
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
    dotfile_dir_path: &Path,
    local_config: &LocalConfig,
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
