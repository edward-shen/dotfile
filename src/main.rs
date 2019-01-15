#[macro_use]
extern crate clap;
extern crate dirs;
use clap::App;

use std::fs;
use std::path::Path;
fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
       .version(crate_version!())
       .author(crate_authors!())
       .get_matches();

    // Should never panic because Location has a default value.
    let config_dir = matches.value_of("location").expect("Location not provided!");
    let config_dir = config_dir.replace("~",
                                dirs::home_dir().expect("Could not find home dir!")
                                                .to_str().expect("Failed to stringify home dir"));
    init_dotfile_dir(&config_dir);
}

fn init_dotfile_dir(path: &str) {
    let config_path = path.to_owned() + "/dotfile.yaml";
    let resolved_path = Path::new(&config_path).to_str().unwrap();
    let config = match fs::read_to_string(&resolved_path) {
        Ok(conf) => conf,
        Err(_e) => {
            println!("Generating new configuration file at {}", resolved_path);
            let init_string = format!("version: {}", crate_version!());
            fs::write(resolved_path, &init_string).expect("Could not write to config file!");
            init_string
        }
    };
    println!("{}", config);
}
