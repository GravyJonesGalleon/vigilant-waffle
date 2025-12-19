use clap::{Parser, Subcommand, command};

use crate::subcmds::{config, pack};

mod conf_io;
mod state_io;
mod subcmds;

#[derive(Parser)]
#[command(name = "mpreg")]
#[command(about = "A Minecraft ModPack REGister", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    Pack(pack::PackArgs),
    Config(config::ConfigArgs),
}

fn main() {
    let configuration = match conf_io::get_config() {
        Ok(configuration) => configuration,
        Err(_) => {
            eprintln!("ERROR: Unable to get configuration!");
            return;
        }
    };
    let state = match state_io::get_state(&configuration) {
        Ok(state) => state,
        Err(_) => {
            eprintln!("ERROR: Unable to read state!");
            return;
        }
    };

    let cli = Cli::parse();
    match cli.command {
        Subcommands::Pack(args) => subcmds::pack::main(args, &configuration, &state),
        Subcommands::Config(args) => subcmds::config::main(args, &configuration, &state),
    }
}
