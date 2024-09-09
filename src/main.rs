use clap::{CommandFactory, Parser, Subcommand};
use std::fs;
use std::{io::Error, path::PathBuf};

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
                match create_file(current_dir) {
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

fn create_file(directory_path: PathBuf) -> Result<(), Error> {
    let directory_name = directory_path.join(".ksync");
    fs::create_dir(directory_name)?;
    Ok(())
}
