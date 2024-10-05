mod commands;
mod config;
mod constants;
mod util;

use clap::Parser;

use commands::commands_enum::Commands;
use commands::commands_impl::{init, new, none};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Init {}) => init(),
        Some(Commands::New { bucket: _ }) => new(),
        None => none(),
    }
}
