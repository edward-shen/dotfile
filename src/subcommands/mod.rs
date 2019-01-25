pub mod add;
pub mod group;
pub mod init;
pub mod install;
pub mod remove;
pub mod use_cmd; // use is a keyword

pub fn get_arg_err_msg(subcommand: &str) -> String {
    format!(
        "Invariant violated: {} handler was provided with non-{} subcommand",
        subcommand, subcommand
    )
}
