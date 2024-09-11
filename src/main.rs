use clap::{CommandFactory, Parser, Subcommand};
use std::fs;

const INIT_DIR_NAME: &str = ".ksync";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init {},
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Init {}) => {
            if let Ok(current_dir) = std::env::current_dir() {
                match fs::create_dir(current_dir.join(INIT_DIR_NAME)) {
                    Ok(()) => println!("ksync directory initiated"),
                    Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                        eprintln!("ksync already initialized")
                    }
                    Err(e) => eprintln!("error creating init directory: {}", e),
                };
            }
        }
        None => {
            Args::command().print_help().unwrap();
            println!();
        }
    }
}
