use clap::{CommandFactory, Parser, Subcommand};
use config::Configurations;
use std::fs;

mod config;

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
    New { bucket: String },
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
        Some(Commands::New { bucket }) => {
            if let Ok(cfgs) = initalized() {
                println!("{bucket} passed with {:?}.", cfgs);
            } else {
                println!("error: ksync not initalized.")
            }
        }
        None => {
            Args::command().print_help().unwrap();
            println!();
        }
    }
}

/// Check for .ksync directory.
fn initalized() -> Result<Configurations, std::io::Error> {
    fs::read_dir(INIT_DIR_NAME)?;
    Ok(Configurations::default())
}
