use clap::{Parser, Subcommand, command};

use crate::subcmds::{config, pack};

mod conf_io;
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
    #[command(arg_required_else_help = true)]
    Pack(pack::PackArgs),
    Config(config::ConfigArgs),
}

fn main() {
    let configuration = match conf_io::get_config() {
        Ok(configuration) => configuration,
        Err(_) => {
            println!("Unable to get configuration!");
            return;
        }
    };

    let cli = Cli::parse();
    match cli.command {
        Subcommands::Pack(args) => subcmds::pack::main(args, &configuration),
        Subcommands::Config(args) => subcmds::config::main(args, &configuration),
    }
}
