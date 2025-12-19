use clap::{Args, Subcommand, command};

use crate::conf_io::Configuration;
use crate::state_io::MpregState;

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
    //Showall,
}

pub fn main(args: ConfigArgs, conf: &Configuration, state: &MpregState) {
    match args.command {
        //Some(ConfigCommands::Showall) => showall(conf),
        Some(ConfigCommands::Show { key }) => show(key, conf),
        Some(ConfigCommands::Set { key, value }) => println!("bum"),
        None => println!("bongle"),
    }
}

fn show(key: Option<String>, conf: &Configuration) {
    let Some(key) = key else {
        return;
    };
}
