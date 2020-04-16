use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Install(Install),
    Uninstall(Uninstall),
}

#[derive(Clap)]
pub struct Install {
    #[clap(long = "no-pre")]
    pub no_pre_install: bool,
    #[clap(long = "no-post")]
    pub no_post_install: bool,
    pub groups_or_sets: Vec<String>,
    #[clap(short, long)]
    pub group: Option<Vec<String>>,
    #[clap(short, long)]
    pub set: Option<Vec<String>>,
    #[clap(short = "G", long)]
    pub groups: Vec<String>,
    #[clap(short = "S", long)]
    pub sets: Vec<String>,
    #[clap(short, long = "config", default_value = "dotfile.yaml")]
    pub config_path: String,
}

#[derive(Clap)]
pub struct Uninstall {
    pub groups_or_sets: Vec<String>,
    #[clap(short, long)]
    pub group: Option<Vec<String>>,
    #[clap(short, long)]
    pub set: Option<Vec<String>>,
    #[clap(short = "G", long)]
    pub groups: Vec<String>,
    #[clap(short = "S", long)]
    pub sets: Vec<String>,
    #[clap(short, long = "config", default_value = "dotfile.yaml")]
    pub config_path: String,
}
