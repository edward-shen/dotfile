use clap::derive::Clap;
use cli::*;
use config::{Config, Set};
use error::DotfileError;
use serde_yaml::from_str;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

mod cli;
mod config;
mod error;

fn main() -> Result<(), DotfileError> {
    match Opts::parse().sub_command {
        SubCommand::Install(args) => install(args),
        SubCommand::Uninstall(args) => uninstall(args),
    }
}

fn install(args: Install) -> Result<(), DotfileError> {
    let config = load_config(&args.config_path)?;
    dbg!(args.group);

    Ok(())
}

fn uninstall(args: Uninstall) -> Result<(), DotfileError> {
    let config = load_config(&args.config_path)?;
    let mut sets = args.sets;
    sets.extend(args.set.unwrap_or_default());
    let mut groups = args.groups;
    groups.extend(args.group.unwrap_or_default());
    let (groups, sets) = get_groups_and_sets(&config, groups, sets, args.groups_or_sets)?;
    let packages = get_packages(&config, groups, sets);

    Ok(())
}

fn get_packages(config: &Config, groups: HashSet<String>, sets: HashSet<String>) -> HashSet<&str> {
    let mut packages = HashSet::new();

    let mut groups = groups;
    // groups.extend(sets.iter().flat_map(|set| {
    //     resolve_groups_from_sets(
    //         config,
    //         config
    //             .sets
    //             .and_then(|sets| Some(sets.get(set).unwrap()))
    //             .unwrap(),
    //         vec![],
    //     )
    // }));

    packages.extend(groups.iter().flat_map(|group| {
        let groups = config.groups.as_ref().unwrap();
        groups.get(group).unwrap().split_whitespace()
    }));

    packages
}

fn resolve_groups_from_sets(config: &Config, set: &Set, acc: &mut Vec<String>) -> Vec<String> {
    acc.extend(set.groups.unwrap_or_default());
    vec![]
}

fn load_config(path: &str) -> Result<Config, DotfileError> {
    Ok(from_str(&read_to_string(path)?)?)
}

fn get_groups_and_sets(
    config: &Config,
    arg_groups: Vec<String>,
    arg_sets: Vec<String>,
    arg_groups_or_sets: Vec<String>,
) -> Result<(HashSet<String>, HashSet<String>), DotfileError> {
    let (mut groups, mut sets, ambiguous_items, mut unknown_items) =
        parse_groups_or_sets(&config, arg_groups_or_sets)?;

    config.groups.as_ref().and_then(|config_groups| {
        classify(arg_groups, config_groups, &mut groups, &mut unknown_items);
        Some(config_groups)
    });

    config.sets.as_ref().and_then(|config_sets| {
        classify(arg_sets, &config_sets, &mut sets, &mut unknown_items);
        Some(config_sets)
    });

    if !ambiguous_items.is_empty() || !unknown_items.is_empty() {
        Err(DotfileError::AmbiguousOrUnknownItems(
            ambiguous_items,
            unknown_items,
        ))
    } else {
        Ok((groups, sets))
    }
}

fn parse_groups_or_sets(
    config: &Config,
    groups_or_sets: Vec<String>,
) -> Result<
    (
        HashSet<String>,
        HashSet<String>,
        HashSet<String>,
        HashSet<String>,
    ),
    DotfileError,
> {
    let mut groups = HashSet::new();
    let mut sets = HashSet::new();
    let mut ambiguous_items = HashSet::new();
    let mut unknown_items = HashSet::new();
    for to_resolve in groups_or_sets {
        let found_set = config.sets.as_ref().and_then(|sets| sets.get(&to_resolve));
        let found_group = config
            .groups
            .as_ref()
            .and_then(|sets| sets.get(&to_resolve));

        match (found_set, found_group) {
            (None, None) => unknown_items.insert(to_resolve),
            (None, Some(_)) => groups.insert(to_resolve),
            (Some(_), None) => sets.insert(to_resolve),
            (Some(_), Some(_)) => ambiguous_items.insert(to_resolve),
        };
    }

    Ok((groups, sets, ambiguous_items, unknown_items))
}

fn classify<T>(
    items: Vec<String>,
    positive_items: &HashMap<String, T>,
    positive_set: &mut HashSet<String>,
    unknown_set: &mut HashSet<String>,
) {
    for item in items {
        if positive_items.get(&item).is_some() {
            positive_set.insert(item);
        } else {
            unknown_set.insert(item);
        }
    }
}
