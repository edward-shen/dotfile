use std::io::Error;
use std::path::PathBuf;

use crate::config::dotfile::Config as GlobalConfig;

pub fn handler(
  (global_config_path, _, args): (&PathBuf, &GlobalConfig, &clap::ArgMatches),
) -> Result<(), Error> {
  println!("{:?}", args);

  let run_scripts = !args.is_present("no_scripts");

  let local_path = PathBuf::new(); // TODO: Load from global_config_path

  if !args.is_present("bool") {
    install_group(&local_path, "common", run_scripts);
  }

  if args.is_present("group") {
    for group_name in args.values_of("group").unwrap() {
      install_group(&local_path, group_name, false);
    }
  }

  if args.is_present("GROUPS") {
    for group_name in args.values_of("GROUPS").unwrap() {
      install_group(&local_path, group_name, run_scripts);
    }
  }

  Ok(())
}

fn install_group(dotfile_dir_path: &PathBuf, group_name: &str, run_scripts: bool) {
  if run_scripts {
    run_script(&dotfile_dir_path.join("pre.sh"));
  }

  println!(
    "Installing packages from {} from {:?}. Scripts: {}",
    group_name, dotfile_dir_path, run_scripts
  );

  if run_scripts {
    run_script(&dotfile_dir_path.join("post.sh"));
  }
}

fn run_script(path: &PathBuf) {}
