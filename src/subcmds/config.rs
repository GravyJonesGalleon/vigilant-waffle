use clap::{Args, Subcommand, command};
use std::env;
use std::path::PathBuf;

use crate::conf_io::Configuration;

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    command: Option<ConfigCommands>,
}

#[derive(Debug, Subcommand)]
enum ConfigCommands {
    #[command(arg_required_else_help = true)]
    Set {
        key: String,
        value: String,
    },
    Show {
        key: Option<String>,
    },
}

pub fn main(args: ConfigArgs, conf: &Configuration) {
    match args.command {
        Some(ConfigCommands::Show { key }) => show(key),
        Some(ConfigCommands::Set { key, value }) => println!("bum"),
        None => println!("bongle"),
    }
}

fn show(key: Option<String>) {
    let Some(home) = env::home_dir() else {
        return ();
    };

    // TODO: make this customisable
    let config_path = home.join(PathBuf::from(format!(".mpreg/config.toml")));
}
