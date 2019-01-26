use std::io::Error;

use std::process::Command;

use crate::subcommands::get_arg_err_msg;
use crate::{Context, LocalConfig};

pub fn handler(context: Context) -> Result<(), Error> {
    let args = context
        .matches
        .subcommand_matches("group")
        .expect(&get_arg_err_msg("group"));

    if !args.is_present("no_config") {
        println!("ASDF");
    }

    let groups: Vec<_> = args.values_of("groups").unwrap().collect();

    let mut packages: Vec<_> = match args.values_of("PKGS") {
        Some(pkgs) => pkgs.collect(),
        None => vec![],
    };

    match args.values_of("PKG") {
        Some(pkg) => packages.append(&mut pkg.collect::<Vec<&str>>()),
        None => (),
    }

    let helper = verify_helper(context.global_config.helper);

    verify_packages(helper, &mut packages);

    add_packages(context.local_config, groups, packages);
    Ok(())
}

fn verify_helper(helper: Option<String>) -> String {
    match helper {
        Some(helper) => helper,
        None => {
            println!("Warn: AUR helper not declared to dotfile. Package verification may fail. Please see dotfile use.");
            String::from("pacman")
        }
    }
}

fn verify_packages(helper: String, packages: &mut Vec<&str>) {
    for package in packages {
        // Command::new(helper).args(&["-Qi", package]).spawn().
    }
}

fn add_packages(context: Option<LocalConfig>, groups: Vec<&str>, packages: Vec<&str>) {}
